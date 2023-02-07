use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};

const CONFIG_PATH_ENV_NAME: &str = "CONFIG_PATH";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub id: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            id: None,
            url: "http://es1-nodeapi.harryphillips.co.uk:81".to_string(),
        }
    }
}

impl Config {
    fn config_path() -> String {
        if let Ok(value) = env::var(CONFIG_PATH_ENV_NAME) {
            value
        } else {
            "config.toml".to_string()
        }
    }

    fn load_from<S: AsRef<str>>(data: S) -> Result<Self, toml::de::Error> {
        toml::from_str(data.as_ref())
    }

    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        Ok(Config::load_from(fs::read_to_string(path)?)?)
    }

    fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        fs::write(path, toml::to_string(&self)?)?;
        Ok(())
    }

    pub fn write_default(&self) -> Result<(), Error> {
        self.write_to_file(Config::config_path())
    }

    pub fn load_default() -> Result<Config, Error> {
        Config::load_from_file(Config::config_path())
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
