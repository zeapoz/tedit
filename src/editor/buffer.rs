use std::path::{Path, PathBuf};

use crate::editor::{Result, viewport::Viewport};

#[derive(Debug, Default)]
pub struct Buffer {
    /// The rows of the buffer.
    rows: Vec<String>,
    /// The path of the file this buffer represents.
    filepath: Option<PathBuf>,
}

impl Buffer {
    /// Open a new file and read its contents.
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(&path)?;

        Ok(Self {
            rows: contents.split("\n").map(|s| s.to_string()).collect(),
            filepath: Some(path.as_ref().to_path_buf()),
        })
    }

    /// Returns the text of the buffer that should be visible on screen.
    pub fn visible_text(&self, viewport: &Viewport) -> String {
        let mut display_text = Vec::with_capacity(viewport.height() as usize);

        for row in self
            .rows
            .iter()
            .skip(viewport.row_offset as usize)
            .take(viewport.height() as usize)
        {
            let visible_line = row
                .chars()
                .skip(viewport.col_offset as usize)
                .take(viewport.width() as usize)
                .collect::<String>();

            display_text.push(visible_line);
        }

        display_text.join("\r\n")
    }

    /// Returns the full text of the buffer.
    pub fn text(&self) -> String {
        self.rows.join("\r\n")
    }
}
