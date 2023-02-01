use std::net::SocketAddr;
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
use crate::user_manager::UserManager;

mod bins;
mod user;

pub fn launch(config: Config, db: Arc<Database>, user_manager: Arc<UserManager>) -> JoinHandle<()> {
    println!("Starting HTTP Server on http://localhost:{}", config.network.http_port);
    tokio::spawn(start_server(utils::all_interfaces(config.network.http_port), db, user_manager))
}

struct ServerState {
    db: Arc<Database>,
    user_manager: Arc<UserManager>,
}

async fn start_server(address: SocketAddr, db: Arc<Database>, user_manager: Arc<UserManager>) {
    let store = MemoryStore::new();
    let secret: [u8; 128] = rand::thread_rng().gen();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    let router = Router::new()
        .nest("/bins", bins::router())
        .nest("/user", user::router())
        .with_state(Arc::new(ServerState { db, user_manager }))
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(session_layer);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod test_utils {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    use std::sync::Arc;
    use reqwest::{Client, IntoUrl, Method, RequestBuilder};
    use crate::db::Database;
    use crate::http_server::start_server;
    use crate::user_manager::UserManager;

    /// Starts the HTTP server with a blank database and returns a test client to use it
    pub async fn start_test_server() -> TestClient {
        let db = Arc::new(Database::new_in_memory().await.unwrap());
        let user_manager = Arc::new(UserManager::new(db.clone()));
        const TEST_SERVER_PORT: u16 = 12345;
        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, TEST_SERVER_PORT));
        tokio::spawn(start_server(address, db, user_manager));
        TestClient {
            client: Client::new().pos,
            host: format!("http://127.0.0.1:{TEST_SERVER_PORT}"),
        }
    }

    pub struct TestClient {
        client: Client,
        host: String,
    }

    impl TestClient {
        pub fn get(&self, path: &str) -> RequestBuilder {
            self.client.get(format!("{}/{path}", self.host))
        }

        pub fn post(&self, path: &str) -> RequestBuilder {
            self.client.post(format!("{}/{path}", self.host))
        }
    }
}
