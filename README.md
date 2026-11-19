<div align="center">

# nxm-config

<!-- Licensed MIT — part of the nxm tooling. -->
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![MCP](https://img.shields.io/badge/MCP-JSON--RPC%20stdio-5A4FCF)](#mcp-configurazione)
[![Version](https://img.shields.io/badge/version-0.2.0-blue)](CHANGELOG.md)

**Shared configuration for the NXM platform · Configurazione condivisa per la piattaforma NXM**

<br>

<picture>
  <source srcset="docs/assets/nexum-banner.webp" type="image/webp">
  <img src="docs/assets/nexum-banner.png" alt="Nexum-AI banner" width="1000" height="559">
</picture>

<br>

`nxm-config` è una libreria Rust che centralizza la configurazione della piattaforma NXM: scoperta dei percorsi, impostazioni tipizzate, caricamento/salvataggio TOML e helper JSON-first per la configurazione dei server MCP.

`nxm-config` is a Rust library centralizing NXM platform configuration: path discovery, typed settings, TOML load/save, and JSON-first helpers for MCP server configuration.

- **Repo:** shared configuration library for the NXM platform
- **Format canonico / canonical format:** TOML per le impostazioni generali, JSON per le configurazioni MCP
- **Override ambientali:** `NXM_CONFIG_HOME`, `NXM_DATABASE_DRIVER`, `NXM_DATABASE_URL`, `NXM_SERVER_HOST`, `NXM_SERVER_PORT`, `NXM_CORS_ORIGINS`

</div>

## Cosa fa / What it does

| IT | EN |
|----|----|
| Scopre la root della configurazione da `NXM_CONFIG_HOME` o `~/.nxmconfig`. | Discovers the config root from `NXM_CONFIG_HOME` or `~/.nxmconfig`. |
| Crea le directory `provider/`, `logs/`, `data/` e il file `config.toml`. | Creates `provider/`, `logs/`, `data/` and `config.toml`. |
| Legge e valida `GeneralConfig` (server, database, generazione). | Reads and validates `GeneralConfig` (server, database, generation). |
| Applica override `NXM_*` e restituisce un struct tipizzato. | Applies `NXM_*` overrides and returns a typed struct. |
| Scrive configurazioni MCP generali e per server in `~/.nxmconfig`. | Writes general and per-server MCP configs under `~/.nxmconfig`. |
| Risolve percorsi assoluti/relativi rispetto alla root di configurazione. | Resolves absolute/relative paths against the config root. |

## Documentazione

- [Architettura](docs/diagrams/architecture.html) — facciata pubblica e moduli interni
- [Precedenza configurazione](docs/diagrams/flow.html) — file TOML, default e override `NXM_*`
- [Layout moduli](docs/diagrams/layout.html) — albero di `lib.rs` e punti d'ingresso esportati
- [Landing documentazione](docs/index.html) — indice delle pagine web

## Esempio rapido / Quick example

```rust
use nxm_config::{ConfigPaths, load_or_create_config};

let paths = ConfigPaths::discover()?;
let config = load_or_create_config(&paths)?;
assert_eq!(config.server.port, 8080);
```

Override tramite variabili d'ambiente / Environment overrides:

```bash
export NXM_SERVER_PORT=9090
export NXM_DATABASE_URL=postgres://nxm:secret@db:5432/nxm
```

Scrittura configurazione MCP per un server / Write MCP server config:

```rust
use nxm_config::write_server_config;

let path = write_server_config("nxm-server", r#"host = "127.0.0.1"\nport = 8080\n"#)?;
```

## Installazione

```toml
# Cargo.toml
[dependencies]
nxm-config = { path = "../shared/nxm-config" }
```

Oppure da crates.io quando pubblicato / or from crates.io when published:

```toml
nxm-config = "0.2"
```

## Regole del progetto / Project rules

- Ogni file sorgente Rust sotto le 300 righe / every Rust source file below 300 lines.
- Test in `tests/`, mai inline con il codice di produzione / tests in `tests/`, never inline with production code.
- `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`.
- Commit Conventional Commits; aggiorna `CHANGELOG.md` e `CONTEXT.md` per le modifiche significative.

## Test

```bash
cargo test --all-targets
```

## Contribuire / Contributing

Contributi benvenuti. PR piccoli e focalizzati (un cambiamento logico ciascuno) sono più facili da revisionare. Usa Conventional Commits e rispetta le gate di qualità Rust.

## Licenza

Open source, licenza **MIT** ([LICENSE](LICENSE)).
