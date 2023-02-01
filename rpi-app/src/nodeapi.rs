use self::grpc_generated::node_api_client::NodeApiClient;
use self::grpc_generated::EnvironmentData;
use crate::util::Stream;
use std::error::Error;
use tokio::sync::mpsc::Receiver;
use tonic::codegen::StdError;
use tonic::transport::Channel;
use tonic::{transport, Status};

pub mod grpc_generated {
    tonic::include_proto!("nodeapi");
}

pub struct Client {
    client: NodeApiClient<Channel>,
}

impl Client {
    pub async fn new<E: TryInto<transport::Endpoint>>(endpoint: E) -> Result<Self, transport::Error>
    where
        E::Error: Into<StdError>,
    {
        Ok(Client {
            client: NodeApiClient::connect(endpoint).await?,
        })
    }

    pub async fn assign_id(&mut self) -> Result<u64, Box<dyn Error>> {
        let node_id = self
            .client
            .assign_id(grpc_generated::Empty {})
            .await?
            .into_inner();
        Ok(node_id.id)
    }

    pub async fn report_environment(
        &mut self,
        receiver: Receiver<EnvironmentData>,
    ) -> Result<(), Status> {
        self.client
            .report_environment(Stream::new(receiver))
            .await?;
        Ok(())
    }
}
