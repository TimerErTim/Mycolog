use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Instrument};

use crate::application::web::routes::try_build_routes;
use crate::config::MycologConfig;
use crate::context::MycologContext;

pub async fn web_server_service(
    context: Arc<MycologContext>,
    shutdown_token: CancellationToken,
) -> anyhow::Result<()> {
    info!("web server service starting...");
    let config = &context.config;
    let socket = SocketAddr::new(config.web_bind_ip.clone(), config.web_bind_port);
    let listener = TcpListener::bind(socket)
        .await
        .inspect_err(|err| error!(?err, "unable to bind web server address"))?;
    let routes = try_build_routes()?;

    run_web_server(listener, shutdown_token, routes.with_state(context)).await
}

async fn run_web_server(
    listener: TcpListener,
    shutdown_token: CancellationToken,
    routes: Router,
) -> anyhow::Result<()> {
    let server_future = axum::serve(listener, routes)
        .with_graceful_shutdown(async move { shutdown_token.cancelled().await });
    info!("web server service started");
    Ok(server_future.await?)
}
