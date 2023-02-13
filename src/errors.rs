use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Error creating config file")]
    CreateFileError(String),
    #[error("Error reading config file")]
    FileReadError(String),
    #[error("Home directory not found")]
    HomeDirectoryNotFound(String),
    #[error("Error serializing config file")]
    Serialize(String),
    #[error("Error creating config directory")]
    CreateDirectoryError(String),
}

#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Bridge button not pressed")]
    ButtonNotPressed,
    #[error("No bridges found")]
    NoBridgesFound,
    #[error("Error sending request")]
    RequestError(String),
    #[error("Error occured in response")]
    ResponseError(String),
    #[error("Save user error")]
    SaveUser(String),
    #[error("Internal error")]
    InternalError(String),
}

impl From<reqwest::Error> for BridgeError {
    fn from(error: reqwest::Error) -> Self {
        BridgeError::RequestError(error.to_string())
    }
}
