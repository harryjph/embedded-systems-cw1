use tokio::task::JoinHandle;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::grpc_server::server::greeter_server::{Greeter, GreeterServer};
use crate::grpc_server::server::{HelloReply, HelloRequest};

pub fn launch() -> JoinHandle<()> {
    tokio::spawn(start_server())
}

async fn start_server() {
    let addr = "[::]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("Starting gRPC Server on http://localhost:50051");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await.unwrap();
}

mod server {
    tonic::include_proto!("server");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = server::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
