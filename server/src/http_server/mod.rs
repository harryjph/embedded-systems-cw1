use crate::config::Config;
use crate::utils;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use itertools::Itertools;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tower_http::cors::{Any, CorsLayer};

mod entities;

pub fn launch(config: Config, data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> JoinHandle<()> {
    tokio::spawn(start_server(config, data_list))
}

struct ServerState {
    data_list: Arc<RwLock<Vec<(f32, f32)>>>,
}

async fn start_server(config: Config, data_list: Arc<RwLock<Vec<(f32, f32)>>>) {
    println!(
        "Starting HTTP Server on http://localhost:{}",
        config.network.http_port
    );

    let router = Router::new()
        .route("/data", get(get_data))
        .route("/bins", get(get_all_bins))
        .route("/bins/:id", get(get_bin))
        .route("/bins/:id/config", get(get_bin_config).post(set_bin_config))
        .with_state(Arc::new(ServerState { data_list }))
        .layer(CorsLayer::new().allow_origin(Any));

    axum::Server::bind(&utils::all_interfaces(config.network.http_port))
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn get_data(State(state): State<Arc<ServerState>>) -> impl IntoResponse {
    let customers = state.data_list.read().await;
    Json(customers.clone())
}

pub async fn get_all_bins() -> impl IntoResponse {
    Json(entities::dummy_data())
}

pub async fn get_bin(Path(id): Path<u64>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(
        entities::dummy_data()
            .into_iter()
            .find_or_first(|it| it.id == id)
            .ok_or(StatusCode::NOT_FOUND)?,
    ))
}

pub async fn get_bin_config(Path(id): Path<u64>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(
        entities::dummy_data()
            .into_iter()
            .find_or_first(|it| it.id == id)
            .ok_or(StatusCode::NOT_FOUND)?
            .config,
    ))
}

pub async fn set_bin_config(
    Path(id): Path<u64>,
    Json(new_config): Json<entities::BinConfig>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut bin = entities::dummy_data()
        .into_iter()
        .find_or_first(|it| it.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    bin.config = new_config;
    Ok(())
}
