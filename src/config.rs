use crate::errors::ConfigError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
}
impl User {
    // Store username(token) used for api calls
    pub async fn save(&self) -> Result<(), ConfigError> {
        let mut file = fs::File::create(
            dirs::home_dir()
                .ok_or(ConfigError::HomeDirectoryNotFound(
                    "Can't find home directory".into(),
                ))?
                .join(".config/rue.conf"),
        )
        .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        let data =
            serde_json::to_string(self).map_err(|e| ConfigError::Serialization(e.to_string()))?;
        file.write_all(data.as_bytes())
            .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;

        Ok(())
    }

    // Load username(token) used for api calls
    pub async fn load() -> Result<Self, ConfigError> {
        let username = fs::read_to_string(
            dirs::home_dir()
                .ok_or(ConfigError::HomeDirectoryNotFound(
                    "Can't find home directory".into(),
                ))?
                .join(".config/rue.conf"),
        )
        .map_err(|e| ConfigError::FileReadError(e.to_string()))?;
        let user = User {
            username: serde_json::from_str(&username)
                .map_err(|e| ConfigError::Deserialization(e.to_string()))?,
        };
        Ok(user)
    }
}
