use self::grpc_generated::node_api_server::{NodeApi, NodeApiServer};
use self::grpc_generated::{Empty, EnvironmentData, NodeId};
use crate::config::Config;
use crate::db::Database;
use crate::utils::all_interfaces;
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

pub fn launch(config: Config, data_sink: Sender<(f32, f32)>, db: Arc<Database>) -> JoinHandle<()> {
    println!(
        "Starting gRPC Server on http://localhost:{}",
        config.network.grpc_port
    );
    let socket_addr = all_interfaces(config.network.grpc_port);
    tokio::spawn(start_server(socket_addr, data_sink, db))
}

async fn start_server(socket_addr: SocketAddr, data_sink: Sender<(f32, f32)>, db: Arc<Database>) {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_generated::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(NodeApiServer::new(NodeApiImpl { data_sink, db }))
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
    data_sink: Sender<(f32, f32)>,
    db: Arc<Database>,
}

#[tonic::async_trait]
impl NodeApi for NodeApiImpl {
    async fn report_environment(
        &self,
        request: Request<Streaming<EnvironmentData>>,
    ) -> Result<Response<Empty>, Status> {
        let mut stream = request.into_inner();
        while let Some(data_result) = stream.next().await {
            let data = data_result?;
            self.data_sink
                .send((data.temperature, data.relative_humidity))
                .await
                .unwrap();
        }
        Ok(Response::new(Empty::default()))
    }

    async fn assign_id(&self, _request: Request<Empty>) -> Result<Response<NodeId>, Status> {
        let id = self
            .db
            .insert_node()
            .await
            .map_err(|_| Status::aborted("Unable to insert into db"))?;
        Ok(Response::new(NodeId { id }))
    }
}
