/// A rectangle on the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub col: usize,
    pub row: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(col: usize, row: usize, width: usize, height: usize) -> Self {
        Self {
            col,
            row,
            width,
            height,
        }
    }

    /// Returns the last row in the rectangle. This is equivalent to `row + height - 1`.
    pub fn last_row(&self) -> usize {
        (self.row + self.height).saturating_sub(1)
    }

    /// Returns an iterator over all rows in the rectangle.
    pub fn rows(&self) -> impl Iterator<Item = usize> {
        self.row..self.row + self.height
    }

    /// Returns an iterator over all columns in the rectangle.
    pub fn cols(&self) -> impl Iterator<Item = usize> {
        self.col..self.col + self.width
    }
}
