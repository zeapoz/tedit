use thiserror::Error;

use crate::editor::{
    buffer::{
        BufferEntry,
        modification::{ActionRange, BufferAction, BufferModification},
    },
    pane::{Pane, cursor::CursorMovement},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("attempted to access pane at index {index}, but the len is {len}")]
    IndexOutOfRange { index: usize, len: usize },
}

/// A manager for multiple panes.
#[derive(Debug, Clone)]
pub struct PaneManager {
    /// The next id to assign to a new pane.
    next_id: usize,
    /// The index of the active pane.
    active_pane: usize,
    /// All panes in the manager.
    panes: Vec<Pane>,
}

impl PaneManager {
    pub fn new() -> Self {
        let panes = Vec::new();
        Self {
            next_id: 0,
            panes,
            active_pane: 0,
        }
    }

    /// Opens a new pane and updates all viewports.
    pub fn open_pane(&mut self, buffer: BufferEntry) {
        let pane = Pane::new(self.next_id, buffer);

        self.next_id += 1;
        self.panes.push(pane);

        let new_index = self.panes.len().saturating_sub(1);
        self.set_active(new_index)
            .expect("index is always in range");
    }

    /// Handles a buffer modification and scrolls the viewports of all panes to stay anchored
    /// relative to their view before the modification.
    pub fn handle_buffer_modification(&mut self, modification: &BufferModification) {
        let (scroll_offset, row): (isize, _) = match &modification.action {
            BufferAction::Insert { start, .. } if modification.action.is_insert_newline() => {
                (1, start.row)
            }
            BufferAction::Delete(ActionRange::Line(row)) => (-1, *row),
            _ => return,
        };

        let active_pane = self.active_pane;
        for pane in self
            .iter_mut()
            .filter(|p| p.id != active_pane && p.buffer_id() == modification.buffer_id)
        {
            // Anchor the cursor to the current row.
            if scroll_offset.is_positive() && pane.cursor.row() > row {
                pane.move_cursor(CursorMovement::Down)
            } else if scroll_offset.is_negative() && pane.cursor.row() >= row {
                pane.move_cursor(CursorMovement::Up)
            }

            // TODO: Handle in UI layer.
            // Anchor the viewport if the viewport is farther down than the affected row.
            // if pane.viewport.row_offset > row + pane.viewport.height() {
            //     pane.scroll_vertically(scroll_offset);
            // }
        }
    }

    /// Sets the active pane to the given index.
    pub fn set_active(&mut self, index: usize) -> Result<(), Error> {
        if index >= self.panes.len() {
            return Err(Error::IndexOutOfRange {
                index,
                len: self.panes.len(),
            });
        }

        self.active_pane = index;
        Ok(())
    }

    /// Sets the active pane to the next pane in the list. Looping around to the first
    /// entry if active pane is the last.
    pub fn next_pane(&mut self) {
        let next_index = self.active_pane.saturating_add(1) % self.panes.len();
        self.active_pane = next_index;
    }

    /// Sets the active pane to the previous pane in the list. Looping around to the last
    /// entry if active pane is the first.
    pub fn prev_pane(&mut self) {
        let prev_index = if self.active_pane == 0 {
            self.panes.len().saturating_sub(1)
        } else {
            self.active_pane.saturating_sub(1)
        };
        self.active_pane = prev_index;
    }

    // TODO: Rethink how to make this never panic.
    /// Returns the active pane as an immutable reference.
    pub fn active(&self) -> &Pane {
        &self.panes[self.active_pane]
    }

    /// Returns the active pane as a mutable reference.
    pub fn active_mut(&mut self) -> &mut Pane {
        &mut self.panes[self.active_pane]
    }

    /// Removes a pane from the list.
    pub fn remove(&mut self, index: usize) -> Pane {
        let removed = self.panes.remove(index);

        // TODO: Figure out a better way to handle non existing panes.
        // Make sure that we still have an active pane.
        // if self.is_empty() {
        //     self.add(Pane::default());
        // }

        self.active_pane = self.active_pane.min(self.panes.len().saturating_sub(1));
        removed
    }

    /// Closes the active pane.
    pub fn close_active(&mut self) -> Pane {
        self.remove(self.active_pane)
    }

    /// Iterate through all panes.
    pub fn iter(&self) -> impl Iterator<Item = &Pane> {
        self.panes.iter()
    }

    /// Iterate through all panes with mutable references.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pane> {
        self.panes.iter_mut()
    }

    /// Returns `true` if only one pane has the given buffer id.
    pub fn is_unique(&self, buffer_id: usize) -> bool {
        self.iter().filter(|p| p.buffer_id() == buffer_id).count() == 1
    }

    /// Returns the number of panes in the manager.
    pub fn num_panes(&self) -> usize {
        self.panes.len()
    }

    /// Returns the index of the active pane.
    pub fn active_pane(&self) -> usize {
        self.active_pane
    }
}
