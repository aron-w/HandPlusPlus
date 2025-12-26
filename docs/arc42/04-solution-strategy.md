# 4. Solution Strategy

## 4.1 Technology Decisions

| Decision | Rationale | Trade-offs |
|----------|-----------|------------|
| **Rust** | Memory safety, performance, cross-platform, strong ecosystem | Steeper learning curve; longer compile times |
| **Tokio Async Runtime** | Precise timing with `tokio::time`, standard in ecosystem | Slight overhead vs raw threads; acceptable for use case |
| **Trait-based Platform Abstraction** | Idiomatic Rust; compile-time dispatch; clear interfaces | More boilerplate than runtime polymorphism |
| **Configuration-as-Code (v1)** | Avoids parser complexity; type-safe; good for learning | Requires recompilation; acceptable for initial version |
| **windows-rs + x11rb** | Official/maintained bindings; safe abstractions | Platform-specific code sections unavoidable |

## 4.2 Top-Level Architecture Pattern

**Event-Driven Pipeline Architecture**

```
┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│   Capture    │─────▶│   Process    │─────▶│   Execute    │
│   (Events)   │      │   (Logic)    │      │   (Actions)  │
└──────────────┘      └──────────────┘      └──────────────┘
       │                     │                      │
       │                     │                      │
   OS Hooks            Binding Match          SendInput/XTest
   Event Stream        State Machine          Platform APIs
```

**Flow:**
1. **Capture**: OS-level hooks/listeners produce stream of `InputEvent`s
2. **Process**: Events matched against configured bindings; state machine tracks holds/sequences
3. **Execute**: Matched bindings trigger `Action`s; executor simulates input via platform APIs

## 4.3 Architecture Principles

### 1. Platform Abstraction via Traits

```rust
pub trait InputCapture {
    fn register_hotkey(&mut self, hotkey: Hotkey) -> Result<()>;
    fn event_stream(&self) -> impl Stream<Item = InputEvent>;
}

pub trait ActionExecutor {
    fn simulate_key(&self, key: Key, state: KeyState) -> Result<()>;
    fn simulate_mouse(&self, button: MouseButton, state: ButtonState) -> Result<()>;
    fn mouse_move(&self, x: i32, y: i32) -> Result<()>;
}
```

**Benefits:**
- Platform-specific implementations hidden behind trait
- Easy to mock for testing
- Clear contract between modules

### 2. Configuration-as-Code Pattern

```rust
// crates/config/src/gaming.rs
use handplusplus_api::prelude::*;

pub fn register_bindings() -> BindingRegistry {
    BindingRegistry::new()
        .bind(
            Hotkey::mouse(MouseButton::Button4),
            Action::sequence()
                .while_held(|| {
                    click(MouseButton::Right)
                        .with_delay(Duration::from_millis(random(30..80)))
                })
                .then(press_key(Key::Enter))
        )
        .bind(
            Hotkey::combo(&[Key::Ctrl, Key::Shift, Key::P]),
            Action::show_palette()
        )
}
```

**Rationale:**
- Type-safe at compile time
- Full Rust language for complex logic
- IDE support (autocomplete, refactoring)
- Staged for future DSL extraction

### 3. Domain-Driven Module Structure

```
crates/
├── input-capture/     # Bounded Context: OS event capture
│   ├── src/platform/windows.rs
│   ├── src/platform/linux.rs
│   └── src/traits.rs
├── action-executor/   # Bounded Context: Input simulation
│   ├── src/platform/windows.rs
│   ├── src/platform/linux.rs
│   └── src/actions.rs
├── binding-engine/    # Bounded Context: Event → Action mapping
│   ├── src/registry.rs
│   └── src/state_machine.rs
└── palette-ui/        # Bounded Context: Command palette
    └── src/lib.rs
```

## 4.4 Key Design Decisions

### ADR-001: Why Configuration-as-Code?

**Status:** Accepted

**Context:** Need to represent complex input sequences like "hold button → repeat action with random delay → press key on release"

**Decision:** Use Rust code as configuration language (compiled into binary) for v1

**Consequences:**
- ✅ Full expressiveness of Rust (loops, conditionals, randomization)
- ✅ Type safety and compile-time validation
- ✅ Learning opportunity for API design
- ❌ Requires recompilation for changes
- ❌ Not accessible to non-programmers
- 🔄 Future: Extract patterns into DSL once proven

### ADR-002: Tokio for Timing

**Status:** Accepted

**Context:** Need precise, randomized delays (30-80ms ranges) and async event handling

**Decision:** Use Tokio async runtime with `tokio::time::sleep` and `interval`

**Consequences:**
- ✅ Standard ecosystem choice; good documentation
- ✅ `Sleep` + jitter is more maintainable than raw threads
- ✅ Integrates with async event streams
- ❌ ~1ms timing overhead vs spin-loop
- ✅ Acceptable for gaming macros (30-80ms range)

### ADR-003: Single Backend per Platform

**Status:** Accepted

**Context:** Multiple options exist (rdev, enigo, direct APIs)

**Decision:** 
- Windows: `windows-rs` + `global-hotkey`
- Linux: `x11rb` + `rdev` or custom evdev

**Consequences:**
- ✅ Simpler maintenance
- ✅ Deep integration with chosen library
- ❌ No fallback if primary library fails
- 🔄 Can add fallback in future if needed

## 4.5 Quality Attribute Strategies

| Quality Goal | Strategy |
|--------------|----------|
| **Low Latency** | Minimize allocations in hot path; use channels for event passing; benchmark critical sections |
| **Reliability** | Extensive error handling; fallback for failed input simulation; health checks for hook registration |
| **Cross-Platform** | Trait abstraction + compile-time dispatch; platform-specific tests; CI on both Windows & Linux |
| **Maintainability** | Clear module boundaries; documented ADRs; examples in each crate; integration tests as documentation |

## 4.6 Roadmap for Evolution

```
v0.1 (Foundation)          v0.2 (Usability)         v0.3 (Advanced)
┌─────────────────────┐   ┌──────────────────┐    ┌─────────────────┐
│ • Config-as-Code    │──▶│ • TOML/YAML DSL  │───▶│ • GUI Config    │
│ • Windows + Linux   │   │ • Tray UI        │    │ • Macro Record  │
│ • Basic Actions     │   │ • Hot Reload     │    │ • Plugin System │
│ • Global Hotkeys    │   │ • More Actions   │    │ • Cloud Sync    │
└─────────────────────┘   └──────────────────┘    └─────────────────┘
       (Learning)            (Refinement)            (Production)
```
