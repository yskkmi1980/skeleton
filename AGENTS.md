# Skeleton Project Guide

Modular full-stack Rust application skeleton (Cargo workspace) with Leptos WASM frontend, Axum backend, and optional Tauri desktop shell.

## Architecture & Crates
- `crates/common`: Shared models, traits, error definitions, and event DTOs.
- `crates/backend`: REST & WebSocket API (Axum 0.8, Tower 0.5, SQLx 0.8 SQLite).
- `crates/frontend`: Web SPA UI (Leptos 0.7 CSR, ComfyUI-inspired layout, 3-Layer architecture).
- `crates/desktop`: Desktop shell (Tauri v2) embedding the Leptos/WASM frontend.

## Verification Commands (作業終了前の必須検証)
- 一括検証:
  - `make verify` (runs `cargo fmt`, `clippy -D warnings`, `test --workspace`, frontend wasm build)
- 個別確認コマンド:
  - `cargo fmt --all -- --check`
  - `cargo check --workspace`
  - `cargo clippy --workspace --exclude frontend --all-targets -- -D warnings`
  - `cargo clippy -p frontend --target wasm32-unknown-unknown -- -D warnings`
  - `cargo test --workspace`
  - `cargo check -p frontend --target wasm32-unknown-unknown` (or `make frontend`)

## Development Flow
```bash
# Frontend development server with live-reload
make frontend-serve

# Backend development server
make dev

# All release builds
make release
```

