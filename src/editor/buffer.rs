use std::{
    fs, io, mem,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use thiserror::Error;

use crate::editor::{
    buffer::{
        modification::{ActionRange, BufferAction},
        row::Row,
    },
    pane::cursor::Cursor,
    ui::geometry::point::Point,
};

pub mod manager;
pub mod modification;
pub mod row;

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
    #[error("file already exists: {0}")]
    FileAlreadyExists(PathBuf),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug, Clone)]
pub struct BufferEntry {
    pub id: usize,
    pub buffer: Arc<RwLock<Buffer>>,
}

impl BufferEntry {
    pub fn new(id: usize, buffer: Arc<RwLock<Buffer>>) -> Self {
        BufferEntry { id, buffer }
    }
}

impl Deref for BufferEntry {
    type Target = Arc<RwLock<Buffer>>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for BufferEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    /// The rows of the buffer.
    rows: Vec<Row>,
    /// The path of the file this buffer represents.
    filepath: Option<PathBuf>,
    /// Whether the buffer has been modified.
    dirty: bool,
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

    /// Inserts a character at the given cursor position.
    pub fn insert_char(&mut self, c: char, cursor: &Cursor) -> BufferAction {
        if let Some(row) = self.rows.get_mut(cursor.row())
            && row.insert_char(cursor.col(), c)
        {
            self.dirty = true;
            return BufferAction::Insert {
                start: cursor.position().into(),
                text: c.to_string(),
            };
        }
        BufferAction::None
    }

    /// Inserts a newline at the given cursor position.
    pub fn insert_newline(&mut self, cursor: &Cursor) -> BufferAction {
        if let Some(row) = self.rows.get_mut(cursor.row()) {
            let (left, right) = row.split_at(cursor.col());
            let _ = mem::replace(row, left);
            // PERF: All items have to be shifted when inserting newlines. We should use a
            // better data structure that doesn't require this to store the text.
            self.rows.insert(cursor.row() + 1, right);
            self.dirty = true;

            return BufferAction::Insert {
                start: cursor.position().into(),
                text: "\n".into(),
            };
        }

        BufferAction::None
    }

    /// Deletes a character at the given cursor position. If the cursor is at the end of the row,
    /// joins the row with the next row.
    pub fn delete_char(&mut self, cursor: &Cursor) -> BufferAction {
        let current_row_len = self
            .rows
            .get(cursor.row())
            .map(|r| r.len())
            .unwrap_or_default();
        if current_row_len == 0 {
            return BufferAction::None;
        }

        // If the cursor is at the last column, join with the next row. Otherwise, just delete the
        // character.
        if cursor.col() == current_row_len {
            let next_row = cursor.row().saturating_add(1);
            return self.append_line_to_line(cursor.row(), next_row);
        } else if let Some(row) = self.rows.get_mut(cursor.row())
            && row.delete_char(cursor.col())
        {
            self.dirty = true;

            let delete_range = ActionRange::PointToPoint {
                from: cursor.position().into(),
                to: cursor.position().into(),
            };
            return BufferAction::Delete(delete_range);
        }

        BufferAction::None
    }

    /// Appends the row at index `right` to the row at index `left`.
    pub fn append_line_to_line(&mut self, from: usize, to: usize) -> BufferAction {
        let right_row = self.rows.remove(from);
        if let Some(row) = self.rows.get_mut(to) {
            row.append_row(&right_row);
            self.dirty = true;

            // FIXME: This is a hack to make sure that the buffer viewport maintains its position
            // when another pane deletes a line.
            return BufferAction::Delete(ActionRange::Line(from));
        }
        BufferAction::None
    }

    /// Finds the next occurrence of the given string in the buffer and returns its position or
    /// `None` if not found.
    pub fn find_next(&self, s: &str, cursor: &Cursor) -> Option<Point> {
        self.rows
            .iter()
            .enumerate()
            .skip(cursor.row())
            .find_map(|(i, row)| {
                // Ensure that the first row is searched from the cursor column.
                let offset = if i == cursor.row() {
                    cursor.col().saturating_add(1)
                } else {
                    0
                };
                row.find_next(s, offset).map(|col| Point::new(col, i))
            })
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

    /// Saves the buffer to the path stored in the buffer.
    pub fn save(&mut self) -> Result<(), Error> {
        let path = self.filepath.as_ref().ok_or(SaveError::MissingPath)?;
        fs::write(path, self.text()).map_err(SaveError::IoError)?;
        self.dirty = false;
        Ok(())
    }

    /// Saves the buffer to the given path. If the file already exists at the given path and
    /// `force` is `false`, the buffer will not be saved and the function will return
    /// an error. If `force` is `true`, the file will instead be overwritten.
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P, force: bool) -> Result<(), Error> {
        if fs::exists(&path)? && !force {
            return Err(SaveError::FileAlreadyExists(path.as_ref().to_path_buf()).into());
        }

        fs::write(&path, self.text()).map_err(SaveError::IoError)?;
        self.filepath = Some(path.as_ref().to_path_buf());
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

    /// Returns true if the buffer has been modified.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Returns the number of lines in the buffer.
    pub fn num_lines(&self) -> usize {
        self.rows.len()
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
