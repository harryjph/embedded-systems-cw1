use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;
use tonic::{Request, Response, Status, Streaming};
use tonic::transport::Server;
use futures_util::StreamExt;
use self::grpc_generated::{Empty, EnvironmentData};
use self::grpc_generated::node_api_server::{NodeApi, NodeApiServer};

pub fn launch(data_sink: Sender<(f32, f32)>) -> JoinHandle<()> {
    tokio::spawn(start_server(data_sink))
}

async fn start_server(data_sink: Sender<(f32, f32)>) {
    println!("Starting gRPC Server on http://localhost:81");

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc_generated::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(NodeApiServer::new(NodeApiImpl { data_sink }))
        .serve("[::]:81".parse().unwrap())
        .await.unwrap();
}

mod grpc_generated {
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
            self.data_sink.send((data.temperature, data.relative_humidity)).await.unwrap();
        }
        Ok(Response::new(Empty::default()))
    }
}
