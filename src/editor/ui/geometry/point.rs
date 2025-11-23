use std::ops::{Add, Sub};

/// A point in a 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub col: usize,
    pub row: usize,
}

impl Point {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.col + rhs.col, self.row + rhs.row)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.col - rhs.col, self.row - rhs.row)
    }
}
