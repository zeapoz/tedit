/// A string with a particular style.
pub struct Span<'a> {
    pub str: &'a str,
    pub style: Style,
}

impl<'a> Span<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            style: Style::default(),
        }
    }

    /// Sets the style of the span.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Returns the span as a vector of cells.
    pub fn as_cells(&self) -> Vec<Cell> {
        self.str
            .chars()
            .map(|c| Cell::new(c).with_style(self.style))
            .collect()
    }
}

/// A frame is a rectangular region of the terminal.
#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub char: char,
    pub style: Style,
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
}

/// The style of a single cell.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    bold: bool,
}

impl Style {
    /// Sets the bold style.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
}

/// Represents a single frame of the cells to render.
#[derive(Debug, Clone)]
pub struct Frame {
    width: usize,
    height: usize,
    /// The cells of the frame in row-major order.
    cells: Vec<Cell>,
    cursor_position: Option<(usize, usize)>,
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

    /// Sets the cursor position for this frame.
    pub fn set_cursor_position(&mut self, col: usize, row: usize) {
        self.cursor_position = Some((col, row));
    }

    /// Hides the cursor for this frame.
    pub fn hide_cursor(&mut self) {
        self.cursor_position = None;
    }

    /// Returns the cursor position for this frame.
    pub fn cursor_position(&self) -> Option<(usize, usize)> {
        self.cursor_position
    }

    /// Returns this frame as a vector of rows.
    pub fn rows(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks_exact(self.width)
    }
}
