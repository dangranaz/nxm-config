use nxm_config::{load_config, save_config, ConfigError, ConfigPaths, DatabaseKind, GeneralConfig};
use std::fs;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn temporary_root(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("nxm-config-{}-{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&root);
    root
}

#[test]
fn save_then_load_preserves_default_configuration() {
    let _guard = ENV_LOCK
        .lock()
        .expect("environment lock should be acquired");
    let root = temporary_root("roundtrip");
    let paths = ConfigPaths::with_root(&root);
    paths
        .ensure_directories()
        .expect("directories should be created");

    let config = GeneralConfig::default();
    save_config(&paths, &config).expect("config should be saved");
    let loaded = load_config(&paths).expect("config should be loaded");

    assert_eq!(loaded, config);
    assert_eq!(loaded.database.driver, DatabaseKind::Sqlite);
    assert_eq!(loaded.server.port, 8080);

    fs::remove_dir_all(&root).expect("temporary root should be removed");
}

#[test]
fn load_rejects_unknown_database_driver() {
    let _guard = ENV_LOCK
        .lock()
        .expect("environment lock should be acquired");
    let root = temporary_root("bad-driver");
    let paths = ConfigPaths::with_root(&root);
    paths
        .ensure_directories()
        .expect("directories should be created");
    fs::write(
        paths.config_file(),
        "[server]\nhost = \"127.0.0.1\"\nport = 8080\ncors_origins = [\"http://localhost:5173\"]\n\
         [database]\ndriver = \"oracle\"\nurl = \"data/nxm.db\"\n\
         [generation]\noutput = \"provider/opencode.json\"\n",
    )
    .expect("config file should be written");

    let error = load_config(&paths).expect_err("unknown driver should be rejected");

    assert!(matches!(error, ConfigError::UnsupportedDriver(driver) if driver == "oracle"));
    fs::remove_dir_all(&root).expect("temporary root should be removed");
}

#[test]
fn environment_overrides_replace_configuration_values() {
    let _guard = ENV_LOCK
        .lock()
        .expect("environment lock should be acquired");
    let root = temporary_root("env-overrides");
    let paths = ConfigPaths::with_root(&root);
    paths
        .ensure_directories()
        .expect("directories should be created");
    save_config(&paths, &GeneralConfig::default()).expect("config should be saved");

    std::env::set_var("NXM_DATABASE_DRIVER", "postgres");
    std::env::set_var("NXM_DATABASE_URL", "postgres://nxm:secret@db:5432/nxm");
    std::env::set_var("NXM_SERVER_HOST", "0.0.0.0");
    std::env::set_var("NXM_SERVER_PORT", "9090");
    std::env::set_var(
        "NXM_CORS_ORIGINS",
        "http://localhost:3000,https://nxm.local",
    );

    let loaded = load_config(&paths).expect("config should be loaded with overrides");

    assert_eq!(loaded.database.driver, DatabaseKind::Postgres);
    assert_eq!(loaded.database.url, "postgres://nxm:secret@db:5432/nxm");
    assert_eq!(loaded.server.host, "0.0.0.0");
    assert_eq!(loaded.server.port, 9090);
    assert_eq!(
        loaded.server.cors_origins,
        vec!["http://localhost:3000", "https://nxm.local"]
    );

    for key in [
        "NXM_DATABASE_DRIVER",
        "NXM_DATABASE_URL",
        "NXM_SERVER_HOST",
        "NXM_SERVER_PORT",
        "NXM_CORS_ORIGINS",
    ] {
        std::env::remove_var(key);
    }
    fs::remove_dir_all(&root).expect("temporary root should be removed");
}

#[test]
fn invalid_port_override_is_rejected() {
    let _guard = ENV_LOCK
        .lock()
        .expect("environment lock should be acquired");
    let root = temporary_root("bad-port");
    let paths = ConfigPaths::with_root(&root);
    paths
        .ensure_directories()
        .expect("directories should be created");
    save_config(&paths, &GeneralConfig::default()).expect("config should be saved");

    std::env::set_var("NXM_SERVER_PORT", "not-a-number");
    let error = load_config(&paths).expect_err("invalid port should be rejected");
    std::env::remove_var("NXM_SERVER_PORT");

    assert!(matches!(error, ConfigError::InvalidValue(_)));
    fs::remove_dir_all(&root).expect("temporary root should be removed");
}
