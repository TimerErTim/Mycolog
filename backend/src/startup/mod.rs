use std::process::exit;
use std::sync::Arc;

use image::codecs::png::CompressionType::Default;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::util::SubscriberInitExt;

use crate::application::{start_backup_task, MycologContext};
use crate::cli::MycologArguments;
use crate::shutdown::take_exit_recevier;
use crate::startup::directories::prepare_application_dirs;
use crate::startup::logging::setup_logging;
use crate::utils::asynchronous::run_catch;

mod database;
mod directories;
mod logging;

#[instrument]
pub async fn startup(arguments: MycologArguments) -> MycologContext {
    info!("Starting up application...");
    // - Check for database import
    // - Sync images with database content (remove unregistered, remove invalid db entries)
    // - Start backup task

    let startup_result = run_catch(async move { try_startup(arguments).await }).await;
    if let Some(context) = startup_result {
        debug!("Successfully prepared context");
        context
    } else {
        error!("Expierenced fatal error during startup: {err}");
        exit(-1);
    }
}

async fn try_startup(arguments: MycologArguments) -> anyhow::Result<MycologContext> {
    setup_logging()?;
    prepare_application_dirs()?;

    build_context().await
}

async fn build_context() -> anyhow::Result<MycologContext> {
    let shutdown_token = CancellationToken::new();
    let exit_receiver =
        take_exit_recevier().ok_or_else(error!("Exit receiver was already in use"))?;
    let surreal = Arc::new(create_global_database_instance()?);
    let backup_task = start_backup_task(Arc::clone(&surreal), shutdown_token.clone());

    Ok(MycologContext {
        db: surreal,
        tasks: Default::default(),
        exit_receiver,
        backup_task,

        shutdown_token,
        task_cancel_token: Default::default(),
    })
}
