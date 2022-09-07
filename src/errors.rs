use std::fmt;

pub enum ConfigError {
    CreateFileError(String),
    FileReadError(String),
    HomeDirectoryNotFound(String),
    Serialize(String),
    CreateDirectoryError(String),
}
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::CreateFileError(msg) => {
                write!(f, "CONFIG_ERROR Create file error: {}", msg)
            }
            ConfigError::FileReadError(msg) => {
                write!(f, "CONFIG_ERROR Create config error: {}", msg)
            }
            ConfigError::HomeDirectoryNotFound(msg) => {
                write!(f, "CONFIG_ERROR Home directory not found: {}", msg)
            }
            ConfigError::Serialize(msg) => {
                write!(f, "CONFIG_ERROR Serialization error: {}", msg)
            }
            ConfigError::CreateDirectoryError(msg) => {
                write!(f, "CONFIG_ERROR create config dirctory: {}", msg)
            }
        }
    }
}

pub enum BridgeError {
    ButtonNotPressed,
    NoBridgesFound,
    RequestError(String),
    ResponseError(String),
}

impl fmt::Debug for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeError::ButtonNotPressed => {
                write!(f, "{:?}", &self)
            }
            BridgeError::NoBridgesFound => {
                write!(f, "{:?}", &self)
            }
            BridgeError::RequestError(msg) => {
                write!(f, "Request error: {}", msg)
            }
            BridgeError::ResponseError(msg) => {
                write!(f, "Response error: {}", msg)
            }
        }
    }
}
