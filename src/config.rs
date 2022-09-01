use crate::errors::ConfigError;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

const FILE_PATH: &str = ".config/rue";
const CONFIG_NAME: &str = "rue.conf";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    bridge_adress: String,
}

impl User {
    fn get_home_dir() -> Result<PathBuf, ConfigError> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            ConfigError::HomeDirectoryNotFound("Can't find home directory".into())
        })?;
        Ok(home_dir)
    }
    pub fn exists() -> bool {
        Path::new(&dirs::home_dir().unwrap().join(".config/rue/rue.conf")).exists()
    }

    pub fn create_path(&self) -> Result<(), ConfigError> {
        let home_dir = User::get_home_dir()?;
        let path = Path::new(&home_dir.join(FILE_PATH)).to_owned();
        fs::create_dir_all(&path).map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        Ok(())
    }
    // Store username(token) used for api calls
    pub async fn save(&self) -> Result<(), ConfigError> {
        self.create_path()?;
        let home_dir = User::get_home_dir()?;
        let mut file = fs::File::create(home_dir.join(FILE_PATH).join(CONFIG_NAME))
            .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        file.write_all(self.username.as_bytes())
            .map_err(|e| ConfigError::CreateFileError(e.to_string()))?;
        println!("User saved");
        Ok(())
    }
    // Load username(token) used for api calls
    pub fn load() -> Result<Self, ConfigError> {
        let user = fs::read_to_string(User::get_home_dir()?.join(FILE_PATH))
            .map_err(|e| ConfigError::FileReadError(e.to_string()))?;
        Ok(user)
    }
}
