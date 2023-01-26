use std::error::Error;
use tonic::codegen::StdError;
use tonic::{IntoStreamingRequest, Status, transport};
use tonic::transport::Channel;
use self::grpc_generated::EnvironmentData;
use self::grpc_generated::node_api_client::NodeApiClient;

pub mod grpc_generated {
    tonic::include_proto!("nodeapi");
}

pub struct Client {
    client: NodeApiClient<Channel>,
}

impl Client {
    async fn new<E: TryInto<transport::Endpoint>>(endpoint: E) -> Result<Self, transport::Error>
        where E::Error: Into<StdError> {
        Ok(Client { client: NodeApiClient::connect(endpoint).await? })
    }

    async fn start_data_stream<S>(&mut self, stream: S) -> Result<(), Status>
        where S: IntoStreamingRequest<Message = EnvironmentData> {
        self.client.report_environment(stream)
            .await?;
        Ok(())
    }
}
