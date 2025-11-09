#[derive(Debug, Default)]
pub struct Cursor {
    /// The column of the cursor.
    col: u16,
    /// The row of the cursor.
    row: u16,
}

impl Cursor {
    /// Returns the row of the cursor.
    pub fn row(&self) -> u16 {
        self.row
    }

    /// Returns the column of the cursor.
    pub fn col(&self) -> u16 {
        self.col
    }

    /// Returns the position of the cursor.
    pub fn position(&self) -> (u16, u16) {
        (self.col, self.row)
    }

    /// Moves the cursor to the given position.
    pub fn move_to(&mut self, col: u16, row: u16) {
        self.col = col;
        self.row = row;
    }

    /// Moves the cursor one column to the left.
    pub fn move_left(&mut self) {
        self.col = self.col.saturating_sub(1);
    }

    /// Moves the cursor one column to the right.
    pub fn move_right(&mut self) {
        self.col = self.col.saturating_add(1);
    }

    /// Moves the cursor one row up.
    pub fn move_up(&mut self) {
        self.row = self.row.saturating_sub(1);
    }

    /// Moves the cursor one row down.
    pub fn move_down(&mut self) {
        self.row = self.row.saturating_add(1);
    }
}
