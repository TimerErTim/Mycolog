use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;

use futures_lite::stream::StreamExt;
use tracing::Instrument;

pub use crate::application::backups::limits::BackupLimit;
use crate::application::backups::service::backup_service;
use crate::context::MycologContext;

mod limits;
mod service;

pub async fn backup_task(context: Arc<MycologContext>) {
    let db = context.db.auth_root();
    let shutdown_token = context.task_cancel_token.clone();
    backup_service(&context.config, db, shutdown_token).await;
}
