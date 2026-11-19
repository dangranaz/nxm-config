# CONTEXT.md — nxm-config

Stato: libreria iniziale implementata il 2026-11-17.

API pubbliche:

- `ConfigPaths::discover()` e `with_root()`.
- Accessori per `provider/`, `logs/`, `data/` e `config.toml`.
- `load_config`, `load_or_create_config`, `save_config`.
- Modelli tipizzati `GeneralConfig`, `ServerSettings`, `DatabaseSettings`, `GenerationSettings`, `DatabaseKind`.
- Override env: `NXM_CONFIG_HOME`, `NXM_DATABASE_DRIVER`, `NXM_DATABASE_URL`, `NXM_SERVER_HOST`, `NXM_SERVER_PORT`, `NXM_CORS_ORIGINS`.

Test: 6 passati (2 paths, 4 settings).

Documentazione web aggiunta:
- `README.md` con banner Nexum-AI (WebP + PNG fallback), esempi rapidi e link ai diagrammi.
- `docs/index.html` landing documentazione con i tre diagrammi.
- `docs/diagrams/architecture.html`, `flow.html`, `layout.html`: diagrammi Diagram Design (doc-wide, tema scuro Nexum, accessibili) con card riepilogative.
- `docs/explainer/how-it-works.html`: one-page explainer bilingue IT/EN.
- Immagine testata convertita da `~/AiProjects/design/Nexum.jpeg` a `docs/assets/nexum-banner.webp` (129 KB) con fallback PNG.
