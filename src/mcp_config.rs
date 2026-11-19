//! MCP-server configuration helpers merged into `nxm-config`.
//!
//! Standard di mercato: **JSON** (file `mcp.json` usato da Claude Desktop,
//! VS Code, Cursor; il protocollo MCP stesso è JSON-RPC). Il crate accetta
//! anche TOML per comodità nell'ecosistema Rust, ma JSON è il formato canonico.
//!
//! ## Convenzione di path
//! I file di configurazione vivono in `~/.nxmconfig/` (directory nascosta nella
//! home dell'utente, creata automaticamente se assente). Per un server chiamato
//! `<nome>` il file atteso è:
//!   `~/.nxmconfig/<nome>_config.json`   (preferito, standard di mercato)
//!   `~/.nxmconfig/<nome>_config.toml`   (accettato)
//!
//! Il nome del server si passa come parametro, oppure si legge dalla variabile
//! d'ambiente `NXM_MCP_SERVER`.
//!
//! Se la directory o il file mancano, l'avvio è bloccato con un errore esplicito.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ConfigError;

/// Variabile d'ambiente che contiene il nome del server (usata da [`load_mcp_config`]).
pub const MCP_SERVER_ENV: &str = "NXM_MCP_SERVER";

/// Directory nascosta nella home dell'utente.
pub const CONFIG_DIR_NAME: &str = ".nxmconfig";

/// Suffisso del file di configurazione (senza estensione).
pub const CONFIG_FILE_SUFFIX: &str = "_config";

/// Nome del file di configurazione generale (senza estensione).
pub const GENERAL_CONFIG_NAME: &str = "general_config";

/// Chiave del parametro che indica la directory di input del progetto.
pub const WORKSPACE_PRJ_KEY: &str = "workspace_prj";

/// Formato del file di configurazione risolto dall'estensione.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    Toml,
}

/// Configurazione MCP caricata e validata.
#[derive(Debug, Clone)]
pub struct McpConfig {
    pub server_name: String,
    pub path: PathBuf,
    pub format: ConfigFormat,
    pub raw: String,
}

/// Risolve la home directory dell'utente.
fn home_dir() -> Result<PathBuf, ConfigError> {
    env::home_dir().ok_or(ConfigError::HomeNotFound)
}

/// Risolve la directory `base/.nxmconfig`, creandola se assente.
pub fn config_dir_in(base: &Path) -> Result<PathBuf, ConfigError> {
    let dir = base.join(CONFIG_DIR_NAME);
    if !dir.is_dir() {
        fs::create_dir_all(&dir)
            .map_err(|e| ConfigError::McpConfigDirCreateFailed(dir.clone(), e))?;
    }
    Ok(dir)
}

/// Risolve la directory `~/.nxmconfig`, creandola se assente.
pub fn config_dir() -> Result<PathBuf, ConfigError> {
    let home = home_dir()?;
    config_dir_in(&home)
}

/// Risolve il path e il formato del file di config per un dato server dentro `base`.
/// Preferisce `.json` (standard di mercato) su `.toml`.
pub fn resolve_server_config_in(
    server_name: &str,
    base: &Path,
) -> Result<(PathBuf, ConfigFormat), ConfigError> {
    let dir = config_dir_in(base)?;
    let base_file = dir.join(format!("{server_name}{CONFIG_FILE_SUFFIX}"));

    let json_path = base_file.with_extension("json");
    let toml_path = base_file.with_extension("toml");

    match (json_path.is_file(), toml_path.is_file()) {
        (true, _) => Ok((json_path, ConfigFormat::Json)),
        (false, true) => Ok((toml_path, ConfigFormat::Toml)),
        (false, false) => Err(ConfigError::McpConfigFileNotFound(json_path)),
    }
}

/// Risolve il path e il formato del file di config per un dato server.
/// Preferisce `.json` (standard di mercato) su `.toml`.
pub fn resolve_server_config(server_name: &str) -> Result<(PathBuf, ConfigFormat), ConfigError> {
    let home = home_dir()?;
    resolve_server_config_in(server_name, &home)
}

/// Carica e valida la configurazione per `server_name` dentro `base`.
pub fn load_server_config_in(server_name: &str, base: &Path) -> Result<McpConfig, ConfigError> {
    let (path, format) = resolve_server_config_in(server_name, base)?;
    let raw =
        fs::read_to_string(&path).map_err(|e| ConfigError::McpConfigIoError(path.clone(), e))?;

    match format {
        ConfigFormat::Json => {
            serde_json::from_str::<serde_json::Value>(&raw)
                .map_err(|e| ConfigError::McpInvalidJson(path.clone(), e))?;
        }
        ConfigFormat::Toml => {
            toml::from_str::<toml::Value>(&raw)
                .map_err(|e| ConfigError::McpInvalidToml(path.clone(), e))?;
        }
    }

    Ok(McpConfig {
        server_name: server_name.to_string(),
        path,
        format,
        raw,
    })
}

