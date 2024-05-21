use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::application::{DatabaseSystem, EmailManager, ImageManager, ScheduleQueries};
use crate::config::MycologConfig;
use crate::secrets::MycologSecrets;
use crate::shutdown::exit::ExitMessage;
use crate::startup::logging::LoggingHandle;

pub struct MycologContext {
    pub config: MycologConfig,
    pub secrets: MycologSecrets,

    pub db: DatabaseSystem,
    pub email: EmailManager,
    pub images: ImageManager,
    pub schedules: ScheduleQueries,

    pub logging: LoggingHandle,

    pub tasks: TaskTracker,

    pub exit_receiver: Mutex<Receiver<ExitMessage>>,
    pub task_cancel_token: CancellationToken,
}
