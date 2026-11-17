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
mod paths;
mod settings;

pub use error::ConfigError;
pub use loader::{load_config, load_or_create_config, save_config};
pub use paths::ConfigPaths;
pub use settings::{
    DatabaseKind, DatabaseSettings, GeneralConfig, GenerationSettings, ServerSettings,
};
