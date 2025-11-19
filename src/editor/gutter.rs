use crate::editor::renderer::{Renderable, RenderingContext, frame::Span, viewport::Viewport};

#[derive(Debug, Clone, Copy)]
pub struct Gutter {
    width: usize,
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
    fn render(&self, ctx: &RenderingContext, mut viewport: Viewport<'_>) {
        for row in 0..viewport.height() {
            // Reserve two spaces at the end of the gutter.
            let padding_width = self.width.saturating_sub(2);
            let document_row = ctx.document.viewport_row_offset() + row;
            let s = format!(
                "{:>width$}  ",
                document_row.saturating_add(1),
                width = padding_width
            );

            viewport.put_span(0, row, Span::new(&s));
        }
    }
}
