use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
}

pub enum ConfigError {
    CreateFileError(std::io::Error),
    FileReadError,
    HomeDirectoryNotFound,
    SerializeError(serde_json::Error),
    DeserializeError(serde_json::Error),
}

impl User {
    // Store username(token) used for api calls
    pub async fn save(&self) -> Result<(), ConfigError> {
        let mut file = fs::File::create(
            dirs::home_dir()
                .ok_or(ConfigError::HomeDirectoryNotFound)?
                .join(".config/rue.conf"),
        )
        .map_err(|e| ConfigError::CreateFileError(e))?;
        let data = serde_json::to_string(self).map_err(|e| ConfigError::SerializeError(e))?;
        file.write_all(data.as_bytes())
            .map_err(|e| ConfigError::CreateFileError(e))?;

        Ok(())
    }

    // Load username(token) used for api calls
    pub async fn load() -> Result<Self, ConfigError> {
        let username = fs::read_to_string(
            dirs::home_dir()
                .ok_or(ConfigError::HomeDirectoryNotFound)?
                .join(".config/rue.conf"),
        )
        .map_err(|_| ConfigError::FileReadError)?;
        let user = User {
            username: serde_json::from_str(&username)
                .map_err(|e| ConfigError::DeserializeError(e))?,
        };
        Ok(user)
    }
}
