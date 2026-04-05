<!-- GSD:project-start source:PROJECT.md -->
## Project

**zj-opencode-scroll-plugin**

A WebAssembly plugin for the Zellij terminal multiplexer that provides custom scrolling and event-handling behavior. The core plugin logic is mostly complete, and this project focuses on productizing it for public release — improving repository organization, adding user configuration options, creating comprehensive documentation, and automating release builds.

**Core Value:** Making a highly functional, specialized Zellij scroll plugin accessible, configurable, and easy to install for the broader public.

### Constraints

- **Tech Stack**: Rust, WebAssembly (`wasm32-wasi`), and Zellij Plugin API.
- **Platform**: Must run seamlessly within Zellij multiplexer environments.
- **Distribution**: Must provide pre-compiled `.wasm` files via GitHub releases so users don't have to compile from source.
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- Rust 2021 - Core plugin logic in `lib.rs`
- Bash - Build and deployment script in `build.sh`
- KDL - Zellij layout configuration in `opencode.kdl`
## Runtime
- Zellij Plugin System (WebAssembly / `wasm32-wasi`)
- Cargo (Rust)
- Lockfile: Not present (`Cargo.lock` is missing or not tracked)
## Frameworks
- `zellij-tile` 0.42.1 - Official SDK for writing Zellij plugins
- Not detected
- Cargo - Rust build system
- `build.sh` - Custom shell script to compile to wasm and copy to `~/.config/zellij/plugins/`
## Key Dependencies
- `zellij-tile` 0.42.1 - Provides the Plugin API needed to subscribe to events (`PaneUpdate`, `Key`) and interface with the Zellij host.
- Not applicable
## Configuration
- Not applicable (no `.env` files or environment variables required at runtime)
- `Cargo.toml` - Rust crate configuration and dependency declaration
- `opencode.kdl` - Zellij layout defining pane setup and plugin loading
## Platform Requirements
- Rust toolchain with `wasm32-wasi` target added (`rustup target add wasm32-wasi`)
- Zellij 0.42.1+ (matching the `zellij-tile` dependency version)
- Zellij terminal multiplexer (loads the `.wasm` binary as a plugin)
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- Rust files: snake_case (e.g., `lib.rs`)
- Shell scripts: kebab-case (e.g., `build.sh`)
- Zellij config: kebab-case (e.g., `opencode.kdl`)
- Rust standard `snake_case` (e.g., `handle_search_key`, `render_status_bar`)
- Rust standard `snake_case` (e.g., `scroll_offset`, `search_buf`)
- Rust standard `PascalCase` (e.g., `State`)
- Constants use `SCREAMING_SNAKE_CASE` (e.g., `SCROLL_STEP`, `MAX_LOG_LINES`)
## Code Style
- Standard `rustfmt` conventions are used, though no custom `rustfmt.toml` is present.
- Code is heavily modularized within the single file using visual comment dividers:
- Standard `cargo clippy` (default rules).
## Import Organization
## Error Handling
- **Arithmetic Safety:** Uses `saturating_add` and `saturating_sub` for scroll offsets to prevent arithmetic overflow/underflow panics:
- **Option/Result Handling:** Uses `flatten()` and `find()` on iterators safely without unwrapping:
## Logging
- Relies entirely on `zellij-tile` plugin abstractions.
## Comments
- Visual section headers for structural organization inside `lib.rs`.
- Brief inline explanations for key bindings:
- Docstrings (`///`) used for constants:
## Function Design
- Small, focused helper functions (e.g., `handle_search_key`, `render_idle`).
- The main `update` function dispatches cleanly to handlers instead of growing large.
- Methods typically take `&mut self` or `&self` and relevant minimal parameters (e.g., `key: Key`, `cols: usize`).
## Module Design
- **Structure:** Single file structure in `lib.rs`.
- **Implementation Blocks:** `impl State` blocks are separated logically by feature (e.g., one block for Key handling, one for Render helpers) instead of throwing everything into one giant `impl`.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Single-file WebAssembly plugin for Zellij terminal workspace
- Event-driven state machine
- Immediate mode UI rendering pattern
## Layers
- Purpose: Maintains plugin state (scroll position, mode, search buffer)
- Location: `lib.rs` (`State` struct)
- Contains: Pure data structures
- Purpose: Processes Zellij events (PaneUpdate, Keypresses)
- Location: `lib.rs` (`impl ZellijPlugin for State::update`)
- Depends on: `zellij-tile` API
- Purpose: Maps user keystrokes to state mutations and scroll commands
- Location: `lib.rs` (`impl State::handle_key`)
- Actions: Calls Zellij scroll commands (`scroll_up`, `scroll_down`, etc.)
- Purpose: Draws the UI based on current state
- Location: `lib.rs` (`impl ZellijPlugin for State::render` and `impl State::render_*`)
- Depends on: `print_text_with_coordinates` from `zellij-tile`
## Data Flow
- Maintained internally in the `State` struct which is registered via `register_plugin!(State)`.
## Key Abstractions
- Purpose: Represents the entire UI and behavior state of the scroll plugin.
- Examples: `lib.rs`
- Pattern: Mutable struct modified by events.
## Entry Points
- Location: `lib.rs` (`impl ZellijPlugin for State::load`)
- Triggers: Zellij loading the WebAssembly module.
- Responsibilities: Subscribes to necessary events (`PaneUpdate`, `Key`).
## Error Handling
- The plugin has concepts of an `error_locked` state triggered by specific output patterns.
- Keeps scroll anchored so errors remain visible.
## Cross-Cutting Concerns
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
