use crate::editor::{document::cursor::Cursor, gutter::Gutter};

#[derive(Debug, Default, Clone, Copy)]
pub struct Viewport {
    /// The column offset of the viewport.
    pub col_offset: usize,
    /// The row offset of the viewport.
    pub row_offset: usize,
    /// The width of the viewport.
    width: usize,
    /// The height of the viewport.
    height: usize,
}

impl Viewport {
    /// Returns a new viewport with the given dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            col_offset: 0,
            row_offset: 0,
            width,
            height,
        }
    }

    /// Scroll the viewport to the given cursor such that the cursor is visible. Returns
    /// `true` if the viewport was scrolled.
    pub fn scroll_to_cursor(&mut self, cursor: &Cursor) -> bool {
        let mut scrolled = false;

        // Vertical scrolling.
        if cursor.row() < self.row_offset {
            self.row_offset = cursor.row();
            scrolled = true;
        } else if cursor.row() >= self.row_offset.saturating_add(self.height) {
            self.row_offset = cursor.row() - self.height + 1;
            scrolled = true;
        }

        // Horizontal scrolling.
        if cursor.col() < self.col_offset {
            self.col_offset = cursor.col();
            scrolled = true;
        } else if cursor.col() >= self.col_offset.saturating_add(self.width) {
            self.col_offset = cursor.col() - self.width + 1;
            scrolled = true;
        }

        scrolled
    }

    /// Returns the logical position from a position on the screen.
    pub fn screen_position(&mut self, col: usize, row: usize, gutter: &Gutter) -> (usize, usize) {
        (
            self.col_offset + col - gutter.width(),
            self.row_offset + row,
        )
    }

    /// Return the width of the viewport.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Return the height of the viewport.
    pub fn height(&self) -> usize {
        self.height
    }
}
