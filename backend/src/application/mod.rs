use std::future::Future;
use std::sync::Arc;

use tokio_util::task::TaskTracker;
use tracing_log::log::info;

use backups::backup_task;
pub use backups::BackupLimit;
pub use database::create_database_system;
pub use database::DatabaseSystem;

use crate::context::MycologContext;
use crate::utils::asynchronous::run_catch;

mod backups;
mod database;
mod email;
mod web;

pub async fn run_application(state: &Arc<MycologContext>) -> i32 {
    info!("Application running now...");

    let application_result = run_catch(try_start_application(Arc::clone(state))).await;
    if let Err(err) = application_result {
        return 1;
    }

    let mut exit_receiver = state.exit_receiver.lock().await;
    tokio::select! {
        _ = tokio::signal::ctrl_c() => 0,
        message = exit_receiver.recv() => if let Some(exit_code) = message {
            exit_code
        } else {
            1
        }
    }
}

pub async fn try_start_application(context: Arc<MycologContext>) -> anyhow::Result<()> {
    let tasks = &context.tasks;

    tasks.spawn(backup_task(Arc::clone(&context)));
    info!("started backup service");

    Ok(())
}
