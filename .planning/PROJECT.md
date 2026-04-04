# zj-opencode-scroll-plugin

## What This Is

A WebAssembly plugin for the Zellij terminal multiplexer that provides custom scrolling and event-handling behavior. The core plugin logic is mostly complete, and this project focuses on productizing it for public release — improving repository organization, adding user configuration options, creating comprehensive documentation, and automating release builds.

## Core Value

Making a highly functional, specialized Zellij scroll plugin accessible, configurable, and easy to install for the broader public.

## Requirements

### Validated

- ✓ Custom scrolling logic and key bindings — existing
- ✓ Event-driven UI rendering within Zellij — existing
- ✓ Error locking mechanism to keep errors visible — existing
- ✓ Basic WASM compilation via shell script — existing

### Active

- [ ] Reorganize repository structure for clarity and standard Rust/Zellij plugin conventions
- [ ] Create high-quality, comprehensive README.md and CONTRIBUTING.md
- [ ] Implement user configuration options (customizable colors, keybindings, behavior)
- [ ] Set up GitHub Actions release automation for building and distributing `.wasm` binaries

### Out of Scope

- [Complete UI overhaul] — The goal is productization and public release of existing functionality, not redesigning the terminal interface.

## Context

This is a brownfield project. The core Rust logic for the Zellij plugin is already written and functional. The plugin utilizes `zellij-tile` to subscribe to events and render its UI. Current distribution relies on a local `build.sh` script, which is not suitable for public adoption. The focus now shifts from core development to developer experience (DX), documentation, and CI/CD.

## Constraints

- **Tech Stack**: Rust, WebAssembly (`wasm32-wasi`), and Zellij Plugin API.
- **Platform**: Must run seamlessly within Zellij multiplexer environments.
- **Distribution**: Must provide pre-compiled `.wasm` files via GitHub releases so users don't have to compile from source.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Prioritize repo polish over UI rewrite | Core logic is done; biggest blocker to adoption is DX/packaging | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-04 after initialization*
