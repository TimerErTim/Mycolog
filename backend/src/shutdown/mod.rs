use std::sync::Mutex;

use lazy_static::lazy_static;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tracing::error;

use crate::application::MycologContext;

pub type ExitMessage = i32;

lazy_static! {
    static ref EXIT_CHANNEL: (Sender<ExitMessage>, Mutex<Option<Receiver<ExitMessage>>>) = {
        let (send, recv) = tokio::sync::mpsc::channel(1);
        (send, Mutex::new(Some(recv)))
    };
}

pub async fn init_exit(error_code: i32) -> anyhow::Result<()> {
    EXIT_CHANNEL.0.send(error_code).await?;
    Ok(())
}

pub fn take_exit_recevier() -> Option<Receiver<ExitMessage>> {
    let mut receiver_guard = EXIT_CHANNEL.1.lock().ok()?;
    receiver_guard.take()
}

pub async fn shutdown(state: MycologContext) -> i32 {
    let shutdown_result = try_shutdown(state).await;

    match shutdown_result {
        Ok(_) => 0,
        Err(err) => {
            error!("Expierenced error during graceful shutdown: {err}");
            1
        }
    }
}

pub async fn try_shutdown(state: MycologContext) -> anyhow::Result<()> {
    state.task_cancel_token.cancel();
    state.tasks.wait().await;

    Ok(())
}
