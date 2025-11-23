// TODO: Error handling for lock operations.
use std::path::Path;

use crate::editor::{
    buffer::{
        BufferEntry, Error,
        modification::{BufferAction, BufferModification},
    },
    geometry::point::Point,
    pane::cursor::Cursor,
};

pub mod cursor;
pub mod manager;

#[derive(Debug, Clone)]
pub struct Pane {
    pub id: usize,
    pub buffer: BufferEntry,
    pub cursor: Cursor,
}

impl Pane {
    pub fn new(id: usize, buffer: BufferEntry) -> Self {
        Self {
            id,
            buffer,
            cursor: Cursor::default(),
        }
    }

    /// Inserts a character at the current cursor position and attempt to advances the cursor
    /// column. Returns the buffer modification and the buffer id.
    pub fn insert_char(&mut self, c: char) -> BufferModification {
        let mut buffer = self.buffer.write().unwrap();
        let modification = buffer.insert_char(c, &self.cursor);
        if let BufferAction::Insert { .. } = modification {
            self.cursor.move_right(&buffer);
        }

        BufferModification::new(self.buffer.id, modification)
    }

    /// Inserts a newline at the current cursor position.
    pub fn insert_newline(&mut self) -> BufferModification {
        let mut buffer = self.buffer.write().unwrap();
        let modification = buffer.insert_newline(&self.cursor);
        self.cursor.move_to_start_of_next_row(&buffer);
        BufferModification::new(self.buffer.id, modification)
    }

    /// Deletes a character at the current cursor position.
    pub fn delete_char(&mut self) -> BufferModification {
        let mut buffer = self.buffer.write().unwrap();
        let modification = buffer.delete_char(&self.cursor);
        BufferModification::new(self.buffer.id, modification)
    }

    /// Deletes a character before the current cursor position.
    pub fn delete_char_before(&mut self) -> BufferModification {
        let mut buffer = self.buffer.write().unwrap();
        if self.cursor.col() == 0 && self.cursor.row() > 0 {
            let prev_row = self.cursor.row().saturating_sub(1);
            let prev_row_len = buffer.row(prev_row).map(|r| r.len()).unwrap_or_default();

            let modification = buffer.append_line_to_line(self.cursor.row(), prev_row);
            self.cursor.move_to(prev_row_len, prev_row, &buffer);
            BufferModification::new(self.buffer_id(), modification)
        } else {
            if self.cursor.col() == 0 && self.cursor.row() == 0 {
                return BufferModification::new(self.buffer_id(), BufferAction::None);
            }
            self.cursor.move_left(&buffer);
            let modification = buffer.delete_char(&self.cursor);
            BufferModification::new(self.buffer_id(), modification)
        }
    }

    /// Finds the next occurrence of the given string in the buffer and returns its position or
    /// `None`.
    pub fn find_next(&mut self, s: &str) -> Option<Point> {
        let buffer = self.buffer.read().unwrap();
        buffer.find_next(s, &self.cursor)
    }

    /// Moves the cursor one column to the left.
    pub fn move_cursor_left(&mut self) {
        let buffer = self.buffer.read().unwrap();
        self.cursor.move_left(&buffer);
    }

    /// Moves the cursor one column to the right.
    pub fn move_cursor_right(&mut self) {
        let buffer = self.buffer.read().unwrap();
        self.cursor.move_right(&buffer);
    }

    /// Moves the cursor one row up.
    pub fn move_cursor_up(&mut self) {
        let buffer = self.buffer.read().unwrap();
        self.cursor.move_up(&buffer);
    }

    /// Moves the cursor one row down.
    pub fn move_cursor_down(&mut self) {
        let buffer = self.buffer.read().unwrap();
        self.cursor.move_down(&buffer);
    }

    /// Moves the cursor to the given position.
    pub fn move_cursor_to(&mut self, col: usize, row: usize) {
        let buffer = self.buffer.read().unwrap();
        self.cursor.move_to(col, row, &buffer);
    }

    /// Moves the cursor to the start of the current row.
    pub fn move_cursor_to_start_of_row(&mut self) {
        self.cursor.move_to_start_of_row();
    }

    /// Moves the cursor to the end of the current row.
    pub fn move_cursor_to_end_of_row(&mut self) {
        let buffer = self.buffer.read().unwrap();
        self.cursor.move_to_end_of_row(&buffer);
    }

    /// Saves the pane.
    pub fn save(&mut self) -> Result<(), Error> {
        let mut buffer = self.buffer.write().unwrap();
        buffer.save()?;
        Ok(())
    }

    /// Saves the pane to the given path.
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P, force: bool) -> Result<(), Error> {
        let mut buffer = self.buffer.write().unwrap();
        buffer.save_as(path, force)?;
        Ok(())
    }

    /// Returns the name of the file associated with the pane.
    pub fn file_name(&self) -> String {
        let buffer = self.buffer.read().unwrap();
        buffer.file_name()
    }

    /// Returns true if the pane has been modified.
    pub fn is_dirty(&self) -> bool {
        let buffer = self.buffer.read().unwrap();
        buffer.is_dirty()
    }

    pub fn buffer_id(&self) -> usize {
        self.buffer.id
    }

    /// Returns the current cursor position.
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor.position()
    }
}
