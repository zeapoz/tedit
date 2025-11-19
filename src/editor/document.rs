use std::path::Path;

use crate::editor::{
    document::{
        buffer::{Buffer, Error},
        cursor::Cursor,
        viewport::Viewport,
    },
    gutter::Gutter,
    renderer::{
        Renderable, RenderingContext, frame::Span, viewport::Viewport as RenderingViewport,
    },
};

pub mod buffer;
pub mod cursor;
pub mod manager;
pub mod viewport;

#[derive(Debug, Default, Clone)]
pub struct Document {
    // TODO: Store in `Arc<RwLock<T>>`.
    buffer: Buffer,
    cursor: Cursor,
    viewport: Viewport,
}

impl Document {
    pub fn new(buffer: Buffer, viewport: Viewport) -> Self {
        Self {
            buffer,
            cursor: Cursor::default(),
            viewport,
        }
    }

    /// Returns a new empty document.
    pub fn new_empty(viewport: Viewport) -> Self {
        Self {
            buffer: Buffer::default(),
            cursor: Cursor::default(),
            viewport,
        }
    }

    /// Inserts a character at the current cursor position and attempt to advances the cursor
    /// column.
    pub fn insert_char(&mut self, c: char) {
        if self.buffer.insert_char(c, &self.cursor) {
            self.cursor.move_right(&self.buffer);
        }
    }

    /// Inserts a newline at the current cursor position.
    pub fn insert_newline(&mut self) {
        self.buffer.insert_newline(&self.cursor);
        self.cursor.move_to_start_of_next_row(&self.buffer);
    }

    /// Deletes a character at the current cursor position.
    pub fn delete_char(&mut self) {
        self.buffer.delete_char(&self.cursor);
    }

    /// Deletes a character before the current cursor position.
    pub fn delete_char_before(&mut self) {
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
            if self.cursor.col() == 0 && self.cursor.row() == 0 {
                return;
            }
            self.cursor.move_left(&self.buffer);
            self.buffer.delete_char(&self.cursor);
        }
    }

    /// Finds the next occurrence of the given string in the buffer and returns its position or
    /// `None`.
    pub fn find_next(&mut self, s: &str) -> Option<(usize, usize)> {
        self.buffer.find_next(s, &self.cursor)
    }

    /// Moves the cursor one column to the left.
    pub fn move_cursor_left(&mut self) {
        self.cursor.move_left(&self.buffer);
    }

    /// Moves the cursor one column to the right.
    pub fn move_cursor_right(&mut self) {
        self.cursor.move_right(&self.buffer);
    }

    /// Moves the cursor one row up.
    pub fn move_cursor_up(&mut self) {
        self.cursor.move_up(&self.buffer);
    }

    /// Moves the cursor one row down.
    pub fn move_cursor_down(&mut self) {
        self.cursor.move_down(&self.buffer);
    }

    /// Moves the cursor to the given position.
    pub fn move_cursor_to(&mut self, col: usize, row: usize) {
        self.cursor.move_to(col, row, &self.buffer);
    }

    /// Moves the cursor to the start of the current row.
    pub fn move_cursor_to_start_of_row(&mut self) {
        self.cursor.move_to_start_of_row();
    }

    /// Moves the cursor to the end of the current row.
    pub fn move_cursor_to_end_of_row(&mut self) {
        self.cursor.move_to_end_of_row(&self.buffer);
    }

    /// Saves the document.
    pub fn save(&mut self) -> Result<(), Error> {
        self.buffer.save()?;
        Ok(())
    }

    /// Saves the document to the given path.
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P, force: bool) -> Result<(), Error> {
        self.buffer.save_as(path, force)?;
        Ok(())
    }

    /// Scrolls the viewport to the cursor.
    pub fn scroll_to_cursor(&mut self) {
        self.viewport.scroll_to_cursor(&self.cursor);
    }

    /// Updates the viewport to match the current window dimensions.
    pub fn update_viewport(&mut self, width: usize, height: usize) {
        self.viewport.update_size(width, height);
    }

    // TODO: These kind of mappings should happen in some UI layer.
    /// Handles a click event and maps the position to the corresponding row and column.
    pub fn click(&mut self, col: usize, row: usize, gutter: &Gutter) {
        let (logical_col, logical_row) = self.viewport.screen_position(col, row, gutter);
        self.move_cursor_to(logical_col, logical_row);
    }

    /// Returns the name of the file associated with the document.
    pub fn file_name(&self) -> String {
        self.buffer.file_name()
    }

    /// Returns true if the document has been modified.
    pub fn is_dirty(&self) -> bool {
        self.buffer.is_dirty()
    }

    /// Returns the row offset of the viewport.
    pub fn viewport_row_offset(&self) -> usize {
        self.viewport.row_offset
    }

    /// Returns the current cursor position.
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor.position()
    }
}

impl Renderable for Document {
    fn render(&self, _ctx: &RenderingContext, mut viewport: RenderingViewport<'_>) {
        // Update viewport to match the dimensions of the given rectangle.
        let start_row = self.viewport.row_offset;
        for row in 0..viewport.height() {
            let buffer_row = start_row + row;
            let row_visible_chars = self
                .buffer
                .row(buffer_row)
                .map(|r| r.visible_chars(&self.viewport))
                .unwrap_or_default();
            viewport.put_span(0, row, Span::new(&row_visible_chars));
        }
    }
}
