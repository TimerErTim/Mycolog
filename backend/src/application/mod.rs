use std::future::Future;

use tokio_util::task::TaskTracker;
use tracing_log::log::info;

pub use backups::start_backup_task;
pub use context::MycologContext;

use crate::application::result::MycologResult;
use crate::utils::asynchronous::run_catch;

mod backups;
mod context;
mod database;
mod email;
mod result;
mod web;

pub async fn run_application(state: &mut MycologContext) -> i32 {
    info!("Application running now...");
    Ok(());

    let application_result = run_catch(try_start_application(state)).await;
    if let Err(err) = application_result {
        return MycologResult {
            exit_code: 1,
            error: Some(err),
        };
    }

    tokio::select! {
        _ = tokio::signal::ctrl_c() => 0,
        message = state.exit_receiver.recv() => if let Some((exit_code, error)) = message {
            exit_code
        } else {
            1
        }
    };
}

pub async fn try_start_application(state: &mut MycologContext) -> anyhow::Result<()> {
    let tasks = TaskTracker::new();

    tasks.spawn(start_backup_task(state));

    Ok(())
}
