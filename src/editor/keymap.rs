use std::{collections::HashMap, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::command::*;

/// Macro to bind keys to commands or actions.
macro_rules! bind_keys {
    ( $map:ident, $( $keycode:expr, $modifiers:expr => $command:expr ),* $(,)? ) => {
        $(
            $map.insert(
                KeyEvent::new($keycode, $modifiers),
                Rc::new(Box::new($command) as Box<dyn Command>),
            );
        )*
    };
}

pub struct Keymap {
    map: HashMap<KeyEvent, Rc<Box<dyn Command + 'static>>>,
}

impl Keymap {
    /// Returns the command name for the given key event, or `None` if no command is bound to the
    /// given event.
    pub fn get(&self, event: &KeyEvent) -> Option<&Rc<Box<dyn Command + 'static>>> {
        self.map.get(event)
    }
}

#[rustfmt::skip]
impl Default for Keymap {
    fn default() -> Self {
        let mut map = HashMap::new();

        // TODO: Implement default values for key actions.
        bind_keys!(map,
            // Editor actions.
            KeyCode::Char('q'), KeyModifiers::CONTROL => Quit {},
            KeyCode::Char('s'), KeyModifiers::CONTROL => Save { path: None },
            KeyCode::Char('p'), KeyModifiers::CONTROL => EnterCommandMode {},
            KeyCode::Char('s'), KeyModifiers::CONTROL => OpenSearch {},
            KeyCode::Char('f'), KeyModifiers::CONTROL => OpenFilesPicker { dir: None },
            // Cursor movements.
            KeyCode::Left, KeyModifiers::NONE => MoveCursorLeft {},
            KeyCode::Right, KeyModifiers::NONE => MoveCursorRight {},
            KeyCode::Up, KeyModifiers::NONE => MoveCursorUp {},
            KeyCode::Down, KeyModifiers::NONE => MoveCursorDown {},
            KeyCode::Home, KeyModifiers::NONE => MoveCursorToStartOfRow {},
            KeyCode::End, KeyModifiers::NONE => MoveCursorToEndOfRow {},
            KeyCode::Char('b'), KeyModifiers::CONTROL => MoveCursorToStartOfBuffer {},
            KeyCode::Char('e'), KeyModifiers::CONTROL => MoveCursorToEndOfBuffer {},
            // Text manipulation.
            KeyCode::Enter, KeyModifiers::NONE => InsertNewline {},
            KeyCode::Delete, KeyModifiers::NONE => DeleteChar {},
            KeyCode::Backspace, KeyModifiers::NONE => DeleteCharBefore {},
        );

        Self { map }
    }
}
