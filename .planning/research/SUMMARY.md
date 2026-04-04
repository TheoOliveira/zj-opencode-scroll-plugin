# Project Research Summary

**Project:** zj-opencode-scroll-plugin  
**Domain:** Production-ready open-source Zellij Rust/WASM plugin  
**Researched:** 2026-04-04  
**Confidence:** MEDIUM-HIGH

## Executive Summary

This project is a **productization effort**, not a greenfield build: the core plugin behavior exists, but public adoption depends on distribution quality, configurability, and operational reliability. The research converges on a standard expert approach for Zellij plugins: keep runtime logic modular (lifecycle shell + domain core), publish prebuilt `.wasm` assets on tagged releases, and provide install/config docs that work in copy-paste form without requiring users to compile Rust.

The recommended path is to sequence work by dependency: first stabilize source boundaries and typed configuration, then enforce CI quality gates, then automate release artifacts, and finally expand docs/examples and maintenance policy. This order is supported across all four research tracks and minimizes rework by avoiding docs and release promises on unstable interfaces.

The highest risks are not “deep algorithm” risks but **integration UX risks**: invisible permission prompts, ambiguous focus/target-pane semantics, config drift, and stale remote plugin caching. Mitigation is clear and concrete: explicit permission-denied states, typed and versioned config contract, deterministic pane targeting with tests, pinned immutable release URLs + checksums, and a tag-gated release pipeline.

## Key Findings

### Recommended Stack

Use a pinned Rust/WASM and Zellij-aligned baseline to eliminate CI drift and host/SDK mismatch: Rust **1.94.1**, `wasm32-wasip1`, and `zellij-tile@0.44.0`. Pair this with GitHub Actions for verify/release separation and Rust-native quality gates (`nextest`, `deny`, `audit`, `clippy`, `rustfmt`, `typos`).

The stack recommendation is intentionally conservative: avoid early packaging complexity (eg `cargo-dist`) and prioritize deterministic `.wasm` release assets with checksums.

**Core technologies:**
- **Rust 1.94.1 + wasm32-wasip1**: reproducible builds and current WASI target naming.
- **zellij-tile 0.44.0**: canonical plugin API aligned with Zellij host versions.
- **GitHub Actions (checkout v6)**: enforce CI quality + controlled release automation.
- **release-plz + gh release/asset upload**: reliable semver/changelog/release execution.
- **cargo-nextest / cargo-deny / cargo-audit / typos-cli**: OSS reliability and maintenance hygiene.

### Expected Features

Launch quality is defined by friction removal and predictability, not feature volume.

**Must have (table stakes):**
- Prebuilt `.wasm` artifacts in versioned GitHub Releases.
- Copy-paste install/activation snippets (`file:` and/or `https:`).
- Documented config contract (keys, defaults, formats).
- Explicit permission behavior + compatibility + troubleshooting docs.
- Configurable pane targeting (remove hardcoded pane assumptions).

**Should have (competitive):**
- Focus-safe scroll semantics bound to explicit target pane identity.
- Behavior presets (minimal/strict/aggressive) to simplify configuration.
- Robust examples pack (tested snippets + GIF/demo).
- Strong automation bar (CI + release + changelog discipline).

**Defer (v2+):**
- Large documentation platform split (full docs site/wiki) until volume justifies it.
- Multi-plugin bundling/monorepo strategy before single-plugin adoption is proven.
- Integration hooks (pipe/message automation) until core behavior semantics are stable.

### Architecture Approach

Adopt a modular runtime boundary: `plugin.rs` orchestrates lifecycle only; `config.rs` parses/validates once; `events.rs` maps event→intent; `state.rs` enforces deterministic transitions; `render/*` remains side-effect free; `actions.rs` encapsulates host commands. Delivery architecture should be two-stage CI/CD (verify then tag-gated release), with docs organized by Diátaxis and examples as executable contracts.

**Major components:**
1. **Plugin API adapter (`plugin.rs`)** — lifecycle wiring, subscriptions, top-level orchestration.
2. **Typed config boundary (`config.rs`)** — schema/defaults/validation/migrations from string map.
3. **Event + state core (`events.rs` + `state.rs`)** — deterministic intent routing and transitions.
4. **Render/actions split (`render/*` + `actions.rs`)** — clean separation of display vs host side effects.
5. **CI/release system (`.github/workflows`)** — quality gates and reproducible artifact publication.

### Critical Pitfalls

1. **Permission UX invisibility** — document prompts/denial recovery and show explicit denied-state UI.
2. **Ad-hoc config model** — introduce versioned typed config with validation and clear precedence.
3. **Focus/target mismatch** — track explicit target pane ID and test focus transitions.
4. **Local-only release assumptions** — tag-gated CI release with artifact existence/hash checks.
5. **Remote cache/version drift** — prefer immutable versioned URLs, publish checksums, document cache bypass.

