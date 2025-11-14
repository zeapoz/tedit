use std::{collections::HashMap, fmt::Debug, rc::Rc};

use thiserror::Error;

use crate::editor::{
    self, Editor,
    prompt::{PromptResponse, confirm::ConfirmPrompt},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing argument(s): {0}")]
    MissingArguments(String),
    #[error("Too many arguments, expected {expected}")]
    TooManyArguments { expected: usize },
    #[allow(clippy::enum_variant_names)]
    #[error(transparent)]
    ExecutionError(#[from] editor::Error),
}

#[derive(Debug, Default, Clone)]
pub struct CommandArgs {
    /// Positional arguments.
    positional: Vec<String>,
}

impl CommandArgs {
    /// Returns a new instance with the given positional arguments.
    pub fn new(positional: Vec<&str>) -> Self {
        Self {
            positional: positional.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Gets a positional argument by index.
    pub fn get_positional(&self, index: usize) -> Option<&str> {
        self.positional.get(index).map(|s| s.as_str())
    }

    /// Returns an iterator over all positional arguments.
    pub fn iter_positional(&self) -> impl Iterator<Item = &String> {
        self.positional.iter()
    }

    /// Returns the number of positional arguments.
    pub fn num_positional(&self) -> usize {
        self.positional.len()
    }
}

/// The `Command` trait defines a command that can be executed by the editor.
pub trait Command: Debug {
    /// Returns the name of the command.
    fn name(&self) -> &'static str;

    /// Returns a description of the command.
    fn description(&self) -> &'static str;

    /// Executes the command.
    fn execute(&self, editor: &mut Editor, args: &CommandArgs) -> Result<(), Error>;
}

/// A registry for all available commands.
#[derive(Debug, Default)]
pub struct CommandRegistry {
    commands: HashMap<&'static str, Rc<dyn Command>>,
}

impl CommandRegistry {
    /// Creates a new command registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new command.
    pub fn register(&mut self, command: Rc<dyn Command>) {
        self.commands.insert(command.name(), command);
    }

    /// Gets a command by name.
    pub fn get(&self, name: &str) -> Option<Rc<dyn Command>> {
        self.commands.get(name).cloned()
    }

    /// Returns an iterator over all commands.
    pub fn get_all_commands(&self) -> impl Iterator<Item = &Rc<dyn Command>> {
        self.commands.values()
    }
}

/// A macro to define and register commands.
#[macro_export]
macro_rules! define_commands {
    ( $( $command_name:ident { name: $name:expr, description: $description:expr, handler: $handler:expr $(,)? } ),* $(,)? ) => {
        $(
            #[derive(Debug)]
            pub struct $command_name;

            impl $crate::editor::command::Command for $command_name {
                fn name(&self) -> &'static str {
                    $name
                }

                fn description(&self) -> &'static str {
                    $description
                }

                fn execute(&self, editor: &mut $crate::editor::Editor, args: &$crate::editor::command::CommandArgs) -> Result<(), $crate::editor::command::Error> {
                    let f: fn(&mut $crate::editor::Editor, &$crate::editor::command::CommandArgs) -> Result<(), $crate::editor::command::Error> = $handler;
                    f(editor, args)
                }
            }
        )*

        pub fn register_commands(registry: &mut $crate::editor::command::CommandRegistry) {
            $(
                registry.register(Rc::new($command_name));
            )*
        }
    };
}

define_commands! {
    // Editor actions.
    Quit {
        name: "quit",
        description: "Quit the editor",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            if !editor.buffer.dirty {
                editor.should_quit = true;
            } else {
                editor.prompt_manager.show_prompt(
                    Box::new(ConfirmPrompt::new("Buffer contains unsaved changes? Do you want to save and quit?")),
                    |editor, response| {
                        match response {
                            PromptResponse::Yes => {
                                editor.save_active_buffer(None::<&str>)?;
                                editor.should_quit = true;
                            },
                            PromptResponse::No => editor.should_quit = true,
                            PromptResponse::Cancel => return Ok(()),
                        };
                        Ok(())
                    }
                );
            }
            Ok(())
        }
    },
    Save {
        name: "save",
        description: "Save the current buffer",
        handler: |editor: &mut Editor, args: &CommandArgs| {
            if args.num_positional() > 1 {
                return Err(Error::TooManyArguments { expected: 1 });
            }

            let path = args.get_positional(0);
            editor.save_active_buffer(path)?;
            Ok(())
        }
    },
    Open {
        name: "open",
        description: "Open a file",
        handler: |editor: &mut Editor, args: &CommandArgs| {
            if args.num_positional() == 0 {
                return Err(Error::MissingArguments("[FILE]".into()));
            };

            for path in args.iter_positional() {
                if let Err(err) = editor.open_file(path) {
                    return Err(Error::ExecutionError(err));
                }
            }
            Ok(())
        }
    },
    EnterInsertMode {
        name: "enter_insert_mode",
        description: "Enter insert mode",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.mode = editor::Mode::Insert;
            Ok(())
        }
    },
    EnterCommandMode {
        name: "enter_command_mode",
        description: "Enter command mode",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.mode = editor::Mode::Command;
            Ok(())
        }
    },
    // Cursor movements.
    MoveCursorLeft {
        name: "move_cursor_left",
        description: "Move the cursor left",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.cursor.move_left(&editor.buffer);
            Ok(())
        }
    },
    MoveCursorRight {
        name: "move_cursor_right",
        description: "Move the cursor right",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.cursor.move_right(&editor.buffer);
            Ok(())
        }
    },
    MoveCursorUp {
        name: "move_cursor_up",
        description: "Move the cursor up",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.cursor.move_up(&editor.buffer);
            Ok(())
        }
    },
    MoveCursorDown {
        name: "move_cursor_down",
        description: "Move the cursor down",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.cursor.move_down(&editor.buffer);
            Ok(())
        }
    },
    MoveCursorToStartOfRow {
        name: "move_cursor_to_start_of_row",
        description: "Move the cursor to the start of the row",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.cursor.move_to_start_of_row();
            Ok(())
        }
    },
    MoveCursorToEndOfRow {
        name: "move_cursor_to_end_of_row",
        description: "Move the cursor to the end of the row",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.cursor.move_to_end_of_row(&editor.buffer);
            Ok(())
        }
    },
    // Text manipulation.
    InsertNewline {
        name: "insert_newline",
        description: "Insert a newline",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.buffer.insert_newline(&editor.cursor);
            editor.cursor.move_to_start_of_next_row(&editor.buffer);
            Ok(())
        }
    },
    DeleteChar {
        name: "delete_char",
        description: "Delete the character under the cursor",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            editor.buffer.delete_char(&editor.cursor);
            Ok(())
        }
    },
    DeleteCharBefore {
        name: "delete_char_before",
        description: "Delete the character before the cursor",
        handler: |editor: &mut Editor, _args: &CommandArgs| {
            if editor.cursor.col() == 0 && editor.cursor.row() > 0 {
                let prev_row = editor.cursor.row().saturating_sub(1);
                let prev_row_len = editor
                    .buffer
                    .row(prev_row)
                    .map(|r| r.len())
                    .unwrap_or_default();

                editor.buffer.join_rows(prev_row, editor.cursor.row());
                editor.cursor.move_to(prev_row_len, prev_row, &editor.buffer);
            } else {
                if editor.cursor.col() == 0 && editor.cursor.row() == 0 {
                    return Ok(());
                }
                editor.cursor.move_left(&editor.buffer);
                editor.buffer.delete_char(&editor.cursor);
            }
            Ok(())
        }
    },
}
