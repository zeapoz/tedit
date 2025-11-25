use crate::editor::ui::{frame::Cell, style::Style};

pub mod container;
pub mod separator;
pub mod span;

/// A trait for every object that can be converted into a vector of cells.
pub trait Widget {
    /// Converts the widget into a vector of cells.
    fn as_cells(&mut self) -> Vec<Cell>;

    /// Returns the width of the widget.
    fn width(&self) -> usize;

    /// Sets the width of the widget. If `None`, the widget will be flexible.
    fn set_width(&mut self, width: Option<usize>);

    /// Styles the widget.
    fn set_style(&mut self, style: Style);
}
