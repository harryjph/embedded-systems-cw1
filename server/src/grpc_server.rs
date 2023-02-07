use self::grpc_generated::node_api_server::{NodeApi, NodeApiServer};
use self::grpc_generated::{Empty, NodeId};
use crate::config::Config;
use crate::db::Database;
use crate::grpc_server::grpc_generated::SensorData;
use crate::utils::all_interfaces;
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

pub fn launch(config: Config, db: Arc<Database>) -> JoinHandle<()> {
    println!(
        "Starting gRPC Server on http://localhost:{}",
        config.network.grpc_port
    );
    let socket_addr = all_interfaces(config.network.grpc_port);
    tokio::spawn(start_server(socket_addr, db))
}

async fn start_server(socket_addr: SocketAddr, db: Arc<Database>) {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_generated::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(NodeApiServer::new(NodeApiImpl { db }))
        .serve(socket_addr)
        .await
        .unwrap();
}

mod grpc_generated {
    tonic::include_proto!("nodeapi");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("nodeapi_descriptor");
}

pub struct NodeApiImpl {
    db: Arc<Database>,
}

#[tonic::async_trait]
impl NodeApi for NodeApiImpl {
    async fn assign_id(&self, _request: Request<Empty>) -> Result<Response<NodeId>, Status> {
        let id = self
            .db
            .insert_node()
            .await
            .map_err(|_| Status::aborted("Unable to insert into db"))?;
        Ok(Response::new(NodeId { id }))
    }

    async fn stream_sensor_data(
        &self,
        request: Request<Streaming<SensorData>>,
    ) -> Result<Response<Empty>, Status> {
        let mut stream = request.into_inner();
        while let Some(sensor_data) = stream.next().await {
            let sensor_data = sensor_data?;
            if let Some(node) = self.db.get_node(sensor_data.id, None).await.map_err(|_| {
                Status::aborted(format!(
                    "Unable to get node with id: {} from database.",
                    sensor_data.id
                ))
            })? {
                let fullness = (sensor_data.distance - node.empty_distance_reading)
                    / (node.full_distance_reading - node.empty_distance_reading);
                self.db
                    .set_node_data(
                        sensor_data.id,
                        fullness.clamp(0.0, 1.0),
                        sensor_data.temperature,
                        sensor_data.relative_humidity,
                    )
                    .await
                    .map_err(|_| {
                        Status::aborted(format!(
                            "Unable to update fullness of bin with id: {}.",
                            sensor_data.id
                        ))
                    })?;
            } else {
                eprintln!(
                    "Node with id: {} does not exist within database yet it sent something.",
                    sensor_data.id
                );
            }
        }
        Ok(Response::new(Empty::default()))
    }
}
