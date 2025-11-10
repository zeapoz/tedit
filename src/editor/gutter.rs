use crate::editor::{Result, backend::TerminalBackend};

#[derive(Debug)]
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

    /// Renders the gutter for the given screen row and viewport.
    pub fn render_row(&self, row: usize, backend: &TerminalBackend) -> Result<()> {
        // Reserve two spaces at the end of the gutter.
        let padding_width = self.width.saturating_sub(2);

        let s = format!("{:>width$}  ", row.saturating_add(1), width = padding_width);
        backend.write(&s)
    }
}
