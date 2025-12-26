# 3. System Context and Scope

## 3.1 Business Context

```
┌─────────────────────────────────────────────────────────────┐
│                      Gaming Application                      │
│                   (Target of Automation)                     │
└────────────────────────┬────────────────────────────────────┘
                         │
                         │ Simulated Input
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                     Operating System                         │
│            (Windows 11 / NixOS with X11/Wayland)            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Input Subsystem (Keyboard/Mouse Event Processing)   │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────┬──────────────────────────────┬─────────────────┘
             │                              │
             │ Hotkey Events                │ API Calls
             │                              │ (SendInput/XTest)
             ▼                              │
┌─────────────────────────────────────────────────────────────┐
│                      HandPlusPlus                            │
│  ┌──────────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Input Capture    │  │   Command    │  │   Action     │  │
│  │  (Global HK)     │  │   Palette    │  │  Executor    │  │
│  └──────────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────────┘
             ▲
             │ Configuration
             │
┌─────────────────────────────────────────────────────────────┐
│                    Developer / User                          │
│              (Defines Bindings via Code)                     │
└─────────────────────────────────────────────────────────────┘
```

### External Entities

| Entity | Description | Interface |
|--------|-------------|-----------|
| **Operating System** | Provides input event APIs and delivers keyboard/mouse events | Platform APIs (Win32, X11, evdev) |
| **Target Applications** | Receive simulated input from HandPlusPlus | Standard OS input events |
| **Developer/User** | Configures bindings by writing Rust code in config module | Rust API (Builder pattern) |
| **System Tray** | Provides UI for starting/stopping, status indication | Native system tray integration |

## 3.2 Technical Context

### Windows 11 Interface

```rust
// Conceptual interface (actual implementation abstracted)

// Input Capture (Global Hotkeys)
Windows::Win32::UI::WindowsAndMessaging::SetWindowsHookEx(
    WH_KEYBOARD_LL | WH_MOUSE_LL
) -> Event Stream

// Input Simulation
Windows::Win32::UI::Input::KeyboardAndMouse::SendInput(
    &[INPUT { ... }]
) -> Result
```

**Libraries Considered:**
- `windows-rs`: Official Microsoft bindings
- `global-hotkey`: Cross-platform hotkey registration
- `enigo`: Cross-platform input simulation (C++ dependency concern)

### Linux (X11) Interface

```rust
// X11 Input Capture
x11rb::xinput::XISelectEvents(...) -> Event Stream

// X11 Input Simulation  
x11rb::xtest::FakeInput(
    KeyPress | ButtonPress, keycode, ...
) -> Result
```

**Libraries Considered:**
- `x11rb`: Safe X11 bindings
- `rdev`: Cross-platform event capture (uses X11 Record extension)
- `evdev`: Direct input device access (requires elevated permissions)

### Linux (Wayland) Considerations

⚠️ **Limited Support**: Wayland security model prevents global input capture by design. Options:
- Compositor-specific protocols (experimental)
- Fall back to evdev (requires root/input group)
- Document limitation; recommend X11 for gaming use case

## 3.3 System Boundary

### In Scope
- Global hotkey registration and event capture
- Complex input sequence execution with timing/randomization
- Command palette UI for in-app control
- Configuration through Rust code (compiled bindings)
- Platform abstraction layer for Windows/Linux

### Out of Scope (v1)
- Runtime scripting language / DSL
- GUI configuration builder
- Input recording / macro recorder
- Cloud sync of configurations
- macOS support
- Mobile platforms
- Network-based remote control
