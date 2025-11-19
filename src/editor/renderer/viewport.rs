use crate::editor::renderer::{
    Rect,
    frame::{Cell, Frame, Span},
};

/// A viewport of a rectangular region of the terminal that can be written to.
#[derive(Debug)]
pub struct Viewport<'a> {
    rect: Rect,
    frame: &'a mut Frame,
}

impl<'a> Viewport<'a> {
    pub fn new(rect: Rect, frame: &'a mut Frame) -> Self {
        Self { rect, frame }
    }

    /// Puts a new cell in the given position. If the position is out of bounds, it will be
    /// ignored.
    pub fn put_cell(&mut self, col: usize, row: usize, cell: Cell) {
        if col >= self.rect.width || row >= self.rect.height {
            return;
        }
        self.frame
            .put_cell(col + self.rect.col, row + self.rect.row, cell);
    }

    /// Puts a new span in the given position. If any cell is out of bounds, it will be ignored.
    pub fn put_span(&mut self, col: usize, row: usize, span: Span) {
        let cells = span.as_cells();
        for (i, cell) in cells.into_iter().enumerate() {
            self.put_cell(col + i, row, cell);
        }
    }

    /// Returns the width of the viewport.
    pub fn width(&self) -> usize {
        self.rect.width
    }

    /// Returns the height of the viewport.
    pub fn height(&self) -> usize {
        self.rect.height
    }
}
