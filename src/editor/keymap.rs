use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub struct Keymap {
    map: HashMap<KeyEvent, &'static str>,
}

impl Keymap {
    /// Returns the command name for the given key event, or `None` if no command is bound to the
    /// given event.
    pub fn get(&self, event: &KeyEvent) -> Option<&'static str> {
        self.map.get(event).copied()
    }
}

#[rustfmt::skip]
impl Default for Keymap {
    fn default() -> Self {
        // TODO: Map to action enums instead of command names.
        let mut map = HashMap::new();
        // Editor actions.
        map.insert(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL), "quit");
        map.insert(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL), "save");
        map.insert(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL), "enter_command_mode");
        map.insert(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL), "open_search");
        // Cursor movements.
        map.insert(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE), "move_cursor_left");
        map.insert(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE), "move_cursor_right");
        map.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), "move_cursor_up");
        map.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), "move_cursor_down");
        map.insert(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE), "move_cursor_to_start_of_row");
        map.insert(KeyEvent::new(KeyCode::End, KeyModifiers::NONE), "move_cursor_to_end_of_row");
        // Text manipulation.
        map.insert(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), "insert_newline");
        map.insert(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE), "delete_char");
        map.insert(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE), "delete_char_before");
        Self { map }
    }
}
