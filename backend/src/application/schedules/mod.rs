use std::path::PathBuf;
use std::sync::Arc;

use surrealdb_core::sql::Statements;
use tracing::{error, warn};

use crate::application::database::load_surql_file;
use crate::application::schedules::service::schedule_service;
use crate::context::MycologContext;

mod service;

#[derive(Clone)]
pub struct ScheduleQueries {
    hourly: Option<Statements>,
    daily: Option<Statements>,
    weekly: Option<Statements>,
}

pub async fn load_schedule_queries(folder: impl Into<PathBuf>) -> anyhow::Result<ScheduleQueries> {
    let folder = folder.into();
    let hourly_file = folder.join("hourly.surql");
    let daily_file = folder.join("daily.surql");
    let weekly_file = folder.join("weekly.surql");

    async fn get_statements_from_file(
        path: impl Into<PathBuf>,
    ) -> anyhow::Result<Option<Statements>> {
        let path = path.into();
        if path.is_file() {
            let statements = load_surql_file(path.clone()).await?.statements;
            if !statements.0.is_empty() {
                Ok(Some(statements))
            } else {
                warn!(
                    "scheduling file {} has been found but was empty and thus ignored",
                    path.display()
                );
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    Ok(ScheduleQueries {
        hourly: get_statements_from_file(hourly_file).await?,
        daily: get_statements_from_file(daily_file).await?,
        weekly: get_statements_from_file(weekly_file).await?,
    })
}

pub async fn schedule_task(context: Arc<MycologContext>) {
    let db = context.db.auth_root();
    let shutdown_token = context.task_cancel_token.clone();
    if let Err(err) = schedule_service(db, &context.schedules, shutdown_token).await {
        error!(?err, "database schedule task stopped working due to error")
    }
}
