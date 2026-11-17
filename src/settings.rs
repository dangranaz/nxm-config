use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

/// Supported database drivers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseKind {
    /// Embedded SQLite database.
    #[default]
    Sqlite,
    /// PostgreSQL database.
    #[serde(rename = "postgres")]
    Postgres,
    /// MongoDB database.
    #[serde(rename = "mongodb")]
    Mongo,
}

impl DatabaseKind {
    /// Stable configuration value of the driver.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sqlite => "sqlite",
            Self::Postgres => "postgres",
            Self::Mongo => "mongodb",
        }
    }
}

impl FromStr for DatabaseKind {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "sqlite" => Ok(Self::Sqlite),
            "postgres" | "postgresql" => Ok(Self::Postgres),
            "mongodb" | "mongo" => Ok(Self::Mongo),
            other => Err(ConfigError::UnsupportedDriver(other.to_string())),
        }
    }
}

/// HTTP server settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_origins: vec!["http://localhost:5173".to_string()],
        }
    }
}

/// Database connection settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub driver: DatabaseKind,
    pub url: String,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            driver: DatabaseKind::Sqlite,
            url: "data/nxm.db".to_string(),
        }
    }
}

/// OpenCode-style configuration generation settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationSettings {
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
}

impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            output: "provider/opencode.json".to_string(),
            default_model: None,
        }
    }
}

/// Complete NXM configuration.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct GeneralConfig {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub generation: GenerationSettings,
}

/// Raw representation used to parse configuration files with typed errors.
#[derive(Deserialize)]
pub(crate) struct RawGeneralConfig {
    #[serde(default)]
    pub(crate) server: ServerSettings,
    pub(crate) database: RawDatabaseSettings,
    #[serde(default)]
    pub(crate) generation: GenerationSettings,
}

#[derive(Deserialize)]
pub(crate) struct RawDatabaseSettings {
    #[serde(default = "default_driver")]
    pub(crate) driver: String,
    #[serde(default = "default_url")]
    pub(crate) url: String,
}

fn default_driver() -> String {
    DatabaseKind::default().as_str().to_string()
}

fn default_url() -> String {
    "data/nxm.db".to_string()
}

impl<'de> Deserialize<'de> for GeneralConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawGeneralConfig::deserialize(deserializer)?;
        let driver = raw
            .database
            .driver
            .parse::<DatabaseKind>()
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            server: raw.server,
            database: DatabaseSettings {
                driver,
                url: raw.database.url,
            },
            generation: raw.generation,
        })
    }
}

impl GeneralConfig {
    /// Applies supported environment-variable overrides and validates the result.
    pub fn with_env_overrides(mut self) -> Result<Self, ConfigError> {
        if let Some(value) = env::var_os("NXM_DATABASE_DRIVER") {
            self.database.driver = value.to_string_lossy().parse()?;
        }
        if let Some(value) = env::var_os("NXM_DATABASE_URL") {
            self.database.url = value.to_string_lossy().into_owned();
        }
        if let Some(value) = env::var_os("NXM_SERVER_HOST") {
            self.server.host = value.to_string_lossy().into_owned();
        }
        if let Some(value) = env::var_os("NXM_SERVER_PORT") {
            self.server.port = value
                .to_string_lossy()
                .parse()
                .map_err(|_| ConfigError::InvalidValue("NXM_SERVER_PORT must be a u16".into()))?;
        }
        if let Some(value) = env::var_os("NXM_CORS_ORIGINS") {
            self.server.cors_origins = value
                .to_string_lossy()
                .split(',')
                .map(str::trim)
                .filter(|origin| !origin.is_empty())
                .map(str::to_string)
                .collect();
        }
        self.validate()?;
        Ok(self)
    }

    /// Validates all configuration values.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.server.host.trim().is_empty() {
            return Err(ConfigError::InvalidValue("server.host is empty".into()));
        }
        if self.server.port == 0 {
            return Err(ConfigError::InvalidValue(
                "server.port must be positive".into(),
            ));
        }
        if self.server.cors_origins.is_empty() {
            return Err(ConfigError::InvalidValue(
                "server.cors_origins is empty".into(),
            ));
        }
        if self.database.url.trim().is_empty() {
            return Err(ConfigError::InvalidValue("database.url is empty".into()));
        }
        if self.generation.output.trim().is_empty() {
            return Err(ConfigError::InvalidValue(
                "generation.output is empty".into(),
            ));
        }
        Ok(())
    }
}
