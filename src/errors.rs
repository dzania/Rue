use std::fmt;

pub enum ConfigError {
    CreateFileError(String),
    #[allow[dead_code)]
    FileReadError(String),
    HomeDirectoryNotFound(String),
}
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            ConfigError::CreateFileError(msg) => write!(f, "Create file error: {}", msg),
            ConfigError::FileReadError(msg) => write!(f, "Create config error: {}", msg),
            ConfigError::HomeDirectoryNotFound(msg) => {
                write!(f, "Home directory not found: {}", msg)
            }
        }
    }
}
