# Pitfalls Research

**Domain:** Rust + Zellij terminal plugin OSS productization
**Researched:** 2026-04-04
**Confidence:** MEDIUM-HIGH

## Critical Pitfalls

### Pitfall 1: Permission UX is undocumented or invisible

**What goes wrong:**
Plugin appears "broken" because users never successfully grant permissions, especially in small status/tab panes or background loading flows.

**Why it happens:**
Zellij permissions are runtime-granted and context-sensitive, but plugin docs often only describe install/config—not first-run permission flows or denial behavior.

**How to avoid:**
- Add a dedicated **Permissions** section in README with exact prompts users should expect (`y/n` behavior, where prompt appears, recovery steps).
- Implement explicit denied-state UI ("Permission denied: open plugin manager or relaunch with visible pane").
- Add a troubleshooting step for clearing/repairing permissions cache and retrying.

**Warning signs:**
- Issues/comments saying "plugin loads but does nothing".
- Users report no permission dialog seen in tiny panes.
- Repeated support questions about "needs restart" or "works only after random keypress".

**Phase to address:**
**Phase 1 (Docs + Onboarding UX)** and **Phase 2 (Runtime UX hardening)**.

---

### Pitfall 2: Config UX is ad-hoc (hardcoded behavior, no schema, no migration path)

**What goes wrong:**
Plugin works only for the author's environment (eg. hardcoded pane name), and config changes/reloads behave inconsistently across layouts/aliases/resurrection.

**Why it happens:**
Teams treat configuration as "just a string map" and skip defining canonical keys, defaults, and compatibility behavior.

**How to avoid:**
- Define a **versioned config contract** (`config_version`, defaults, validation, deprecations).
- Remove hardcoded targeting (eg. pane-title contains "opencode") and expose explicit keys.
- Add startup validation that prints actionable config errors, not silent fallback.
- Document precedence: layout config vs alias config vs runtime config-change event.

**Warning signs:**
- "Reload plugin" does not reflect updated config.
- Different behavior between fresh session and resurrected session.
- Growing list of one-off config keys with no typed parsing.

**Phase to address:**
**Phase 2 (Config model + UX)**.

---

### Pitfall 3: Input/focus model mismatch (plugin pane vs target pane)

**What goes wrong:**
Scroll/search commands affect the wrong pane (or the plugin itself), causing "it works sometimes" behavior.

**Why it happens:**
In Zellij, many actions operate on focused pane; plugin authors assume implicit target routing without tracking pane IDs and focus state.

**How to avoid:**
- Resolve and store explicit target pane ID(s), not title heuristics alone.
- Add visible "current target" indicator in plugin UI.
- Build tests for key handling + focus transitions (focused plugin, focused terminal, multiple tabs).

**Warning signs:**
- Bug reports with "scroll key does nothing" or "scrolls wrong window".
- Repro requires specific focus order to work.

**Phase to address:**
**Phase 2 (Behavior correctness + test harness)**.

---

### Pitfall 4: CI/release still reflects local-dev assumptions

**What goes wrong:**
Releases publish broken/missing `.wasm` assets, or builds break after toolchain changes (eg. WASI target naming transitions).

**Why it happens:**
Plugin project keeps a local `build.sh` mindset and does not enforce deterministic CI builds, artifact checks, and install-path verification.

**How to avoid:**
- Build and attach `.wasm` in GitHub Actions on tag.
- Add release job checks:
  - artifact exists at expected path
  - size/hash generated (`sha256sum`)
  - smoke check: plugin launches in Zellij test session
- Pin Rust toolchain in CI (`rust-toolchain.toml`) and explicitly manage WASI target choice.

**Warning signs:**
- "GitHub release exists but no wasm asset".
- Users report install URL 404 / wrong filename.
- Build failures after Rust upgrades around `wasm32-wasi` / `wasm32-wasip1` naming.

**Phase to address:**
**Phase 3 (CI/CD + release pipeline)**.

---

### Pitfall 5: Remote distribution semantics are ignored (cache/version pinning)

**What goes wrong:**
Users load stale or corrupted plugin binaries when using remote URLs, especially with mutable `latest/download` links or concurrent loads.

**Why it happens:**
Teams optimize for convenience links, not immutable release URLs + cache invalidation strategy.

**How to avoid:**
- In docs, recommend **versioned release URLs** (immutable tags) over floating `latest` links.
- Publish checksums per release and include verify instructions.
- Document `skip_plugin_cache` usage for upgrades/debugging.
- Prefer alias-based install examples so users can swap pinned versions cleanly.

**Warning signs:**
- "Works after clearing cache" reports.
- Random plugin load failures on fresh installs or multi-tab startup.
- Mismatch between reported plugin version and latest release.

**Phase to address:**
**Phase 3 (Release distribution hardening)** and **Phase 1 (Install docs)**.

---

### Pitfall 6: Maintenance plan ends at v1.0 (no compatibility/test policy)

