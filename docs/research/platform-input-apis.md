# Platform Input API Research

This document compares cross-platform input capture and simulation libraries for Rust.

## Requirements

1. **Global Hotkey Capture**: Register system-wide hotkeys that work even when app is not focused
2. **Input Simulation**: Send keyboard/mouse events programmatically
3. **Low Latency**: <5ms response time for gaming
4. **Cross-Platform**: Windows 11 and NixOS (X11) minimum
5. **Safe Abstractions**: Avoid raw FFI where possible

---

## Windows Platform

### Win32 API (Direct)

**Library**: `windows-rs` (official Microsoft bindings)

**Capture**:
- `SetWindowsHookEx(WH_KEYBOARD_LL)` - Low-level keyboard hook
- `SetWindowsHookEx(WH_MOUSE_LL)` - Low-level mouse hook
- `RegisterHotKey()` - System-wide hotkey registration

**Simulation**:
- `SendInput()` - Inject input events into system

**Pros**:
- ✅ Official, well-maintained bindings
- ✅ Maximum control and performance
- ✅ Comprehensive Windows API access

**Cons**:
- ❌ Windows-only code
- ❌ Requires understanding of Win32 details
- ❌ Verbose API

**Decision**: Use for Windows implementation. Wrap in trait for abstraction.

---

### `global-hotkey` crate

**Repository**: https://github.com/tauri-apps/global-hotkey

**Features**:
- Cross-platform hotkey registration (Windows, macOS, Linux)
- Event-based API
- Part of Tauri ecosystem

**Pros**:
- ✅ Cross-platform
- ✅ Simple, modern API
- ✅ Well-maintained (Tauri project)

**Cons**:
- ❌ Hotkey registration only, no input simulation
- ❌ May be too high-level for complex patterns

**Decision**: Candidate for capture layer; evaluate vs direct Win32 hooks.

---

### `enigo` crate

**Repository**: https://github.com/enigo-rs/enigo

**Features**:
- Cross-platform input simulation
- Keyboard and mouse control

**Pros**:
- ✅ Cross-platform (Windows, macOS, Linux)
- ✅ Simple API

**Cons**:
- ❌ C++ dependency (libxdo on Linux)
- ❌ Less maintained recently
- ❌ No capture functionality

**Decision**: Avoid due to C++ dependency; prefer pure Rust.

---

## Linux Platform (X11)

### X11 Record Extension

**Library**: `x11rb` (safe X11 bindings)

**Capture**:
- `XRecordEnableContext()` - Capture all input events system-wide
- Requires XRECORD extension

**Simulation**:
- `XTest` extension: `XTestFakeKeyEvent()`, `XTestFakeButtonEvent()`

**Pros**:
- ✅ Pure Rust bindings
- ✅ Safe API (no raw pointers)
- ✅ Comprehensive X11 support

**Cons**:
- ❌ X11-only (not Wayland)
- ❌ Verbose compared to higher-level libraries

**Decision**: Use for X11 implementation.

---

### `rdev` crate

**Repository**: https://github.com/Narsil/rdev

**Features**:
- Cross-platform input capture (keyboard, mouse)
- Event listener API
- Windows, macOS, Linux (X11)

**Pros**:
- ✅ Cross-platform
- ✅ Simple event stream API
- ✅ Active development

**Cons**:
- ❌ Capture only, no simulation
- ❌ Uses XRECORD on Linux (may have permission issues)

**Decision**: Strong candidate for capture layer across platforms.

---

### `evdev` crate

**Repository**: https://github.com/emberian/evdev

**Features**:
- Direct `/dev/input` device access (Linux kernel interface)
- Low-level control

**Pros**:
- ✅ Wayland compatible (bypasses display server)
- ✅ Lowest latency possible
- ✅ Pure Rust

**Cons**:
- ❌ Requires elevated permissions or `input` group membership
- ❌ Linux-only
- ❌ Complex device management

**Decision**: Linux fallback for Wayland; requires documentation of permissions.

---

## Recommended Architecture

### Capture Layer

```
Platform Detection
       │
       ├──▶ Windows: windows-rs (SetWindowsHookEx)
       │
       ├──▶ Linux + X11: x11rb (XRecord) OR rdev
       │
       └──▶ Linux + Wayland: evdev (with permission warnings)
```

**Decision**: 
- **Windows**: Direct `windows-rs` hooks
- **Linux X11**: `x11rb` for full control, OR `rdev` for simplicity
- **Wayland**: Document limitation; suggest X11 for gaming

### Simulation Layer

```
Platform Detection
       │
       ├──▶ Windows: windows-rs (SendInput)
       │
       ├──▶ Linux + X11: x11rb (XTest)
       │
       └──▶ Linux + Wayland: evdev write events
```

---

## Alternatives Considered

### `inputbot`
- Unmaintained (last update 2020)
- ❌ Rejected

### `autopilot-rs`
- Higher-level automation (screen reading, etc.)
- ❌ Too high-level; overkill for input simulation

### `device_query`
- Simple polling-based input state checking
- ❌ No hotkey registration; polling not suitable

---

## Implementation Plan

### Phase 1: Windows Only
1. Use `windows-rs` for both capture and simulation
2. Implement `InputCapture` and `ActionExecutor` traits
3. Test with simple hotkeys

### Phase 2: Linux X11
1. Use `x11rb` for capture (XRecord) and simulation (XTest)
2. Platform-specific implementations behind traits
3. Test on NixOS

### Phase 3: Wayland (Stretch)
1. Document limitations
2. Provide `evdev` fallback with permission instructions
3. Warn about security model conflicts

---

## Benchmark Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Hotkey Response Time | <5ms | Event capture → action trigger |
| Input Simulation Latency | <1ms | Action request → OS event |
| Event Processing Throughput | >1000 events/sec | Sustained load handling |

---

## Security Considerations

### Windows
- Low-level hooks may trigger antivirus false positives
- Document need for exception rules
- UIPI may block input to elevated processes

### Linux
- XRecord requires X server access (DISPLAY env var)
- evdev requires `/dev/input` permissions
- Document udev rules or group membership

---

## References

- Windows Input: https://learn.microsoft.com/en-us/windows/win32/inputdev/keyboard-input
- X11 Record: https://www.x.org/releases/X11R7.6/doc/libXtst/recordlib.html
- X11 Test: https://www.x.org/releases/X11R7.6/doc/libXtst/xtestlib.html
- evdev protocol: https://www.kernel.org/doc/html/latest/input/input.html
