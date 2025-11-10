#[derive(Debug)]
pub struct Row {
    /// The text of the row.
    text: String,
    /// The length of the row.
    len: usize,
}

impl Row {
    /// Returns a new row with the given text.
    pub fn new<S: Into<String>>(s: S) -> Self {
        let text = s.into();
        let len = text.len();
        Self { text, len }
    }

    /// Returns the text of the row.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the length of the row.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns an iterator over the characters of the row.
    pub fn chars(&self) -> impl Iterator<Item = char> {
        self.text.chars()
    }
}
