# HandPlusPlus - AI Coding Agent Instructions

## Project Overview

**HandPlusPlus** is a cross-platform desktop application for automating complex keyboard and mouse input sequences, built in Rust. It serves as both a learning project for software architecture and a practical replacement for AutoHotkey on Windows and Linux.

**Domain**: System-level input automation for gaming and productivity  
**Primary Use Case**: Execute complex macros with precise timing (e.g., "hold Mouse4 → rapid right-click with randomization → press Enter on release")

## Architecture-First Workflow ⚠️

**CRITICAL**: This project follows **Arc42-first development**:
1. **No code before architecture**: Document decisions in `docs/arc42/` before implementing
2. **Create ADRs**: New patterns require Architecture Decision Records in `docs/adr/`
3. **Update Arc42**: Keep living documentation in sync with implementation

**Arc42 Structure**:
- [Chapter 1: Introduction & Goals](../docs/arc42/01-introduction-and-goals.md) - Quality goals, stakeholders
- [Chapter 2: Constraints](../docs/arc42/02-architecture-constraints.md) - Technical/organizational constraints
- [Chapter 3: Context & Scope](../docs/arc42/03-context-and-scope.md) - System boundaries, interfaces
- [Chapter 4: Solution Strategy](../docs/arc42/04-solution-strategy.md) - Key decisions, trade-offs
- [Chapter 5: Building Blocks](../docs/arc42/05-building-block-view.md) - Module structure, responsibilities

## Tech Stack

- **Language**: Rust 2021 edition (stable toolchain)
- **Async Runtime**: Tokio (for timing, event loops)
- **Platform APIs**: 
  - Windows: `windows-rs` (Win32 hooks, SendInput)
  - Linux: `x11rb` (X11 Record, XTest)
- **Build System**: Cargo workspace
- **Environment**: Nix flakes + direnv for reproducibility

## Development Workflow

### Setup

```bash
# Enter Nix development shell (auto with direnv)
nix develop

# Or manually allow direnv
direnv allow

# Verify setup
rustc --version
cargo --version
```

See [docs/setup/nix-environment.md](../docs/setup/nix-environment.md) for platform-specific instructions.

### Build & Run

```bash
# Build entire workspace
cargo build

# Run main application
cargo run -p handplusplus

# Build for release (optimized)
cargo build --release

# Run tests across workspace
cargo test --workspace

# Watch mode (auto-rebuild)
cargo watch -x run
```

### Architecture Pattern: Event-Driven Pipeline

```
Input Capture → Binding Engine → Action Executor
     ↓               ↓                  ↓
OS Hooks      State Machine      SendInput/XTest
```

**Flow**: OS events → matched against bindings → actions executed with timing

## Code Conventions

### Module Structure (Domain-Driven)

```
crates/
├── input-capture/     # Bounded Context: OS event capture
│   └── Trait: InputCapture
├── action-executor/   # Bounded Context: Input simulation  
│   └── Trait: ActionExecutor
├── binding-engine/    # Bounded Context: Event→Action mapping
│   └── BindingRegistry, EventProcessor
├── palette-ui/        # Bounded Context: Command palette
├── config/            # User bindings (configuration-as-code)
└── handplusplus/      # Main orchestration
```

**Dependency Rule**: Higher-level modules depend on lower-level; no circular deps.

### Platform Abstraction Pattern

**Always use traits for platform-specific code**:

```rust
// Define trait in crate root
pub trait InputCapture {
    fn register_hotkey(&mut self, hotkey: Hotkey) -> Result<()>;
    fn event_stream(&self) -> impl Stream<Item = InputEvent>;
}

// Platform-specific implementations in submodules
#[cfg(windows)]
mod windows_impl { /* ... */ }

#[cfg(target_os = "linux")]  
mod linux_impl { /* ... */ }

// Re-export platform implementation
pub use platform::PlatformCapture;
```

### Configuration-as-Code Pattern (v1)

User bindings are **Rust code** in `crates/config/src/`:

