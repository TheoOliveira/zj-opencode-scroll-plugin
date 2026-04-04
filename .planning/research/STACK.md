# Stack Research

**Domain:** Production-ready open-source Zellij plugin (Rust/WASM)  
**Researched:** 2026-04-04  
**Confidence:** MEDIUM-HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended | Confidence |
|------------|---------|---------|-----------------|------------|
| Rust toolchain | **1.94.1 stable** (channel date 2026-03-26) | Build, lint, test, cross-target compile | Pinning a known stable toolchain removes CI drift and keeps plugin builds reproducible across contributors. | HIGH |
| Zellij plugin SDK (`zellij-tile`) | **0.44.0** | Official plugin API (events, rendering, commands) | This is the canonical SDK used by Zellij plugin docs and examples; version-aligning with Zellij avoids API mismatch churn. | HIGH |
| WASI compile target | **wasm32-wasip1** | Build `.wasm` artifact for plugin runtime | Rust renamed `wasm32-wasi` to `wasm32-wasip1`; adopting the current target avoids deprecation/terminology drift while keeping WASI Preview1 compatibility. | HIGH |
| GitHub Actions | **actions/checkout@v6** + workflow perms | CI and release automation | Current official action major; combines least-privilege token controls with modern runner compatibility for predictable OSS maintenance. | HIGH |
| Release automation | **release-plz/action@v0.5.128** | Release PRs, version/changelog automation | This is the current Rust-native release workflow standard in 2026 for OSS crates/repos, minimizing manual release mistakes and keeping changelogs disciplined. | MEDIUM |

### Supporting Libraries & Tools

| Library / Tool | Version | Purpose | When to Use | Confidence |
|---|---:|---|---|---|
| `cargo-nextest` | **0.9.132** | Fast, reliable Rust test runner | Use in CI for PR/test workflows (faster and better reporting than plain `cargo test` at scale). | HIGH |
| `cargo-deny` | **0.19.0** | License and dependency policy checks | Use on every PR to prevent license/security policy regressions in transitive deps. | HIGH |
| `cargo-audit` | **0.22.1** | RustSec vulnerability scanning | Use in scheduled CI and release gating for vulnerability detection. | HIGH |
| `typos-cli` | **1.45.0** | Spellcheck docs/config/code identifiers | Use as low-friction quality gate for public OSS polish. | HIGH |
| `wasm-opt` (Binaryen) | latest stable Binaryen release | Optimize `.wasm` size/perf | Use in release builds before attaching artifacts to GitHub Releases. | MEDIUM |
| `softprops/action-gh-release` | **v2.6.1** | Upload `.wasm` + checksums as release assets | Use on tag/release workflows when you need deterministic, explicit artifact upload behavior. | HIGH |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `rustfmt` + `clippy` (from pinned toolchain) | Formatting and linting | Run in CI with `-D warnings` for release branch quality.
| `just` (optional) | Task runner for consistent local/CI commands | Prefer over ad-hoc shell scripts for repeatable developer workflows.
| `taplo` (optional) | TOML formatting/linting | Useful once CI/release config grows (Cargo, workflow fragments, metadata).

## Installation

