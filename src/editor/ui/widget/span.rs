use crate::editor::ui::{frame::Cell, style::Style, widget::Widget};

/// A string with a particular style.
#[derive(Debug, Default, Clone)]
pub struct Span {
    /// The text of the span.
    pub text: String,
    /// The style of the span.
    pub style: Style,
}

impl Span {
    pub fn new(str: &str) -> Self {
        Self {
            text: str.to_string(),
            style: Style::default(),
        }
    }

    /// Sets the style of the span.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style.apply(style);
        self
    }
}

impl Widget for Span {
    fn as_cells(&mut self) -> Vec<Cell> {
        self.text
            .chars()
            .map(|c| Cell::new(c).with_style(self.style))
            .collect()
    }

    fn width(&self) -> usize {
        self.text.len()
    }

    fn set_width(&mut self, width: Option<usize>) {
        if let Some(width) = width {
            self.text.truncate(width);
        }
    }

    fn set_style(&mut self, style: Style) {
        self.style.apply(style);
    }
}
