# Feature Research

**Domain:** Open-source Zellij plugin repository productization (public adoption)
**Researched:** 2026-04-04
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Versioned GitHub releases with prebuilt `.wasm` artifact | Most popular Zellij plugins ask users to download a ready-made `.wasm` from Releases rather than compile locally | MEDIUM | Trigger release on semver tags (`v*.*.*`), attach `target/wasm32-wasip1/release/<plugin>.wasm` |
| Copy-paste install + activation snippets (`file:` and/or `https:`) | Zellij plugin loading is URL-based; users expect exact `config.kdl` or keybind snippets to work immediately | LOW | Include both `LaunchOrFocusPlugin` and `load_plugins` examples when relevant |
| Configuration contract documented (keys, defaults, value formats) | Zellij plugins are configured by key/value map; users expect stable options and defaults | MEDIUM | Add explicit config table; avoid hidden magic values |
| Permission behavior clearly documented | Plugins can request sensitive permissions and prompt users; users expect to know why prompts appear | MEDIUM | List required permissions and which features degrade when denied |
| Compatibility statement (minimum Zellij version, target, platform caveats) | Plugin behavior depends on Zellij API/version and WASI target; users expect known-good versions | LOW | Include “Tested on Zellij >= X” and wasm target used in CI |
| Troubleshooting section (cache reset + logs) | Plugin cache/permission state issues are common in real usage | LOW | Include cache paths (Linux/macOS) and link to Zellij plugin logs docs |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| First-class pane targeting configuration (no hardcoded pane title) | Makes plugin usable beyond one workflow; broadens adoption | MEDIUM | Replace hardcoded `"opencode"` matching with configurable selector + sensible default |
| Safe, predictable scroll-control semantics | Reduces confusion when plugin focus differs from target pane focus | HIGH | Explicitly bind actions to target pane ID rather than “current focus” assumptions |
| “Behavior presets” (minimal/strict/aggressive) | Gives useful defaults without forcing many knobs | MEDIUM | Map multiple internal settings to one preset key; keep advanced overrides optional |
| Robust examples pack (GIF + tested snippets for common setups) | Shortens time-to-success and lowers support burden | LOW | Ship `examples/` with verified KDL snippets for keybinding and layout modes |
| Automation quality bar (lint/test/release + changelog) | Signals reliability and reduces regressions for public users | MEDIUM | CI: lint/test on PR; release workflow on tags; optional generated changelog |
| Integration hooks (pipe/message commands for automation) | Enables advanced workflows and ecosystem integration | HIGH | Add documented message/pipe commands only after core UX is stable |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| “Source-only install” (no release artifacts) | Maintainers avoid CI/release setup | Blocks mainstream users; high friction; inconsistent builds | Publish signed/tagged release `.wasm` artifacts |
| Massive config surface at launch | Power users request deep customization quickly | Increases support load and docs debt before core behavior is stable | Start with small, opinionated config + presets; expand from user feedback |
| Hardcoded pane name/workflow assumptions | Fast path during early development | Fails for most users; perceived as broken | Configurable pane targeting with fallback logic |
| Requesting broad permissions “just in case” | Easier implementation | Trust/safety concerns; prompt fatigue | Request minimum permissions and document why each is needed |

## Feature Dependencies

```text
[Release artifact pipeline]
    └──requires──> [Semver tag strategy]
                        └──requires──> [Versioning policy in docs]

[Install snippets]
    └──requires──> [Stable plugin location strategy (file/https)]

[Config reference]
    └──requires──> [Config keys implemented in load() path]
                        └──requires──> [No hardcoded pane assumptions]

[Permission documentation]
    └──requires──> [Permission minimization in code]

[Integration hooks (pipe/message)]
    └──requires──> [Stable pane targeting + deterministic behavior]
```

### Dependency Notes

- **Release artifact pipeline requires semver tags:** users discover/install via `releases/latest/download/*.wasm`; without disciplined tagging, install docs drift.
- **Config reference requires implemented config path:** documenting keys before wiring runtime behavior (eg, pane selector) creates false expectations.
- **Permission documentation requires permission minimization:** clear docs only help if requested permissions are actually scoped to features.
- **Integration hooks require deterministic core behavior:** automation commands are brittle if focus/target-pane semantics are ambiguous.

## MVP Definition

### Launch With (v1)

Minimum viable product — what's needed to validate the concept.

- [ ] Prebuilt `.wasm` in GitHub Releases via tag-triggered workflow — removes compile friction.
- [ ] README with copy-paste install + keybind/layout snippets — enables immediate adoption.
- [ ] Configurable pane target + documented defaults — removes hardcoded workflow lock-in.
- [ ] Permission/compatibility/troubleshooting sections — reduces onboarding and support churn.

### Add After Validation (v1.x)

Features to add once core is working.

- [ ] Presets + expanded config matrix — add once baseline usage patterns emerge.
- [ ] Integration hooks (pipe/message commands) — add after behavior semantics are stable.
- [ ] CONTRIBUTING + lightweight architecture notes — expand once external contribution flow starts.

### Future Consideration (v2+)

Features to defer until product-market fit is established.

- [ ] Auto-generated docs site or wiki split — defer until docs volume justifies maintenance cost.
- [ ] Multi-plugin bundle/repo monorepo strategy — defer until single-plugin adoption is proven.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Release `.wasm` automation | HIGH | MEDIUM | P1 |
| Install/config docs with examples | HIGH | LOW | P1 |
| Configurable pane targeting | HIGH | MEDIUM | P1 |
| Permission + compatibility docs | HIGH | LOW | P1 |
| Focus-safe scroll semantics | HIGH | HIGH | P2 |
| Presets and advanced customization | MEDIUM | MEDIUM | P2 |
| Pipe/message integration hooks | MEDIUM | HIGH | P3 |

**Priority key:**
- P1: Must have for launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

## Competitor Feature Analysis

| Feature | zjstatus | room | Our Approach |
|---------|----------|------|--------------|
| Release artifact distribution | Yes (`releases/latest/download/*.wasm`) | Yes (`room.wasm` in releases) | Match this as baseline; no source-only install |
| Copy-paste config snippets | Extensive (layout + load_plugins examples) | Clear keybind snippet | Provide minimal “works in 2 minutes” snippet first, then advanced examples |
| Config documentation depth | High (wiki + examples) | Moderate (README options) | Keep README canonical; optional wiki later |
| CI/release automation | Yes (tag-triggered release workflow) | Yes (simple tag release workflow) | Start simple tag-based release; add changelog automation after stability |

## Sources

- Zellij docs: Plugins, Loading, Configuration, Permissions, Events, Commands
  - https://zellij.dev/documentation/plugins (HIGH)
  - https://zellij.dev/documentation/plugin-loading (HIGH)
  - https://zellij.dev/documentation/plugin-api-configuration (HIGH)
  - https://zellij.dev/documentation/plugin-api-permissions (HIGH)
  - https://zellij.dev/documentation/plugin-api-events (HIGH)
  - https://zellij.dev/documentation/plugin-api-commands (HIGH)
- Ecosystem scan (topic + representative repos)
  - https://github.com/topics/zellij-plugin (MEDIUM: discovery source)
  - https://github.com/dj95/zjstatus + README/workflows (MEDIUM-HIGH)
  - https://github.com/rvcas/room + README/workflows (MEDIUM-HIGH)
  - https://github.com/fresh2dev/zellij-autolock + README/workflow (MEDIUM-HIGH)
- Project context
  - `.planning/PROJECT.md`
  - `.planning/codebase/CONCERNS.md`

---
*Feature research for: Open-source Zellij plugin productization*
*Researched: 2026-04-04*