```bash
# Pin toolchain (commit rust-toolchain.toml)
rustup toolchain install 1.94.1
rustup target add wasm32-wasip1

# Project dependency
cargo add zellij-tile@0.44.0

# CI / QA toolchain
cargo install cargo-nextest --version 0.9.132
cargo install cargo-deny --version 0.19.0
cargo install cargo-audit --version 0.22.1
cargo install typos-cli --version 1.45.0
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| `release-plz` for release PR/versioning | Manual tag + handwritten changelog | Only for very low-frequency hobby releases where automation overhead is unwanted. |
| `softprops/action-gh-release` asset upload | GitHub CLI (`gh release create`) in shell step | Use CLI only if you need highly custom release scripting and are willing to maintain that script surface. |
| `wasm32-wasip1` target naming | legacy `wasm32-wasi` naming | Temporary compatibility in older docs/scripts; migrate new automation to `wasm32-wasip1`. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Ad-hoc `build.sh` as primary release path | Not reproducible across contributors/runners; easy to drift from CI behavior; poor auditability. | Declarative GitHub Actions workflows + pinned toolchain + explicit release job. |
| Unpinned floating action/tool versions (`@main`, unversioned installs) | Supply-chain and reproducibility risk; subtle CI breakage over time. | Pin action majors/minors (or SHAs for stricter repos) and pin critical cargo tool versions. |
| Treating plugin as “source-only install” for end users | Raises adoption friction; users should not compile Rust/WASM to try plugin. | Publish signed/tagged `.wasm` binaries with checksums on GitHub Releases. |
| Assuming `wasm32-wasi` naming is future-proof | Rust platform naming moved to `wasm32-wasip1`; stale naming creates maintenance debt. | Standardize on `wasm32-wasip1` in docs and CI (keep compatibility note only where required). |

## Stack Patterns by Variant

**If you want fastest path to public plugin release (recommended now):**
- Use: `zellij-tile` + pinned Rust + GitHub Actions + `softprops/action-gh-release`
- Because: you need reliable `.wasm` artifact publishing first, not packaging complexity.

**If you later expand into multi-artifact distribution (installers/packages):**
- Add: `cargo-dist@0.31.0`
- Because: it shines for cross-platform native packaging, but is overkill for single `.wasm` plugin release early on.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `zellij-tile@0.44.0` | `zellij@0.44.x` | Keep SDK and host Zellij minor versions aligned to reduce API surprises. |
| `rust@1.94.1` | `wasm32-wasip1` target | Target is current Rust naming for WASI Preview1. |
| `actions/checkout@v6` | Actions Runner `>= v2.329.0` (for container credential behavior) | Documented in checkout v6 README. |
| `release-plz/action@v0.5.128` | Repo with proper workflow permissions | Requires explicit GitHub token permissions for PR/release operations. |

## Prescriptive Stack Decision (2026 default)

Use this baseline for this repo:

1. **Pinned Rust toolchain (`1.94.1`) + `wasm32-wasip1` target**
2. **`zellij-tile@0.44.0`**
3. **GitHub Actions CI:** fmt, clippy, nextest, deny, audit, typos
4. **Release workflow:** semantic version/tag process + `.wasm` + checksum upload to GitHub Releases
5. **Keep `cargo-dist` out of MVP** unless you start shipping non-WASM artifacts

This is the lowest-complexity stack that still meets “production-ready OSS maintenance” standards.

## Sources

- Rust stable channel manifest: https://static.rust-lang.org/dist/channel-rust-stable.toml (HIGH)
- Rust target docs (`wasm32-wasip1`): https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html (HIGH)
- Zellij plugin docs: https://zellij.dev/documentation/plugins , https://zellij.dev/documentation/plugin-lifecycle , https://zellij.dev/documentation/plugin-dev-env (MEDIUM: docs still show legacy target name in examples)
- crates.io API (`zellij`, `zellij-tile`, `release-plz`, `cargo-nextest`, `cargo-deny`, `cargo-audit`, `typos-cli`, `cargo-dist`) (HIGH)
- release-plz docs quickstart: https://release-plz.dev/docs/github/quickstart (HIGH)
- GitHub Actions checkout v6: https://github.com/actions/checkout (HIGH)
- GitHub token permissions guidance: https://docs.github.com/en/actions/security-guides/automatic-token-authentication#permissions-for-the-github_token (HIGH)
- `softprops/action-gh-release` repo docs/releases: https://github.com/softprops/action-gh-release (HIGH)
- Binaryen (`wasm-opt`) project docs: https://github.com/WebAssembly/binaryen (MEDIUM)

---
*Stack research for: Zellij plugin productization (release-readiness milestone)*
*Researched: 2026-04-04*
