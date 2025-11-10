use crate::editor::buffer::Buffer;

#[derive(Debug, Default)]
pub struct Cursor {
    /// The column of the cursor.
    col: usize,
    /// The row of the cursor.
    row: usize,
    /// The last remembered column of the cursor. This value takes precedence over the current
    /// column when moving the cursor vertically and updates on horizontal movements.
    last_col: usize,
}

impl Cursor {
    /// Returns the row of the cursor.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the column of the cursor.
    pub fn col(&self) -> usize {
        self.col
    }

    /// Returns the position of the cursor.
    pub fn position(&self) -> (usize, usize) {
        (self.col, self.row)
    }

    /// Moves the cursor to the given position.
    pub fn move_to(&mut self, col: usize, row: usize, buffer: &Buffer) {
        if let Some(buffer_row) = buffer.row(row) {
            self.row = row;
            self.col = col.min(buffer_row.len());
            self.last_col = self.col;
        }
    }

    /// Moves the cursor one column to the left.
    pub fn move_left(&mut self, buffer: &Buffer) {
        if let Some(row) = buffer.row(self.row) {
            self.col = row.len().min(self.col.saturating_sub(1));
            self.last_col = self.col;
        }
    }

    /// Moves the cursor one column to the right.
    pub fn move_right(&mut self, buffer: &Buffer) {
        if let Some(row) = buffer.row(self.row) {
            self.col = row.len().min(self.col.saturating_add(1));
            self.last_col = self.col;
        }
    }

    /// Moves the cursor one row up.
    pub fn move_up(&mut self, buffer: &Buffer) {
        self.row = self.row.saturating_sub(1);
        if let Some(row) = buffer.row(self.row) {
            if self.col > row.len() {
                self.col = row.len();
            } else if self.col < self.last_col {
                self.col = self.last_col.min(row.len());
            }
        }
    }

    /// Moves the cursor one row down.
    pub fn move_down(&mut self, buffer: &Buffer) {
        let next_row = self.row.saturating_add(1);
        if let Some(row) = buffer.row(next_row) {
            self.row = next_row;

            if self.col > row.len() {
                self.col = row.len();
            } else if self.col < self.last_col {
                self.col = self.last_col.min(row.len());
            }
        }
    }

    /// Moves the cursor to the end of the current row.
    pub fn move_to_end_of_row(&mut self, buffer: &Buffer) {
        if let Some(row) = buffer.row(self.row) {
            self.col = row.len();
            self.last_col = row.len();
        }
    }

    /// Moves the cursor to the start of the current row.
    pub fn move_to_start_of_row(&mut self) {
        self.col = 0;
        self.last_col = 0;
    }

    pub fn move_to_start_of_next_row(&mut self, buffer: &Buffer) {
        let next_row = self.row.saturating_add(1);
        if let Some(row) = buffer.row(next_row) {
            self.row = next_row;
            self.col = 0;
            self.last_col = 0;
        }
    }
}
