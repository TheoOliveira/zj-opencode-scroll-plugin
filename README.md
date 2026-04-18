# opencode-scroll-plugin

A Zellij terminal multiplexer plugin that provides custom scrolling, auto-scroll to errors, and search functionality for the OpenCode AI assistant pane.

## Features

- **Auto-detection**: Automatically detects when a pane named "opencode" is focused
- **Custom scrolling**: Enhanced scroll controls beyond default Zellij behavior
- **Error lock**: Automatically scrolls to top and locks when errors/panics are detected in the output
- **Search**: Search through recent output with `/` key
- **Follow mode**: Quick jump between top/bottom of buffer

## Keybindings

| Key | Action |
|-----|--------|
| `u` / `PageUp` | Scroll up 10 lines |
| `d` / `PageDown` | Scroll down 10 lines |
| `g` / `Home` | Jump to top of buffer |
| `G` / `End` | Jump to bottom (follow mode) |
| `Up` | Scroll up 1 line |
| `Down` | Scroll down 1 line |
| `/` | Open search bar |
| `c` | Clear error lock, return to follow mode |
| `Esc` | Close search bar |

## Installation

### Prerequisites

- Rust toolchain with `wasm32-wasi` target:
  ```bash
  rustup target add wasm32-wasi
  ```
- Zellij 0.42+

### Build

```bash
./build.sh
```

This compiles the plugin to WebAssembly and copies it to `~/.config/zellij/plugins/`.

### Usage

**Option 1: Via layout**
```bash
zellij --layout opencode.kdl
```

**Option 2: Run manually**
```bash
zellij run --plugin ~/.config/zellij/plugins/opencode-scroll-plugin.wasm opencode
```

## Error Detection Keywords

The plugin auto-locks to top when output contains any of:
- `ERROR`, `error`, `FAIL`, `fail`, `panic`
- `thread 'main'`, `COMPILATION ERROR`, `Build failed`, `error:`

## Configuration

No configuration required. The plugin auto-detects any pane with "opencode" in its title.

## Development

```bash
# Check compilation
cargo check

# Build for release
cargo build --release --target wasm32-wasi
```

## License

MIT