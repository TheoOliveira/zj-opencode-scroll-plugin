# Architecture Research

**Domain:** Rust/WASM Zellij plugin productization (docs + CI/CD + config + source boundaries)
**Researched:** 2026-04-04
**Confidence:** MEDIUM-HIGH

## Standard Architecture

### System Overview

```text
┌──────────────────────────────────────────────────────────────────────────┐
│                         User + Maintainer Interfaces                    │
├──────────────────────────────────────────────────────────────────────────┤
│  docs/ (Diátaxis)   README/CONTRIBUTING   examples/*.kdl   Releases    │
└───────────────┬──────────────────────────────────────────────────────────┘
                │ install + configure + contribute
┌───────────────▼──────────────────────────────────────────────────────────┐
│                          Plugin Runtime Boundary                         │
├──────────────────────────────────────────────────────────────────────────┤
│  src/plugin.rs (ZellijPlugin impl)                                      │
│  src/config.rs (BTreeMap<String,String> -> typed Config)                │
│  src/events.rs (Event -> intent)                                        │
│  src/state.rs (state + reducers)                                        │
│  src/render/* (pure render functions)                                   │
│  src/actions.rs (zellij-tile command wrappers)                          │
└───────────────┬──────────────────────────────────────────────────────────┘
                │ subscribe/update/render/commands
┌───────────────▼──────────────────────────────────────────────────────────┐
│                               Zellij Host                               │
├──────────────────────────────────────────────────────────────────────────┤
│  Event stream  +  Permissions  +  Plugin config updates                 │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│                             Delivery Boundary                           │
├──────────────────────────────────────────────────────────────────────────┤
│  .github/workflows/ci.yml      (lint/test/build smoke checks)           │
│  .github/workflows/release.yml (tag-triggered release + assets)         │
│  .github/release.yml           (auto release-note categories)            │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| Plugin API Adapter | Own `load/update/render`, subscribe to events, ask for permissions, receive runtime config changes | `ZellijPlugin` impl in `src/plugin.rs` with thin orchestration |
| Config Boundary | Parse/validate plugin configuration from layout/CLI (`BTreeMap<String, String>`) into typed settings | `src/config.rs` + defaults + validation + migration handling |
| Event/Intent Layer | Convert raw Zellij `Event` variants to domain intents (scroll/search/lock/toggle) | `src/events.rs` with exhaustive matching and intent enums |
| State Core | Hold plugin state and deterministic transitions | `src/state.rs` with reducer-like pure mutations |
| Render Layer | Convert state to terminal draw commands without hidden side effects | `src/render/*` functions called by `render()` |
| CI Verification Pipeline | Enforce quality and compatibility before release | GH Actions workflow with fmt/clippy/tests/build target checks |
| Release Pipeline | Build `.wasm`, checksum, publish assets + notes | Tag-triggered GH Action + `gh release create` |
| Docs System | Separate onboarding, task docs, API reference, design rationale | Diátaxis-aligned docs tree under `docs/` |

## Recommended Project Structure

```text
.
├── src/
│   ├── lib.rs                     # register_plugin! + module wiring only
│   ├── plugin.rs                  # ZellijPlugin lifecycle orchestration
│   ├── config.rs                  # typed config + defaults + validation
│   ├── events.rs                  # Event -> intent translation
│   ├── state.rs                   # state structs + transitions
│   ├── actions.rs                 # wrappers over zellij-tile commands
│   └── render/
│       ├── mod.rs                 # render entrypoint
│       ├── frame.rs               # structural rendering
│       └── status.rs              # status/diagnostic UI rendering
├── tests/
│   ├── config_parsing.rs          # config compatibility tests
│   ├── state_transitions.rs       # event/state behavior tests
│   └── render_snapshots.rs        # deterministic render output tests
├── examples/
│   └── opencode.kdl               # documented plugin config examples
├── docs/
│   ├── tutorials/                 # first-time install/run
│   ├── how-to/                    # task-focused usage recipes
│   ├── reference/                 # config keys, keybindings, permissions
│   └── explanation/               # architecture and design rationale
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                 # PR/main verification
│   │   ├── release.yml            # push tags v* -> release assets
│   │   └── reusable-rust-wasm.yml # shared build job
│   └── release.yml                # auto-generated release note categories
├── scripts/
│   └── dev-build.sh               # local convenience only (non-source-of-truth)
├── Cargo.toml
├── Cargo.lock
├── README.md
└── CONTRIBUTING.md
```

### Structure Rationale

- **`src/` split by responsibility:** Prevents growth into another monolithic `lib.rs`; keeps runtime logic testable.
- **`config.rs` as hard boundary:** Zellij config arrives as string map; parse once, validate once, then use typed config everywhere.
- **`tests/` as contract tests:** Protects productization changes (especially config compatibility and behavior regression).
- **`docs/` using Diátaxis:** Keeps docs maintainable as features grow (separate learning, tasks, reference, rationale).
- **`.github/workflows/` split into CI and release:** Avoids accidental releases and reduces workflow coupling.
- **`examples/` for real KDL snippets:** Docs stay executable and less likely to drift.

## Architectural Patterns

### Pattern 1: Lifecycle Shell + Domain Core

**What:** Keep `ZellijPlugin` lifecycle methods as thin adapters; push logic into domain modules.
**When to use:** Always, once plugin logic exceeds simple demos.
**Trade-offs:** Slightly more files; much better testability and maintainability.

**Example:**
```rust
fn update(&mut self, event: Event) -> bool {
    if let Some(intent) = self.event_router.to_intent(event) {
        self.state.apply(intent);
        return self.state.needs_render();
    }
    false
}
```

### Pattern 2: Typed Config with Compatibility Defaults

**What:** Parse `BTreeMap<String, String>` from `load()` and runtime config-change events into a typed `Config` struct.
**When to use:** Any plugin exposing user-configurable behavior.
**Trade-offs:** Upfront parsing code, but prevents scattered stringly-typed bugs.

**Example:**
```rust
pub fn from_kv(input: &BTreeMap<String, String>) -> Result<Config, ConfigError> {
    Ok(Config {
        target_pane: input.get("target_pane").cloned().unwrap_or("opencode".into()),
        scroll_step: input.get("scroll_step").and_then(|v| v.parse().ok()).unwrap_or(1),
    })
}
```

### Pattern 3: Two-Stage CI/CD (Verify then Release)

**What:** Separate CI checks from tag-triggered release publication.
**When to use:** Any public plugin distributing binary assets.
**Trade-offs:** Slightly more workflow setup; far safer release process.

**Example:**
```yaml
on:
  push:
    tags: ["v*"]
jobs:
  verify:
    uses: ./.github/workflows/reusable-rust-wasm.yml
  release:
    needs: verify
    permissions:
      contents: write
```

### Pattern 4: Diátaxis Docs for Plugin Ecosystem

**What:** Organize docs by user intent (tutorial/how-to/reference/explanation), not by internal folders.
**When to use:** As soon as project is public and has contributors.
**Trade-offs:** Requires deliberate doc taxonomy; drastically better navigability.

## Data Flow

### Runtime Flow

```text
[Zellij loads plugin]
    ↓
load(configuration: BTreeMap<String,String>)
    ↓
parse/validate config (config.rs)
    ↓
subscribe(EventType::...) + optional request_permission(...)
    ↓
update(event) receives async events
    ↓
event -> intent mapping (events.rs)
    ↓
state transition (state.rs)
    ↓ (bool)
render(rows, cols) -> draw commands (render/*)
```

### Config Update Flow

```text
[User edits KDL / runtime reconfigure]
    ↓
Event::PluginConfigurationChanged(BTreeMap)
    ↓
config parser + compatibility defaults
    ↓
state/config store update
    ↓
render refresh
```

### Delivery Flow

```text
[PR/push]
    ↓
CI workflow (fmt, clippy, tests, wasm build)
    ↓
[tag push: vX.Y.Z]
    ↓
release workflow builds wasm + checksums
    ↓
upload artifact + gh release create --generate-notes
    ↓
GitHub Release assets consumed by users
```

### Key Data Flows

1. **Event-to-render loop:** Zellij Event → Intent → State Transition → Render Request.
2. **Config-to-behavior loop:** KDL/CLI config → Typed Config → Runtime behavior without touching render/action internals.
3. **Commit-to-install loop:** Tag → CI verification → release asset → user installs via plugin URL or local file.

## Component Boundaries (explicit)

| Boundary | Owns | Must NOT own |
|----------|------|--------------|
| `plugin.rs` | lifecycle wiring, subscriptions, top-level orchestration | business rules, parsing details, rendering details |
| `config.rs` | schema, defaults, validation, migration aliases | event handling and rendering |
| `events.rs` | raw event decoding and intent mapping | direct UI drawing |
| `state.rs` | state transitions and invariants | external side effects |
| `actions.rs` | commands to Zellij host | parsing user config |
| `render/*` | display-only formatting and layout | mutating domain state |
| CI workflows | verification + publication gates | project runtime behavior |
| Docs tree | user and contributor guidance | hidden implementation details as “reference” |

## Suggested Build Order (roadmap implication)

1. **Source boundary refactor first**
   - Split monolithic `lib.rs` into modules without changing behavior.
   - Why first: unlocks safe config and docs evolution.

2. **Typed configuration layer second**
   - Implement `config.rs`, wire `load()` and `PluginConfigurationChanged`.
   - Why second: enables user-facing flexibility before broad docs/release promises.

3. **CI verification pipeline third**
   - Add PR checks (fmt/clippy/tests/wasm build) and required-status policy.
   - Why third: prevents architecture regressions while adding release/docs.

4. **Release automation fourth**
   - Tag-triggered workflow, release assets, checksums, generated notes categories.
   - Why fourth: only automate publication once quality gates are stable.

5. **Docs architecture fifth (parallelizable with 3/4 once interfaces settle)**
   - Create Diátaxis docs and examples mapped to real config keys/releases.
   - Why fifth: avoids documenting unstable interfaces.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| Solo maintainer / few users | Single crate is fine; keep strict module boundaries and one CI workflow |
| Multiple contributors / monthly releases | Add reusable workflows, stronger branch protection, snapshot tests for rendering |
| Frequent releases / broad community usage | Add compatibility policy for config keys, explicit deprecation path, changelog automation discipline |

### Scaling Priorities

1. **First bottleneck:** monolithic source file causing risky edits → split modules + tests.
2. **Second bottleneck:** release inconsistency/manual steps → deterministic tag-driven pipeline.

## Anti-Patterns

### Anti-Pattern 1: “Everything in `lib.rs`”

**What people do:** Keep lifecycle, config parsing, event logic, and rendering all in one file.
**Why it's wrong:** Increases coupling and regression risk; blocks parallel work.
**Do this instead:** Keep `lib.rs` tiny and move behavior into dedicated modules.

### Anti-Pattern 2: Stringly-Typed Config Everywhere

**What people do:** Read `BTreeMap` keys ad hoc in unrelated modules.
**Why it's wrong:** Silent defaults and inconsistent behavior across features.
**Do this instead:** Parse once into typed config and pass immutable config references.

### Anti-Pattern 3: Release-on-main without tag gate

**What people do:** Publish release assets on every main push.
**Why it's wrong:** Unclear versioning and accidental user-facing breakage.
**Do this instead:** Tag-triggered releases (`v*`) with explicit permissions and generated notes.

### Anti-Pattern 4: README-only docs

**What people do:** Put install, config, contributor, and architecture details in one README.
**Why it's wrong:** Documentation decays quickly and is hard to navigate.
**Do this instead:** Diátaxis split + `README` as index/quickstart only.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| Zellij plugin host | `ZellijPlugin` lifecycle (`load/update/render`) + event subscription | Event order is asynchronous; plugin logic must not assume strict order |
| Zellij config system | KDL/CLI config → `load(BTreeMap<...>)` and runtime config-change events | Supports both startup config and runtime updates |
| GitHub Actions | CI on PR/push, release on tag push | Use least-privilege `GITHUB_TOKEN` permissions |
| GitHub Releases | Attach `.wasm` assets and generated notes | Use deterministic filenames + checksums for trust |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `plugin.rs` ↔ `events.rs` | direct function calls (event-to-intent) | keep translation exhaustive and testable |
| `plugin.rs` ↔ `config.rs` | typed `Config` value | parser is single source of truth |
| `plugin.rs` ↔ `state.rs` | intent application API | state transitions deterministic |
| `state.rs` ↔ `render/*` | immutable state view | avoid mutations during render |
| workflows ↔ release assets | workflow artifacts + `gh release` | release job depends on successful verify job |

## Sources

- Zellij plugin overview and loading: https://zellij.dev/documentation/plugins and https://zellij.dev/documentation/plugin-loading.html (**HIGH**)
- Zellij plugin lifecycle and events/config/permissions: https://zellij.dev/documentation/plugin-lifecycle.html, https://zellij.dev/documentation/plugin-api-events.html, https://zellij.dev/documentation/plugin-api-configuration.html, https://zellij.dev/documentation/plugin-api-permissions.html (**HIGH**)
- `zellij-tile` API (`ZellijPlugin`, event model): https://docs.rs/zellij-tile/latest/zellij_tile/ and https://docs.rs/zellij-tile/latest/zellij_tile/trait.ZellijPlugin.html (**HIGH**)
- Cargo package layout and module organization guidance: https://doc.rust-lang.org/cargo/guide/project-layout.html and https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html (**HIGH**)
- WASI target rename (`wasm32-wasi` → `wasm32-wasip1`): https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html (**HIGH**)
- GitHub Actions workflow syntax, triggers, reusable workflows, artifacts, token permissions, Rust CI, release notes:  
  - https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions  
  - https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows  
  - https://docs.github.com/en/actions/how-tos/reuse-automations/reuse-workflows  
  - https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts  
  - https://docs.github.com/en/actions/security-guides/automatic-token-authentication  
  - https://docs.github.com/en/actions/use-cases-and-examples/building-and-testing/building-and-testing-rust  
  - https://docs.github.com/en/repositories/releasing-projects-on-github/automatically-generated-release-notes (**HIGH**)
- GitHub CLI release automation reference (`gh release create`): https://cli.github.com/manual/gh_release_create (**MEDIUM-HIGH**, official but CLI-layer)
- Documentation architecture framework (Diátaxis): https://diataxis.fr/ (**MEDIUM-HIGH**)

---
*Architecture research for: Zellij plugin productization*
*Researched: 2026-04-04*
