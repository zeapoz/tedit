use thiserror::Error;

use crate::editor::{
    buffer::BufferEntry,
    pane::{Pane, viewport::Viewport},
    renderer::{Rect, Renderable, RenderingContext, viewport::Viewport as RenderingViewport},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("attempted to access pane at index {index}, but the len is {len}")]
    IndexOutOfRange { index: usize, len: usize },
}

/// A basic pane layout that organizes panes into equally-sized bars.
#[derive(Debug, Clone)]
struct BarsLayout {
    pub rects: Vec<Rect>,
}

impl BarsLayout {
    /// Calculate the layout confitguration based on the number of panes and the size of the pane
    /// manager rectangle.
    pub fn calculate_layout(panes: &[Pane], rect: Rect) -> BarsLayout {
        Self {
            rects: rect.split_vertically_n(panes.len()),
        }
    }
}

/// A manager for multiple panes.
#[derive(Debug, Clone)]
pub struct PaneManager {
    /// All panes in the manager.
    panes: Vec<Pane>,
    /// The index of the active pane.
    active_pane: usize,
    /// The size of the pane manager. This is used to calculate the size of the viewports.
    rect: Rect,
    /// The layout of the panes.
    layout: BarsLayout,
}

impl PaneManager {
    pub fn new(rect: Rect) -> Self {
        let panes = Vec::new();
        let layout = BarsLayout::calculate_layout(&panes, rect);
        Self {
            panes,
            active_pane: 0,
            rect,
            layout,
        }
    }

    /// Opens a new pane.
    pub fn open_pane(&mut self, buffer: BufferEntry, viewport: Viewport) {
        let pane = Pane::new(buffer, viewport);
        self.add(pane);
    }

    /// Adds a new [`Pane`] and sets it as the active pane.
    pub fn add(&mut self, pane: Pane) {
        self.panes.push(pane);

        let new_index = self.panes.len().saturating_sub(1);
        self.set_active(new_index)
            .expect("index is always in range");

        self.update_viewports();
    }

    /// Updates the viewports of all panes.
    fn update_viewports(&mut self) {
        self.layout = BarsLayout::calculate_layout(&self.panes, self.rect);
        for (pane, rect) in self.panes.iter_mut().zip(self.layout.rects.iter()) {
            pane.update_viewport(rect.width, rect.height);
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
        self.update_viewports();

        removed
    }

    /// Closes the active pane.
    pub fn close_active(&mut self) -> Pane {
        self.remove(self.active_pane)
    }

    /// Returns the screen position of the cursor in the active pane.
    pub fn active_cursor_screen_position(&self) -> (usize, usize) {
        let active_rect = self.layout.rects[self.active_pane];
        let (mut col, mut row) = self.active().relaive_cursor_position();
        col = col.saturating_add(active_rect.col);
        row = row.saturating_add(active_rect.row);
        (col, row)
    }

    /// Iterate through all panes.
    pub fn iter(&self) -> impl Iterator<Item = &Pane> {
        self.panes.iter()
    }

    /// Iterate through all panes with mutable references.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Pane> {
        self.panes.iter_mut()
    }

    /// Pop the last pane from the list.
    pub fn pop(&mut self) -> Option<Pane> {
        self.panes.pop()
    }

    /// Returns `true` if the pane list is empty.
    pub fn is_empty(&self) -> bool {
        self.panes.is_empty()
    }

    /// Returns `true` if only one pane has the given buffer id.
    pub fn is_unique(&self, buffer_id: usize) -> bool {
        self.iter().filter(|p| p.buffer_id() == buffer_id).count() == 1
    }

    /// Returns the number of panes in the manager.
    pub fn num_panes(&self) -> usize {
        self.panes.len()
    }
}

impl Renderable for PaneManager {
    fn render(&self, ctx: &RenderingContext, mut viewport: RenderingViewport) {
        for (pane, rect) in self.panes.iter().zip(self.layout.rects.iter()) {
            let pane_viewport = viewport.sub_rect(*rect).unwrap();
            pane.render(ctx, pane_viewport);
        }
    }
}
