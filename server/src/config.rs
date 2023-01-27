use std::error::Error;
use std::{env, fs};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub email: EmailConfig,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub http_port: u16,
    pub grpc_port: u16,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_server_address: String,
    pub smtp_server_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            http_port: 80,
            grpc_port: 81,
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        EmailConfig {
            smtp_server_address: "example.com".to_string(),
            smtp_server_port: 465,
            smtp_username: "username".to_string(),
            smtp_password: "password".to_string(),
        }
    }
}

impl Config {
    fn load_from<S: AsRef<str>>(data: S) -> Result<Self, toml::de::Error> {
        toml::from_str(data.as_ref())
    }

    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        Ok(Config::load_from(fs::read_to_string(path)?.as_str())?)
    }

    pub fn load_default() -> Result<Config, Box<dyn Error>> {
        let config_path = if let Ok(value) = env::var("CONFIG_PATH") { value } else { "config.toml".to_string() };
        Config::load_from_file(config_path)
    }
}
