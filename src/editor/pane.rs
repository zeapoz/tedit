// TODO: Error handling for lock operations.
use std::path::Path;

use crate::editor::{
    buffer::{BufferEntry, Error},
    pane::{cursor::Cursor, gutter::Gutter, viewport::Viewport},
    renderer::{
        Renderable, RenderingContext,
        frame::{Line, Span},
        style::{Color, Style},
        viewport::Viewport as RenderingViewport,
    },
};

pub mod cursor;
pub mod gutter;
pub mod manager;
pub mod viewport;

#[derive(Debug, Clone)]
pub struct Pane {
    buffer: BufferEntry,
    cursor: Cursor,
    gutter: Gutter,
    viewport: Viewport,
}

impl Pane {
    pub fn new(buffer: BufferEntry, viewport: Viewport) -> Self {
        Self {
            buffer,
            cursor: Cursor::default(),
            gutter: Gutter::default(),
            viewport,
        }
    }

    /// Inserts a character at the current cursor position and attempt to advances the cursor
    /// column.
    pub fn insert_char(&mut self, c: char) {
        let mut buffer = self.buffer.write().unwrap();
        if buffer.insert_char(c, &self.cursor) {
            self.cursor.move_right(&buffer);
        }
    }

    /// Inserts a newline at the current cursor position.
    pub fn insert_newline(&mut self) {
        let mut buffer = self.buffer.write().unwrap();
        buffer.insert_newline(&self.cursor);
        self.cursor.move_to_start_of_next_row(&buffer);
    }

    /// Deletes a character at the current cursor position.
    pub fn delete_char(&mut self) {
        let mut buffer = self.buffer.write().unwrap();
        buffer.delete_char(&self.cursor);
    }

    /// Deletes a character before the current cursor position.
    pub fn delete_char_before(&mut self) {
        let mut buffer = self.buffer.write().unwrap();
        if self.cursor.col() == 0 && self.cursor.row() > 0 {
            let prev_row = self.cursor.row().saturating_sub(1);
            let prev_row_len = buffer.row(prev_row).map(|r| r.len()).unwrap_or_default();

            buffer.join_rows(prev_row, self.cursor.row());
            self.cursor.move_to(prev_row_len, prev_row, &buffer);
        } else {
            if self.cursor.col() == 0 && self.cursor.row() == 0 {
                return;
            }
            self.cursor.move_left(&buffer);
            buffer.delete_char(&self.cursor);
        }
    }

    /// Finds the next occurrence of the given string in the buffer and returns its position or
    /// `None`.
    pub fn find_next(&mut self, s: &str) -> Option<(usize, usize)> {
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
    pub fn click(&mut self, col: usize, row: usize) {
        let logical_col = self.viewport.col_offset + col - self.gutter.width();
        let logical_row = self.viewport.row_offset + row;
        self.move_cursor_to(logical_col, logical_row);
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

    /// Returns the relative cursor position, the position of the cursror relative to the viewport
    /// and offset by the gutter.
    pub fn relaive_cursor_position(&self) -> (usize, usize) {
        let (mut col, mut row) = self.cursor.position();
        col = col.saturating_sub(self.viewport.col_offset) + self.gutter.width();
        row = row.saturating_sub(self.viewport.row_offset);
        (col, row)
    }

    /// Renders teh gutter.
    fn render_gutter(&self, mut viewport: RenderingViewport) {
        let cursor_row = self.cursor_position().1;
        for row in 0..self.viewport.height() {
            // Reserve two spaces at the end of the gutter.
            let padding_width = self.gutter.width().saturating_sub(2);
            let pane_row = self.viewport.row_offset + row;
            let s = format!(
                "{:>width$}  ",
                pane_row.saturating_add(1),
                width = padding_width
            );

            let style = if cursor_row == pane_row {
                Style::new().bold().fg(Color::DarkYellow)
            } else {
                Style::default()
            };

            viewport.put_line(
                row,
                Line::new(viewport.width(), vec![Span::new(&s)]).with_style(style),
            );
        }
    }
}

impl Renderable for Pane {
    fn render(&self, _ctx: &RenderingContext, mut viewport: RenderingViewport) {
        let (gutter_view, mut buffer_view) = viewport.split_horizontally_exact(self.gutter.width());
        self.render_gutter(gutter_view);

        let buffer = self.buffer.read().unwrap();
        let start_row = self.viewport.row_offset;
        for row in 0..buffer_view.height() {
            let buffer_row = start_row + row;
            let row_visible_chars = buffer
                .row(buffer_row)
                .map(|r| r.visible_chars(&self.viewport))
                .unwrap_or_default();
            buffer_view.put_line(
                row,
                Line::new(buffer_view.width(), vec![Span::new(&row_visible_chars)]),
            );
        }
    }
}