**What goes wrong:**
Project accumulates regressions, stale issues, and incompatible behavior across Zellij versions despite "backwards compatible" expectations.

**Why it happens:**
No explicit maintenance contract: no supported-version matrix, no regression tests for core state transitions, no triage cadence.

**How to avoid:**
- Publish support policy: tested Zellij versions + Rust MSRV/toolchain policy.
- Add minimum regression suite (key handling, config parsing, pane targeting, permission-denied behavior).
- Set issue templates requesting Zellij version, layout snippet, and plugin config.
- Schedule dependency/API review each release cycle.

**Warning signs:**
- Reopened bugs after minor refactors.
- "Cannot reproduce" loops due to missing environment data.
- Large drift between docs and actual behavior.

**Phase to address:**
**Phase 4 (Post-release maintenance system)**.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hardcoding target pane title | Fast demo | Non-reusable plugin, brittle behavior | Never for OSS release |
| Stringly-typed config without validation | Quick iteration | Silent misconfig, support burden | MVP only if validation is next milestone |
| Manual release by local script | Fast first publish | Irreproducible artifacts, broken downloads | Never after first public users |
| No tests for state transitions | Saves initial time | Regressions in key/focus logic | Only pre-alpha prototypes |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Zellij permissions | Assume one-time request always visible/works | Design explicit granted/denied/pending UX; document recovery |
| Zellij config (layout/alias/runtime) | Assume one source of truth | Document precedence and test resurrection/reload flows |
| GitHub Releases + URL loading | Use mutable `latest` URLs only | Publish pinned version URLs + checksums + cache guidance |
| Rust WASI target | Keep legacy target naming forever | Pin toolchain and verify target naming in CI |

## "Looks Done But Isn't" Checklist

- [ ] **README install:** Includes permission prompt behavior, cache refresh path, and pinned URL example
- [ ] **Config support:** Keys are validated, defaults documented, and invalid values surfaced clearly
- [ ] **Release automation:** Tag creates `.wasm` + checksum + release notes automatically
- [ ] **Compatibility:** Tested against at least one current and one previous Zellij version
- [ ] **Regression safety:** Core key/focus/config transitions covered by automated tests

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Permission UX failure | MEDIUM | Add denied-state UI + docs patch + quick point release |
| Config model drift | HIGH | Introduce schema versioning + migration shim + deprecation notes |
| Broken release assets | MEDIUM | Rebuild from CI, republish assets, invalidate docs/examples |
| Cache/version mismatch in the field | MEDIUM | Publish cache-bypass guidance and pinned URL migration note |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Permission UX undocumented/invisible | Phase 1 + 2 | Fresh install test shows clear permission flow + denial path |
| Ad-hoc config model | Phase 2 | Invalid config fails loudly; reload/resurrection behavior documented |
| Input/focus mismatch | Phase 2 | Automated tests pass for focused plugin vs focused target pane |
| CI/release local assumptions | Phase 3 | Tag-triggered workflow produces downloadable wasm + checksum |
| Remote URL/cache pitfalls | Phase 1 + 3 | Docs use pinned URLs; upgrade guide includes cache strategy |
| No long-term maintenance contract | Phase 4 | Published support matrix + issue template + scheduled triage |

## Sources

- Zellij docs: Plugins overview, loading, lifecycle, API events/commands/permissions/configuration  
  - https://zellij.dev/documentation/plugins  
  - https://zellij.dev/documentation/plugin-loading.html  
  - https://zellij.dev/documentation/plugin-lifecycle.html  
  - https://zellij.dev/documentation/plugin-api-events.html  
  - https://zellij.dev/documentation/plugin-api-commands.html  
  - https://zellij.dev/documentation/plugin-api-permissions.html  
  - https://zellij.dev/documentation/plugin-api-configuration.html  
  - https://zellij.dev/documentation/keybindings-possible-actions.html#launchorfocusplugin
- Zellij docs: Plugin aliases and upgrade notes  
  - https://zellij.dev/documentation/plugin-aliases.html  
  - https://zellij.dev/documentation/plugin-upgrading.html
- Rust target docs (WASI target naming/status)  
  - https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html  
  - https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip2.html
- Zellij community issue evidence (real-world pitfalls)  
  - https://github.com/zellij-org/zellij/issues/4982 (background plugin permission flow)  
  - https://github.com/zellij-org/zellij/issues/4749 (permission prompt visibility in small panes)  
  - https://github.com/zellij-org/zellij/issues/3994 (reload not applying config)  
  - https://github.com/zellij-org/zellij/issues/3521 (config conflict on resurrection)  
  - https://github.com/zellij-org/zellij/issues/3479 (parallel remote plugin download/cache breakage)
- Project-local inputs  
  - `.planning/PROJECT.md`  
  - `.planning/codebase/CONCERNS.md`

---
*Pitfalls research for: Zellij plugin OSS productization*
*Researched: 2026-04-04*
