use crate::editor::viewport::Viewport;

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

    /// Inserts a character at the given index. Returns `true` if the character was inserted,
    /// `false` otherwise.
    pub fn insert_char(&mut self, index: usize, c: char) -> bool {
        if index > self.len {
            return false;
        } else if index == self.len {
            self.append_char(c);
            return true;
        }

        self.text.insert(index, c);
        self.len += 1;
        true
    }

    /// Appends a character to the end of the row.
    pub fn append_char(&mut self, c: char) {
        self.text.push(c);
        self.len += 1;
    }

    /// Deletes a character at the given index. Returns `true` if the character was deleted,
    /// `false` otherwise.
    pub fn delete_char(&mut self, index: usize) -> bool {
        if index >= self.len {
            return false;
        }
        self.text.remove(index);
        self.len -= 1;
        true
    }

    /// Splits the row at the given index and returns a tuple containing the parts.
    pub fn split_at(&self, index: usize) -> (Self, Self) {
        let (left, right) = self.text.split_at(index);
        (Row::new(left), Row::new(right))
    }

    /// Appends a row to the end of this row.
    pub fn append_row(&mut self, row: &Self) {
        self.text.push_str(&row.text);
        self.len += row.len;
    }

    /// Returns a `Vec` of characters that should be visible on screen given a [`Viewport`].
    pub fn visible_chars(&self, viewport: &Viewport) -> Vec<char> {
        self.chars()
            .skip(viewport.col_offset)
            .take(viewport.width())
            .collect()
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
