use std::cell::RefCell;

use crate::editor::ui::{
    frame::{Cell, Frame},
    geometry::rect::Rect,
    text::{Line, Span},
};

/// A viewport of a rectangular region of the terminal that can be written to.
#[derive(Debug, Clone)]
pub struct Viewport<'a> {
    rect: Rect,
    frame: &'a RefCell<Frame>,
}

impl<'a> Viewport<'a> {
    pub fn new(rect: Rect, frame: &'a RefCell<Frame>) -> Self {
        Self { rect, frame }
    }

    /// Merges a [`Cell`] with one in the frame. If the position is out of bounds, it will be
    /// ignored.
    pub fn merge_cell(&mut self, col: usize, row: usize, cell: Cell) {
        if col >= self.rect.width || row >= self.rect.height {
            return;
        }
        let mut frame = self.frame.borrow_mut();
        let frame_cell = frame.cell_mut(col + self.rect.col, row + self.rect.row);
        frame_cell.apply(&cell);
    }

    /// Puts a new span in the given position. If any cell is out of bounds, it will be ignored.
    pub fn put_span(&mut self, col: usize, row: usize, span: Span) {
        let cells = span.as_cells();
        for (i, cell) in cells.into_iter().enumerate() {
            self.merge_cell(col + i, row, cell);
        }
    }

    /// Puts a new line in the given position. If the position is out of bounds, it will be
    /// ignored.
    pub fn put_line(&mut self, row: usize, mut line: Line) {
        let cells = line.as_cells();
        for (i, cell) in cells.into_iter().enumerate() {
            self.merge_cell(i, row, cell);
        }
    }

    /// Fills the viewport with the given cell.
    pub fn fill(&mut self, cell: Cell) {
        let cells = self.rect.width * self.rect.height;
        for i in 0..cells {
            self.merge_cell(i % self.rect.width, i / self.rect.width, cell);
        }
    }

    /// Returns a sub rect of the viewport. Returns `None` if the given rect is not a valid sub
    /// rect of this viewport.
    pub fn sub_rect(&mut self, rect: Rect) -> Option<Viewport<'a>> {
        if rect.col < self.rect.col
            || rect.row < self.rect.row
            || rect.col + rect.width > self.rect.col + self.rect.width
            || rect.row + rect.height > self.rect.row + self.rect.height
        {
            return None;
        }

        Some(Viewport::new(rect, self.frame))
    }

    /// Splits the viewport in two and returns the left and right viewports. Calculates the parts
    /// based on the ratio given.
    pub fn split_horizontally(&mut self, ratio: f32) -> (Viewport<'a>, Viewport<'a>) {
        let (left, right) = self.rect.split_vertically(ratio);
        let left = Viewport::new(left, self.frame);
        let right = Viewport::new(right, self.frame);
        (left, right)
    }

    /// Splits the viewport in two and returns the left and right viewports.
    pub fn split_horizontally_exact(&mut self, col: usize) -> (Viewport<'a>, Viewport<'a>) {
        let (left, right) = self.rect.split_vertically_exact(col);
        let left = Viewport::new(left, self.frame);
        let right = Viewport::new(right, self.frame);
        (left, right)
    }

    /// Returns the width of the viewport.
    pub fn width(&self) -> usize {
        self.rect.width
    }

    /// Returns the height of the viewport.
    pub fn height(&self) -> usize {
        self.rect.height
    }

    /// Returns the rect of the viewport.
    pub fn rect(&self) -> Rect {
        self.rect
    }
}
