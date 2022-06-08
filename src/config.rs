use crate::errors::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

static FILE_PATH: &str = ".config/rue";
static CONFIG_NAME: &str = "rue.conf";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
}

impl User {
    pub fn create_path(&self) -> Result<(), ConfigError> {
        let home_dir = dirs::home_dir().ok_or(ConfigError::HomeDirectoryNotFound(
            "Can't find home directory".into(),
        ))?;
        let path = Path::new(&home_dir.join(FILE_PATH)).to_owned();
        fs::create_dir_all(&path).map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        Ok(())
    }
    // Store username(token) used for api calls
    pub async fn save(&self) -> Result<(), ConfigError> {
        self.create_path()?;
        let mut file = fs::File::create(format!("{}/{}", FILE_PATH, CONFIG_NAME))
            .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        file.write_all(&self.username.as_bytes())
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
                .join(FILE_PATH),
        )
        .map_err(|e| ConfigError::FileReadError(e.to_string()))?;
        let user = User { username };
        Ok(user)
    }
}
