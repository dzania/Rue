use crate::errors::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufReader, Write},
    net::IpAddr,
    path::{Path, PathBuf},
};

const CONFIG_DIR: &str = ".config/rue";
const CONFIG_NAME: &str = "rue.conf";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub bridge_address: IpAddr,
}

impl User {
    fn get_config_path() -> Result<PathBuf, ConfigError> {
        match dirs::home_dir() {
            Some(home) => {
                let config_dir_path = Path::new(&home.join(CONFIG_DIR)).to_owned();
                if !config_dir_path.exists() {
                    fs::create_dir(&config_dir_path)
                        .map_err(|e| ConfigError::CreateDirectoryError(e.to_string()))?;
                };
                Ok(config_dir_path.join(CONFIG_NAME))
            }
            None => Err(ConfigError::HomeDirectoryNotFound(
                "No $HOME directory found for config".into(),
            )),
        }
    }

    // Store username(token) used for api calls
    pub async fn save(&self) -> Result<(), ConfigError> {
        let config_file_path = User::get_config_path()?;
        let mut file = fs::File::create(&config_file_path)
            .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;

        let json: String =
            serde_json::to_string(&self).map_err(|e| ConfigError::Serialize(e.to_string()))?;

        file.write_all(json.as_bytes())
            .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;

        Ok(())
    }

    // Load user used for api calls
    pub fn load() -> Result<Self, ConfigError> {
        let file_path = User::get_config_path()?;
        let file =
            fs::File::open(file_path).map_err(|e| ConfigError::FileReadError(e.to_string()))?;
        let reader = BufReader::new(file);
        let user = serde_json::from_reader(reader)
            .map_err(|e| ConfigError::FileReadError(e.to_string()))?;

        Ok(user)
    }
}
