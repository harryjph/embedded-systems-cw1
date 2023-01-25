mod http_server;
mod grpc_server;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let http_server_handle = http_server::launch();
    let grpc_server_handle = grpc_server::launch();

    http_server_handle.await.unwrap();
    grpc_server_handle.await.unwrap();
}
