use std::fs::{create_dir_all, File, OpenOptions};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use tracing::instrument::WithSubscriber;
use tracing::{Level, Subscriber};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::FilterExt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::{SubscriberInitExt, TryInitError};
use tracing_subscriber::{FmtSubscriber, Layer, Registry};

type BoxedLogger<S> = Box<dyn Layer<S> + Send + Sync>;

pub struct LoggingHandle {
    file_guard: Option<WorkerGuard>,
}

pub(super) fn setup_logging() -> anyhow::Result<LoggingHandle> {
    // Base subscriber
    let subscriber = Registry::default();

    // Build subscriber with layers
    let stdout_log = stdout_logger();
    let (file_log, file_guard) = file_logger().unzip();
    tracing::subscriber::set_global_default(subscriber.with(file_log).with(stdout_log))?;

    let handle = LoggingHandle { file_guard };
    Ok(handle)
}

fn stdout_logger<S: Subscriber>() -> BoxedLogger<S>
where
    for<'a> S: LookupSpan<'a>,
{
    let mut stdout_log = tracing_subscriber::fmt::layer();

    if cfg!(feature = "dev-env") {
        stdout_log
            .pretty()
            .with_span_events(FmtSpan::FULL)
            .map_writer(|writer| writer.with_max_level(Level::TRACE))
            .boxed()
    } else if cfg!(feature = "prod-env") {
        stdout_log
            .pretty()
            .with_ansi(true)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .map_writer(|writer| writer.with_max_level(Level::DEBUG))
            .boxed()
    } else {
        stdout_log.boxed()
    }
}

fn file_logger<S: Subscriber>() -> Option<(BoxedLogger<S>, WorkerGuard)>
where
    for<'a> S: LookupSpan<'a>,
{
    create_dir_all("logs/").ok()?;

    if cfg!(feature = "dev-env") {
        return None;
    }

    let file = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_suffix("log")
        .build("logs/")
        .ok()?;
    let (writer, guard) = NonBlocking::new(file);
    let file_log = tracing_subscriber::fmt::layer()
        .with_writer(writer.with_max_level(Level::DEBUG))
        .with_ansi(false);

    if cfg!(feature = "prod-env") {
        let log = file_log.with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
        return Some((log.boxed(), guard));
    }

    Some((file_log.boxed(), guard))
}
