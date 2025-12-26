# 2. Architecture Constraints

## 2.1 Technical Constraints

| Constraint | Background | Impact |
|------------|------------|--------|
| **Rust Language** | Type safety, memory safety, cross-platform | Forces clear ownership, limits runtime reflection, enables zero-cost abstractions |
| **Windows 11 Support** | Primary development platform | Must use Win32 APIs: `SendInput`, `SetWindowsHookEx`, `RegisterHotKey` |
| **NixOS Support** | Secondary target platform | X11 XTest extension or evdev for input; Wayland has limitations |
| **System-Level Access** | Global hotkeys require OS-level hooks | May need elevated permissions; security implications |
| **No Runtime Scripting (v1)** | Configuration = Code initially | Requires recompilation for changes; staged for future DSL |

## 2.2 Organizational Constraints

| Constraint | Background |
|------------|------------|
| **Solo Developer** | Single developer learning project; prioritize clarity over optimization |
| **Arc42-First Workflow** | No implementation before architectural decisions documented |
| **Open Source** | Public repository; clean commit history; documented ADRs |

## 2.3 Conventions

### Development Environment
- **Nix Flakes**: Reproducible development environments across platforms
- **direnv**: Automatic environment activation
- **Rust Stable**: Latest stable toolchain; no nightly features unless justified

### Code Style
- **Rust 2021 Edition**
- **rustfmt** with default configuration
- **clippy** warnings enforced in CI
- Module structure mirrors architectural building blocks

### Documentation
- ADRs (Architecture Decision Records) in `docs/adr/`
- Arc42 living documentation updated with implementation
- Inline code documentation for public APIs

## 2.4 Platform-Specific Constraints

### Windows 11
- **Low-Level Hooks**: `WH_KEYBOARD_LL`, `WH_MOUSE_LL` for global hotkeys
- **UIPI (User Interface Privilege Isolation)**: May block input to elevated processes
- **Antivirus**: Input simulation may trigger false positives
- **DPI Awareness**: Mouse coordinates must account for scaling

### NixOS / Linux
- **X11 vs Wayland**: X11 has XTest; Wayland requires compositor cooperation or evdev
- **evdev**: Requires `/dev/input` access (root or input group)
- **Permissions**: udev rules or user in `input` group
- **Display Server Detection**: Runtime check for X11/Wayland

## 2.5 Trade-offs Accepted

| Decision | Trade-off |
|----------|-----------|
| **Configuration-as-Code** | Flexibility ↔ Simplicity: Accept recompilation for v1 to avoid DSL complexity |
| **No GUI Config (v1)** | User Experience ↔ Scope: Focus on architecture patterns before UI |
| **Single Input Backend per Platform** | Flexibility ↔ Maintenance: One proven library per OS instead of multiple fallbacks |
| **Tokio Async Runtime** | Simplicity ↔ Precision: Accept minor timing overhead for ecosystem benefits |
