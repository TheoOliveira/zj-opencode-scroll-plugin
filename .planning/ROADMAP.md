# Roadmap: zj-opencode-scroll-plugin

## Overview

This roadmap takes the plugin from “working locally” to “ready for broad public adoption” by sequencing work around real user outcomes: predictable configuration behavior, trustworthy release distribution, and documentation that lets both users and contributors succeed without guesswork.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (1.1, 1.2): Urgent insertions (if needed later)

- [ ] **Phase 1: Configuration Contract & Runtime Safety** - Make plugin behavior configurable, default-safe, and validation-friendly.
- [ ] **Phase 2: Release Automation & Artifact Trust** - Ship deterministic `.wasm` artifacts through gated CI and tag-driven releases.
- [ ] **Phase 3: User + Contributor Documentation Surface** - Provide complete install/config/troubleshooting/contribution guidance.

## Phase Details

### Phase 1: Configuration Contract & Runtime Safety
**Goal**: Users can run and control plugin behavior through a reliable configuration contract instead of hardcoded assumptions.
**Depends on**: Nothing (first phase)
**Requirements**: CFG-01, CFG-02, CFG-03
**Success Criteria** (what must be TRUE):
  1. User can run the plugin successfully with no explicit configuration and get sensible default behavior.
  2. User can set target pane behavior via configuration and observe plugin behavior follow that configuration.
  3. User receives clear validation feedback when configuration values are invalid.
**Plans**: TBD

### Phase 2: Release Automation & Artifact Trust
**Goal**: Users and maintainers can rely on automated, repeatable, and verifiable release delivery for `.wasm` artifacts.
**Depends on**: Phase 1
**Requirements**: REL-01, REL-02, REL-03, REL-04
**Success Criteria** (what must be TRUE):
  1. Each semver tag produces a GitHub Release with a versioned `.wasm` artifact available for download.
  2. Release artifacts use deterministic naming and include published checksums users can verify.
  3. Maintainer can trigger release publishing through semver tagging without manual artifact assembly.
  4. Release publication is blocked unless CI quality gates (format, lint, tests, wasm build) pass.
**Plans**: TBD

### Phase 3: User + Contributor Documentation Surface
**Goal**: Users and contributors can install, configure, troubleshoot, and contribute using documentation alone.
**Depends on**: Phase 1, Phase 2
**Requirements**: DOC-01, DOC-02, DOC-03, DOC-04, DOC-05
**Success Criteria** (what must be TRUE):
  1. User can install the plugin from a prebuilt release artifact by following a copy-paste README quickstart.
  2. User can configure the plugin from documented key/value references that include defaults and examples.
  3. User can understand required permissions and recover from denied permissions using documentation guidance.
  4. User can troubleshoot common install/runtime failures using documented cache-reset and log-check steps.
  5. Contributor can set up the project and submit a contribution using CONTRIBUTING instructions.
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Configuration Contract & Runtime Safety | 0/TBD | Not started | - |
| 2. Release Automation & Artifact Trust | 0/TBD | Not started | - |
| 3. User + Contributor Documentation Surface | 0/TBD | Not started | - |
