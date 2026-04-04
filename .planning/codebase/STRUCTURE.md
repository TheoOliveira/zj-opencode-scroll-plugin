# Codebase Structure

**Analysis Date:** 2026-04-04

## Directory Layout

```
[project-root]/
├── build.sh                 # Build script wrapper
├── Cargo.toml               # Rust package and dependency definition
├── lib.rs                   # Core plugin source code
├── opencode.kdl             # Zellij layout configuration
└── opencode-scroll-plugin.tar.gz # Released build artifact
```

## Directory Purposes

This project utilizes a completely flat structure at the root level, avoiding subdirectories for source files due to its small size (single-file plugin).

## Key File Locations

**Entry Points & Core Logic:**
- `lib.rs`: Contains the entire plugin implementation, including state, event handling, and rendering.

**Configuration & Build:**
- `Cargo.toml`: Rust project configuration, defining `crate-type = ["cdylib"]` and dependency on `zellij-tile`.
- `build.sh`: Shell script used to compile the Rust project to WebAssembly.
- `opencode.kdl`: Zellij configuration/layout file to load the plugin.

## Naming Conventions

**Files:**
- Kebab-case for project name/artifacts: `opencode-scroll-plugin`
- Standard Rust file naming: `lib.rs`

## Where to Add New Code

**New Feature / Core Logic:**
- Implementation: Since the project is a single file, all new state, event handling, and rendering logic should be added directly to `lib.rs`. If the codebase grows significantly, it should be split into modules within a `src/` directory.

---

*Structure analysis: 2026-04-04*