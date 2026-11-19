//! Shared configuration types and filesystem layout for the NXM platform.
//!
//! # Examples
//!
//! ```
//! use nxm_config::{load_or_create_config, ConfigPaths};
//! let paths = ConfigPaths::with_root("/tmp/nxm-example");
//! let config = load_or_create_config(&paths).expect("default config is valid");
//! assert_eq!(config.server.port, 8080);
//! ```

mod error;
mod loader;
mod mcp_config;
mod paths;
mod settings;

pub use error::ConfigError;
pub use loader::{load_config, load_or_create_config, save_config};
pub use mcp_config::{
    config_dir, config_dir_in, load_mcp_config, load_server_config, load_server_config_in,
    resolve_general_config_in, resolve_server_config, resolve_server_config_in, server_config_path,
    workspace_prj, workspace_prj_in, write_general_config, write_general_config_in,
    write_server_config, write_server_config_in,
};
pub use mcp_config::{ConfigFormat, McpConfig, MCP_SERVER_ENV};
pub use paths::ConfigPaths;
pub use settings::{
    DatabaseKind, DatabaseSettings, GeneralConfig, GenerationSettings, ServerSettings,
};
