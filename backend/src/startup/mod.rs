use std::fs::File;
use std::process::exit;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use tokio::sync::Mutex as AsyncMutex;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::util::SubscriberInitExt;

use crate::application::{
    create_database_system, create_email_manager, create_image_manager, load_schedule_queries,
    EmailManager,
};
use crate::cli::MycologArguments;
use crate::config::parse_config;
use crate::context::MycologContext;
use crate::secrets::parse_secrets;
use crate::shutdown::exit::take_exit_recevier;
use crate::startup::directories::prepare_application_dirs;
use crate::startup::logging::{setup_logging, LoggingHandle};
use crate::utils::asynchronous::run_catch;

mod directories;
pub mod logging;

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
    let logging = setup_logging()?;
    prepare_application_dirs()?;

    build_context(arguments, logging).await
}

#[instrument(skip(logging))]
async fn build_context(
    arguments: MycologArguments,
    logging: LoggingHandle,
) -> anyhow::Result<MycologContext> {
    let config = parse_config(arguments);
    let secrets = parse_secrets();
    let db = create_database_system(&config, &secrets).await?;
    let email = create_email_manager(&config, &secrets, &db).await?;
    let images = create_image_manager(&config, &secrets, &db).await?;
    let schedules = load_schedule_queries("schedules/").await?;

    let exit_receiver =
        AsyncMutex::new(take_exit_recevier().ok_or(anyhow!("exit receiver was already in use"))?);

    Ok(MycologContext {
        config,
        secrets,
        exit_receiver,
        db,
        email,
        images,
        schedules,
        logging,
        tasks: Default::default(),
        task_cancel_token: Default::default(),
    })
}
