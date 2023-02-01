use crate::config::Config;
use crate::utils;
use axum::Router;
use std::sync::Arc;
use axum_sessions::async_session::MemoryStore;
use axum_sessions::SessionLayer;
use rand::Rng;
use tokio::task::JoinHandle;
use tower_http::cors::{Any, CorsLayer};
use crate::db::Database;

mod bins;
mod user;

pub fn launch(config: Config, db: Arc<Database>) -> JoinHandle<()> {
    tokio::spawn(start_server(config, db))
}

struct ServerState {
    db: Arc<Database>,
}

async fn start_server(config: Config, db: Arc<Database>) {
    println!(
        "Starting HTTP Server on http://localhost:{}",
        config.network.http_port
    );

    let store = MemoryStore::new();
    let secret: [u8; 128] = rand::thread_rng().gen();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    let router = Router::new()
        .nest("/bins", bins::router())
        .nest("/user", user::router())
        .with_state(Arc::new(ServerState { db }))
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(session_layer);

    axum::Server::bind(&utils::all_interfaces(config.network.http_port))
        .serve(router.into_make_service())
        .await
        .unwrap();
}
