use nxm_config::ConfigPaths;
use std::fs;

#[test]
fn with_root_creates_provider_logs_and_data_directories() {
    let root = std::env::temp_dir().join(format!("nxm-config-paths-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);

    let paths = ConfigPaths::with_root(&root);
    paths
        .ensure_directories()
        .expect("directories should be created");

    assert!(paths.provider_dir().is_dir());
    assert!(paths.logs_dir().is_dir());
    assert!(paths.data_dir().is_dir());
    assert_eq!(paths.config_file(), root.join("config.toml"));

    fs::remove_dir_all(&root).expect("temporary root should be removed");
}

#[test]
fn resolve_keeps_absolute_paths_and_resolves_relative_paths() {
    let root = std::env::temp_dir().join(format!("nxm-config-resolve-{}", std::process::id()));
    let paths = ConfigPaths::with_root(&root);
    let absolute = root.join("absolute.db");

    assert_eq!(
        paths.resolve("/tmp/test.db"),
        std::path::PathBuf::from("/tmp/test.db")
    );
    assert_eq!(paths.resolve("data/nxm.db"), root.join("data/nxm.db"));
    assert_eq!(paths.resolve(&absolute), absolute);
}
