use crate::config::Config;
use crate::db::Database;
use crate::mailer::Mailer;
use crate::timer::RealTimer;
use crate::user_manager::UserManager;
use anyhow::Error;
use std::io;
use std::io::ErrorKind;
use std::process::exit;
use std::sync::Arc;

mod config;
mod db;
mod grpc_server;
mod http_server;
mod mailer;
mod timer;
mod user_manager;
mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    let config = load_config();
    let db = Arc::new(Database::load_default().await?);
    let mailer = Arc::new(Mailer::new(config.email.clone())?);
    let user_manager = Arc::new(UserManager::new(db.clone()));

    let http_server_handle = http_server::launch(config.clone(), db.clone(), user_manager.clone());
    let grpc_server_handle =
        grpc_server::launch(config.clone(), db.clone(), mailer.clone(), RealTimer::new());

    tokio::select! {
        _ = http_server_handle => { println!("HTTP Server Stopped! Shutting down."); }
        _ = grpc_server_handle => { println!("gRPC Server Stopped! Shutting down."); }
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
