# Technology Stack

**Analysis Date:** 2026-04-04

## Languages

**Primary:**
- Rust 2021 - Core plugin logic in `lib.rs`

**Secondary:**
- Bash - Build and deployment script in `build.sh`
- KDL - Zellij layout configuration in `opencode.kdl`

## Runtime

**Environment:**
- Zellij Plugin System (WebAssembly / `wasm32-wasi`)

**Package Manager:**
- Cargo (Rust)
- Lockfile: Not present (`Cargo.lock` is missing or not tracked)

## Frameworks

**Core:**
- `zellij-tile` 0.42.1 - Official SDK for writing Zellij plugins

**Testing:**
- Not detected

**Build/Dev:**
- Cargo - Rust build system
- `build.sh` - Custom shell script to compile to wasm and copy to `~/.config/zellij/plugins/`

## Key Dependencies

**Critical:**
- `zellij-tile` 0.42.1 - Provides the Plugin API needed to subscribe to events (`PaneUpdate`, `Key`) and interface with the Zellij host.

**Infrastructure:**
- Not applicable

## Configuration

**Environment:**
- Not applicable (no `.env` files or environment variables required at runtime)

**Build:**
- `Cargo.toml` - Rust crate configuration and dependency declaration
- `opencode.kdl` - Zellij layout defining pane setup and plugin loading

## Platform Requirements

**Development:**
- Rust toolchain with `wasm32-wasi` target added (`rustup target add wasm32-wasi`)
- Zellij 0.42.1+ (matching the `zellij-tile` dependency version)

**Production:**
- Zellij terminal multiplexer (loads the `.wasm` binary as a plugin)

---

*Stack analysis: 2026-04-04*