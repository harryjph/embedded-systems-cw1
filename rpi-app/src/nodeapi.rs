use tokio::sync::mpsc::{Sender, Receiver};
use tonic::codegen::StdError;
use tonic::{Status, transport};
use tonic::transport::Channel;
use crate::util::Stream;
use self::grpc_generated::EnvironmentData;
use self::grpc_generated::node_api_client::NodeApiClient;

pub mod grpc_generated {
    tonic::include_proto!("nodeapi");
}

pub struct Client {
    client: NodeApiClient<Channel>,
}

impl Client {
    pub async fn new<E: TryInto<transport::Endpoint>>(endpoint: E) -> Result<Self, transport::Error>
        where E::Error: Into<StdError> {
        Ok(Client { client: NodeApiClient::connect(endpoint).await? })
    }

    pub async fn assign_id(&mut self, sender: Sender<u32>) -> Result<(), Status> {
        let id = self.client.assign_id(grpc_generated::Empty{}).await?.into_inner();
        sender.send(id.id).await;
        println!("Got assigned an id: {}", id.id);
        Ok(())
    }

    pub async fn report_environment(&mut self, receiver: Receiver<EnvironmentData>) -> Result<(), Status> {
        self.client.report_environment(Stream::new(receiver)).await?;
        Ok(())
    }
}
