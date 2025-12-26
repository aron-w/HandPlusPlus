# HandPlusPlus

> Cross-platform hotkey automation in Rust - Learn architecture by building a real-world AutoHotkey replacement

[![Rust](https://img.shields.io/badge/Rust-2026-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Project Mission

HandPlusPlus is a **dual-purpose project**:

1. **Learning Vehicle**: Master software architecture principles (Arc42, domain-driven design, platform abstraction)
2. **Practical Tool**: Replace AutoHotkey with a native, cross-platform Rust application for gaming automation

**Current Status**: **Foundation Phase** - Architecture documented, workspace structure created, ready for implementation

---

## What is HandPlusPlus?

A desktop application for automating complex keyboard and mouse input sequences with:

- **Global Hotkeys**: System-wide bindings that work across all applications
- **Complex Sequences**: Support for loops, delays, randomization, conditional logic
- **Command Palette**: Ctrl+P style interface for quick actions (inspired by VSCode)
- **Configuration-as-Code**: Define bindings in Rust (type-safe, compile-time validated)

### Example Use Case

```rust
// Gaming macro: Hold Mouse4 → rapid right-click (30-80ms random) → Enter on release
Hotkey::mouse(MouseButton::Button4)
    .action(
        Action::while_held(
            Action::repeat(
                Action::click(RightButton)
                    .then(Action::random_delay(30..80))
            )
        )
        .then_on_release(Action::press(Enter))
    )
```

---

## Architecture

**Arc42-First Development**: All architectural decisions are documented **before** implementation.

```
┌──────────────────────────────────────────────────────────┐
│                   HandPlusPlus                            │
│                                                           │
│  Input Capture  →  Binding Engine  →  Action Executor   │
│   (OS Hooks)       (State Machine)      (SendInput)      │
└──────────────────────────────────────────────────────────┘
```

**Key Patterns**:
- **Event-Driven Pipeline**: OS events → matched against bindings → actions executed
- **Platform Abstraction via Traits**: `InputCapture`, `ActionExecutor`
- **Configuration-as-Code**: Bindings compiled into binary (v1)
- **Domain-Driven Design**: Clear bounded contexts per crate

**Full documentation**: [docs/arc42/](docs/arc42/)

---

## Quick Start
### Prerequisites

- **NixOS / Linux**: Nix with flakes enabled
- **Windows 11**: WSL2 + Nix (see [docs/setup/nix-environment.md](docs/setup/nix-environment.md))

### Setup

```bash
# Clone repository
git clone https://github.com/yourusername/HandPlusPlus.git
cd HandPlusPlus

# Allow direnv to load development environment
direnv allow

# Or manually enter Nix shell
nix develop

# Build the project
cargo build

# Run (currently shows placeholder messages)
cargo run -p handplusplus
```

### Verify Environment

```bash
rustc --version   # Should show Rust stable
cargo --version
rust-analyzer --version
```

---

## Project Structure

```
HandPlusPlus/
├── docs/
│   ├── arc42/              # Architecture documentation (Chapters 1-5)
│   ├── research/           # Platform API comparisons
│   └── setup/              # Development environment guides
├── crates/
│   ├── handplusplus/       # Main application (entry point)
│   ├── input-capture/      # Platform abstraction for hotkey capture
│   ├── action-executor/    # Platform abstraction for input simulation
│   ├── binding-engine/     # Event→Action matching & state machine
│   ├── palette-ui/         # Command palette interface
│   └── config/             # User bindings (configuration-as-code)
├── Cargo.toml              # Workspace definition
├── flake.nix               # Nix development environment
└── .envrc                  # direnv integration
```

---

## Development Workflow

### Build & Test

```bash
# Build all crates
cargo build

# Build optimized release
cargo build --release

# Run tests
cargo test --workspace

# Run linter
cargo clippy

# Auto-rebuild on file changes
cargo watch -x run
```

### Adding a New Binding

1. Edit `crates/config/src/lib.rs`
2. Define hotkey and action using builder API
3. Recompile: `cargo build`

### Adding Platform Support

1. Implement `InputCapture` trait in new platform module
2. Implement `ActionExecutor` trait
3. Add `#[cfg(target_os = "...")]` conditional compilation
4. **Update Arc42 Chapter 3** (System Context)
5. Document in `docs/research/platform-input-apis.md`

**See**: [.github/copilot-instructions.md](.github/copilot-instructions.md) for detailed patterns

---

## Learning Objectives

This project teaches:

1. **Arc42 Architecture Documentation**: Document before code
2. **Platform Abstraction Patterns**: Traits for OS-specific code
3. **Event-Driven Architecture**: Async event processing with Tokio
4. **Domain-Driven Design**: Clear module boundaries
5. **Rust Best Practices**: Workspace organization, error handling, testing

---

## 🗺️ Roadmap

### v0.1 - Foundation (Current)
- [x] Arc42 documentation (Chapters 1-5)
- [x] Workspace structure
- [ ] Windows input capture (SetWindowsHookEx)
- [ ] Windows input simulation (SendInput)
- [ ] Linux X11 capture (XRecord)
- [ ] Linux X11 simulation (XTest)
- [ ] Basic binding engine
- [ ] Configuration-as-code API

### v0.2 - Usability
- [ ] Command palette UI (egui or iced)
- [ ] System tray integration
- [ ] TOML/YAML configuration DSL
- [ ] Hot-reload support
- [ ] More action types (mouse move, scrolling)

### v0.3 - Advanced
- [ ] GUI configuration builder
- [ ] Macro recording
- [ ] Plugin system

---

## 📖 Documentation

| Document | Description |
|----------|-------------|
| [Arc42 Chapter 1](docs/arc42/01-introduction-and-goals.md) | Introduction & Goals |
| [Arc42 Chapter 2](docs/arc42/02-architecture-constraints.md) | Constraints |
| [Arc42 Chapter 3](docs/arc42/03-context-and-scope.md) | System Context & Scope |
| [Arc42 Chapter 4](docs/arc42/04-solution-strategy.md) | Solution Strategy & ADRs |
| [Arc42 Chapter 5](docs/arc42/05-building-block-view.md) | Building Blocks (Modules) |
| [Platform APIs Research](docs/research/platform-input-apis.md) | Windows vs Linux input APIs |
| [Nix Setup Guide](docs/setup/nix-environment.md) | Development environment |
| [Copilot Instructions](.github/copilot-instructions.md) | AI agent guidance |

---

## Contributing

This is currently a personal learning project. Contributions are welcome once the foundation is stable (v0.2+).

**Before contributing**:
1. Read [Arc42 documentation](docs/arc42/)
2. Understand platform abstraction pattern
3. Check [docs/research/platform-input-apis.md](docs/research/platform-input-apis.md) for API details

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Related Projects

- [AutoHotkey](https://www.autohotkey.com/) - Windows automation (inspiration)
- [rdev](https://github.com/Narsil/rdev) - Cross-platform input capture
- [global-hotkey](https://github.com/tauri-apps/global-hotkey) - Hotkey registration
- [x11rb](https://github.com/psychon/x11rb) - Safe X11 bindings

---


## Status
 Architecture complete, implementation in progress. See [Roadmap](#-roadmap) for current phase.
