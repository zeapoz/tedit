use crate::editor::ui::{frame::Cell, style::Style, widget::Widget};

/// A widget representing a fixed-width whitespace separator.
#[derive(Debug, Default, Clone)]
pub struct WhitespaceSeparator {
    /// The width of the separator.
    pub width: usize,
    /// The style of the separator.
    pub style: Style,
}

impl WhitespaceSeparator {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            style: Style::default(),
        }
    }
}

impl Widget for WhitespaceSeparator {
    fn into_cells(self) -> Vec<Cell> {
        std::iter::repeat_n(Cell::default().with_style(self.style), self.width).collect()
    }
    
    fn width(&self) -> usize {
        self.width
    }
    
    fn with_width(mut self, width: Option<usize>) -> Self {
        self.width = width.unwrap_or_default();
        self
    }

    fn with_style(mut self, style: Style) -> Self {
        self.style.apply(style);
        self
    }
}
