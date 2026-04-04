# Testing Patterns

**Analysis Date:** 2026-04-04

## Test Framework

**Runner:**
- Standard `cargo test` (Rust built-in test runner).
- *Note:* Currently, testing is not explicitly configured or functioning due to the `lib.rs` file location (at project root instead of `src/` without `lib.path` in `Cargo.toml`).

**Assertion Library:**
- Standard `std::assert!` macros (though no tests are currently present).

**Run Commands:**
```bash
cargo test              # Expected command to run all tests (currently failing)
```

## Test File Organization

**Location:**
- No test files or `#[cfg(test)]` modules were detected in `lib.rs` or the project root.

**Naming:**
- N/A

## Test Structure

**Suite Organization:**
- Not applicable (no tests detected).

**Patterns:**
- No testing patterns have been established.

## Mocking

**Framework:** None.
- As a Zellij plugin (`zellij_tile::prelude::*`), testing usually requires mocking the plugin host API or separating domain logic from the `ZellijPlugin` implementation. This separation is not currently present in `lib.rs`.

## Fixtures and Factories

**Test Data:**
- None detected.

## Coverage

**Requirements:** None enforced.

**View Coverage:**
- Not applicable.

## Test Types

**Unit Tests:**
- Not used.

**Integration Tests:**
- Not used.

**E2E Tests:**
- Manual testing is required by compiling the Wasm plugin using `build.sh` and loading it into Zellij using `opencode.kdl`.

## Common Patterns

**Current Testing Strategy:**
- Exclusively manual testing via `build.sh` and Zellij layout injection (`opencode.kdl`).

---

*Testing analysis: 2026-04-04*