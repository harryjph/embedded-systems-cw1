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
    let state = ServerState { db, user_manager };
    tokio::spawn(start_server(utils::all_interfaces(config.network.http_port), Arc::new(state)))
}

struct ServerState {
    db: Arc<Database>,
    user_manager: Arc<UserManager>,
}

async fn start_server(address: SocketAddr, state: Arc<ServerState>) {
    let store = MemoryStore::new();
    let secret: [u8; 128] = rand::thread_rng().gen();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    let router = Router::new()
        .nest("/bins", bins::router())
        .nest("/user", user::router())
        .with_state(state)
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
    use reqwest::{Client, RequestBuilder};
    use crate::db::Database;
    use crate::http_server::{ServerState, start_server};
    use crate::user_manager::UserManager;

    /// Starts the HTTP server with a blank database and returns a test client to use it
    pub(super) async fn start_test_server(nested_path: &str) -> (TestClient, Arc<ServerState>) {

        let db = Arc::new(Database::new_in_memory().await.unwrap());
        let user_manager = Arc::new(UserManager::new(db.clone()));
        let state = Arc::new(ServerState { db, user_manager });

        let port = portpicker::pick_unused_port().expect("No free TCP ports");
        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port));

        tokio::spawn(start_server(address, state.clone()));
        let test_client = TestClient {
            client: Client::new(),
            host: format!("http://127.0.0.1:{port}{nested_path}"),
        };
        (test_client, state)
    }

    pub struct TestClient {
        client: Client,
        host: String,
    }

    impl TestClient {
        pub fn get(&self, path: &str) -> RequestBuilder {
            self.client.get(format!("{}{path}", self.host))
        }

        pub fn post(&self, path: &str) -> RequestBuilder {
            self.client.post(format!("{}{path}", self.host))
        }
    }
}
