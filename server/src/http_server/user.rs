use crate::http_server::util::{bad_request, ErrorResponse};
use crate::http_server::ServerState;
use anyhow::Error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const SESSION_EMAIL_KEY: &str = "signed_in_to";

pub(super) fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/", get(get_user))
        .route("/register", post(register_and_login))
        .route("/login", post(login))
        .route("/logout", get(logout))
}

pub fn get_signed_in_email(session: &ReadableSession) -> Result<String, ErrorResponse> {
    if let Some(email) = session.get::<String>(SESSION_EMAIL_KEY) {
        Ok(email)
    } else {
        Err((StatusCode::UNAUTHORIZED, String::new()))
    }
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    email: String,
}

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

async fn get_user(
    session: ReadableSession,
    State(state): State<Arc<ServerState>>
) -> Result<Json<UserInfo>, ErrorResponse> {
    let user_email = get_signed_in_email(&session)?;
    let user = state.db.get_user(user_email.as_str())
        .await
        .map_err(bad_request)?;
    Ok(Json(UserInfo { email: user.email }))
}

async fn set_logged_in(session: &mut WritableSession, email: &str) -> Result<(), Error> {
    session.insert(SESSION_EMAIL_KEY, email)?;
    Ok(())
}

async fn register_and_login(
    State(state): State<Arc<ServerState>>,
    mut session: WritableSession,
    Form(input): Form<LoginForm>,
) -> Result<(), ErrorResponse> {
    state
        .user_manager
        .register(input.email.as_str(), input.password.as_str())
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
) -> Result<(), ErrorResponse> {
    state
        .user_manager
        .login(input.email.as_str(), input.password.as_str())
        .await
        .map_err(bad_request)?;

    set_logged_in(&mut session, input.email.as_str())
        .await
        .map_err(bad_request)?;

    Ok(())
}

async fn logout(mut session: WritableSession) {
    session.destroy()
}

#[cfg(test)]
mod tests {
    use crate::http_server::test_utils::start_test_server;
    use crate::user_manager::tests::{TEST_EMAIL, TEST_PASSWORD};
    use std::collections::HashMap;
    use crate::http_server::user::UserInfo;

    #[tokio::test]
    async fn test_register_and_logout() {
        let (client, _) = start_test_server("/user").await;

        let mut params = HashMap::new();
        params.insert("email", TEST_EMAIL);
        params.insert("password", TEST_PASSWORD);

        client
            .post("/register")
            .form(&params)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        client
            .post("/logout")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }

    #[tokio::test]
    async fn test_login_and_logout() {
        let (client, state) = start_test_server("/user").await;
        state
            .user_manager
            .register(TEST_EMAIL, TEST_PASSWORD)
            .await
            .unwrap();

        let mut params = HashMap::new();
        params.insert("email", TEST_EMAIL);
        params.insert("password", TEST_PASSWORD);

        client
            .post("/login")
            .form(&params)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        client
            .post("/logout")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_user() {
        let (client, state) = start_test_server("/user").await;

        let mut params = HashMap::new();
        params.insert("email", TEST_EMAIL);
        params.insert("password", TEST_PASSWORD);

        client
            .post("/register")
            .form(&params)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        let response = client
            .get("/")
            .send()
            .await
            .unwrap()
            .json::<UserInfo>()
            .await
            .unwrap();

        assert_eq!(response.email, TEST_EMAIL.to_lowercase());
    }
}
