use crate::db::entity::node;
use crate::db::Database;
use crate::http_server::user::get_signed_in_email;
use crate::http_server::util::{bad_request, not_found, ErrorResponse};
use crate::http_server::ServerState;
use crate::routing::{self, Network, Node};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_sessions::extractors::ReadableSession;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub(super) fn router() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/", get(get_all_owned))
        .route("/unowned", get(get_all_unowned))
        .route("/:node_id", get(get_one))
        .route("/:node_id/config", get(get_config).post(set_config))
        .route("/:node_id/claim", post(take_ownership))
        .route("/:node_id/release", post(release_ownership))
        .route("/route", post(get_bin_route))
}

#[derive(Debug, Serialize, Deserialize)]
struct Bin {
    pub id: u32,
    pub config: BinConfig,
    pub fullness: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub last_updated: DateTime<Utc>,
}

impl From<node::Model> for Bin {
    fn from(node: node::Model) -> Self {
        Bin {
            id: node.id,
            config: node.clone().into(),
            fullness: node.fullness,
            temperature: node.temperature,
            humidity: node.humidity,
            last_updated: node.data_last_updated,
        }
    }
}

impl From<Bin> for routing::Node {
    fn from(bin: Bin) -> Self {
        Self {
            x_coord: bin.config.longitude,
            y_coord: bin.config.latitude,
            node_id: bin.id as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BinConfig {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    /// The distance reading above which the bin is considered to be empty
    pub empty_distance_reading: f32,
    /// The distance reading below which the bin is considered to be full
    pub full_distance_reading: f32,
}

impl From<node::Model> for BinConfig {
    fn from(node: node::Model) -> Self {
        BinConfig {
            name: node.name,
            latitude: node.latitude,
            longitude: node.longitude,
            empty_distance_reading: node.empty_distance_reading,
            full_distance_reading: node.full_distance_reading,
        }
    }
}

#[derive(Serialize)]
struct BinRoute {
    route: Vec<u32>, // Route of bin ids
}

#[derive(Deserialize)]
struct RouteInfo {
    start_longitude: f64,
    start_latitude: f64,
}

async fn get_one(
    State(state): State<Arc<ServerState>>,
    session: ReadableSession,
    Path(node_id): Path<u32>,
) -> Result<Json<Bin>, ErrorResponse> {
    let user_email = get_signed_in_email(&session)?;
    Ok(Json(
        state
            .db
            .get_node(node_id, Some(user_email.as_str()))
            .await
            .map_err(bad_request)?
            .ok_or(not_found())?
            .into(),
    ))
}

async fn get_all_bins(db: &Arc<Database>, owner: Option<&str>) -> Result<Vec<Bin>, ErrorResponse> {
    Ok(db
        .get_nodes(owner)
        .await
        .map_err(bad_request)?
        .into_iter()
        .map(Into::into)
        .collect())
}

async fn get_all(db: &Arc<Database>, owner: Option<&str>) -> Result<Json<Vec<Bin>>, ErrorResponse> {
    Ok(Json(get_all_bins(db, owner).await?))
}

async fn get_all_owned(
    State(state): State<Arc<ServerState>>,
    session: ReadableSession,
) -> Result<Json<Vec<Bin>>, ErrorResponse> {
    let user_email = get_signed_in_email(&session)?;
    get_all(&state.db, Some(user_email.as_str())).await
}

async fn get_all_unowned(
    State(state): State<Arc<ServerState>>,
    session: ReadableSession,
) -> Result<Json<Vec<Bin>>, ErrorResponse> {
    // Still require that the user is signed in but allow any account
    let _ = get_signed_in_email(&session)?;
    get_all(&state.db, None).await
}

async fn take_ownership(
    State(state): State<Arc<ServerState>>,
    session: ReadableSession,
    Path(node_id): Path<u32>,
) -> Result<(), ErrorResponse> {
    let user_email = get_signed_in_email(&session)?;
    state
        .db
        .set_node_owner(node_id, None, Some(user_email.as_str()))
        .await
        .map_err(bad_request)?;
    Ok(())
}

async fn release_ownership(
    State(state): State<Arc<ServerState>>,
    session: ReadableSession,
    Path(node_id): Path<u32>,
) -> Result<(), ErrorResponse> {
    let user_email = get_signed_in_email(&session)?;
    state
        .db
        .set_node_owner(node_id, Some(user_email.as_str()), None)
        .await
        .map_err(bad_request)?;
    Ok(())
}

async fn get_config(
    State(state): State<Arc<ServerState>>,
    session: ReadableSession,
    Path(node_id): Path<u32>,
) -> Result<Json<BinConfig>, ErrorResponse> {
    let user_email = get_signed_in_email(&session)?;
    Ok(Json(
        state
            .db
            .get_node(node_id, Some(user_email.as_str()))
            .await
            .map_err(bad_request)?
            .ok_or(not_found())?
            .into(),
    ))
}

async fn set_config(
    State(state): State<Arc<ServerState>>,
    session: ReadableSession,
    Path(node_id): Path<u32>,
    Json(new_config): Json<BinConfig>,
) -> Result<(), ErrorResponse> {
    let user_email = get_signed_in_email(&session)?;
    state
        .db
        .set_node_config(
            node_id,
            Some(user_email.as_str()),
            new_config.name,
            new_config.latitude,
            new_config.longitude,
            new_config.empty_distance_reading,
            new_config.full_distance_reading,
        )
        .await
        .map_err(bad_request)?;
    Ok(())
}

#[derive(Deserialize)]
struct RouteConfig {
    starting_latitude: f64,
    starting_longitude: f64,
}

async fn get_bin_route(
    session: ReadableSession,
    State(state): State<Arc<ServerState>>,
    Json(route_config): Json<RouteConfig>,
) -> Result<Json<BinRoute>, ErrorResponse> {
    // THIS WILL BE CHANGED BY NODE CONFIG IF WE ADD IT!
    use crate::grpc_server::FULLNESS_THRESHOLD;

    let user_email = get_signed_in_email(&session)?;
    let mut nodes: Vec<routing::Node> = get_all_bins(&state.db, Some(user_email.as_str()))
        .await?
        .into_iter()
        .filter(|bin| bin.fullness >= FULLNESS_THRESHOLD)
        .map(|bin| bin.into())
        .collect();

    if nodes.is_empty() {
        Ok(Json(BinRoute { route: vec![] }))
    } else {
        let max_id = nodes
            .iter()
            .map(|n| n.node_id)
            .reduce(usize::max)
            .unwrap_or(0);
        let start_node = Node::new(route_config.starting_longitude, route_config.starting_latitude, max_id + 1, 0.0);
        nodes.push(start_node);
        println!("{nodes:?}");
        println!("{start_node:?}");
        let mut network = Network::new_euclidean(nodes, start_node, 999999.0);
        let route: Vec<u32> = network
            .christofides()
            .into_iter()
            .map(|id| id as u32)
            .collect();

        Ok(Json(BinRoute { route: route[1..route.len() - 1].into() }))
    }
}

#[cfg(test)]
mod tests {
    use crate::http_server::test_utils::start_test_server;

    #[tokio::test]
    async fn test_get_bins_is_secured() {
        let (client, _) = start_test_server("/bins").await;

        // Try accessing bins without login
        client
            .get("/")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .expect_err("Accessed bins without being logged in");

        // Try accessing bins with login
        client.register_and_login().await;
        client
            .get("/")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }
}
