// Configuration module: Define bindings here
// This is the "configuration-as-code" pattern

use binding_engine::BindingRegistry;
use input_capture::{Hotkey, Key, MouseButton, Modifier, Trigger};
use action_executor::Action;
use std::time::Duration;

/// Register all user-defined bindings
pub fn register_all_bindings() -> BindingRegistry {
    BindingRegistry::new()
        // Example: F1 → Press Enter
        .bind(
            Hotkey::key(Key::F1),
            Action::PressKey(Key::Enter),
        )
        // Example: Mouse4 → Right click
        .bind(
            Hotkey::mouse(MouseButton::Button4),
            Action::Click(MouseButton::Right),
        )
        // Example: Ctrl+Shift+P → Show palette (future)
        .bind(
            Hotkey::combo(
                &[Modifier::Ctrl, Modifier::Shift],
                Trigger::Key(Key::P),
            ),
            Action::Sequence(vec![
                // Placeholder for palette action
                Action::Delay(Duration::from_millis(100)),
            ]),
        )
}

// Example: Gaming-specific bindings (separate module)
pub mod gaming {
    use super::*;

    pub fn rapid_click_binding() -> (Hotkey, Action) {
        // Mouse4 hold → Rapid right-click every 50ms
        let hotkey = Hotkey::mouse(MouseButton::Button4);
        let action = Action::RepeatWhileHeld {
            actions: vec![
                Action::Click(MouseButton::Right),
                Action::Delay(Duration::from_millis(50)),
            ],
            interval: Duration::from_millis(50),
        };
        (hotkey, action)
    }

    // TODO: Add more gaming patterns
}
