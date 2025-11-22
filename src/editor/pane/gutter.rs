use crate::editor::renderer::{
    Renderable, RenderingContext,
    frame::{Line, Span},
    style::{Color, Style},
    viewport::Viewport,
};

// TODO: Make this adapt to the current buffer/be configurable.
/// The width of the gutter.
const GUTTER_WIDTH: usize = 6;

#[derive(Debug, Clone, Copy)]
pub struct Gutter {
    width: usize,
}

impl Default for Gutter {
    fn default() -> Self {
        Self {
            width: GUTTER_WIDTH,
        }
    }
}

impl Gutter {
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    /// Returns the width of the gutter.
    pub fn width(&self) -> usize {
        self.width
    }
}

impl Renderable for Gutter {
    fn render(&self, ctx: &RenderingContext, mut viewport: Viewport) {
        let active_pane = ctx.pane_manager.active();
        let cursor_row = active_pane.cursor_position().1;
        for row in 0..viewport.height() {
            // Reserve two spaces at the end of the gutter.
            let padding_width = self.width.saturating_sub(2);
            let pane_row = active_pane.viewport.row_offset + row;
            let s = format!(
                "{:>width$}  ",
                pane_row.saturating_add(1),
                width = padding_width
            );

            let style = if cursor_row == row {
                Style::new().bold().fg(Color::DarkYellow)
            } else {
                Style::default()
            };

            viewport.put_line(
                row,
                Line::new(viewport.width(), vec![Span::new(&s)]).with_style(style),
            );
        }
    }
}
