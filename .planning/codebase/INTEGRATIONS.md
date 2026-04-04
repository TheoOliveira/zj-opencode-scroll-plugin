# External Integrations

**Analysis Date:** 2026-04-04

## APIs & External Services

**Host Application:**
- Zellij Terminal Multiplexer - The plugin integrates directly with Zellij's Plugin API to read terminal state, intercept key events, and render the scrollbar/status UI.
  - SDK/Client: `zellij-tile` (Rust crate)
  - Auth: None required (local plugin execution)

## Data Storage

**Databases:**
- None

**File Storage:**
- Local filesystem only - The compiled plugin is copied to `~/.config/zellij/plugins/` by `build.sh`

**Caching:**
- None

## Authentication & Identity

**Auth Provider:**
- Custom (None) - The plugin runs completely locally within the user's terminal session without any authentication boundaries.

## Monitoring & Observability

**Error Tracking:**
- None

**Logs:**
- Relies on internal Zellij logging and stdout. Output from the pane is intercepted and analyzed for error keywords locally within the plugin state.

## CI/CD & Deployment

**Hosting:**
- Local Terminal (Zellij)

**CI Pipeline:**
- None

## Environment Configuration

**Required env vars:**
- None

**Secrets location:**
- Not applicable

## Webhooks & Callbacks

**Incoming:**
- None (Uses Zellij `Event` callbacks such as `PaneUpdate` and `Key`)

**Outgoing:**
- None

---

*Integration audit: 2026-04-04*