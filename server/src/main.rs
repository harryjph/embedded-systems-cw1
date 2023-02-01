use crate::config::Config;
use crate::db::Database;
use crate::mailer::Mailer;
use std::io;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::Arc;
use anyhow::Error;
use tokio::spawn;
use tokio::sync::{mpsc, RwLock};

mod config;
mod db;
mod grpc_server;
mod http_server;
mod mailer;
mod user_manager;
mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    let config = load_config();
    let db = Arc::new(Database::new().await?);
    let mailer = Mailer::new(config.email.clone())?;
    let user_manager = UserManager::new(db.clone());

    let (data_in, mut data_out) = mpsc::channel(1);
    let lock = Arc::new(RwLock::new(Vec::new()));

    let http_server_handle = http_server::launch(config.clone(), lock.clone());
    let grpc_server_handle = grpc_server::launch(config.clone(), data_in, db.clone());

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

    exit(3);
}

fn load_config() -> Config {
    match Config::load_default() {
        Ok(config) => return config,
        Err(e) => match e.downcast::<io::Error>() {
            Ok(e) => {
                if e.kind() == ErrorKind::NotFound {
                    eprintln!("No config found. Example Config:");
                    eprintln!("{}", toml::to_string_pretty(&Config::default()).unwrap());
                    exit(1);
                } else {
                    eprintln!("Failed to load config: {e}")
                }
            }
            Err(e) => eprintln!("Failed to load config: {e}"),
        },
    }

    exit(2);
}