/// Carica e valida la configurazione per `server_name`.
pub fn load_server_config(server_name: &str) -> Result<McpConfig, ConfigError> {
    let home = home_dir()?;
    load_server_config_in(server_name, &home)
}

/// Comodità: legge il nome server da `NXM_MCP_SERVER` e carica la config.
pub fn load_mcp_config() -> Result<McpConfig, ConfigError> {
    let server_name = env::var(MCP_SERVER_ENV).map_err(|_| ConfigError::McpServerNameMissing)?;
    load_server_config(&server_name)
}

/// Restituisce solo il percorso risolto, o un errore bloccante.
pub fn server_config_path(server_name: &str) -> Result<PathBuf, ConfigError> {
    resolve_server_config(server_name).map(|(p, _)| p)
}

/// Scrive il contenuto TOML nel file `base/.nxmconfig/<nome>_config.toml`.
/// Crea la directory se assente. Sovrascrive se esiste.
pub fn write_server_config_in(
    server_name: &str,
    content: &str,
    base: &Path,
) -> Result<PathBuf, ConfigError> {
    let dir = config_dir_in(base)?;
    let path = dir.join(format!("{server_name}{CONFIG_FILE_SUFFIX}.toml"));
    fs::write(&path, content).map_err(|e| ConfigError::McpWriteFailed(path.clone(), e))?;
    Ok(path)
}

/// Scrive il contenuto TOML nel file `~/.nxmconfig/<nome>_config.toml`.
/// Crea la directory `~/.nxmconfig/` se assente. Sovrascrive se esiste.
pub fn write_server_config(server_name: &str, content: &str) -> Result<PathBuf, ConfigError> {
    let home = home_dir()?;
    write_server_config_in(server_name, content, &home)
}

/// Risolve il path e il formato del file di configurazione generale dentro `base`.
/// Preferisce `.json` su `.toml`. Restituisce `None` se nessuno dei due esiste.
pub fn resolve_general_config_in(
    base: &Path,
) -> Result<Option<(PathBuf, ConfigFormat)>, ConfigError> {
    let dir = config_dir_in(base)?;
    let json_path = dir.join(format!("{GENERAL_CONFIG_NAME}.json"));
    let toml_path = dir.join(format!("{GENERAL_CONFIG_NAME}.toml"));

    match (json_path.is_file(), toml_path.is_file()) {
        (true, _) => Ok(Some((json_path, ConfigFormat::Json))),
        (false, true) => Ok(Some((toml_path, ConfigFormat::Toml))),
        (false, false) => Ok(None),
    }
}

/// Legge il valore di `workspace_prj` dalla configurazione generale.
///
/// Se il file di configurazione generale manca, o non contiene il parametro
/// `workspace_prj`, restituisce la directory di default `base/.nxmconfig`
/// (cioè `$HOME/.nxmconfig` quando `base` è la home dell'utente).
pub fn workspace_prj_in(base: &Path) -> Result<PathBuf, ConfigError> {
    let default = base.join(CONFIG_DIR_NAME);
    let Some((path, format)) = resolve_general_config_in(base)? else {
        return Ok(default);
    };
    let raw =
        fs::read_to_string(&path).map_err(|e| ConfigError::McpConfigIoError(path.clone(), e))?;
    let found: Option<String> = match format {
        ConfigFormat::Json => {
            let v: serde_json::Value = serde_json::from_str(&raw)
                .map_err(|e| ConfigError::McpInvalidJson(path.clone(), e))?;
            v.get(WORKSPACE_PRJ_KEY)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        }
        ConfigFormat::Toml => {
            let v: toml::Value =
                toml::from_str(&raw).map_err(|e| ConfigError::McpInvalidToml(path.clone(), e))?;
            v.get(WORKSPACE_PRJ_KEY)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        }
    };
    match found {
        Some(s) if !s.is_empty() => Ok(PathBuf::from(s)),
        _ => Ok(default),
    }
}

/// Legge `workspace_prj` dalla configurazione generale (`~/.nxmconfig`).
pub fn workspace_prj() -> Result<PathBuf, ConfigError> {
    let home = home_dir()?;
    workspace_prj_in(&home)
}

/// Scrive il contenuto nel file di configurazione generale
/// `base/.nxmconfig/general_config.toml`. Crea la directory se assente.
pub fn write_general_config_in(content: &str, base: &Path) -> Result<PathBuf, ConfigError> {
    let dir = config_dir_in(base)?;
    let path = dir.join(format!("{GENERAL_CONFIG_NAME}.toml"));
    fs::write(&path, content).map_err(|e| ConfigError::McpWriteFailed(path.clone(), e))?;
    Ok(path)
}

/// Scrive il contenuto nel file di configurazione generale `~/.nxmconfig/general_config.toml`.
/// Crea la directory `~/.nxmconfig/` se assente. Sovrascrive se esiste.
pub fn write_general_config(content: &str) -> Result<PathBuf, ConfigError> {
    let home = home_dir()?;
    write_general_config_in(content, &home)
}
