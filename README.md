# opencode-scroll-plugin

[![Release](https://img.shields.io/github/v/release/TheoOliveira/zj-opencode-scroll-plugin?display_name=tag&sort=semver)](https://github.com/TheoOliveira/zj-opencode-scroll-plugin/releases)
[![Downloads](https://img.shields.io/github/downloads/TheoOliveira/zj-opencode-scroll-plugin/total)](https://github.com/TheoOliveira/zj-opencode-scroll-plugin/releases)
[![License](https://img.shields.io/github/license/TheoOliveira/zj-opencode-scroll-plugin)](#license)
[![Rust](https://img.shields.io/badge/Rust-2021-black?logo=rust)](https://www.rust-lang.org/)
[![Zellij](https://img.shields.io/badge/Zellij-Plugin-2f855a)](https://zellij.dev/)

Fast, predictable scrolling for long `opencode` output inside Zellij.

`opencode-scroll-plugin` keeps navigation tight when panes are noisy: quick movement keys, follow mode, and an error-focused lock that helps you keep failures in view.

## Why this plugin

- Built for long-running AI/tool output where default scroll flow feels slow.
- Keeps context stable while you inspect errors and jump back to live output.
- Lightweight UI: useful status and search controls without clutter.

## Features

- Smart pane detection for panes with `opencode` in the title.
- Fast movement: line scroll, page scroll, jump top/bottom.
- Error lock mode to keep known failure states visible.
- Follow mode to return to newest output instantly.
- In-plugin search bar for quick log navigation.

## Demo

Short terminal demo (navigation + follow mode + error lock):

![Demo preview coming soon](https://img.shields.io/badge/demo-GIF%20coming%20soon-1f6feb)

Until the GIF lands, this is the fastest way to try the flow:

1. Start with `zellij --layout opencode.kdl`
2. Generate long output in your `opencode` pane
3. Test keys: `u`, `d`, `g`, `G`, `/`, and `c`

## Quick Start (Release)

Install the latest prebuilt `.wasm` and run it:

```bash
mkdir -p ~/.config/zellij/plugins
curl -fL \
  https://github.com/TheoOliveira/zj-opencode-scroll-plugin/releases/latest/download/opencode-scroll-plugin.wasm \
  -o ~/.config/zellij/plugins/opencode-scroll-plugin.wasm

zellij run --plugin file:~/.config/zellij/plugins/opencode-scroll-plugin.wasm
```

## Configure Once in Zellij

Add plugin aliases to `~/.config/zellij/config.kdl`:

```kdl
plugins {
  opencode-scroll location="https://github.com/TheoOliveira/zj-opencode-scroll-plugin/releases/latest/download/opencode-scroll-plugin.wasm"
  // Optional local-dev override:
  // opencode-scroll-dev location="file:~/.config/zellij/plugins/opencode-scroll-plugin.wasm"
}
```

Then use it in any layout:

```kdl
pane {
  plugin location="opencode-scroll"
}
```

You can also use the included layout directly: `zellij --layout opencode.kdl`.

## Local Development

Build + install to local plugins dir:

```bash
./build.sh
```

Installed path:

`~/.config/zellij/plugins/opencode-scroll-plugin.wasm`

## Keybindings

| Key | Action |
| --- | --- |
| `u` or `PageUp` | Scroll up 10 lines |
| `d` or `PageDown` | Scroll down 10 lines |
| `Up` | Scroll up 1 line |
| `Down` | Scroll down 1 line |
| `g` or `Home` | Jump to top |
| `G` or `End` | Jump to bottom (follow mode) |
| `/` | Open search input |
| `c` | Clear error lock and follow latest output |
| `Esc` or `Enter` | Close search input |

## Build from Source

Prerequisites:

- Rust toolchain
- `wasm32-wasip1` target (`rustup target add wasm32-wasip1`)
- Zellij `0.42.1+`

Build commands:

```bash
cargo check
cargo build --release --target wasm32-wasip1
```

Artifact:

`target/wasm32-wasip1/release/opencode_scroll_plugin.wasm`

## Compatibility

- Built with `zellij-tile = 0.42.1`.
- Runtime target is WASI Preview1 (`wasm32-wasip1`).
- Best experience when the target pane title includes `opencode`.

## Contributing

Contributions are welcome and appreciated.

1. Fork and branch: `git checkout -b feat/my-change`
2. Make focused changes with clear commit messages
3. Verify locally:
   - `cargo check`
   - `cargo build --release --target wasm32-wasip1`
4. Update docs if behavior changes
5. Open a PR with problem, approach, and verification notes

If you are unsure where to start, open an issue with your idea first.

## Roadmap

- User-configurable keybindings/behavior.
- Search UX improvements (navigation + highlighting).
- Configurable error-pattern sets.
- Expanded examples and docs.

## License

MIT
