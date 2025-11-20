use crate::editor::renderer::style::Style;

pub mod diff;

/// A line of text containing multiple spans.
#[derive(Debug, Clone)]
pub struct Line<'a> {
    /// The spans of the line.
    pub spans: Vec<Span<'a>>,
    /// The number of columns in the line.
    pub width: usize,
    /// The style of the line. This is sets the default style for all spans. Spans with their own
    /// style will override this.
    pub style: Style,
}

impl<'a> Line<'a> {
    pub fn new(cols: usize, spans: Vec<Span<'a>>) -> Self {
        Self {
            spans,
            width: cols,
            style: Style::default(),
        }
    }

    /// Sets the style of the line.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Inserts a given separator between each span.
    pub fn with_separator(mut self, separator: &'a str) -> Self {
        if self.spans.is_empty() {
            return self;
        }

        let mut new_spans = Vec::new();

        let mut span_iter = self.spans.iter_mut();
        let prev_span = span_iter.next().expect("spans was unexpectedly empty");
        new_spans.push(*prev_span);

        for span in span_iter {
            let searator_span = Span::new(separator).with_style(self.style);
            new_spans.push(searator_span);
            new_spans.push(*span);
        }

        self.spans = new_spans;
        self
    }

    /// Returns the line as a vector of cells, padded to `self.col`.
    pub fn as_cells(&self) -> Vec<Cell> {
        let mut cells: Vec<Cell> = self
            .spans
            .as_slice()
            .iter()
            .flat_map(|&(mut s)| {
                s.style = s.style.merge(self.style);
                s.as_cells()
            })
            .collect();

        // Pad with default cells until reaching `self.col`.
        if cells.len() < self.width {
            cells.resize(self.width, Cell::default().with_style(self.style));
        }

        cells
    }
}

/// A string with a particular style.
#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    /// The string of the span.
    pub str: &'a str,
    /// The style of the span.
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
