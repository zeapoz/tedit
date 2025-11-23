use crate::editor::geometry::{anchor::Anchor, point::Point};

/// A rectangle.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub col: usize,
    pub row: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(col: usize, row: usize, width: usize, height: usize) -> Self {
        Self {
            col,
            row,
            width,
            height,
        }
    }

    /// Anchors a new rect to this rect. The new rect will be placed at the given anchor in the
    /// parent rect.
    pub fn anchored_on(self, parent: Rect, anchor: Anchor) -> Rect {
        match anchor {
            Anchor::TopLeft => self.move_to(parent.top_left()),
            Anchor::TopRight => self.move_to(parent.top_right() - Point::new(self.width, 0)),
            Anchor::BottomLeft => self.move_to(parent.bottom_left() - Point::new(0, self.height)),
            Anchor::BottomRight => {
                self.move_to(parent.bottom_right() - Point::new(self.width, self.height))
            }
        }
    }

    /// Splits the rectangle into n equally-sized parts vertically.
    pub fn split_vertically_n(self, n: usize) -> Vec<Rect> {
        if n == 0 {
            return vec![];
        }

        let mut rects = Vec::with_capacity(n);
        let base_width = self.width / n;
        let remainder = self.width % n;
        let mut current_col = self.col;

        for i in 0..n {
            // Distribute the remainder pixels among the first few rectangles.
            let extra = if i < remainder { 1 } else { 0 };
            let rect_width = base_width + extra;

            rects.push(Rect {
                col: current_col,
                row: self.row,
                width: rect_width,
                height: self.height,
            });

            current_col += rect_width;
        }
        rects
    }

    /// Splits the rectangle in two and returns the left and right rectangles.
    pub fn split_vertically(self, ratio: f32) -> (Rect, Rect) {
        let col = (self.width as f32 * ratio).round() as usize;
        let left = Rect::new(self.col, self.row, col, self.height);
        let right = Rect::new(
            self.col + col,
            self.row,
            self.width.saturating_sub(col),
            self.height,
        );
        (left, right)
    }

    /// Splits the rectangle in two and returns the left and right rectangles.
    pub fn split_vertically_exact(self, col: usize) -> (Rect, Rect) {
        let left = Rect::new(self.col, self.row, col, self.height);
        let right = Rect::new(
            self.col + col,
            self.row,
            self.width.saturating_sub(col),
            self.height,
        );
        (left, right)
    }

    /// Splits the rectangle into n equally-sized parts horizontally.
    pub fn split_horizontally_n(self, n: usize) -> Vec<Rect> {
        if n == 0 {
            return vec![];
        }

        let mut rects = Vec::with_capacity(n);
        let base_height = self.height / n;
        let remainder = self.height % n;
        let mut current_row = self.row;

        for i in 0..n {
            // Distribute the remainder pixels among the first few rectangles.
            let extra = if i < remainder { 1 } else { 0 };
            let rect_height = base_height + extra;

            rects.push(Rect {
                col: self.col,
                row: current_row,
                width: self.width,
                height: rect_height,
            });

            current_row += rect_height;
        }
        rects
    }

    /// Splits the rectangle in two and returns the top and bottom rectangles.
    pub fn split_horizontally(self, ratio: f32) -> (Rect, Rect) {
        let row = (self.height as f32 * ratio).round() as usize;
        let top = Rect::new(self.col, self.row, self.width, row);
        let bottom = Rect::new(
            self.col,
            self.row + row,
            self.width,
            self.height.saturating_sub(row),
        );
        (top, bottom)
    }

    /// Splits the rectangle in two and returns the top and bottom rectangles.
    pub fn split_horizontally_exact(self, row: usize) -> (Rect, Rect) {
        let top = Rect::new(self.col, self.row, self.width, row);
        let bottom = Rect::new(
            self.col,
            self.row + row,
            self.width,
            self.height.saturating_sub(row),
        );
        (top, bottom)
    }

    /// Moves this rect to another position.
    pub fn move_to(mut self, Point { col, row }: Point) -> Rect {
        self.col = col;
        self.row = row;
        self
    }

    /// Moves this rect by the given offset.
    pub fn offset(mut self, col: isize, row: isize) -> Rect {
        self.col = (self.col as isize + col) as usize;
        self.row = (self.row as isize + row) as usize;
        self
    }

    /// Returns the size of the rect.
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Returns the top left point of the rect.
    pub fn top_left(&self) -> Point {
        Point::new(self.col, self.row)
    }

    /// Returns the top right point of the rect.
    pub fn top_right(&self) -> Point {
        Point::new(self.col + self.width, self.row)
    }

    /// Returns the bottom left point of the rect.
    pub fn bottom_left(&self) -> Point {
        Point::new(self.col, self.row + self.height)
    }

    /// Returns the bottom right point of the rect.
    pub fn bottom_right(&self) -> Point {
        Point::new(self.col + self.width, self.row + self.height)
    }
}
