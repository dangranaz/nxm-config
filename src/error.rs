use std::path::PathBuf;

use thiserror::Error;

/// Errors produced while discovering or loading NXM configuration.
#[derive(Debug, Error)]
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
    /// MCP server name not provided (neither as argument nor via env `NXM_MCP_SERVER`).
    #[error("MCP server name missing: pass it as argument or set the NXM_MCP_SERVER environment variable")]
    McpServerNameMissing,
    /// Impossible to create the `~/.nxmconfig` directory.
    #[error("impossible to create the configuration directory {0}: {1}")]
    McpConfigDirCreateFailed(PathBuf, std::io::Error),
    /// No `<name>_config.json` nor `<name>_config.toml` file found.
    #[error("MCP configuration file not found for {0}")]
    McpConfigFileNotFound(PathBuf),
    /// Unsupported file extension for MCP configuration.
    #[error("unsupported extension for MCP configuration file: {0} (use .json or .toml)")]
    McpUnsupportedExtension(PathBuf),
    /// Invalid JSON in MCP configuration file.
    #[error("invalid JSON in MCP configuration file {0}: {1}")]
    McpInvalidJson(PathBuf, serde_json::Error),
    /// Invalid TOML in MCP configuration file.
    #[error("invalid TOML in MCP configuration file {0}: {1}")]
    McpInvalidToml(PathBuf, toml::de::Error),
    /// I/O error reading an MCP configuration file.
    #[error("I/O error reading MCP configuration file {0}: {1}")]
    McpConfigIoError(PathBuf, std::io::Error),
    /// I/O error writing an MCP configuration file.
    #[error("I/O error writing MCP configuration file {0}: {1}")]
    McpWriteFailed(PathBuf, std::io::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
