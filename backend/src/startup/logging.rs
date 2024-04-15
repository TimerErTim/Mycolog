use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::util::{SubscriberInitExt, TryInitError};
use tracing_subscriber::FmtSubscriber;

pub fn setup_logging() -> Result<(), TryInitError> {
    // Base subscriber
    let builder = FmtSubscriber::builder();

    // Config specific subscriber
    if cfg!(feature = "dev-env") {
        builder
            .pretty()
            .with_max_level(Level::TRACE)
            .with_span_events(FmtSpan::FULL)
            .finish()
            .try_init()
    } else if cfg!(feature = "prod-env") {
        builder
            .with_max_level(Level::DEBUG)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_ansi(false)
            .finish()
            .try_init()
    } else {
        builder.finish().try_init()
    }
}
