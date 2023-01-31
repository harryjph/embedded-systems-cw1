use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tonic::{Request, Response, Status, Streaming};
use tonic::transport::Server;
use futures_util::StreamExt;
use crate::config::Config;
use crate::db::Database;
use self::grpc_generated::{Empty, EnvironmentData, NodeId};
use crate::utils;
use self::grpc_generated::node_api_server::{NodeApi, NodeApiServer};

pub fn launch<'a>(config: Config, data_sink: Sender<(f32, f32)>, db: Arc<Database>) -> JoinHandle<()> {
    tokio::spawn(start_server(config, data_sink, db))
}

async fn start_server(config: Config, data_sink: Sender<(f32, f32)>, db: Arc<Database>) {
    println!("Starting gRPC Server on http://localhost:{}", config.network.grpc_port);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_generated::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(NodeApiServer::new(NodeApiImpl { data_sink, db }))
        .serve(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.network.grpc_port).into())
        .await.unwrap();
}

mod grpc_generated {
    tonic::include_proto!("nodeapi");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("nodeapi_descriptor");
}

pub struct NodeApiImpl {
    data_sink: Sender<(f32, f32)>,
    db: Arc<Database>,
}

#[tonic::async_trait]
impl NodeApi for NodeApiImpl {
    async fn report_environment(&self, request: Request<Streaming<EnvironmentData>>) -> Result<Response<Empty>, Status> {
        let mut stream = request.into_inner();
        while let Some(data_result) = stream.next().await {
            let data = data_result?;
            self.data_sink.send((data.temperature, data.relative_humidity)).await.unwrap();
        }
        Ok(Response::new(Empty::default()))
    }

    async fn assign_id(&self, _request: Request<Empty>) -> Result<Response<NodeId>, Status> {
        let id = self.db.insert_node(123.32, 123.321).await.map_err(|_| Status::aborted("Unable to insert into db"))?;
        Ok(Response::new(NodeId{ id: id }))
    }
}
