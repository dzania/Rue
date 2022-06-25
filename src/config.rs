use crate::errors::ConfigError;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

const FILE_PATH: &str = ".config/rue";
const CONFIG_FILE_NAME: &str = "rue.conf";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
}

impl User {
    pub fn read_path() -> Result<PathBuf, ConfigError> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            ConfigError::HomeDirectoryNotFound("Can't find home directory".into())
        })?;
        // create dir
        let path = PathBuf::from(&home_dir.join(FILE_PATH));
        fs::create_dir_all(&path).map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        Ok(PathBuf::from(
            format!("{}/{}", FILE_PATH, CONFIG_FILE_NAME).as_str(),
        ))
    }
    // Store username(token) used for api calls
    pub async fn save(&self) -> Result<(), ConfigError> {
        let path = User::read_path()?;
        let mut file =
            fs::File::create(path).map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        file.write_all(self.username.as_bytes())
            .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;

        Ok(())
    }

    // Load username(token) used for api calls
    pub async fn load() -> Result<Self, ConfigError> {
        let path = User::read_path()?;
        let username =
            fs::read_to_string(path).map_err(|e| ConfigError::FileReadError(e.to_string()))?;
        let user = User { username };
        Ok(user)
    }
}