```rust
// crates/config/src/lib.rs
pub fn register_all_bindings() -> BindingRegistry {
    BindingRegistry::new()
        .bind(
            Hotkey::mouse(MouseButton::Button4),
            Action::while_held(
                Action::repeat(
                    Action::click(RightButton)
                        .then(Action::random_delay(30..80))
                )
            )
        )
}
```

**Rationale**: Type-safe, full Rust expressiveness, staged for future DSL.

### Naming Conventions

- **Traits**: `FooCapture`, `BarExecutor` (noun + verb)
- **Platform modules**: `windows_impl.rs`, `linux_impl.rs`
- **Types**: `PascalCase` (e.g., `InputEvent`, `Hotkey`)
- **Functions**: `snake_case` (e.g., `register_hotkey`)

## Key Integration Points

### OS Input APIs

**Windows** ([research](../docs/research/platform-input-apis.md)):
- Capture: `SetWindowsHookEx(WH_KEYBOARD_LL | WH_MOUSE_LL)`
- Simulate: `SendInput()`

**Linux X11**:
- Capture: `XRecordEnableContext()` (X11 Record extension)
- Simulate: `XTestFakeKeyEvent()`, `XTestFakeButtonEvent()`

**Platform Detection**:
```rust
#[cfg(windows)]
use input_capture::platform::WindowsCapture;

#[cfg(target_os = "linux")]
use input_capture::platform::X11Capture;
```

### Async Event Processing

- Use `tokio::spawn` for concurrent tasks
- `tokio::time::sleep` for delays (not `std::thread::sleep`)
- `tokio_stream::Stream` for event streams

## Important Files & Directories

- [Cargo.toml](../Cargo.toml) - Workspace definition, shared dependencies
- [flake.nix](../flake.nix) - Nix environment with Rust + platform libs
- [docs/arc42/](../docs/arc42/) - **Read first**: Architecture documentation
- [docs/research/platform-input-apis.md](../docs/research/platform-input-apis.md) - Platform API comparison
- [crates/config/src/lib.rs](../crates/config/src/lib.rs) - Example binding definitions
- [crates/input-capture/src/lib.rs](../crates/input-capture/src/lib.rs) - Core trait definitions

## Common Patterns

### Adding a New Binding

1. Edit `crates/config/src/lib.rs`
2. Use builder API: `Hotkey::combo()`, `Action::sequence()`
3. Recompile (configuration-as-code)

### Adding Platform Support

1. Implement `InputCapture` trait in new platform module
2. Implement `ActionExecutor` trait
3. Add conditional compilation: `#[cfg(target_os = "...")]`
4. Update Arc42 Chapter 3 (Context & Scope)
5. Document in `docs/research/platform-input-apis.md`

### Adding a New Action Type

1. Add variant to `action_executor::Action` enum
2. Implement execution logic in `Action::execute()`
3. Update `config` crate examples
4. Document in Arc42 Chapter 5 (Building Blocks)

## Testing Strategy

- **Unit tests**: In each crate's `src/` (inline or `tests/` subdir)
- **Integration tests**: `tests/` directory at workspace root
- **Platform-specific tests**: Use `#[cfg(target_os = "...")]`
- **Mock traits**: Create test doubles for `InputCapture`/`ActionExecutor`

## Notes for AI Agents

- **Architecture-first**: Reference Arc42 before suggesting implementations
- **Trait-based design**: Always abstract platform code behind traits
- **No premature optimization**: Prioritize clarity; benchmark if performance matters
- **ADRs for decisions**: Document "why" in `docs/adr/NNNN-title.md` (future)
- **Staged roadmap**: v0.1 = config-as-code, v0.2 = DSL, v0.3 = GUI (see Arc42 Chapter 4)
- **Learning project**: Explain architectural patterns in comments/docs
- **Platform differences**: Windows vs Linux APIs differ significantly; check research doc
- **Wayland limitations**: Document unsupported; recommend X11 for gaming
