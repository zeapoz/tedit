use crate::editor::pane::cursor::CursorMovement;
use std::{collections::HashMap, fmt::Debug, rc::Rc};

use define_commands_macro::define_commands;
use thiserror::Error;

use crate::editor::{
    self, Editor,
    prompt::{PromptResponse, PromptType, confirm::ConfirmPrompt, search::SearchPrompt},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing argument: {0}")]
    MissingArgument(String),
    #[error("Invalid argument for {name}: {error}")]
    InvalidArgument { name: String, error: String },
    #[allow(clippy::enum_variant_names)]
    #[error(transparent)]
    ExecutionError(#[from] editor::Error),
}

/// The `CommandSpec` trait defines the specification of a command.
pub trait CommandSpec {
    /// Returns the name of the command.
    fn name(&self) -> &'static str;

    /// Returns a description of the command.
    fn description(&self) -> &'static str;

    /// Parses a string of arguments into a runnable command.
    fn parse(&self, raw_args: &str) -> Result<Box<dyn Command>, Error>;
}

/// A command that encompasses a runnable command and its arguments.
pub trait Command {
    /// Executes the command.
    fn execute(&self, editor: &mut Editor) -> Result<(), Error>;
}

/// A registry for all available commands.
#[derive(Default)]
pub struct CommandRegistry {
    commands: HashMap<String, Rc<dyn CommandSpec>>,
}

impl CommandRegistry {
    /// Creates a new command registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new command.
    pub fn register(&mut self, command: Rc<dyn CommandSpec>) {
        self.commands.insert(command.name().to_lowercase(), command);
    }

    /// Gets a command by name.
    pub fn get(&self, name: &str) -> Option<Rc<dyn CommandSpec>> {
        self.commands.get(name).cloned()
    }

    /// Returns an iterator over all commands.
    pub fn get_all_commands(&self) -> impl Iterator<Item = &Rc<dyn CommandSpec>> {
        self.commands.values()
    }
}

define_commands! {
    // Editor actions.
    Quit {
        description: "Quit the editor",
        handler: {
            if !editor.pane_manager.iter().any(|d| d.is_dirty()) {
                editor.should_quit = true;
            } else {
                editor.prompt_manager.show_prompt(
                    PromptType::Confirm(ConfirmPrompt::new("There are open panes with unsaved changes, do you want to save them before quitting?")),
                    |editor, response| {
                        match response {
                            PromptResponse::Yes => {
                                editor.buffer_manager.save_all_buffers()?;
                                editor.should_quit = true;
                            },
                            PromptResponse::No => editor.should_quit = true,
                            _ => return Ok(()),
                        };
                        Ok(())
                    }
                );
            }
        }
    },
    Save {
        description: "Save the current pane",
        args: [ path: Option<String> ],
        handler: { editor.save_active_buffer(self.path.clone())?; }
    },
    OpenSearch {
        description: "Open a search prompt",
        handler: {
            editor.prompt_manager.show_prompt(
                PromptType::Search(SearchPrompt::new(editor.pane_manager.active_mut().clone())),
                |editor, response| {
                    // TODO: Use text to populate a new search state struct in editor for jumping
                    // between all search results.
                    if let PromptResponse::Text(text) = response {
                        let message = format!("Searched for: {text}");
                        editor.show_message(&message);
                    }
                    Ok(())
                }
            );
        }
    },
    EnterInsertMode {
        description: "Enter insert mode",
        handler: { editor.mode = editor::Mode::Insert; }
    },
    EnterCommandMode {
        description: "Enter command mode",
        handler: { editor.mode = editor::Mode::Command; }
    },
    // // Pane and buffer handling.
    Open {
        description: "Open a file",
        args: [ path: String ],
        handler: { editor.open_file(self.path.clone())?; }
    },
    DuplicatePane {
        description: "Duplicate the current pane",
        handler: {
            let path = editor.pane_manager.active().file_name();
            if let Err(err) = editor.open_file(path) {
                return Err(Error::ExecutionError(err));
            }
        }
    },
    ClosePane {
        description: "Close the current pane",
        handler: { editor.close_active_pane()?; }
    },
    NextPane {
        description: "Open next pane",
        handler: { editor.pane_manager.next_pane(); }
    },
    PrevPane {
        description: "Open previous pane",
        handler: { editor.pane_manager.prev_pane(); }
    },
    ListBuffer {
        description: "Lists all open panes",
        handler: {
            let file_names: Vec<String> = editor.buffer_manager
                .iter_buffer_names()
                .enumerate()
                .map(|(i, file)| format!("{}:{}", i + 1, file))
                .collect();
            editor.show_message(&file_names.join(" "));
        }
    },
    // TODO: Merge these into a single command?
    // Cursor movements.
    MoveCursorLeft {
        description: "Move the cursor left",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::Left); }
    },
    MoveCursorRight {
        description: "Move the cursor right",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::Right); }
    },
    MoveCursorUp {
        description: "Move the cursor up",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::Up); }
    },
    MoveCursorDown {
        description: "Move the cursor down",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::Down); }
    },
    MoveCursorToStartOfRow {
        description: "Move the cursor to the start of the row",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::StartOfRow); },
    },
    MoveCursorToEndOfRow {
        description: "Move the cursor to the end of the row",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::EndOfRow); }
    },
    MoveCursorToStartOfBuffer {
        description: "Move the cursor to the start of the buffer",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::StartOfBuffer); }
    },
    MoveCursorToEndOfBuffer {
        description: "Move the cursor to the end of the buffer",
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::EndOfBuffer); }
    },
    MoveCursorToLine {
        description: "Move the cursor to the selected line",
        args: [ line: usize ],
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::Line(self.line)); }
    },
    MoveCursorToPosition {
        description: "Move the cursor to the given column and row",
        args: [ col: usize, row: usize ],
        handler: { editor.pane_manager.active_mut().move_cursor(CursorMovement::Position(self.col, self.row)); }
    },
    // Text manipulation.
    InsertNewline {
        description: "Insert a newline",
        handler: {
            let buffer_mod = editor.pane_manager.active_mut().insert_newline();
            editor.pane_manager.handle_buffer_modification(&buffer_mod);
        }
    },
    DeleteChar {
        description: "Delete the character under the cursor",
        handler: {
            let buffer_mod = editor.pane_manager.active_mut().delete_char();
            editor.pane_manager.handle_buffer_modification(&buffer_mod);
        }
    },
    DeleteCharBefore {
        description: "Delete the character before the cursor",
        handler: {
            let buffer_mod = editor.pane_manager.active_mut().delete_char_before();
            editor.pane_manager.handle_buffer_modification(&buffer_mod);
        }
    },
}
