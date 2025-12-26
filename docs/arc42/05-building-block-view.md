# 5. Building Block View

## 5.1 Level 1: System Overview (Whitebox)

```
┌────────────────────────────────────────────────────────────────┐
│                        HandPlusPlus                             │
│                                                                 │
│  ┌──────────────────┐         ┌─────────────────────────────┐ │
│  │  Main App        │────────▶│   Binding Registry          │ │
│  │  (Entry Point)   │         │   (Config Loader)           │ │
│  └────────┬─────────┘         └─────────────────────────────┘ │
│           │                                                     │
│           │ Coordinates                                         │
│           │                                                     │
│  ┌────────▼─────────┐         ┌─────────────────────────────┐ │
│  │  Event Loop      │────────▶│   Binding Engine            │ │
│  │  (Tokio Runtime) │         │   (State Machine)           │ │
│  └────────┬─────────┘         └─────────┬───────────────────┘ │
│           │                             │                      │
│           │                             │ Triggers             │
│           │                             │                      │
│  ┌────────▼─────────┐         ┌─────────▼───────────────────┐ │
│  │  Input Capture   │         │   Action Executor           │ │
│  │  (Platform Abstr)│         │   (Platform Abstr)          │ │
│  └──────────────────┘         └─────────────────────────────┘ │
│           ▲                             │                      │
│           │                             ▼                      │
│  ┌────────┴─────────┐         ┌─────────────────────────────┐ │
│  │  Palette UI      │         │   Timing Engine             │ │
│  │  (Command Input) │         │   (Delays/Randomization)    │ │
│  └──────────────────┘         └─────────────────────────────┘ │
└────────────────────────────────────────────────────────────────┘
```

### Building Blocks

| Component | Responsibility | Interface |
|-----------|----------------|-----------|
| **Main App** | Application lifecycle, module initialization | Entry point `fn main()` |
| **Binding Registry** | Loads configuration, stores hotkey→action mappings | `register_bindings()` |
| **Event Loop** | Tokio runtime, coordinates async tasks | Tokio `#[tokio::main]` |
| **Input Capture** | Platform abstraction for global hotkey capture | `trait InputCapture` |
| **Binding Engine** | Matches events to bindings, manages state (holds, sequences) | `process_event(InputEvent)` |
| **Action Executor** | Platform abstraction for input simulation | `trait ActionExecutor` |
| **Timing Engine** | Delays, intervals, randomization | `random_delay(range)` |
| **Palette UI** | Command palette overlay (Ctrl+P) | `show_palette()` |

## 5.2 Level 2: Input Capture (Whitebox)

```
┌──────────────────────────────────────────────────────────┐
│                    input-capture                          │
│                                                           │
│  ┌─────────────────────────────────────────────────────┐ │
│  │              InputCapture Trait                      │ │
│  │  + register_hotkey(Hotkey) -> Result<()>           │ │
│  │  + event_stream() -> impl Stream<Item=InputEvent>  │ │
│  └─────────────────────────────────────────────────────┘ │
│                       ▲           ▲                       │
│                       │           │                       │
│          ┌────────────┴───┐   ┌──┴──────────────┐        │
│          │                │   │                 │        │
│  ┌───────▼─────────┐  ┌───▼────────────┐  ┌────▼──────┐ │
│  │ WindowsCapture  │  │  X11Capture    │  │EvdevCapt. │ │
│  │ (SetHookEx)     │  │  (XRecord)     │  │(/dev/input)│ │
│  └─────────────────┘  └────────────────┘  └───────────┘ │
└──────────────────────────────────────────────────────────┘
```

### Responsibilities

- **InputCapture Trait**: Defines platform-independent interface
- **WindowsCapture**: Windows implementation using `SetWindowsHookEx` low-level hooks
- **X11Capture**: Linux X11 implementation using XRecord extension or XInput
- **EvdevCapture**: Linux fallback using direct `/dev/input` access

### Key Abstractions

```rust
pub enum InputEvent {
    KeyPress(Key),
    KeyRelease(Key),
    MousePress(MouseButton),
    MouseRelease(MouseButton),
    MouseMove { x: i32, y: i32 },
}

pub struct Hotkey {
    modifiers: Vec<Modifier>, // Ctrl, Shift, Alt, Win/Super
    trigger: Trigger,         // Key, MouseButton
}
```

## 5.3 Level 2: Action Executor (Whitebox)

```
┌──────────────────────────────────────────────────────────┐
│                  action-executor                          │
│                                                           │
│  ┌─────────────────────────────────────────────────────┐ │
│  │            ActionExecutor Trait                      │ │
│  │  + simulate_key(Key, KeyState) -> Result<()>       │ │
│  │  + simulate_mouse(MouseButton, State) -> Result<()>│ │
│  │  + mouse_move(x, y) -> Result<()>                  │ │
│  └─────────────────────────────────────────────────────┘ │
│                       ▲           ▲                       │
│                       │           │                       │
│          ┌────────────┴───┐   ┌──┴──────────────┐        │
│          │                │   │                 │        │
│  ┌───────▼─────────┐  ┌───▼────────────┐  ┌────▼──────┐ │
│  │ WindowsExecutor │  │  X11Executor   │  │EvdevExec. │ │
│  │ (SendInput)     │  │  (XTest)       │  │(write evt)│ │
│  └─────────────────┘  └────────────────┘  └───────────┘ │
│                                                           │
│  ┌─────────────────────────────────────────────────────┐ │
│  │                  Action Types                        │ │
│  │  • PressKey / ReleaseKey                            │ │
│  │  • Click / Hold / Release (Mouse)                   │ │
│  │  • Sequence (ordered actions)                       │ │
│  │  • Repeat (loop with condition)                     │ │
│  │  • Delay (fixed or random)                          │ │
│  └─────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────┘
```

