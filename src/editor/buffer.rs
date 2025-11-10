use std::path::{Path, PathBuf};

use crate::editor::{Result, buffer::row::Row, viewport::Viewport};

mod row;

#[derive(Debug, Default)]
pub struct Buffer {
    /// The rows of the buffer.
    rows: Vec<Row>,
    /// The path of the file this buffer represents.
    filepath: Option<PathBuf>,
}

impl Buffer {
    /// Open a new file and read its contents.
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(&path)?;

        Ok(Self {
            rows: contents.split("\n").map(Row::new).collect(),
            filepath: Some(path.as_ref().to_path_buf()),
        })
    }

    /// Returns the text of the buffer that should be visible on screen.
    pub fn visible_text(&self, viewport: &Viewport) -> String {
        let mut display_text = Vec::with_capacity(viewport.height());

        for row in self
            .rows
            .iter()
            .skip(viewport.row_offset)
            .take(viewport.height())
        {
            let visible_line = row
                .chars()
                .skip(viewport.col_offset)
                .take(viewport.width())
                .collect::<String>();

            display_text.push(visible_line);
        }

        display_text.join("\r\n")
    }

    /// Returns the full text of the buffer.
    pub fn text(&self) -> String {
        self.rows
            .iter()
            .map(|r| r.text())
            .collect::<Vec<&str>>()
            .join("\r\n")
    }

    /// Returns the row at the given index or `None` if the index is out of bounds.
    pub fn row(&self, row: usize) -> Option<&Row> {
        self.rows.get(row)
    }
}
