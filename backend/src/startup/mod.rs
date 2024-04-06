use std::process::exit;
use std::sync::Arc;

use anyhow::anyhow;
use tokio::sync::Mutex;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::util::SubscriberInitExt;

use crate::application::{create_database_system, load_schedule_queries};
use crate::cli::MycologArguments;
use crate::config::parse_config;
use crate::context::MycologContext;
use crate::secrets::parse_secrets;
use crate::shutdown::exit::take_exit_recevier;
use crate::startup::directories::prepare_application_dirs;
use crate::startup::logging::setup_logging;
use crate::utils::asynchronous::run_catch;

mod directories;
mod logging;

#[instrument]
pub async fn startup(arguments: MycologArguments) -> Arc<MycologContext> {
    info!("Starting up application...");
    // - Check for database import
    // - Sync images with database content (remove unregistered, remove invalid db entries)
    // - Start backup task

    match run_catch(async move { try_startup(arguments).await }).await {
        Ok(context) => {
            debug!("Successfully prepared context");
            Arc::new(context)
        }
        Err(err) => {
            error!(%err, "Expierenced fatal error during startup");
            exit(-1);
        }
    }
}

async fn try_startup(arguments: MycologArguments) -> anyhow::Result<MycologContext> {
    setup_logging()?;
    prepare_application_dirs()?;

    build_context(arguments).await
}

#[instrument]
async fn build_context(arguments: MycologArguments) -> anyhow::Result<MycologContext> {
    let config = parse_config(arguments);
    let secrets = parse_secrets();
    let db = create_database_system(&config, &secrets).await?;
    let schedules = load_schedule_queries("schedules/").await?;

    let exit_receiver =
        Mutex::new(take_exit_recevier().ok_or(anyhow!("exit receiver was already in use"))?);

    Ok(MycologContext {
        config,
        secrets,
        exit_receiver,
        db,
        schedules,
        tasks: Default::default(),
        task_cancel_token: Default::default(),
    })
}
