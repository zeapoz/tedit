use crate::editor::cursor::Cursor;

#[derive(Debug)]
pub struct Viewport {
    /// The column offset of the viewport.
    pub col_offset: u16,
    /// The row offset of the viewport.
    pub row_offset: u16,
    /// The width of the viewport.
    width: u16,
    /// The height of the viewport.
    height: u16,
}

impl Viewport {
    /// Returns a new viewport with the given dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            col_offset: 0,
            row_offset: 0,
            width,
            height,
        }
    }

    /// Scroll the viewport to the given cursor such that the cursor is always visible. Returns
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

    /// Returns the screen position of the cursor relative to the viewport.
    pub fn cursor_screen_position(&mut self, cursor: &Cursor) -> (u16, u16) {
        (
            cursor.col() - self.col_offset,
            cursor.row() - self.row_offset,
        )
    }

    /// Return the width of the viewport.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Return the height of the viewport.
    pub fn height(&self) -> u16 {
        self.height
    }
}
