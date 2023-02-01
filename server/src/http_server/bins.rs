use std::sync::Arc;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use axum_sessions::extractors::ReadableSession;
use itertools::Itertools;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::http_server::ServerState;
use crate::http_server::user::get_signed_in_email;

pub(super) fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/", get(get_all))
        .route("/:id", get(get_one))
        .route("/:id/config", get(get_config).post(set_config))
}

#[derive(Debug, Serialize, Deserialize)]
struct Bin {
    pub id: u64,
    pub config: BinConfig,
    pub fullness: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct BinConfig {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    /// If fullness is above this value, the bin is full
    pub full_threshold: f64,
}

async fn get_one(Path(id): Path<u64>, session: ReadableSession) -> Result<impl IntoResponse, StatusCode> {
    let user_email = get_signed_in_email(&session)?;
    Ok(Json(
        dummy_data()
            .into_iter()
            .find_or_first(|it| it.id == id)
            .ok_or(StatusCode::NOT_FOUND)?,
    ))
}

async fn get_all(session: ReadableSession) -> Result<impl IntoResponse, StatusCode> {
    let user_email = get_signed_in_email(&session)?;
    Ok(Json(dummy_data()))
}

async fn get_config(Path(id): Path<u64>, session: ReadableSession) -> Result<impl IntoResponse, StatusCode> {
    let user_email = get_signed_in_email(&session)?;
    Ok(Json(
        dummy_data()
            .into_iter()
            .find_or_first(|it| it.id == id)
            .ok_or(StatusCode::NOT_FOUND)?
            .config,
    ))
}

async fn set_config(
    Path(id): Path<u64>,
    session: ReadableSession,
    Json(new_config): Json<BinConfig>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_email = get_signed_in_email(&session)?;
    let mut bin = dummy_data()
        .into_iter()
        .find_or_first(|it| it.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    bin.config = new_config;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::http_server::test_utils::start_test_server;

    #[tokio::test]
    async fn test_get_bins_is_secured() {
        let (client, _) = start_test_server("/bins").await;

        // Try accessing bins without login
        client.get("/")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .expect_err("Accessed bins without being logged in");

        // Try accessing bins with login
        client.register_and_login().await;
        client.get("/")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }
}

fn dummy_data() -> [Bin; 5] {
    let mut rng = rand::thread_rng();
    [
        Bin {
            id: 1,
            config: BinConfig {
                name: "Gilbert".to_string(),
                latitude: 51.501,
                longitude: -0.142,
                full_threshold: 80.0,
            },
            fullness: rng.gen_range(0.0..1.0),
        },
        Bin {
            id: 2,
            config: BinConfig {
                name: "Godfried".to_string(),
                latitude: 51.501,
                longitude: -0.145,
                full_threshold: 80.0,
            },
            fullness: rng.gen_range(0.0..1.0),
        },
        Bin {
            id: 3,
            config: BinConfig {
                name: "Stephen".to_string(),
                latitude: 51.498,
                longitude: -0.177,
                full_threshold: 81.0,
            },
            fullness: rng.gen_range(0.0..1.0),
        },
        Bin {
            id: 4,
            config: BinConfig {
                name: "Fry".to_string(),
                latitude: 51.470,
                longitude: -0.454,
                full_threshold: 90.0,
            },
            fullness: rng.gen_range(0.0..1.0),
        },
        Bin {
            id: 5,
            config: BinConfig {
                name: "Norbert".to_string(),
                latitude: 51.162,
                longitude: -0.177,
                full_threshold: 70.0,
            },
            fullness: rng.gen_range(0.0..1.0),
        },
    ]
}
