use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use crate::editor::buffer::{self, Buffer, BufferEntry};

/// A manager for multiple panes.
#[derive(Debug, Default, Clone)]
pub struct BufferManager {
    /// The next id to assign to a new buffer.
    next_id: usize,
    /// All buffers in the manager.
    buffers: Vec<BufferEntry>,
}

impl BufferManager {
    /// Opens an empty file and returns a reference to the buffer.
    pub fn open_empty_file(&mut self) -> BufferEntry {
        let buffer = Buffer::default();
        self.add(buffer)
    }

    /// Opens a new or existing file and returns a reference to the buffer.
    pub fn open_new_or_existing_file<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<BufferEntry, buffer::Error> {
        // Check if the buffer already exists in the manager.
        if let Some(entry) = self.get_buffer_by_path(&path) {
            return Ok(entry);
        }
        let buffer = Buffer::open_new_or_existing_file(&path)?;
        Ok(self.add(buffer))
    }

    /// Adds a new [`Buffer`] and returns a reference to the new entry.
    fn add(&mut self, buffer: Buffer) -> BufferEntry {
        let buffer = Arc::new(RwLock::new(buffer));
        let entry = BufferEntry::new(self.next_id, buffer);
        self.next_id += 1;
        self.buffers.push(entry.clone());
        entry
    }

    /// Gets a buffer by path. Returns `None` if the buffer doesn't exist.
    pub fn get_buffer_by_path<P: AsRef<Path>>(&self, path: P) -> Option<BufferEntry> {
        for entry in &self.buffers {
            let buffer = entry.buffer.read().ok()?;
            if let Some(file_path) = &buffer.filepath
                && file_path == path.as_ref()
            {
                return Some(entry.clone());
            }
        }
        None
    }

    /// Returns the index of the buffer with the given id.
    fn buffer_index(&self, id: usize) -> Option<usize> {
        self.buffers.iter().position(|entry| entry.id == id)
    }

    /// Closes the buffer with the given id.
    pub fn close(&mut self, id: usize) -> Option<BufferEntry> {
        let index = self.buffer_index(id)?;
        Some(self.buffers.remove(index))
    }

    /// Saves all open buffers.
    pub fn save_all_buffers(&self) -> Result<(), buffer::Error> {
        for entry in &self.buffers {
            entry.buffer.write().unwrap().save()?;
        }
        Ok(())
    }

    /// Returns an iterator over all buffers.
    pub fn iter(&self) -> impl Iterator<Item = &BufferEntry> {
        self.buffers.iter()
    }
}
