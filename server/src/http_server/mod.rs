use std::sync::Arc;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::get;
use itertools::Itertools;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use crate::config::Config;
use crate::utils;

mod entities;

pub fn launch(config: Config, data_list: Arc<RwLock<Vec<(f32, f32)>>>) -> JoinHandle<()> {
    tokio::spawn(start_server(config, data_list))
}

async fn start_server(config: Config, data_list: Arc<RwLock<Vec<(f32, f32)>>>) {
    println!("Starting HTTP Server on http://localhost:{}", config.network.http_port);

    let router = Router::new()
        .route("/data", get(get_data))
        .route("/bins", get(get_all_bins))
        .route("/bins/:id", get(get_bin))
        .route("/bins/:id/config", get(get_bin_config).post(set_bin_config))
        .with_state(data_list);

    axum::Server::bind(&utils::all_interfaces(config.network.http_port))
        .serve(router.into_make_service())
        .await.unwrap();
}

pub async fn get_data(State(data_list): State<Arc<RwLock<Vec<(f32, f32)>>>>) -> impl IntoResponse {
    let customers = data_list.read().await;
    Json(customers.clone())
}

pub async fn get_all_bins() -> impl IntoResponse {
    Json(entities::dummy_data())
}

pub async fn get_bin(Path(id): Path<u64>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(entities::dummy_data().into_iter()
        .find_or_first(|it| it.id == id)
        .ok_or(StatusCode::NOT_FOUND)?))
}

pub async fn get_bin_config(Path(id): Path<u64>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(entities::dummy_data().into_iter()
        .find_or_first(|it| it.id == id)
        .ok_or(StatusCode::NOT_FOUND)?
        .config))
}

pub async fn set_bin_config(Path(id): Path<u64>, Json(new_config): Json<entities::BinConfig>) -> Result<impl IntoResponse, StatusCode> {
    let mut bin = entities::dummy_data().into_iter()
        .find_or_first(|it| it.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    bin.config = new_config;
    Ok(())
}
