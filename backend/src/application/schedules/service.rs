use std::future::IntoFuture;
use std::time::Duration;

use tokio::time::{interval, Interval};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, info_span, Instrument};

use crate::application::database::DatabaseRootAccess;
use crate::application::ScheduleQueries;
use crate::config::MycologConfig;

pub async fn schedule_service(
    db: DatabaseRootAccess,
    queries: &ScheduleQueries,
    shutdown_token: CancellationToken,
) -> anyhow::Result<()> {
    let mut hourly_timer = interval(Duration::from_hours(1));
    let mut daily_timer = interval(Duration::from_days(1));
    let mut weekly_timer = interval(Duration::from_weeks(1));

    info!("started schedule service");
    while !shutdown_token.is_cancelled() {
        let (schedule, Some(statements)) = tokio::select!(
            _ = shutdown_token.cancelled() => break,
            _ = hourly_timer.tick() => ("hourly", queries.hourly.as_ref()),
            _ = daily_timer.tick() => ("daily", queries.daily.as_ref()),
            _ = weekly_timer.tick() => ("weekly", queries.weekly.as_ref())
        ) else {
            continue;
        };
        info!(schedule, "executing database query...");
        let responses = match db
            .query(statements.clone())
            .into_future()
            .instrument(info_span!("schedule_query", schedule))
            .await
        {
            Ok(responses) => responses,
            Err(err) => {
                error!(schedule, ?err, "query for database failed");
                continue;
            }
        };
        if let Err(err) = responses.check() {
            error!(schedule, ?err, "query for database responded with error");
            continue;
        }
        info!(schedule, "successfully executed database query");
    }
    Ok(())
}
