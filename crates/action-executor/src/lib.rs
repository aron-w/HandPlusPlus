use anyhow::Result;
use std::time::Duration;
use std::pin::Pin;
use std::future::Future;

// Re-export types from input-capture for convenience
pub use input_capture::{Key, MouseButton};

/// Key or button state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Press,
    Release,
}

/// Platform abstraction for simulating input
pub trait ActionExecutor: Send + Sync {
    /// Simulate a key press or release
    fn simulate_key(&self, key: Key, state: InputState) -> Result<()>;

    /// Simulate a mouse button press or release
    fn simulate_mouse(&self, button: MouseButton, state: InputState) -> Result<()>;

    /// Move mouse cursor to absolute position
    fn mouse_move_abs(&self, x: i32, y: i32) -> Result<()>;

    /// Move mouse cursor by relative offset
    fn mouse_move_rel(&self, dx: i32, dy: i32) -> Result<()>;
}

/// High-level actions composed of executor primitives
#[derive(Debug, Clone)]
pub enum Action {
    /// Press and release a key
    PressKey(Key),

    /// Click a mouse button
    Click(MouseButton),

    /// Hold a key down
    HoldKey(Key),

    /// Release a held key
    ReleaseKey(Key),

    /// Sequence of actions executed in order
    Sequence(Vec<Action>),

    /// Repeat actions while condition is true
    RepeatWhileHeld {
        actions: Vec<Action>,
        interval: Duration,
    },

    /// Delay execution
    Delay(Duration),

    /// Random delay within range
    RandomDelay { min: Duration, max: Duration },

    /// Type a text string
    TypeText(String),
}

impl Action {
    /// Execute this action using the provided executor
    pub fn execute<'a>(
        &'a self,
        executor: &'a impl ActionExecutor,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            match self {
                Action::PressKey(key) => {
                    executor.simulate_key(*key, InputState::Press)?;
                    executor.simulate_key(*key, InputState::Release)?;
                }
                Action::Click(button) => {
                    executor.simulate_mouse(*button, InputState::Press)?;
                    executor.simulate_mouse(*button, InputState::Release)?;
                }
                Action::HoldKey(key) => {
                    executor.simulate_key(*key, InputState::Press)?;
                }
                Action::ReleaseKey(key) => {
                    executor.simulate_key(*key, InputState::Release)?;
                }
                Action::Sequence(actions) => {
                    for action in actions {
                        action.execute(executor).await?;
                    }
                }
                Action::Delay(duration) => {
                    tokio::time::sleep(*duration).await;
                }
                Action::RandomDelay { min, max } => {
                    use rand::Rng;
                    let delay = rand::thread_rng().gen_range(min.as_millis()..=max.as_millis());
                    tokio::time::sleep(Duration::from_millis(delay as u64)).await;
                }
                Action::RepeatWhileHeld { .. } => {
                    // This requires state tracking from binding-engine
                    todo!("RepeatWhileHeld requires integration with event loop")
                }
                Action::TypeText(_text) => {
                    // TODO: Map characters to key sequences
                    todo!("TypeText requires character-to-key mapping")
                }
            }
            Ok(())
        })
    }
}

// Platform-specific implementations
#[cfg(windows)]
pub mod platform {
    pub use super::windows_impl::WindowsExecutor as PlatformExecutor;
}

#[cfg(target_os = "linux")]
pub mod platform {
    pub use super::linux_impl::X11Executor as PlatformExecutor;
}

#[cfg(windows)]
mod windows_impl {
    use super::*;

    pub struct WindowsExecutor;

    impl ActionExecutor for WindowsExecutor {
        fn simulate_key(&self, _key: Key, _state: InputState) -> Result<()> {
            todo!("Implement using SendInput")
        }

        fn simulate_mouse(&self, _button: MouseButton, _state: InputState) -> Result<()> {
            todo!("Implement using SendInput")
        }

        fn mouse_move_abs(&self, _x: i32, _y: i32) -> Result<()> {
            todo!("Implement using SendInput")
        }

        fn mouse_move_rel(&self, _dx: i32, _dy: i32) -> Result<()> {
            todo!("Implement using SendInput")
        }
    }
}

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::*;

    pub struct X11Executor;

    impl ActionExecutor for X11Executor {
        fn simulate_key(&self, _key: Key, _state: InputState) -> Result<()> {
            todo!("Implement using XTest extension")
        }

        fn simulate_mouse(&self, _button: MouseButton, _state: InputState) -> Result<()> {
            todo!("Implement using XTest extension")
        }

        fn mouse_move_abs(&self, _x: i32, _y: i32) -> Result<()> {
            todo!("Implement using XTest extension")
        }

        fn mouse_move_rel(&self, _dx: i32, _dy: i32) -> Result<()> {
            todo!("Implement using XTest extension")
        }
    }
}
