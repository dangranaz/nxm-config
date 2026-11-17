# CONTEXT.md — nxm-config

Stato: libreria iniziale implementata il 2026-11-17.

API pubbliche:

- `ConfigPaths::discover()` e `with_root()`.
- Accessori per `provider/`, `logs/`, `data/` e `config.toml`.
- `load_config`, `load_or_create_config`, `save_config`.
- Modelli tipizzati `GeneralConfig`, `ServerSettings`, `DatabaseSettings`, `GenerationSettings`, `DatabaseKind`.
- Override env: `NXM_CONFIG_HOME`, `NXM_DATABASE_DRIVER`, `NXM_DATABASE_URL`, `NXM_SERVER_HOST`, `NXM_SERVER_PORT`, `NXM_CORS_ORIGINS`.

Test: 6 passati (2 paths, 4 settings).
