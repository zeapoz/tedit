use std::{
    mem,
    path::{Path, PathBuf},
};

use crate::editor::{
    Result, backend::TerminalBackend, buffer::row::Row, cursor::Cursor, viewport::Viewport,
};

mod row;

#[derive(Debug, Default)]
pub struct Buffer {
    /// The rows of the buffer.
    rows: Vec<Row>,
    /// The path of the file this buffer represents.
    filepath: Option<PathBuf>,
    /// Whether the buffer has been modified.
    pub dirty: bool,
}

impl Buffer {
    /// Open a new file and read its contents.
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(&path)?;

        Ok(Self {
            rows: contents.split("\n").map(Row::new).collect(),
            filepath: Some(path.as_ref().to_path_buf()),
            dirty: false,
        })
    }

    /// Inserts a character at the given cursor position. Returns `true` if the character was
    /// inserted, `false` otherwise.
    pub fn insert_char(&mut self, c: char, cursor: &Cursor) -> bool {
        if let Some(row) = self.rows.get_mut(cursor.row())
            && row.insert_char(cursor.col(), c)
        {
            self.dirty = true;
            return true;
        }
        false
    }

    /// Inserts a newline at the given cursor position.
    pub fn insert_newline(&mut self, cursor: &Cursor) {
        if let Some(row) = self.rows.get_mut(cursor.row()) {
            let (left, right) = row.split_at(cursor.col());
            let _ = mem::replace(row, left);
            // PERF: All items have to be shifted when inserting newlines. We should use a
            // better data structure that doesn't require this to store the text.
            self.rows.insert(cursor.row() + 1, right);
            self.dirty = true;
        }
    }

    /// Deletes a character at the given cursor position. If the cursor is at the end of the row,
    /// joins the row with the next row.
    pub fn delete_char(&mut self, cursor: &Cursor) {
        let current_row_len = self
            .rows
            .get(cursor.row())
            .map(|r| r.len())
            .unwrap_or_default();

        // If the cursor is at the last column, join with the next row. Otherwise, just delete the
        // character.
        if cursor.col() == current_row_len {
            let next_row = cursor.row().saturating_add(1);
            self.join_rows(cursor.row(), next_row);
        } else if let Some(row) = self.rows.get_mut(cursor.row())
            && row.delete_char(cursor.col())
        {
            self.dirty = true;
        }
    }

    /// Joins two rows together by appending the row at index `right` to the row at index `left`.
    pub fn join_rows(&mut self, left: usize, right: usize) {
        let right_row = self.rows.remove(right);
        if let Some(row) = self.rows.get_mut(left) {
            row.append_row(&right_row);
            self.dirty = true;
        }
    }

    /// Returns the row at the given index or `None` if the index is out of bounds.
    pub fn row(&self, row: usize) -> Option<&Row> {
        self.rows.get(row)
    }

    /// Renders the row at the given screen row and viewport.
    pub fn render_row(
        &self,
        row: usize,
        viewport: &Viewport,
        backend: &TerminalBackend,
    ) -> Result<()> {
        let visible_chars: String = self
            .row(row)
            .map(|r| r.visible_chars(viewport))
            .unwrap_or_default()
            .iter()
            .collect();

        backend.write(&visible_chars)
    }

    /// Returns the path of the file this buffer represents, or `[Empty File]` if none.
    pub fn file_name(&self) -> String {
        /// The file name to use for an empty buffer.
        const EMPTY_FILE: &str = "[Empty File]";

        self.filepath
            .as_ref()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or(EMPTY_FILE.into())
    }
}
