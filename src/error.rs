use thiserror::Error;

/// Errors produced while discovering or loading NXM configuration.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    /// The home directory could not be determined.
    #[error("unable to determine the home directory")]
    HomeNotFound,
    /// A filesystem operation on the configuration directory failed.
    #[error("filesystem operation failed: {0}")]
    Io(String),
    /// The configuration file could not be parsed.
    #[error("invalid configuration file: {0}")]
    Parse(String),
    /// The database driver is not supported.
    #[error("unsupported database driver: {0}")]
    UnsupportedDriver(String),
    /// A configuration value is invalid.
    #[error("invalid configuration value: {0}")]
    InvalidValue(String),
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
