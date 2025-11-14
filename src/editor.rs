use std::{fmt, path::Path};

use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use thiserror::Error;

use crate::editor::{
    backend::TerminalBackend,
    buffer::Buffer,
    command::{CommandArgs, CommandRegistry, register_commands},
    command_palette::CommandPalette,
    cursor::Cursor,
    gutter::Gutter,
    keymap::Keymap,
    prompt::{PromptManager, PromptResponse, PromptStatus, confirm::ConfirmPrompt},
    status_bar::{Message, StatusBar},
    viewport::Viewport,
};

pub mod backend;
mod buffer;
mod command;
mod command_palette;
mod cursor;
mod gutter;
mod keymap;
mod prompt;
mod status_bar;
mod viewport;

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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
    /// The currently open buffer.
    buffer: Buffer,
    /// The cursor position.
    cursor: Cursor,
    /// The current viewport.
    viewport: Viewport,
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
            Buffer::open_file(path).unwrap_or_default()
        } else {
            Buffer::default()
        };

        let (columns, rows) = backend.size()?;
        let gutter = Gutter::new(GUTTER_WIDTH);
        let status_bar = StatusBar::default();

        let viewport = Viewport::new(
            columns as usize - gutter.width(),
            rows as usize - status_bar.height(),
        );

        // Initialize commands and keybindings.
        let mut command_registry = CommandRegistry::new();
        register_commands(&mut command_registry);

        let command_palette = CommandPalette::new(&command_registry);

        Ok(Self {
            buffer,
            cursor: Cursor::default(),
            viewport,
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
        self.buffer = Buffer::open_new_or_existing_file(&path)?;
        self.cursor.move_to(0, 0, &self.buffer);
        Ok(())
    }

    /// Runs the editor main loop.
    pub fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.viewport.scroll_to_cursor(&self.cursor);
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
                } else if let KeyCode::Char(c) = event.code
                    && self.buffer.insert_char(c, &self.cursor)
                {
                    self.cursor.move_right(&self.buffer);
                }
            }
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column,
                row,
                ..
            }) => {
                let (logical_col, logical_row) =
                    self.viewport
                        .screen_position(column as usize, row as usize, &self.gutter);
                self.cursor.move_to(logical_col, logical_row, &self.buffer);
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
                    if let Some(command) = self.command_registry.get(command.name)
                        && let Err(err) = command.execute(self, &args)
                    {
                        self.show_err_message(&err.to_string());
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
            match active.prompt.handle_input(&key) {
                PromptStatus::Pending => {}
                PromptStatus::Done(action) => {
                    let active = self.prompt_manager.active_prompt.take().unwrap();
                    if let Err(err) = (active.callback)(self, action) {
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
            if let Err(buffer::Error::SaveError(buffer::SaveError::FileAlreadyExists(_))) =
                self.buffer.save_as(&path, false)
            {
                self.prompt_manager.show_prompt(
                    Box::new(ConfirmPrompt::new(
                        "File already exists, do you want to overwrite it?",
                    )),
                    move |editor, response| {
                        if response == PromptResponse::Yes {
                            editor.buffer.save_as(&path, true)?;
                        }
                        Ok(())
                    },
                )
            }
        } else {
            self.buffer.save()?;
        }

        Ok(())
    }

    /// Exits the editor.
    pub fn exit(&self) -> Result<()> {
        self.backend.deinitialize()?;
        Ok(())
    }

    /// Updates the state of the editor.
    pub fn update(&mut self) {
        self.status_bar.update()
    }

    /// Renders the editor to the terminal.
    pub fn render(&self) -> Result<()> {
        self.backend.hide_cursor()?;
        self.backend.move_cursor(0, 0)?;
        self.backend.clear_all()?;

        for screen_row in 0..self.viewport.height() {
            let logical_row = self.viewport.row_offset + screen_row;
            if self.buffer.row(logical_row).is_some() {
                self.gutter.render_row(logical_row, &self.backend)?;
                self.buffer
                    .render_row(logical_row, &self.viewport, &self.backend)?;
            }

            self.backend.write("\r\n")?;
        }

        self.status_bar
            .render(self.mode, &self.buffer, &self.cursor, &self.backend)?;

        // TODO: Implement a compositor.
        if let Some(active) = &self.prompt_manager.active_prompt {
            active.prompt.render(&self.backend)?;
        } else if self.mode == Mode::Command {
            self.command_palette.render(&self.backend)?;
        } else {
            self.cursor
                .render(&self.viewport, &self.gutter, &self.backend)?;
        }

        self.backend.show_cursor()?;
        self.backend.flush()?;

        Ok(())
    }
}
