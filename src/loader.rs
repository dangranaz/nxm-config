use crate::error::ConfigError;
use crate::paths::ConfigPaths;
use crate::settings::{DatabaseSettings, GeneralConfig, RawGeneralConfig};
use std::fs;

/// Loads and validates the configuration file.
pub fn load_config(paths: &ConfigPaths) -> Result<GeneralConfig, ConfigError> {
    tracing::info!(target: "nxm::config::load", path = %paths.config_file().display(), "loading configuration");
    let contents = fs::read_to_string(paths.config_file())
        .map_err(|error| ConfigError::Parse(error.to_string()))?;
    let raw: RawGeneralConfig =
        toml::from_str(&contents).map_err(|error| ConfigError::Parse(error.to_string()))?;
    let config = GeneralConfig {
        server: raw.server,
        database: DatabaseSettings {
            driver: raw.database.driver.parse()?,
            url: raw.database.url,
        },
        generation: raw.generation,
    };
    config.with_env_overrides()
}

/// Loads configuration, creating a default file when none exists.
pub fn load_or_create_config(paths: &ConfigPaths) -> Result<GeneralConfig, ConfigError> {
    paths.ensure_directories()?;
    if !paths.config_file().exists() {
        save_config(paths, &GeneralConfig::default())?;
    }
    load_config(paths)
}

/// Writes a validated configuration file.
pub fn save_config(paths: &ConfigPaths, config: &GeneralConfig) -> Result<(), ConfigError> {
    config.validate()?;
    if let Some(parent) = paths.config_file().parent() {
        fs::create_dir_all(parent)?;
    }
    let contents =
        toml::to_string_pretty(config).map_err(|error| ConfigError::Parse(error.to_string()))?;
    fs::write(paths.config_file(), contents)?;
    tracing::info!(target: "nxm::config::save", path = %paths.config_file().display(), "configuration saved");
    Ok(())
}
