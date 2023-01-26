use std::sync::Arc;
use tokio::spawn;
use tokio::sync::{mpsc, RwLock};

mod http_server;
mod grpc_server;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let (data_in, mut data_out) = mpsc::channel(1);
    let lock = Arc::new(RwLock::new(Vec::new()));

    let http_server_handle = http_server::launch(lock.clone());
    let grpc_server_handle = grpc_server::launch(data_in);

    let data_handler_handle = spawn(async move {
        while let Some(data) = data_out.recv().await {
            lock.write().await.push(data);
        }
    });

    // TODO shutdown if any service stops
    http_server_handle.await.unwrap();
    grpc_server_handle.await.unwrap();
    data_handler_handle.await.unwrap();
}
