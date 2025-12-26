use std::collections::HashMap;
use input_capture::{Hotkey, InputEvent};
use action_executor::Action;
use anyhow::Result;

/// Registry mapping hotkeys to actions
pub struct BindingRegistry {
    bindings: HashMap<Hotkey, Action>,
}

impl BindingRegistry {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Add a hotkey → action binding
    pub fn bind(mut self, hotkey: Hotkey, action: Action) -> Self {
        self.bindings.insert(hotkey, action);
        self
    }

    /// Get action for a hotkey (if registered)
    pub fn get_action(&self, hotkey: &Hotkey) -> Option<&Action> {
        self.bindings.get(hotkey)
    }

    /// Check if a hotkey is registered
    pub fn is_registered(&self, hotkey: &Hotkey) -> bool {
        self.bindings.contains_key(hotkey)
    }

    /// Number of registered bindings
    pub fn len(&self) -> usize {
        self.bindings.len()
    }
}

/// State tracker for complex input patterns
pub struct StateTracker {
    // Track which keys/buttons are currently held
    held_keys: Vec<input_capture::Key>,
    held_buttons: Vec<input_capture::MouseButton>,
}

impl StateTracker {
    pub fn new() -> Self {
        Self {
            held_keys: Vec::new(),
            held_buttons: Vec::new(),
        }
    }

    /// Update state based on incoming event
    pub fn update(&mut self, event: &InputEvent) {
        match event {
            InputEvent::KeyPress(key) => {
                if !self.held_keys.contains(key) {
                    self.held_keys.push(*key);
                }
            }
            InputEvent::KeyRelease(key) => {
                self.held_keys.retain(|k| k != key);
            }
            InputEvent::MousePress(button) => {
                if !self.held_buttons.contains(button) {
                    self.held_buttons.push(*button);
                }
            }
            InputEvent::MouseRelease(button) => {
                self.held_buttons.retain(|b| b != button);
            }
            _ => {}
        }
    }

    /// Check if a key is currently held
    pub fn is_key_held(&self, key: &input_capture::Key) -> bool {
        self.held_keys.contains(key)
    }

    /// Check if a button is currently held
    pub fn is_button_held(&self, button: &input_capture::MouseButton) -> bool {
        self.held_buttons.contains(button)
    }
}

/// Event processor matches events to bindings
pub struct EventProcessor {
    registry: BindingRegistry,
    state: StateTracker,
}

impl EventProcessor {
    pub fn new(registry: BindingRegistry) -> Self {
        Self {
            registry,
            state: StateTracker::new(),
        }
    }

    /// Process an input event and return matching action (if any)
    pub fn process_event(&mut self, event: InputEvent) -> Option<Action> {
        // Update state tracker
        self.state.update(&event);

        // TODO: Match event against registered hotkeys
        // For now, stub implementation
        None
    }
}
