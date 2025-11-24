use crate::editor::ui::{geometry::point::Point, style::Style};

/// A frame is a rectangular region of the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub char: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            // Use a space as the default character to overwrite the previous character.
            char: ' ',
            style: Default::default(),
        }
    }
}

impl Cell {
    pub fn new(char: char) -> Self {
        Self {
            char,
            style: Style::default(),
        }
    }

    /// Sets the style of the cell.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Applies the given cell over the current cell.
    pub fn apply(&mut self, other: &Cell) {
        self.char = other.char;
        self.style.force_apply(other.style);
    }
}

/// Represents a single frame of the cells to render.
#[derive(Debug, Clone)]
pub struct Frame {
    width: usize,
    height: usize,
    /// The cells of the frame in row-major order.
    cells: Vec<Cell>,
    cursor_position: Option<Point>,
}

impl Frame {
    /// Creates a new frame with the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::default(); width * height];
        Self {
            width,
            height,
            cells,
            cursor_position: None,
        }
    }

    /// Puts a new cell in the given position.
    pub fn put_cell(&mut self, col: usize, row: usize, cell: Cell) {
        let index = row * self.width + col;
        self.cells[index] = cell;
    }

    pub fn cell_mut(&mut self, col: usize, row: usize) -> &mut Cell {
        let index = row * self.width + col;
        &mut self.cells[index]
    }

    /// Sets the cursor position for this frame.
    pub fn set_cursor_position(&mut self, point: Point) {
        self.cursor_position = Some(point);
    }

    /// Hides the cursor for this frame.
    pub fn hide_cursor(&mut self) {
        self.cursor_position = None;
    }

    /// Returns the cursor position for this frame.
    pub fn cursor_position(&self) -> Option<Point> {
        self.cursor_position
    }

    /// Returns this frame as a vector of rows.
    pub fn rows(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks_exact(self.width)
    }
}

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
