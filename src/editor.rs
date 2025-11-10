use std::{io, path::Path};

use crossterm::event::{
    self, Event, KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use thiserror::Error;

use crate::editor::{
    backend::TerminalBackend, buffer::Buffer, cursor::Cursor, gutter::Gutter, viewport::Viewport,
};

pub mod backend;
mod buffer;
mod cursor;
mod gutter;
mod viewport;

// TODO: Make this adapt to the current buffer/be configurable.
/// The width of the gutter.
const GUTTER_WIDTH: usize = 6;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(#[from] io::Error),
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
    /// The terminal backend.
    backend: TerminalBackend,
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
        let viewport = Viewport::new(columns as usize - gutter.width(), rows as usize);

        Ok(Self {
            buffer,
            cursor: Cursor::default(),
            viewport,
            gutter,
            backend,
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
        loop {
            self.viewport.scroll_to_cursor(&self.cursor);
            self.render()?;

            match event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Left => self.cursor.move_left(&self.buffer),
                    KeyCode::Right => self.cursor.move_right(&self.buffer),
                    KeyCode::Up => self.cursor.move_up(&self.buffer),
                    KeyCode::Down => self.cursor.move_down(&self.buffer),
                    KeyCode::Home => self.cursor.move_to_start_of_row(),
                    KeyCode::End => self.cursor.move_to_end_of_row(&self.buffer),
                    KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Char(c) => {
                        if self.buffer.insert_char(c, &self.cursor) {
                            self.cursor.move_right(&self.buffer);
                        }
                    }
                    KeyCode::Enter => {
                        self.buffer.insert_newline(&self.cursor);
                        self.cursor.move_to_start_of_next_row(&self.buffer);
                    }
                    KeyCode::Backspace => {
                        // Check if we want to join the current row with the previous row. If so,
                        // join the current row with the previous row. Otherwise, move the cursor
                        // to the left and delete the character.
                        if self.cursor.col() == 0 && self.cursor.row() > 0 {
                            let prev_row = self.cursor.row().saturating_sub(1);
                            let prev_row_len = self
                                .buffer
                                .row(prev_row)
                                .map(|r| r.len())
                                .unwrap_or_default();

                            self.buffer.join_rows(prev_row, self.cursor.row());
                            self.cursor.move_to(prev_row_len, prev_row, &self.buffer);
                        } else {
                            // Don't delete characters if the cursor is at the start of the buffer.
                            if self.cursor.col() == 0 && self.cursor.row() == 0 {
                                continue;
                            }
                            self.cursor.move_left(&self.buffer);
                            self.buffer.delete_char(&self.cursor);
                        }
                    }
                    KeyCode::Delete => {
                        self.buffer.delete_char(&self.cursor);
                    }
                    _ => {}
                },
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
            if screen_row > 0 {
                self.backend.write("\r\n")?;
            }

            let logical_row = self.viewport.row_offset + screen_row;
            self.gutter.render_row(logical_row, &self.backend)?;
            self.buffer
                .render_row(logical_row, &self.viewport, &self.backend)?;
        }

        self.cursor
            .render(&self.viewport, &self.gutter, &self.backend)?;

        self.backend.show_cursor()?;
        self.backend.flush()?;

        Ok(())
    }
}
