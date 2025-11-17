use crate::editor::{
    backend::{self, RenderingBackend},
    renderer::{Rect, Renderable, RenderingContext},
};

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
    fn render(
        &self,
        ctx: &RenderingContext,
        rect: Rect,
        backend: &mut RenderingBackend,
    ) -> Result<(), backend::Error> {
        for row in rect.rows() {
            backend.move_cursor(rect.col, row)?;

            // Reserve two spaces at the end of the gutter.
            let padding_width = self.width.saturating_sub(2);
            let document_row = ctx.document.viewport_row_offset() + row;
            let s = format!(
                "{:>width$}  ",
                document_row.saturating_add(1),
                width = padding_width
            );
            backend.write(&s)?;
        }
        Ok(())
    }
}
