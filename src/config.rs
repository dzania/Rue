use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
}

enum ConfigError {
    CreateFileError(std::io::Error),
    ReadError,
    HomeDirectoryNotFound,
    SerializationError(serde_json::Error),
}

const PATH: Option<std::path::PathBuf> = dirs::home_dir();

impl User {
    // Store username(token) used for api calls
    pub async fn save(&self) -> Result<(), ConfigError> {
        let mut file = fs::File::create(
            PATH.ok_or(ConfigError::HomeDirectoryNotFound)?
                .join(".config/rue.conf"),
        )
        .map_err(|e| ConfigError::CreateFileError(e))?;
        let data = serde_json::to_string(self).map_err(|e| ConfigError::SerializationError(e))?;
        file.write_all(data.as_bytes())
            .map_err(|e| ConfigError::CreateFileError(e))?;

        Ok(())
    }

    // Load username(token) used for api calls
    pub async fn load() -> Result<(), ()> {
        Ok(())
    }
}
