use crate::config::Config;
use crate::db::Database;
use crate::user_manager::UserManager;
use crate::utils::all_interfaces;
use axum::headers::HeaderValue;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::routing::get_service;
use axum::Router;
use axum_sessions::async_session::MemoryStore;
use axum_sessions::{SameSite, SessionLayer};
use rand::Rng;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

mod bins;
mod user;
mod util;

const FRONTEND_PATH_ENV_NAME: &str = "FRONTEND_PATH";

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

    let allowed_origins = ["http://localhost", "http://localhost:3000"];

    let router = Router::new()
        .nest("/api", api_router())
        .nest("/app", frontend_router())
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(allowed_origins.map(|it| it.parse::<HeaderValue>().unwrap()))
                .allow_credentials(true)
                .allow_headers([CONTENT_TYPE]),
        )
        .layer(SessionLayer::new(store, &secret).with_same_site_policy(SameSite::None));

    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn api_router() -> Router<Arc<ServerState>> {
    Router::new()
        .nest("/bins", bins::router())
        .nest("/user", user::router())
}

fn frontend_router() -> Router<Arc<ServerState>> {
    if let Ok(frontend_path) = env::var(FRONTEND_PATH_ENV_NAME) {
        Router::new().nest_service(
            "/",
            get_service(
                ServeDir::new(&frontend_path)
                    .not_found_service(ServeFile::new(frontend_path + "/index.html")),
            )
            .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR }),
        )
    } else {
        eprintln!("Warning: Frontend not found and will not be served");
        Router::new()
    }
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
                .get(format!("{}/api{}{path}", self.host, self.nested_path))
        }

        pub fn post(&self, path: &str) -> RequestBuilder {
            self.client
                .post(format!("{}/api{}{path}", self.host, self.nested_path))
        }

        /// Registers a new account and logs the test client in
        pub async fn register_and_login(&self) {
            let mut params = HashMap::new();
            params.insert("email", TEST_EMAIL);
            params.insert("password", TEST_PASSWORD);

            self.client
                .post(format!("{}/api/user/register", self.host))
                .form(&params)
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap();
        }
    }
}
