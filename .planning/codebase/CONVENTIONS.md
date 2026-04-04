# Coding Conventions

**Analysis Date:** 2026-04-04

## Naming Patterns

**Files:**
- Rust files: snake_case (e.g., `lib.rs`)
- Shell scripts: kebab-case (e.g., `build.sh`)
- Zellij config: kebab-case (e.g., `opencode.kdl`)

**Functions:**
- Rust standard `snake_case` (e.g., `handle_search_key`, `render_status_bar`)

**Variables:**
- Rust standard `snake_case` (e.g., `scroll_offset`, `search_buf`)

**Types:**
- Rust standard `PascalCase` (e.g., `State`)
- Constants use `SCREAMING_SNAKE_CASE` (e.g., `SCROLL_STEP`, `MAX_LOG_LINES`)

## Code Style

**Formatting:**
- Standard `rustfmt` conventions are used, though no custom `rustfmt.toml` is present.
- Code is heavily modularized within the single file using visual comment dividers:
  ```rust
  // ─── Section Name ─────────────────────────────────────────────────────────────
  ```

**Linting:**
- Standard `cargo clippy` (default rules).

## Import Organization

**Order:**
1. Standard library (`std::...`)
2. External crates (`zellij_tile::prelude::*`)

## Error Handling

**Patterns:**
- **Arithmetic Safety:** Uses `saturating_add` and `saturating_sub` for scroll offsets to prevent arithmetic overflow/underflow panics:
  ```rust
  self.scroll_offset = self.scroll_offset.saturating_sub(1);
  ```
- **Option/Result Handling:** Uses `flatten()` and `find()` on iterators safely without unwrapping:
  ```rust
  let focused = pane_manifest.panes.values().flatten().find(|p| p.is_focused);
  if let Some(pane) = focused { ... }
  ```

## Logging

**Framework:** None configured directly in the plugin itself.
- Relies entirely on `zellij-tile` plugin abstractions.

## Comments

**When to Comment:**
- Visual section headers for structural organization inside `lib.rs`.
- Brief inline explanations for key bindings:
  ```rust
  // Jump to bottom (follow mode)
  ```
- Docstrings (`///`) used for constants:
  ```rust
  /// Keywords that trigger auto-lock-to-top so the error is always visible
  const ERROR_KEYWORDS: &[&str] = ...
  ```

## Function Design

**Size:**
- Small, focused helper functions (e.g., `handle_search_key`, `render_idle`).
- The main `update` function dispatches cleanly to handlers instead of growing large.

**Parameters:**
- Methods typically take `&mut self` or `&self` and relevant minimal parameters (e.g., `key: Key`, `cols: usize`).

## Module Design

- **Structure:** Single file structure in `lib.rs`.
- **Implementation Blocks:** `impl State` blocks are separated logically by feature (e.g., one block for Key handling, one for Render helpers) instead of throwing everything into one giant `impl`.

---

*Convention analysis: 2026-04-04*