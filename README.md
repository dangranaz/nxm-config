<div align="center">

# nxm-config

<!-- Licensed MIT — part of the nxm tooling. -->
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![MCP](https://img.shields.io/badge/MCP-JSON--RPC%20stdio-5A4FCF)](#mcp-configuration)
[![Version](https://img.shields.io/badge/version-0.2.0-blue)](CHANGELOG.md)

**Shared configuration for the NXM platform**

<br>

<picture>
  <source srcset="docs/assets/nexum-banner.webp" type="image/webp">
  <img src="docs/assets/nexum-banner.png" alt="Nexum-AI banner" width="1000" height="559">
</picture>

<br>

`nxm-config` is a Rust library that centralizes NXM platform configuration: path discovery, typed settings, TOML load/save, and JSON-first helpers for MCP server configuration.

- **Repo:** shared configuration library for the NXM platform
- **Canonical format:** TOML for general settings, JSON for MCP configurations
- **Environment overrides:** `NXM_CONFIG_HOME`, `NXM_DATABASE_DRIVER`, `NXM_DATABASE_URL`, `NXM_SERVER_HOST`, `NXM_SERVER_PORT`, `NXM_CORS_ORIGINS`

</div>

## What it does

| Feature | Description |
|---------|-------------|
| Config root discovery | Discovers the config root from `NXM_CONFIG_HOME` or `~/.nxmconfig`. |
| Directory creation | Creates `provider/`, `logs/`, `data/` and the `config.toml` file. |
| Configuration reading | Reads and validates `GeneralConfig` (server, database, generation). |
| Environment overrides | Applies `NXM_*` overrides and returns a typed struct. |
| MCP configuration | Writes general and per-server MCP configs under `~/.nxmconfig`. |
| Path resolution | Resolves absolute/relative paths against the config root. |

## Documentation

- [Architecture](docs/diagrams/architecture.html) — public facade and internal modules
- [Configuration precedence](docs/diagrams/flow.html) — TOML file, defaults and `NXM_*` overrides
- [Module layout](docs/diagrams/layout.html) — `lib.rs` tree and exported entry points
- [Documentation landing](docs/index.html) — index of web pages

## Quick example

```rust
use nxm_config::{ConfigPaths, load_or_create_config};

let paths = ConfigPaths::discover()?;
let config = load_or_create_config(&paths)?;
assert_eq!(config.server.port, 8080);
```

Environment variable overrides:

```bash
export NXM_SERVER_PORT=9090
export NXM_DATABASE_URL=postgres://nxm:secret@db:5432/nxm
```

Write MCP server configuration:

```rust
use nxm_config::write_server_config;

let path = write_server_config("nxm-server", r#"host = "127.0.0.1"\nport = 8080\n"#)?;
```

## Installation

```toml
# Cargo.toml
[dependencies]
nxm-config = { path = "../shared/nxm-config" }
```

Or from crates.io when published:

```toml
nxm-config = "0.2"
```

## Project rules

- Every Rust source file below 300 lines.
- Tests in `tests/`, never inline with production code.
- `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`.
- Conventional Commits; update `CHANGELOG.md` and `CONTEXT.md` for significant changes.

## Tests

```bash
cargo test --all-targets
```

## Contributing

Contributions are welcome. Small, focused PRs (one logical change each) are easier to review. Use Conventional Commits and respect the Rust quality gates.

## License

Open source, **MIT** license ([LICENSE](LICENSE)).
