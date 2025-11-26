use crate::editor::buffer::Buffer;

/// All available cursor movements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMovement {
    Left,
    Right,
    Up,
    Down,
    StartOfRow,
    StartOfNextRow,
    EndOfRow,
    StartOfBuffer,
    EndOfBuffer,
    Line(usize),
    Position(usize, usize),
}

#[derive(Debug, Default, Clone, Copy)]
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

    /// Moves the cursor based on the provided [`CursorMovement`].
    pub fn handle_movement(&mut self, movement: CursorMovement, buffer: &Buffer) {
        match movement {
            CursorMovement::Left => self.move_left(buffer),
            CursorMovement::Right => self.move_right(buffer),
            CursorMovement::Up => self.move_up(buffer),
            CursorMovement::Down => self.move_down(buffer),
            CursorMovement::StartOfRow => self.move_to_start_of_row(),
            CursorMovement::StartOfNextRow => self.move_to_start_of_next_row(buffer),
            CursorMovement::EndOfRow => self.move_to_end_of_row(buffer),
            CursorMovement::StartOfBuffer => self.move_to_start_of_buffer(buffer),
            CursorMovement::EndOfBuffer => self.move_to_end_of_buffer(buffer),
            CursorMovement::Line(line) => self.move_to_line(line, buffer),
            CursorMovement::Position(col, row) => self.move_to(col, row, buffer),
        }
    }

    /// moves the cursor to the given position.
    fn move_to(&mut self, col: usize, row: usize, buffer: &Buffer) {
        if let Some(buffer_row) = buffer.row(row) {
            self.row = row;
            self.col = col.min(buffer_row.len());
            self.last_col = self.col;
        }
    }

    /// Moves the cursor to the given line in the buffer. If the line is out of bounds, the cursor
    /// will be moved to the last line.
    fn move_to_line(&mut self, line: usize, buffer: &Buffer) {
        let line = line.saturating_sub(1);
        if let Some(row) = buffer.row(line) {
            self.row = line;
            self.col = self.col.min(row.len());
        } else {
            self.move_to_end_of_buffer(buffer);
        }
    }

    /// Moves the cursor one column to the left.
    fn move_left(&mut self, buffer: &Buffer) {
        if let Some(row) = buffer.row(self.row) {
            self.col = row.len().min(self.col.saturating_sub(1));
            self.last_col = self.col;
        }
    }

    /// Moves the cursor one column to the right.
    fn move_right(&mut self, buffer: &Buffer) {
        if let Some(row) = buffer.row(self.row) {
            self.col = row.len().min(self.col.saturating_add(1));
            self.last_col = self.col;
        }
    }

    /// Moves the cursor one row up.
    fn move_up(&mut self, buffer: &Buffer) {
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
    fn move_down(&mut self, buffer: &Buffer) {
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
    fn move_to_end_of_row(&mut self, buffer: &Buffer) {
        if let Some(row) = buffer.row(self.row) {
            self.col = row.len();
            self.last_col = row.len();
        }
    }

    /// Moves the cursor to the start of the current row.
    fn move_to_start_of_row(&mut self) {
        self.col = 0;
        self.last_col = 0;
    }

    /// Moves the cursor to the start of the next row.
    fn move_to_start_of_next_row(&mut self, buffer: &Buffer) {
        let next_row = self.row.saturating_add(1);
        if buffer.row(next_row).is_some() {
            self.row = next_row;
            self.col = 0;
            self.last_col = 0;
        }
    }

    /// Moves to the start of the buffer.
    fn move_to_start_of_buffer(&mut self, buffer: &Buffer) {
        if let Some(row) = buffer.row(0) {
            self.row = 0;
            self.col = self.col.min(row.len());
        }
    }

    /// Moves to the end of the buffer.
    fn move_to_end_of_buffer(&mut self, buffer: &Buffer) {
        let last_row = buffer.num_lines().saturating_sub(1);
        if let Some(row) = buffer.row(last_row) {
            self.row = last_row;
            self.col = self.col.min(row.len());
        }
    }
}
