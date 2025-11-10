use std::{
    io::{self, Write, stdout},
    path::Path,
};

use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use thiserror::Error;

use crate::editor::{backend::TerminalBackend, buffer::Buffer, cursor::Cursor, viewport::Viewport};

pub mod backend;
mod buffer;
mod cursor;
mod viewport;

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
}

impl Editor {
    /// Returns a new editor.
    pub fn new<P: AsRef<Path>>(file: Option<P>) -> Result<Self> {
        let buffer = if let Some(path) = file {
            Buffer::read_file(path).unwrap_or_default()
        } else {
            Buffer::default()
        };

        let (columns, rows) = TerminalBackend::size()?;
        Ok(Self {
            buffer,
            cursor: Cursor::default(),
            viewport: Viewport::new(columns as usize, rows as usize),
        })
    }

    /// Opens a file and loads its contents into the editor.
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.buffer = Buffer::read_file(path)?;
        self.cursor.move_to(0, 0);
        Ok(())
    }

    /// Runs the editor main loop.
    pub fn run(&mut self) -> Result<()> {
        self.render()?;

        loop {
            let needs_redraw = self.viewport.scroll_to_cursor(&self.cursor);
            if needs_redraw {
                self.render()?;
            }

            let (cursor_column, cursor_row) = self.viewport.cursor_screen_position(&self.cursor);
            TerminalBackend::move_cursor(cursor_column as u16, cursor_row as u16)?;

            match event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Left => self.cursor.move_left(&self.buffer),
                    KeyCode::Right => self.cursor.move_right(&self.buffer),
                    KeyCode::Up => self.cursor.move_up(&self.buffer),
                    KeyCode::Down => self.cursor.move_down(&self.buffer),
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    column,
                    row,
                    ..
                }) => {
                    let (logical_col, logical_row) =
                        self.viewport.screen_position(column as usize, row as usize);
                    self.cursor.move_to(logical_col, logical_row);
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Renders the editor to the terminal.
    pub fn render(&self) -> Result<()> {
        TerminalBackend::move_cursor(0, 0)?;
        TerminalBackend::clear()?;

        write!(stdout(), "{}", self.buffer.visible_text(&self.viewport))?;
        Ok(())
    }
}
