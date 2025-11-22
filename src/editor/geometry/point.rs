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
