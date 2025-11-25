use crate::editor::ui::{frame::Cell, style::Style, widget::Widget};

/// A widget representing a fixed-width whitespace separator.
#[derive(Debug, Clone)]
pub struct WhitespaceSeparator {
    /// The width of the separator.
    pub width: usize,
    /// The style of the separator.
    pub style: Style,
}

impl Default for WhitespaceSeparator {
    fn default() -> Self {
        Self {
            width: Self::DEFAULT_WIDTH,
            style: Default::default(),
        }
    }
}

impl WhitespaceSeparator {
    pub const DEFAULT_WIDTH: usize = 1;

    pub fn new(width: usize) -> Self {
        Self {
            width,
            style: Style::default(),
        }
    }
}

impl Widget for WhitespaceSeparator {
    fn as_cells(&mut self) -> Vec<Cell> {
        std::iter::repeat_n(Cell::default().with_style(self.style), self.width).collect()
    }

    fn width(&self) -> usize {
        self.width
    }

    fn set_width(&mut self, width: Option<usize>) {
        self.width = width.unwrap_or_default();
    }

    fn set_style(&mut self, style: Style) {
        self.style.apply(style);
    }
}
