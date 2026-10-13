# Application Skeleton

A clean, reusable full-stack Rust application skeleton extracted from RWY. It provides a modern architecture with a decoupled domain model, rich UI layout system, unified static check pipeline, and desktop packaging capabilities.

## Architecture

- **Backend (`crates/backend`)**:
  - [Axum 0.8](https://github.com/tokio-rs/axum) async REST and WebSocket server.
  - [SQLx 0.8](https://github.com/launchbadge/sqlx) SQLite embedded database integration.
  - Unified error handling via `AppError`.
- **Frontend (`crates/frontend`)**:
  - [Leptos 0.7](https://leptos.dev) Client-Side Rendering (CSR) WebAssembly framework.
  - Dark / Light / Modern / Industrial multi-theme support with instant anti-flicker CSS variable engine.
  - ComfyUI-inspired node and docking panel layout (`DockPanel`, `NodeFrame`, `CollapsibleGroup`).
  - i18n localization (English & Japanese) and timezone management.
- **Common (`crates/common`)**:
  - Shared models, event schemas (`ClientEvent`, `ServerEvent`), and system status DTOs.
- **Desktop (`crates/desktop`)**:
  - [Tauri v2](https://tauri.app) native desktop wrapper targeting Linux, macOS, and Windows.

## Quick Start

### Prerequisites
- Rust (stable toolchain)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `trunk`: `cargo install trunk`

### Build & Run
```bash
# Verify entire workspace (formatting, clippy, tests, wasm build)
make verify

# Run backend development server
make dev

# Run frontend dev server with hot reload
make frontend-serve

# Run desktop app
make run-desktop
```

