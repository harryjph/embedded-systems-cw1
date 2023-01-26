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

    tokio::select! {
        _ = http_server_handle => { println!("HTTP Server Stopped! Shutting down."); }
        _ = grpc_server_handle => { println!("gRPC Server Stopped! Shutting down."); }
        _ = data_handler_handle => { println!("Data Handler Stopped! Shutting down."); }
    }
}
