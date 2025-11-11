use std::{io, path::Path};

use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use thiserror::Error;

use crate::editor::{
    backend::TerminalBackend,
    buffer::Buffer,
    command::{CommandRegistry, register_commands},
    cursor::Cursor,
    gutter::Gutter,
    keymap::Keymap,
    status_bar::StatusBar,
    viewport::Viewport,
};

pub mod backend;
mod buffer;
mod command;
mod cursor;
mod gutter;
mod keymap;
mod status_bar;
mod viewport;

// TODO: Make this adapt to the current buffer/be configurable.
/// The width of the gutter.
const GUTTER_WIDTH: usize = 6;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),
    #[error("buffer error: {0}")]
    BufferError(#[from] buffer::Error),
}

#[derive(Debug)]
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
    /// The mapping from key to command.
    keymap: Keymap,
    /// Whether the editor should quit.
    pub should_quit: bool,
}

impl Editor {
    /// Returns a new editor.
    pub fn new<P: AsRef<Path>>(file: Option<P>) -> Result<Self> {
        let backend = TerminalBackend::initialize()?;

        let buffer = if let Some(path) = file {
            Buffer::read_file(path).unwrap_or_default()
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

        Ok(Self {
            buffer,
            cursor: Cursor::default(),
            viewport,
            gutter,
            status_bar,
            backend,
            command_registry,
            keymap: Keymap::default(),
            should_quit: false,
        })
    }

    /// Opens a file and loads its contents into the editor.
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.buffer = Buffer::read_file(path)?;
        self.cursor.move_to(0, 0, &self.buffer);
        Ok(())
    }

    /// Runs the editor main loop.
    pub fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.viewport.scroll_to_cursor(&self.cursor);
            self.render()?;

            match event::read()? {
                Event::Key(event) => {
                    if let Some(command_name) = self.keymap.get(&event) {
                        if let Some(command) = self.command_registry.get(command_name) {
                            command.execute(self);
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

        self.exit()
    }

    /// Exits the editor.
    pub fn exit(&self) -> Result<()> {
        self.backend.deinitialize()
    }

    /// Renders the editor to the terminal.
    pub fn render(&self) -> Result<()> {
        self.backend.hide_cursor()?;
        self.backend.move_cursor(0, 0)?;
        self.backend.clear()?;

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
            .render(&self.buffer, &self.cursor, &self.backend)?;

        self.cursor
            .render(&self.viewport, &self.gutter, &self.backend)?;

        self.backend.show_cursor()?;
        self.backend.flush()?;

        Ok(())
    }
}
