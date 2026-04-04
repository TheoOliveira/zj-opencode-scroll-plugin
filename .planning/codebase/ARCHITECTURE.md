# Architecture

**Analysis Date:** 2026-04-04

## Pattern Overview

**Overall:** Event-Driven WebAssembly Plugin (Zellij)

**Key Characteristics:**
- Single-file WebAssembly plugin for Zellij terminal workspace
- Event-driven state machine
- Immediate mode UI rendering pattern

## Layers

**State Management:**
- Purpose: Maintains plugin state (scroll position, mode, search buffer)
- Location: `lib.rs` (`State` struct)
- Contains: Pure data structures

**Event Handling:**
- Purpose: Processes Zellij events (PaneUpdate, Keypresses)
- Location: `lib.rs` (`impl ZellijPlugin for State::update`)
- Depends on: `zellij-tile` API

**Key Binding Logic:**
- Purpose: Maps user keystrokes to state mutations and scroll commands
- Location: `lib.rs` (`impl State::handle_key`)
- Actions: Calls Zellij scroll commands (`scroll_up`, `scroll_down`, etc.)

**Rendering:**
- Purpose: Draws the UI based on current state
- Location: `lib.rs` (`impl ZellijPlugin for State::render` and `impl State::render_*`)
- Depends on: `print_text_with_coordinates` from `zellij-tile`

## Data Flow

**Event Loop:**

1. Zellij dispatches an event (e.g., `Event::PaneUpdate` or `Event::Key`).
2. Plugin's `update` method is invoked.
3. State is mutated (e.g., `scroll_offset` updated, `opencode_active` toggled).
4. `update` returns `true` if rendering is needed.
5. Zellij calls the plugin's `render` method.
6. Plugin issues drawing commands via `print_text_with_coordinates`.

**State Management:**
- Maintained internally in the `State` struct which is registered via `register_plugin!(State)`.

## Key Abstractions

**Plugin State (`State`):**
- Purpose: Represents the entire UI and behavior state of the scroll plugin.
- Examples: `lib.rs`
- Pattern: Mutable struct modified by events.

## Entry Points

**Plugin Initialization:**
- Location: `lib.rs` (`impl ZellijPlugin for State::load`)
- Triggers: Zellij loading the WebAssembly module.
- Responsibilities: Subscribes to necessary events (`PaneUpdate`, `Key`).

## Error Handling

**Strategy:** Error locking mechanism

**Patterns:**
- The plugin has concepts of an `error_locked` state triggered by specific output patterns.
- Keeps scroll anchored so errors remain visible.

## Cross-Cutting Concerns

**Terminal Interfacing:** Uses `zellij-tile::prelude::*` for all terminal interactions, UI drawing, and terminal dimension calculations.

---

*Architecture analysis: 2026-04-04*