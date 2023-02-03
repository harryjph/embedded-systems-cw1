use anyhow::Error;
use axum::http::StatusCode;

pub type ErrorResponse = (StatusCode, String);

pub fn bad_request(e: Error) -> ErrorResponse {
    (StatusCode::BAD_REQUEST, e.to_string())
}

pub fn not_found() -> ErrorResponse {
    (StatusCode::NOT_FOUND, String::new())
}