### Key Abstractions

```rust
pub enum Action {
    PressKey(Key),
    ReleaseKey(Key),
    Click(MouseButton),
    Sequence(Vec<Action>),
    RepeatWhileHeld { actions: Vec<Action>, interval: Duration },
    Delay(Duration),
    RandomDelay { min: Duration, max: Duration },
}

impl Action {
    pub async fn execute(&self, executor: &impl ActionExecutor) -> Result<()>;
}
```

## 5.4 Level 2: Binding Engine (Whitebox)

```
┌──────────────────────────────────────────────────────────┐
│                   binding-engine                          │
│                                                           │
│  ┌────────────────────────────────────────────────────┐  │
│  │            Binding Registry                         │  │
│  │  • Map<Hotkey, Action>                             │  │
│  │  • add_binding(Hotkey, Action)                     │  │
│  │  • get_action(Hotkey) -> Option<Action>           │  │
│  └────────────────────────────────────────────────────┘  │
│                           │                               │
│                           ▼                               │
│  ┌────────────────────────────────────────────────────┐  │
│  │              Event Processor                        │  │
│  │  • process_event(InputEvent) -> Option<Action>    │  │
│  │  • State machine for holds/sequences               │  │
│  └────────────────────────────────────────────────────┘  │
│                           │                               │
│                           ▼                               │
│  ┌────────────────────────────────────────────────────┐  │
│  │              State Tracker                          │  │
│  │  • Active holds (which keys/buttons held)          │  │
│  │  • Sequence progress (multi-key combos)            │  │
│  │  • Cooldowns / Rate limiting                       │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

### Responsibilities

- **Binding Registry**: Storage for hotkey→action mappings; loaded from configuration
- **Event Processor**: Matches incoming `InputEvent`s against registered hotkeys; handles combo keys
- **State Tracker**: Maintains state for complex patterns (holds, sequences, cooldowns)

### Example State Machine

```
Mouse4 Press    While Held         Mouse4 Release
    │              Loop                  │
    │               │                    │
    ▼               ▼                    ▼
  [IDLE] ────▶ [HOLDING] ────▶ [SEQUENCE_END] ────▶ [IDLE]
                   │
                   │ Every 30-80ms
                   ▼
              Right Click
```

## 5.5 Level 2: Configuration Module

```
┌──────────────────────────────────────────────────────────┐
│                       config                              │
│                                                           │
│  ┌────────────────────────────────────────────────────┐  │
│  │           User Configuration Files                  │  │
│  │  • gaming.rs  (game-specific bindings)             │  │
│  │  • global.rs  (system-wide shortcuts)              │  │
│  │  • lib.rs     (exports register_all_bindings)      │  │
│  └────────────────────────────────────────────────────┘  │
│                           │                               │
│                           │ Uses                          │
│                           ▼                               │
│  ┌────────────────────────────────────────────────────┐  │
│  │        Builder API (Fluent Interface)              │  │
│  │  • Hotkey::combo([Ctrl, P])                        │  │
│  │  • Action::sequence()                              │  │
│  │       .while_held(|| ...)                          │  │
│  │       .with_delay(...)                             │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

### Example Configuration

```rust
// crates/config/src/gaming.rs
use handplusplus_api::prelude::*;

pub fn gaming_bindings() -> Vec<Binding> {
    vec![
        // Complex sequence: Hold Mouse4 → rapid right-clicks → Enter on release
        Binding::new(Hotkey::mouse(Mouse4))
            .action(
                Action::while_held(
                    Action::repeat(
                        Action::click(RightButton)
                            .then(Action::random_delay(30..80))
                    )
                )
                .then_on_release(Action::press(Enter))
            ),
        
        // Simple: F1 → Type text
        Binding::new(Hotkey::key(F1))
            .action(Action::type_text("gg ez")),
    ]
}
```

## 5.6 Module Dependencies

```
┌─────────────────────────────────────────────────────────────┐
│                      handplusplus (main)                     │
└────┬──────────────────────────────────┬───────────────┬──────┘
     │                                  │               │
     ▼                                  ▼               ▼
┌────────────┐      ┌──────────────────────────┐  ┌──────────┐
│   config   │      │   binding-engine         │  │palette-ui│
└────┬───────┘      └────┬────────────────┬────┘  └──────────┘
     │                   │                │
     │                   ▼                ▼
     │          ┌────────────────┐  ┌────────────────┐
     └─────────▶│ input-capture  │  │action-executor │
                └────────────────┘  └────────────────┘
```

**Dependency Rules:**
- Config depends on action-executor (for Action types)
- Binding-engine depends on input-capture and action-executor
- Main orchestrates all modules
- No circular dependencies
