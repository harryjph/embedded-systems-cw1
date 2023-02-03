use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};

const CONFIG_PATH_ENV_NAME: &str = "CONFIG_PATH";

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub network: NetworkConfig,
    pub email: EmailConfig,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfig {
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EmailSecurity {
    /// Initiates a TLS connection
    #[serde(rename = "tls")]
    TLS,
    /// Initiates a plaintext connection and upgrades using the "STARTTLS" command
    #[serde(rename = "starttls")]
    StartTLS,
    /// No encryption
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_server_address: String,
    #[serde(default = "default_smtp_port")]
    pub smtp_server_port: u16,
    #[serde(default = "default_smtp_security")]
    pub smtp_security: EmailSecurity,
    pub smtp_username: String,
    pub smtp_password: String,
}

fn default_http_port() -> u16 { 80 }
fn default_grpc_port() -> u16 { 81 }
fn default_smtp_port() -> u16 { 465 }
fn default_smtp_security() -> EmailSecurity { EmailSecurity::TLS }

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            http_port: default_http_port(),
            grpc_port: default_grpc_port(),
        }
    }
}

impl Default for EmailSecurity {
    fn default() -> Self {
        default_smtp_security()
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        EmailConfig {
            smtp_server_address: "example.com".to_string(),
            smtp_server_port: default_smtp_port(),
            smtp_security: Default::default(),
            smtp_username: "username".to_string(),
            smtp_password: "password".to_string(),
        }
    }
}

impl Config {
    fn load_from<S: AsRef<str>>(data: S) -> Result<Self, toml::de::Error> {
        toml::from_str(data.as_ref())
    }

    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        Ok(Config::load_from(fs::read_to_string(path)?)?)
    }

    pub fn load_default() -> Result<Config, Error> {
        let config_path = if let Ok(value) = env::var(CONFIG_PATH_ENV_NAME) {
            value
        } else {
            "config.toml".to_string()
        };
        Config::load_from_file(config_path)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, CONFIG_PATH_ENV_NAME};
    use std::env;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_save_load() {
        let _file = setup_default_config();
        assert_eq!(Config::load_default().unwrap(), Config::default());
    }

    /// Sets up the default config so that load_default() will load it.
    /// Returns the temp file as dropping it will delete the file, so it should be dropped at the end of the test.
    fn setup_default_config() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(toml::to_string(&Config::default()).unwrap().as_bytes())
            .unwrap();
        let file_path = file.path();
        env::set_var(CONFIG_PATH_ENV_NAME, file_path.to_str().unwrap());
        file
    }
}
