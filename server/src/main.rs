mod http_server;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let http_server_handle = http_server::launch();
    http_server_handle.await.unwrap();
}
