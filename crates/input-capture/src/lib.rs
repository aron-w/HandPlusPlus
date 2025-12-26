use tokio_stream::Stream;
use anyhow::Result;

/// Platform-independent input event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    KeyPress(Key),
    KeyRelease(Key),
    MousePress(MouseButton),
    MouseRelease(MouseButton),
    MouseMove { x: i32, y: i32 },
}

/// Keyboard keys (subset for demonstration)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    
    // Modifiers
    Ctrl, Shift, Alt, Meta, // Meta = Win/Super
    
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Special
    Enter, Escape, Space, Tab, Backspace,
}

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4, // Side button (back)
    Button5, // Side button (forward)
}

/// Modifiers for hotkey combinations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Meta,
}

/// Hotkey definition (trigger + optional modifiers)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hotkey {
    pub modifiers: Vec<Modifier>,
    pub trigger: Trigger,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Trigger {
    Key(Key),
    MouseButton(MouseButton),
}

impl Hotkey {
    pub fn key(key: Key) -> Self {
        Self {
            modifiers: Vec::new(),
            trigger: Trigger::Key(key),
        }
    }

    pub fn mouse(button: MouseButton) -> Self {
        Self {
            modifiers: Vec::new(),
            trigger: Trigger::MouseButton(button),
        }
    }

    pub fn combo(modifiers: &[Modifier], trigger: Trigger) -> Self {
        Self {
            modifiers: modifiers.to_vec(),
            trigger,
        }
    }
}

/// Platform abstraction for global input capture
pub trait InputCapture: Send + Sync {
    /// Register a global hotkey
    fn register_hotkey(&mut self, hotkey: Hotkey) -> Result<()>;

    /// Stream of input events
    fn event_stream(&self) -> Box<dyn Stream<Item = InputEvent> + Send + Unpin>;

    /// Stop capturing input
    fn stop(&mut self) -> Result<()>;
}

// Platform-specific implementations
#[cfg(windows)]
pub mod platform {
    pub use super::windows_impl::WindowsCapture as PlatformCapture;
}

#[cfg(target_os = "linux")]
pub mod platform {
    pub use super::linux_impl::X11Capture as PlatformCapture;
}

// Stub implementations (to be completed)
#[cfg(windows)]
mod windows_impl {
    use super::*;

    pub struct WindowsCapture;

    impl InputCapture for WindowsCapture {
        fn register_hotkey(&mut self, _hotkey: Hotkey) -> Result<()> {
            todo!("Implement using SetWindowsHookEx")
        }

        fn event_stream(&self) -> Box<dyn Stream<Item = InputEvent> + Send + Unpin> {
            todo!("Implement event stream from Windows hooks")
        }

        fn stop(&mut self) -> Result<()> {
            todo!("Unhook Windows hooks")
        }
    }
}

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::*;

    pub struct X11Capture;

    impl InputCapture for X11Capture {
        fn register_hotkey(&mut self, _hotkey: Hotkey) -> Result<()> {
            todo!("Implement using X11 XInput or XRecord")
        }

        fn event_stream(&self) -> Box<dyn Stream<Item = InputEvent> + Send + Unpin> {
            todo!("Implement event stream from X11")
        }

        fn stop(&mut self) -> Result<()> {
            todo!("Close X11 connection")
        }
    }
}