## Implications for Roadmap

Based on cross-file dependencies, suggested phase structure:

### Phase 1: Product Surface Baseline (Structure + Onboarding Contract)
**Rationale:** Must stabilize module boundaries and install/onboarding expectations before adding feature breadth.  
**Delivers:** `lib.rs` decomposition, minimal typed config scaffold, README quickstart with pinned install paths, permissions/compatibility/troubleshooting baseline.  
**Addresses:** Table-stakes install snippets, compatibility statement, initial config contract.  
**Avoids:** Permission UX invisibility, README drift, hardcoded-workflow assumptions.

### Phase 2: Behavior Correctness & Config UX
**Rationale:** Core value depends on deterministic pane targeting and predictable runtime behavior.  
**Delivers:** Full typed config model, configurable target pane, explicit focus-safe semantics, event/state tests for focus and config reload paths.  
**Uses:** `zellij-tile@0.44.0` event/config APIs, modular boundaries from Phase 1.  
**Implements:** `config.rs` + `events.rs` + `state.rs` contracts.  
**Avoids:** Ad-hoc config drift, “works sometimes” focus bugs.

### Phase 3: CI/CD & Distribution Hardening
**Rationale:** Public adoption requires frictionless install and trustworthy release artifacts.  
**Delivers:** Verify workflow (fmt/clippy/tests/wasm build), tag-triggered release with `.wasm` + checksums, immutable install URL guidance, optional changelog automation.  
**Addresses:** P1 release automation + reliability differentiator.  
**Avoids:** Missing/broken assets, target naming drift, stale-cache confusion.

### Phase 4: Docs Depth & Maintenance System
**Rationale:** After behavior and release stability, scale contributor/user support sustainably.  
**Delivers:** Diátaxis docs expansion, examples pack, support/version policy, issue templates, release cadence hygiene.  
**Addresses:** Differentiator docs quality and long-term OSS maintainability.  
**Avoids:** Post-v1 maintenance decay and repeated support loops.

### Phase Ordering Rationale

- Config/documentation quality depends on stable module and config boundaries.
- Release automation should follow verified behavior (not precede it).
- Integration hooks and advanced presets should wait until focus/target semantics are proven.
- This sequence directly maps pitfall prevention to earliest feasible phase.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** Focus/target-pane behavior across tabs, resurrection, and config reload edge cases need scenario-specific validation.
- **Phase 3:** Remote distribution/cache semantics and release URL strategy may require additional field validation across user environments.

Phases with standard patterns (can likely skip deeper research):
- **Phase 1:** Module split + baseline docs are well-established Rust/Zellij patterns.
- **Phase 4:** Diátaxis docs structure and OSS maintenance templates are mature, low-uncertainty patterns.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Strong official-source grounding (Rust docs, Zellij docs, GitHub Actions docs, crates metadata). |
| Features | HIGH | Consistent ecosystem evidence across representative plugin repos + official API expectations. |
| Architecture | MEDIUM-HIGH | Patterns are robust and standard, but some project-specific implementation details remain to validate in code. |
| Pitfalls | MEDIUM-HIGH | Backed by official docs plus real issue evidence; exact prevalence in this repo needs implementation-time confirmation. |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- **Current code-to-target gap:** Exact delta between existing `lib.rs` behavior and proposed modular boundary/test plan should be validated at roadmap decomposition time.
- **Compatibility matrix definition:** Specific minimum supported Zellij versions and deprecation policy require explicit maintainer decision.
- **Permission flow UX details:** Prompt visibility/denial recovery should be tested in small-pane and background-loading scenarios before final docs freeze.
- **Release URL policy:** Decide whether docs default to immutable tag URLs only or include guarded `latest` guidance.

## Sources

### Primary (HIGH confidence)
- Rust official docs (stable channel + WASI target): toolchain pinning and `wasm32-wasip1` guidance.
- Zellij official docs (plugins/lifecycle/loading/config/events/commands/permissions): runtime model and user-facing integration behavior.
- GitHub Actions official docs (workflow syntax, triggers, permissions, artifacts, release notes): CI/CD and release architecture.
- `zellij-tile` docs.rs API: lifecycle and event API boundaries.

### Secondary (MEDIUM confidence)
- Ecosystem plugin repositories (`zjstatus`, `room`, `zellij-autolock`) for feature and release-distribution norms.
- `release-plz` and `softprops/action-gh-release` project docs for release automation conventions.
- Diátaxis framework docs for documentation architecture.

### Tertiary (LOW confidence)
- Community issue threads used as pitfall signal (useful directional evidence; severity/frequency still context-dependent).

---
*Research completed: 2026-04-04*  
*Ready for roadmap: yes*
