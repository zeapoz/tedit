use thiserror::Error;

use crate::editor::document::Document;

#[derive(Debug, Error)]
pub enum Error {
    #[error("attempted to access document at index {index}, but the len is {len}")]
    IndexOutOfRange { index: usize, len: usize },
}

/// A manager for multiple documents.
#[derive(Debug, Default, Clone)]
pub struct DocumentManager {
    /// All documents in the manager.
    documents: Vec<Document>,
    /// The index of the active document.
    active_document: usize,
}

impl DocumentManager {
    /// Adds a new [`Document`] and sets it as the active document.
    pub fn add(&mut self, document: Document) {
        self.documents.push(document);

        let new_index = self.documents.len().saturating_sub(1);
        self.set_active(new_index)
            .expect("index is always in range");
    }

    /// Sets the active document to the given index.
    pub fn set_active(&mut self, index: usize) -> Result<(), Error> {
        if index >= self.documents.len() {
            return Err(Error::IndexOutOfRange {
                index,
                len: self.documents.len(),
            });
        }

        self.active_document = index;
        Ok(())
    }

    /// Sets the active document to the next document in the list. Looping around to the first
    /// entry if active document is the last.
    pub fn next_document(&mut self) {
        let next_index = self.active_document.saturating_add(1) % self.documents.len();
        self.active_document = next_index;
    }

    /// Sets the active document to the previous document in the list. Looping around to the last
    /// entry if active document is the first.
    pub fn prev_document(&mut self) {
        let prev_index = if self.active_document == 0 {
            self.documents.len().saturating_sub(1)
        } else {
            self.active_document.saturating_sub(1)
        };
        self.active_document = prev_index;
    }

    // TODO: Rethink how to make this never panic.
    /// Returns the active document as an immutable reference.
    pub fn active(&self) -> &Document {
        &self.documents[self.active_document]
    }

    /// Returns the active document as a mutable reference. If the document list is empty opens up
    /// and returns a reference to a new empty document.
    pub fn active_mut(&mut self) -> &mut Document {
        if self.is_empty() {
            self.add(Document::default());
        }
        &mut self.documents[self.active_document]
    }

    /// Removes a document from the list.
    pub fn remove(&mut self, index: usize) -> Document {
        let removed = self.documents.remove(index);

        // Make sure that we still have an active document.
        if self.is_empty() {
            self.add(Document::default());
        }

        self.active_document = self
            .active_document
            .min(self.documents.len().saturating_sub(1));

        removed
    }

    /// Removes the active document from the list.
    pub fn remove_active(&mut self) -> Document {
        self.remove(self.active_document)
    }

    /// Iterate through all documents.
    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.documents.iter()
    }

    /// Iterate through all documents with mutable references.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Document> {
        self.documents.iter_mut()
    }

    /// Pop the last document from the list.
    pub fn pop(&mut self) -> Option<Document> {
        self.documents.pop()
    }

    /// Returns `true` if the document list is empty.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}
