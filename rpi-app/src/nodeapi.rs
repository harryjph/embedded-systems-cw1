use self::grpc_generated::node_api_client::NodeApiClient;
use self::grpc_generated::SensorData;
use crate::nodeapi::grpc_generated::Empty;
use crate::util::Stream;
use anyhow::Error;
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

    pub async fn assign_id(&mut self) -> Result<u32, Error> {
        let node_id = self.client.assign_id(Empty {}).await?.into_inner();
        Ok(node_id.id)
    }

    pub async fn stream_sensor_data(
        &mut self,
        receiver: Receiver<SensorData>,
    ) -> Result<(), Status> {
        self.client
            .stream_sensor_data(Stream::new(receiver))
            .await?;
        Ok(())
    }
}
