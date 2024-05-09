use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::head;
use axum::RequestExt;
use tracing::{instrument, trace, trace_span, Level};

use crate::context::MycologContext;

#[derive(Clone, Copy)]
pub enum AdminStatus {
    Authorized,
    Unauthorized,
}

pub async fn authorize_admin(
    State(context): State<Arc<MycologContext>>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Response {
    let authorize_span = trace_span!("authorize_admin").entered();
    let mut status = AdminStatus::Unauthorized;
    if let Some(admin_header) = headers.get("admin") {
        trace!(header = ?admin_header, "got admin header");
        if let Ok(admin_token) = admin_header.to_str() {
            if &context.secrets.admin.token() == admin_token.trim() {
                status = AdminStatus::Authorized;
            }
        }
    }
    drop(authorize_span);

    req.extensions_mut().insert(status);
    next.run(req).await
}
