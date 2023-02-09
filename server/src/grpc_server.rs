use self::grpc_generated::node_api_server::{NodeApi, NodeApiServer};
use self::grpc_generated::{Empty, SensorData, NodeId};
use crate::config::Config;
use crate::db::Database;
use crate::mailer::Mailer;
use crate::timer::Timer;
use crate::utils::all_interfaces;
use futures_util::StreamExt;
use std::marker::Send;
use std::marker::Sync;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

const FULLNESS_THRESHOLD: f32 = 0.8;

pub fn launch<T: Timer + Sync + Send + 'static>(
    config: Config,
    db: Arc<Database>,
    mailer: Arc<Mailer>,
    timer: T,
) -> JoinHandle<()> {
    println!(
        "Starting gRPC Server on http://localhost:{}",
        config.network.grpc_port
    );
    let socket_addr = all_interfaces(config.network.grpc_port);
    tokio::spawn(start_server(socket_addr, db, mailer, timer))
}

async fn start_server<T: Timer + Sync + Send + 'static>(
    socket_addr: SocketAddr,
    db: Arc<Database>,
    mailer: Arc<Mailer>,
    timer: T,
) {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_generated::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(NodeApiServer::new(NodeApiImpl { db, mailer, timer }))
        .serve(socket_addr)
        .await
        .unwrap();
}

mod grpc_generated {
    tonic::include_proto!("nodeapi");

    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("nodeapi_descriptor");
}

pub struct NodeApiImpl<T: Timer + Sync + Send> {
    db: Arc<Database>,
    mailer: Arc<Mailer>,
    timer: T,
}

impl<T: Timer + Sync + Send + 'static> NodeApiImpl<T> {
    async fn handle_email(&self, email: &str, fullness: f32) -> Result<(), Status> {
        if fullness >= FULLNESS_THRESHOLD {
            let now = self.timer.get_time();
            let time = self.db.get_user_last_email_time(email).await.map_err(|_| {
                Status::aborted(format!(
                    "Unable to get last email time for email: {}",
                    email
                ))
            })?;
            if let Some(time) = time {
                if (now - time).num_days() < 1 {
                    return Ok(());
                }
            }
            self.mailer
                .send_email(
                    email.to_string(),
                    "Bin Bot".to_string(),
                    "binny@binbot.com".to_string(),
                    "You have full bins!".to_string(),
                    "Check https://es1.harryphillips.co.uk/app to see which bins are full!"
                        .to_string(),
                )
                .await
                .map_err(|e| eprintln!("Warning: Failed to send email: {e}")).unwrap();
            self.db
                .set_user_last_email_time(email, now)
                .await
                .map_err(|_| {
                    Status::aborted(
                        "Could not update database with last time the email was sent to the user",
                    )
                })?;
        }
        Ok(())
    }
}

#[tonic::async_trait]
impl <T: Timer + Sync + Send + 'static> NodeApi for NodeApiImpl<T> {
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

                if let Some(email) = node.owner {
                    self.handle_email(&email, fullness).await?;
                }

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
