use std::{fmt, path::Path};

use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use thiserror::Error;

use crate::editor::{
    backend::TerminalBackend,
    command::{CommandArgs, CommandRegistry, register_commands},
    command_palette::CommandPalette,
    document::{
        Document,
        buffer::{self, Buffer},
        viewport::Viewport,
    },
    gutter::Gutter,
    keymap::Keymap,
    prompt::{PromptAction, PromptManager, PromptResponse, PromptStatus, confirm::ConfirmPrompt},
    renderer::{RenderingContext, compositor::Compositor},
    status_bar::{Message, StatusBar},
};

pub mod backend;
mod command;
mod command_palette;
mod document;
mod gutter;
mod keymap;
mod prompt;
mod renderer;
mod status_bar;

// TODO: Make this adapt to the current buffer/be configurable.
/// The width of the gutter.
const GUTTER_WIDTH: usize = 6;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BufferError(#[from] buffer::Error),
    #[error(transparent)]
    BackendError(#[from] backend::Error),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// A mode for editing text.
    #[default]
    Insert,
    /// A mode for running commands.
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Mode::Insert => "INS",
            Mode::Command => "CMD",
        };
        write!(f, "{s}")
    }
}

pub struct Editor {
    /// The currently open document.
    document: Document,
    /// The gutter.
    gutter: Gutter,
    /// The status bar.
    status_bar: StatusBar,
    /// The terminal backend.
    backend: TerminalBackend,
    /// The command registry.
    command_registry: CommandRegistry,
    /// The command palette.
    command_palette: CommandPalette,
    /// The mapping from key to command.
    keymap: Keymap,
    /// The prompt manager.
    prompt_manager: PromptManager,
    /// The current mode.
    pub mode: Mode,
    /// Whether the editor should quit.
    pub should_quit: bool,
}

impl Editor {
    /// Returns a new editor.
    pub fn new<P: AsRef<Path>>(file: Option<P>) -> Result<Self> {
        let backend = TerminalBackend::initialize()?;

        let buffer = if let Some(path) = file {
            Buffer::open_new_or_existing_file(path).unwrap_or_default()
        } else {
            Buffer::default()
        };

        let (columns, rows) = backend.size()?;
        let gutter = Gutter::new(GUTTER_WIDTH);
        let status_bar = StatusBar::default();

        let viewport = Viewport::new(columns - gutter.width(), rows - status_bar.height());
        let document = Document::new(buffer, viewport);

        // Initialize commands and keybindings.
        let mut command_registry = CommandRegistry::new();
        register_commands(&mut command_registry);

        let command_palette = CommandPalette::new(&command_registry);

        Ok(Self {
            document,
            gutter,
            status_bar,
            backend,
            command_registry,
            command_palette,
            keymap: Keymap::default(),
            prompt_manager: PromptManager::default(),
            mode: Mode::default(),
            should_quit: false,
        })
    }

