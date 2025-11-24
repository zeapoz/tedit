use crate::editor::ui::{frame::Cell, style::Style};

pub mod container;
pub mod separator;
pub mod span;

/// A trait for every object that can be converted into a vector of cells.
pub trait Widget {
    /// Converts the widget into a vector of cells.
    fn into_cells(self) -> Vec<Cell>;

    /// Returns the width of the widget.
    fn width(&self) -> usize;

    /// Sets the width of the widget. If `None`, the widget will be flexible.
    fn with_width(self, width: Option<usize>) -> Self;

    /// Styles the widget.
    fn with_style(self, style: Style) -> Self;
}
