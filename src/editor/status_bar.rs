use crossterm::style::Stylize;

use crate::editor::{Mode, Result, backend::TerminalBackend, buffer::Buffer, cursor::Cursor};

#[derive(Debug)]
pub struct StatusBar {
    /// The height of the status bar.
    height: usize,
}

impl StatusBar {
    const DEFAULT_HEIGHT: usize = 1;

    /// Returns the height of the status bar.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Renders the status bar.
    pub fn render(
        &self,
        mode: Mode,
        buffer: &Buffer,
        cursor: &Cursor,
        backend: &TerminalBackend,
    ) -> Result<()> {
        let mode_string = mode.to_string();
        let file_name = buffer.file_name().bold();

        let dirty_marker = if buffer.dirty {
            "*".bold().to_string()
        } else {
            " ".into()
        };

        let (cursor_col, cursor_row) = cursor.position();
        let cursor_position = format!("{}:{}", cursor_row + 1, cursor_col + 1);

        let status = format!("{mode_string} {file_name}{dirty_marker} {cursor_position}");

        backend.write(&status)
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            height: Self::DEFAULT_HEIGHT,
        }
    }
}
