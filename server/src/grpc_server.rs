use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tonic::{Request, Response, Status, Streaming};
use tonic::transport::Server;
use nodeapi::node_api_server::{NodeApi, NodeApiServer};
use futures_util::StreamExt;
use crate::grpc_server::nodeapi::{Empty, EnvironmentData};

pub fn launch(data_sink: Sender<(f32, f32)>) -> JoinHandle<()> {
    tokio::spawn(start_server(data_sink))
}

async fn start_server(data_sink: Sender<(f32, f32)>) {
    println!("Starting gRPC Server on http://localhost:50051");

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(nodeapi::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(NodeApiServer::new(NodeApiImpl { data_sink }))
        .serve("[::]:50051".parse().unwrap())
        .await.unwrap();
}

mod nodeapi {
    tonic::include_proto!("nodeapi");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("nodeapi_descriptor");
}

pub struct NodeApiImpl {
    data_sink: Sender<(f32, f32)>
}

#[tonic::async_trait]
impl NodeApi for NodeApiImpl {
    async fn report_environment(&self, request: Request<Streaming<EnvironmentData>>) -> Result<Response<Empty>, Status> {
        let mut stream = request.into_inner();
        while let Some(data_result) = stream.next().await {
            let data = data_result?;
            println!("Got Data: {:?}", data);
            self.data_sink.send((data.temperature, data.relative_humidity)).await.unwrap();
        }
        Ok(Response::new(nodeapi::Empty{}))
    }
}
