use crate::editor::geometry::point::Point;

/// Represents a change to a buffer with a specific buffer id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferModification {
    pub buffer_id: usize,
    pub action: BufferAction,
}

impl BufferModification {
    pub fn new(buffer_id: usize, action: BufferAction) -> Self {
        Self { buffer_id, action }
    }
}

/// A modification to a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferAction {
    /// An insert operation containing the insertet text and the position it was inserted at.
    Insert { start: Point, text: String },
    /// A delete operation containing the range of text that was deleted.
    Delete(ActionRange),
    /// An append operation containing the indexes of the rows of text that operated on.
    AppendLineToLine { from: usize, to: usize },
    /// Represents that the buffer was not been modified.
    None,
}

impl BufferAction {
    /// Returns true if the action is an insert newline action.
    pub fn is_insert_newline(&self) -> bool {
        matches!(self, BufferAction::Insert { text, .. } if text == "\n")
    }
}

/// Descrives the range of text that was affected by a buffer action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionRange {
    /// A full line identified by the row index.
    Line(usize),
    /// A range of characters identified by the start and end point.
    PointToPoint { from: Point, to: Point },
}
