use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use crate::http_server::ServerState;

pub(super) fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register() -> Result<impl IntoResponse, StatusCode> {
    Ok("not implemented")
}

async fn login() -> Result<impl IntoResponse, StatusCode> {
    Ok("not implemented")
}
