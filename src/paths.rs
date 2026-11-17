use crate::error::ConfigError;
use std::env;
use std::path::{Path, PathBuf};

/// Filesystem locations used by the NXM platform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigPaths {
    root: PathBuf,
}

impl ConfigPaths {
    /// Builds paths rooted at `root`, useful for tests and isolated deployments.
    ///
    /// # Examples
    ///
    /// ```
    /// use nxm_config::ConfigPaths;
    /// let paths = ConfigPaths::with_root("/tmp/nxm");
    /// assert_eq!(paths.provider_dir(), std::path::PathBuf::from("/tmp/nxm/provider"));
    /// ```
    pub fn with_root(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Discovers the configuration root from `NXM_CONFIG_HOME` or `$HOME/.nxmconfig`.
    pub fn discover() -> Result<Self, ConfigError> {
        if let Some(root) = env::var_os("NXM_CONFIG_HOME") {
            return Ok(Self::with_root(PathBuf::from(root)));
        }
        let home = env::var_os("HOME").ok_or(ConfigError::HomeNotFound)?;
        Ok(Self::with_root(PathBuf::from(home).join(".nxmconfig")))
    }

    /// Creates all required directories if they do not exist.
    pub fn ensure_directories(&self) -> Result<(), ConfigError> {
        fs_create_dir_all(&self.provider_dir())?;
        fs_create_dir_all(&self.logs_dir())?;
        fs_create_dir_all(&self.data_dir())?;
        Ok(())
    }

    /// Directory containing generated provider configuration.
    pub fn provider_dir(&self) -> PathBuf {
        self.root.join("provider")
    }

    /// Directory containing rotating logs.
    pub fn logs_dir(&self) -> PathBuf {
        self.root.join("logs")
    }

    /// Directory containing application data.
    pub fn data_dir(&self) -> PathBuf {
        self.root.join("data")
    }

    /// Path of the general configuration file.
    pub fn config_file(&self) -> PathBuf {
        self.root.join("config.toml")
    }

    /// Resolves `value` relative to the configuration root when it is not absolute.
    pub fn resolve(&self, value: impl AsRef<Path>) -> PathBuf {
        let value = value.as_ref();
        if value.is_absolute() {
            value.to_path_buf()
        } else {
            self.root.join(value)
        }
    }
}

fn fs_create_dir_all(path: &Path) -> Result<(), ConfigError> {
    std::fs::create_dir_all(path).map_err(|error| ConfigError::Io(error.to_string()))
}