    /// Opens a file and loads its contents into the editor.
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        // TODO: Keep track of all open buffers.
        let buffer = Buffer::open_new_or_existing_file(&path)?;
        let (width, height) = self.backend.size()?;
        let viewport = Viewport::new(width, height);
        self.document = Document::new(buffer, viewport);
        Ok(())
    }

    /// Runs the editor main loop.
    pub fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.document.scroll_to_cursor();
            self.update();
            self.render()?;

            let event = event::read()?;

            // Handle prompt input first.
            if self.prompt_manager.active_prompt.is_some() {
                self.handle_prompt_input(event);
                continue;
            }

            match self.mode {
                Mode::Insert => self.handle_insert_mode_input(event),
                Mode::Command => self.handle_command_mode_input(event),
            };

            // Only quit if there is no active prompt.
            if self.should_quit && self.prompt_manager.active_prompt.is_none() {
                break;
            }
        }
        self.exit()
    }

    /// Handles event input in insert mode.
    pub fn handle_insert_mode_input(&mut self, event: Event) {
        match event {
            Event::Key(event) => {
                if let Some(command_name) = self.keymap.get(&event) {
                    if let Some(command) = self.command_registry.get(command_name) {
                        // TODO: Store command arguments in keybindings.
                        if let Err(err) = command.execute(self, &CommandArgs::default()) {
                            self.show_err_message(&err.to_string());
                        }
                    }
                } else if let KeyCode::Char(c) = event.code {
                    self.document.insert_char(c);
                }
            }
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                ..
            }) => {
                self.document
                    .click(column as usize, row as usize, &self.gutter);
            }
            _ => {}
        }
    }

    /// Handles event input in command mode.
    pub fn handle_command_mode_input(&mut self, event: Event) {
        if let Event::Key(event) = event {
            // TODO: Implement command mode keybindings as commands with context (mode).
            match event.code {
                KeyCode::Esc => self.exit_command_mode(),
                KeyCode::Enter => {
                    let (command, args) = self.command_palette.parse_query();
                    match self.command_registry.get(&command) {
                        None => self.show_err_message(&format!("No such command: {command}")),
                        Some(command) => {
                            if let Err(err) = command.execute(self, &args) {
                                self.show_err_message(&err.to_string());
                            }
                        }
                    }
                    self.exit_command_mode();
                }
                KeyCode::Tab => self.command_palette.autocomplete_or_next(),
                KeyCode::Char(c) => self.command_palette.insert_char(c),
                KeyCode::Down | KeyCode::BackTab => self.command_palette.select_previous_command(),
                KeyCode::Up => self.command_palette.select_next_command(),
                KeyCode::Backspace => self.command_palette.delete_char(),
                _ => {}
            }
        }
    }

    fn handle_prompt_input(&mut self, event: Event) {
        if let Event::Key(key) = event
            && let Some(active) = self.prompt_manager.active_prompt.as_mut()
        {
            let status = active.prompt.process_key(&key);
            match status {
                PromptStatus::Pending => {}
                PromptStatus::Changed => {
                    let action = active.prompt.on_changed();
                    if let PromptAction::MoveCursor { col, row } = action {
                        self.document.move_cursor_to(col, row);
                    }
                }
                PromptStatus::Done(response) => {
                    let active = self.prompt_manager.active_prompt.take().unwrap();
                    if let Err(err) = (active.callback)(self, response) {
                        self.show_err_message(&err.to_string());
                    }
                }
            }
        }
    }

    /// Shows a message in the status bar.
    pub fn show_message(&mut self, s: &str) {
        let message = Message::new(s);
        self.status_bar.set_message(message);
    }

    /// Shows an error message in the status bar.
    pub fn show_err_message(&mut self, s: &str) {
        self.show_message(&format!("Error: {s}"))
    }

    /// Exits command mode and cleans up the stored query.
    pub fn exit_command_mode(&mut self) {
        self.command_palette.clear_query();
        self.mode = Mode::Insert;
    }

    /// Saves the active buffer.
    pub fn save_active_buffer<P: AsRef<Path>>(&mut self, path: Option<P>) -> Result<()> {
        let path = path.map(|p| p.as_ref().to_path_buf());

        // It a path was given, attempt to save the buffer to that path, prompting to overwrite if
        // the file already exists. Otherwise, save the buffer to the current path.
        if let Some(path) = path {
            // TODO: Use eyre to handle errors instead of long matches.
            if let Err(buffer::Error::SaveError(buffer::SaveError::FileAlreadyExists(_))) =
                self.document.save_as(&path, false)
            {
                self.prompt_manager.show_prompt(
                    Box::new(ConfirmPrompt::new(
                        "File already exists, do you want to overwrite it?",
                    )),
                    move |editor, response| {
                        if response == PromptResponse::Yes {
                            editor.document.save_as(&path, true)?;
                        }
                        Ok(())
                    },
                )
            }
        } else {
            self.document.save()?;
        }

        Ok(())
    }

    /// Exits the editor.
    pub fn exit(&mut self) -> Result<()> {
        self.backend.deinitialize()?;
        Ok(())
    }

    /// Updates the state of the editor.
    pub fn update(&mut self) {
        self.status_bar.update()
    }

    /// Renders the editor to the terminal.
    pub fn render(&mut self) -> Result<()> {
        self.backend.hide_cursor()?;
        self.backend.move_cursor(0, 0)?;
        self.backend.clear_all()?;

        // TODO: Store in editor.
        let compositor = Compositor {
            gutter: &self.gutter,
            document: &self.document,
            status_bar: &self.status_bar,
            prompt_manager: &self.prompt_manager,
            command_palette: &self.command_palette,
        };

        let mut rendering_context = RenderingContext {
            backend: &mut self.backend,
            mode: &self.mode,
            document: &self.document,
        };
        compositor.render(&mut rendering_context)?;

        self.backend.show_cursor()?;
        self.backend.flush()?;

        Ok(())
    }
}
