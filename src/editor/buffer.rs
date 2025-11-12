use std::{
    fs, io, mem,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::editor::{
    backend::{self, TerminalBackend},
    buffer::row::Row,
    cursor::Cursor,
    viewport::Viewport,
};

mod row;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to open buffer: {0}")]
    OpenError(#[from] io::Error),
    #[error("failed to save buffer: {0}")]
    SaveError(#[from] SaveError),
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("no path specified")]
    MissingPath,
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug)]
pub struct Buffer {
    /// The rows of the buffer.
    rows: Vec<Row>,
    /// The path of the file this buffer represents.
    filepath: Option<PathBuf>,
    /// Whether the buffer has been modified.
    pub dirty: bool,
}

impl Buffer {
    /// Opens an existing file, or if it doesn't exist, opens a new buffer set to the given path.
    pub fn open_new_or_existing_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        if let Ok(true) = fs::exists(&path) {
            Self::open_file(&path)
        } else {
            Ok(Self::open_new(&path))
        }
    }

    /// Opens a new buffer set to the given path.
    pub fn open_new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            rows: vec![Row::default()],
            filepath: Some(path.as_ref().to_path_buf()),
            dirty: false,
        }
    }

    /// Open a new file and read its contents.
    pub fn open_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let contents = fs::read_to_string(&path)?;

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

    /// Returns the full text of the buffer as a [`String`].
    pub fn text(&self) -> String {
        self.rows
            .iter()
            .map(|r| r.text().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Renders the row at the given screen row and viewport.
    pub fn render_row(
        &self,
        row: usize,
        viewport: &Viewport,
        backend: &TerminalBackend,
    ) -> Result<(), backend::Error> {
        let visible_chars: String = self
            .row(row)
            .map(|r| r.visible_chars(viewport))
            .unwrap_or_default()
            .iter()
            .collect();

        backend.write(&visible_chars)
    }

    /// Saves the buffer to the given path. Or the current path if none is given.
    pub fn save<P: AsRef<Path>>(&mut self, path: Option<P>) -> Result<(), Error> {
        let path = match (path.as_ref(), self.filepath.as_ref()) {
            (None, None) => return Err(SaveError::MissingPath.into()),
            (None, Some(p)) => p,
            (Some(p), None) | (Some(p), Some(_)) => {
                // Update the filepath to the new path.
                let path = p.as_ref().to_path_buf();
                self.filepath = Some(path);

                p.as_ref()
            }
        };

        fs::write(path, self.text()).map_err(SaveError::IoError)?;
        self.dirty = false;
        Ok(())
    }

    /// Returns the path of the file this buffer represents, or `[No Filename]` if none.
    pub fn file_name(&self) -> String {
        /// The file name to use for an empty buffer.
        const NO_FILENAME: &str = "[No Filename]";

        self.filepath
            .as_ref()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or(NO_FILENAME.into())
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            rows: vec![Row::default()],
            filepath: Default::default(),
            dirty: Default::default(),
        }
    }
}
