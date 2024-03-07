use std::sync::Arc;

use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::shutdown::ExitMessage;

pub struct MycologContext {
    pub db: Arc<Surreal<Any>>,

    pub tasks: TaskTracker,

    pub exit_receiver: Receiver<ExitMessage>,
    pub task_cancel_token: CancellationToken,
}
