use crate::editor::ui::{frame::Cell, style::Style, widget::Widget};

/// A string with a particular style.
#[derive(Debug, Default, Clone, Copy)]
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
}

impl<'a> Widget for Span<'a> {
    fn into_cells(self) -> Vec<Cell> {
        self.str
            .chars()
            .map(|c| Cell::new(c).with_style(self.style))
            .collect()
    }
    
    fn width(&self) -> usize {
        self.str.len()
    }

    fn with_width(self, _width: Option<usize>) -> Self {
        self
    }

    fn with_style(mut self, style: Style) -> Self {
        self.style.apply(style);
        self
    }
}
