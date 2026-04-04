# Codebase Concerns

**Analysis Date:** 2026-04-04

## Tech Debt

**Stubbed Search Implementation:**
- Issue: The search bar UI collects user input into `search_buf`, but pressing `\n` to submit only closes the UI without dispatching any search action.
- Files: `lib.rs`
- Impact: Search functionality is completely broken/missing.
- Fix approach: Implement integration with Zellij's search API (e.g., dispatching `search_down` or equivalent actions) upon Enter key.

**Dead Code & Unimplemented Error Auto-Lock:**
- Issue: The constants `MAX_LOG_LINES` and `ERROR_KEYWORDS` are defined but never used. The struct field `log_lines` is never populated.
- Files: `lib.rs`
- Impact: The core feature of automatically locking scroll to the top when an error is detected is missing.
- Fix approach: Subscribe to pane log output (if Zellij API permits) or remove dead fields if this approach is impossible in the current Zellij version. Connect `ERROR_KEYWORDS` detection to toggle `error_locked = true`.

**Hardcoded Pane Targeting:**
- Issue: The plugin explicitly checks if the pane title contains "opencode" (`pane.title.to_lowercase().contains("opencode")`). 
- Files: `lib.rs`
- Impact: Makes the plugin rigid and useless for other logs or tasks unless they are named exactly "opencode".
- Fix approach: Use configuration settings from `BTreeMap` in the `load()` function to define the target pane name dynamically, defaulting to "opencode" if omitted.

## Fragile Areas

**Scroll Action Ambiguity:**
- Files: `lib.rs`
- Why fragile: The key bindings (u, d, g, G) trigger `scroll_up()`, `scroll_down()`, etc. These Zellij actions typically affect the *currently focused pane*. If the user is typing in the plugin to scroll, the plugin itself is focused, which may result in the plugin scrolling instead of the target "opencode" pane.
- Safe modification: Ensure actions explicitly target the pane ID of the "opencode" pane, or clarify focus management so the user stays in the target pane while the plugin listens to global inputs (though global inputs might not be natively supported without intercepting).

## Test Coverage Gaps

**Total Lack of Tests:**
- What's not tested: Key event handling, state mutations, string manipulation (search buffer), and conditional rendering logic.
- Files: `lib.rs`
- Risk: High risk of regressions when implementing the missing search or error-lock features. Search buffer edge cases (e.g. backspacing on empty) could panic if `pop()` is used unsafely on strings (though `String::pop` returns an `Option`, so it's safe in Rust, but behaviorally untested).
- Priority: High. Introduce unit tests for `State` transitions and key handlers (`handle_key` / `handle_search_key`).

## Security Considerations

**Input Handling:**
- Risk: Potential unbounded growth of `search_buf` if a user spams characters without submitting.
- Files: `lib.rs`
- Current mitigation: None.
- Recommendations: Add a reasonable max length limit to `search_buf` inside `handle_search_key` to avoid unbounded memory allocation.

---

*Concerns audit: 2026-04-04*