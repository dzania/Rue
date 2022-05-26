use std::fmt;

pub enum ConfigError {
    CreateFileError(String),
    FileReadError(String),
    HomeDirectoryNotFound(String),
    Serialization(String),
    Deserialization(String),
}
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            ConfigError::CreateFileError(msg) => write!(f, "GRPC error: {}", msg),
            ConfigError::FileReadError(msg) => write!(f, "LDAP error: {}", msg),
            ConfigError::HomeDirectoryNotFound(msg) => write!(f, "Object not found: {}", msg),
            ConfigError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            ConfigError::Deserialization(msg) => write!(f, "Authorization error: {}", msg),
        }
    }
}
