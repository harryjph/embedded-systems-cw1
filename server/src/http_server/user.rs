use std::sync::Arc;
use anyhow::Error;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Form, Router};
use axum::extract::State;
use axum::routing::post;
use axum_sessions::extractors::WritableSession;
use serde::Deserialize;
use crate::http_server::ServerState;

pub(super) fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/register", post(register_and_login))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

async fn set_logged_in(session: &mut WritableSession, email: &str) -> Result<(), Error> {
    session.insert("signed_in_to", email)?;
    Ok(())
}

fn bad_request(e: Error) -> (StatusCode, impl IntoResponse) {
    (StatusCode::BAD_REQUEST, e.to_string())
}

async fn register_and_login(
    State(state): State<Arc<ServerState>>,
    mut session: WritableSession,
    Form(input): Form<LoginForm>,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    state.user_manager.register(input.email.clone(), input.password)
        .await
        .map_err(bad_request)?;

    set_logged_in(&mut session, input.email.as_str())
        .await
        .map_err(bad_request)?;

    Ok(())
}

async fn login(
    State(state): State<Arc<ServerState>>,
    mut session: WritableSession,
    Form(input): Form<LoginForm>,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    state.user_manager.login(input.email.clone(), input.password)
        .await
        .map_err(bad_request)?;

    set_logged_in(&mut session, input.email.as_str())
        .await
        .map_err(bad_request)?;

    Ok(())
}

async fn logout(mut session: WritableSession, ) -> impl IntoResponse {
    session.destroy()
}
