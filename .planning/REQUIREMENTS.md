# Requirements: zj-opencode-scroll-plugin

**Defined:** 2026-04-04
**Core Value:** Making a highly functional, specialized Zellij scroll plugin accessible, configurable, and easy to install for the broader public.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Documentation

- [ ] **DOC-01**: User can install the plugin from a prebuilt release artifact by following a copy-paste quickstart in README.
- [ ] **DOC-02**: User can configure the plugin using a documented key/value reference with defaults and examples.
- [ ] **DOC-03**: User can understand required plugin permissions and recover from denied permissions using documented guidance.
- [ ] **DOC-04**: User can troubleshoot common install/runtime problems using documented cache-reset and log-check steps.
- [ ] **DOC-05**: Contributor can start contributing by following a CONTRIBUTING guide with setup, standards, and PR flow.

### Configuration

- [ ] **CFG-01**: User can set plugin target pane behavior via configuration rather than hardcoded pane assumptions.
- [ ] **CFG-02**: User can run the plugin with sensible defaults when no explicit configuration is provided.
- [ ] **CFG-03**: User receives clear validation feedback when configuration values are invalid.

### Releases & CI

- [ ] **REL-01**: User can download a versioned `.wasm` plugin artifact from each tagged GitHub Release.
- [ ] **REL-02**: User can trust release artifacts via published checksums and deterministic artifact naming.
- [ ] **REL-03**: Maintainer can trigger automated release publishing from semver git tags.
- [ ] **REL-04**: Maintainer can rely on CI quality gates (format, lint, tests, wasm build) before release publication.

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Behavior Extensions

- **BEH-01**: User can select behavior presets (minimal/strict/aggressive) for scroll interaction style.
- **BEH-02**: User can use integration hooks (pipe/message commands) for workflow automation.

### Docs Scale

- **DOC-06**: User can navigate expanded docs in a full docs site or wiki with deeper tutorials and references.

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Full terminal UI redesign | Project focus is productization/release readiness of existing plugin behavior |
| Multi-plugin monorepo strategy | Premature before validating adoption of this single plugin |
| Broad advanced integration surface in v1 | Adds complexity before core distribution and configurability are stable |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| DOC-01 | Phase TBD | Pending |
| DOC-02 | Phase TBD | Pending |
| DOC-03 | Phase TBD | Pending |
| DOC-04 | Phase TBD | Pending |
| DOC-05 | Phase TBD | Pending |
| CFG-01 | Phase TBD | Pending |
| CFG-02 | Phase TBD | Pending |
| CFG-03 | Phase TBD | Pending |
| REL-01 | Phase TBD | Pending |
| REL-02 | Phase TBD | Pending |
| REL-03 | Phase TBD | Pending |
| REL-04 | Phase TBD | Pending |

**Coverage:**
- v1 requirements: 12 total
- Mapped to phases: 0
- Unmapped: 12 ⚠️

---
*Requirements defined: 2026-04-04*
*Last updated: 2026-04-04 after initial definition*
