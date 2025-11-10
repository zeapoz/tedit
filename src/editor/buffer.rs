use std::{
    mem,
    path::{Path, PathBuf},
};

use crate::editor::{Result, buffer::row::Row, cursor::Cursor, viewport::Viewport};

mod row;

#[derive(Debug, Default)]
pub struct Buffer {
    /// The rows of the buffer.
    rows: Vec<Row>,
    /// The path of the file this buffer represents.
    filepath: Option<PathBuf>,
}

impl Buffer {
    /// Open a new file and read its contents.
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(&path)?;

        Ok(Self {
            rows: contents.split("\n").map(Row::new).collect(),
            filepath: Some(path.as_ref().to_path_buf()),
        })
    }

    /// Inserts a character at the given cursor position. Returns `true` if the character was
    /// inserted, `false` otherwise.
    pub fn insert_char(&mut self, c: char, cursor: &Cursor) -> bool {
        if let Some(row) = self.rows.get_mut(cursor.row()) {
            row.insert_char(cursor.col(), c)
        } else {
            false
        }
    }

    /// Inserts a newline at the given cursor position.
    pub fn insert_newline(&mut self, cursor: &Cursor) {
        if let Some(row) = self.rows.get_mut(cursor.row()) {
            let (left, right) = row.split_at(cursor.col());
            let _ = mem::replace(row, left);
            // PERF: All items have to be shifted when inserting newlines. We should use a
            // better data structure that doesn't require this to store the text.
            self.rows.insert(cursor.row() + 1, right);
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
            let next_index = cursor.row().saturating_add(1);
            let next_row = self.rows.remove(next_index);
            if let Some(row) = self.rows.get_mut(cursor.row()) {
                row.append_row(&next_row);
            }
        } else if let Some(row) = self.rows.get_mut(cursor.row()) {
            row.delete_char(cursor.col());
        }
    }

    /// Joins two rows together by appending the row at index `right` to the row at index `left`.
    pub fn join_rows(&mut self, left: usize, right: usize) {
        let right_row = self.rows.remove(right);
        if let Some(row) = self.rows.get_mut(left) {
            row.append_row(&right_row);
        }
    }

    /// Returns the text of the buffer that should be visible on screen.
    pub fn visible_text(&self, viewport: &Viewport) -> String {
        let mut display_text = Vec::with_capacity(viewport.height());

        for row in self
            .rows
            .iter()
            .skip(viewport.row_offset)
            .take(viewport.height())
        {
            let visible_line = row
                .chars()
                .skip(viewport.col_offset)
                .take(viewport.width())
                .collect::<String>();

            display_text.push(visible_line);
        }

        display_text.join("\r\n")
    }

    /// Returns the full text of the buffer.
    pub fn text(&self) -> String {
        self.rows
            .iter()
            .map(|r| r.text())
            .collect::<Vec<&str>>()
            .join("\r\n")
    }

    /// Returns the row at the given index or `None` if the index is out of bounds.
    pub fn row(&self, row: usize) -> Option<&Row> {
        self.rows.get(row)
    }
}
