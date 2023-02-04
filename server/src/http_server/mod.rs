use crate::config::Config;
use crate::db::Database;
use crate::user_manager::UserManager;
use crate::utils::all_interfaces;
use axum::Router;
use axum_sessions::async_session::MemoryStore;
use axum_sessions::{SameSite, SessionLayer};
use rand::Rng;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::headers::HeaderValue;
use tokio::task::JoinHandle;
use tower_http::cors::CorsLayer;

mod bins;
mod user;
mod util;

pub fn launch(config: Config, db: Arc<Database>, user_manager: Arc<UserManager>) -> JoinHandle<()> {
    println!(
        "Starting HTTP Server on http://localhost:{}",
        config.network.http_port
    );
    let state = ServerState { db, user_manager };
    let socket_addr = all_interfaces(config.network.http_port);
    tokio::spawn(start_server(socket_addr, Arc::new(state)))
}

struct ServerState {
    db: Arc<Database>,
    user_manager: Arc<UserManager>,
}

async fn start_server(socket_addr: SocketAddr, state: Arc<ServerState>) {
    let store = MemoryStore::new();
    let secret: [u8; 128] = rand::thread_rng().gen();

    let mut router = Router::new()
        .nest("/bins", bins::router())
        .nest("/user", user::router())
        .with_state(state)
        .layer(CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_credentials(true))
        .layer(SessionLayer::new(store, &secret)
            .with_same_site_policy(SameSite::None));

    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod test_utils {
    use crate::db::Database;
    use crate::http_server::{start_server, ServerState};
    use crate::user_manager::tests::{TEST_EMAIL, TEST_PASSWORD};
    use crate::user_manager::UserManager;
    use reqwest::{Client, RequestBuilder};
    use std::collections::HashMap;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    /// Starts the HTTP server with a blank database and returns a test client to use it
    pub(super) async fn start_test_server(nested_path: &str) -> (TestClient, Arc<ServerState>) {
        let db = Arc::new(Database::new_in_memory().await.unwrap());
        let user_manager = Arc::new(UserManager::new(db.clone()));
        let state = Arc::new(ServerState { db, user_manager });

        let port = portpicker::pick_unused_port().expect("No free TCP ports");
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
        tokio::spawn(start_server(address, state.clone()));

        let test_client = TestClient {
            client: Client::builder().cookie_store(true).build().unwrap(),
            host: format!("http://localhost:{port}"),
            nested_path: nested_path.to_string(),
        };
        (test_client, state)
    }

    pub struct TestClient {
        client: Client,
        host: String,
        nested_path: String,
    }

    impl TestClient {
        pub fn get(&self, path: &str) -> RequestBuilder {
            self.client
                .get(format!("{}{}{path}", self.host, self.nested_path))
        }

        pub fn post(&self, path: &str) -> RequestBuilder {
            self.client
                .post(format!("{}{}{path}", self.host, self.nested_path))
        }

        /// Registers a new account and logs the test client in
        pub async fn register_and_login(&self) {
            let mut params = HashMap::new();
            params.insert("email", TEST_EMAIL);
            params.insert("password", TEST_PASSWORD);

            self.client
                .post(format!("{}/user/register", self.host))
                .form(&params)
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap();
        }
    }
}
