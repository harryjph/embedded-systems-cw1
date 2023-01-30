use std::io;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::{mpsc, RwLock};
use crate::config::Config;
use crate::db::Database;

mod config;
mod db;
mod http_server;
mod grpc_server;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let config = load_config();
    let db = Database::new().await.unwrap();

    let (data_in, mut data_out) = mpsc::channel(1);
    let lock = Arc::new(RwLock::new(Vec::new()));

    let http_server_handle = http_server::launch(config.clone(), lock.clone());
    let grpc_server_handle = grpc_server::launch(config.clone(), data_in);

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
    println!("lul");

    exit(3);
}

fn load_config() -> Config {
    match Config::load_default() {
        Ok(config) => return config,
        Err(e) => {
            match e.downcast::<io::Error>() {
                Ok(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        eprintln!("No config found. Example Config:");
                        eprintln!("{}", toml::to_string_pretty(&Config::default()).unwrap());
                        exit(1);
                    } else {
                        eprintln!("Failed to load config: {e}")
                    }
                },
                Err(e) => eprintln!("Failed to load config: {e}")
            }
        },
    }

    exit(2);
}
