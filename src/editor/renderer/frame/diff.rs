use crate::editor::renderer::frame::{Cell, Frame};

/// A cell that also knows its location in the frame.
pub struct RowDiff<'a> {
    pub col: usize,
    pub row: usize,
    pub cell: &'a Cell,
}

impl<'a> RowDiff<'a> {
    pub fn new(col: usize, row: usize, cell: &'a Cell) -> Self {
        Self { col, row, cell }
    }
}

/// A diff between two frames.
pub struct FrameDiff<'a> {
    /// The cells that have changed between the two frames.
    pub cells: Vec<RowDiff<'a>>,
}

impl<'a> FrameDiff<'a> {
    /// Returns the diff between two frames.
    pub fn compute(prev: &Frame, next: &'a Frame) -> Self {
        let mut cells = Vec::new();

        for row in 0..next.height {
            for col in 0..next.width {
                let idx = row * next.width + col;
                if prev.cells[idx] != next.cells[idx] {
                    cells.push(RowDiff::new(col, row, &next.cells[idx]));
                }
            }
        }

        Self { cells }
    }
}
