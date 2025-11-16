use crate::editor::{
    Mode,
    backend::{self, TerminalBackend},
    document::Document,
};

pub use rect::Rect;

pub mod compositor;
mod rect;

/// A context for rendering objects.
pub struct RenderingContext<'a> {
    pub backend: &'a mut TerminalBackend,
    pub mode: &'a Mode,
    pub document: &'a Document,
}

/// A trait for types that can be rendered to the terminal.
pub trait Renderable {
    /// Renders the object to the terminal.
    fn render(&self, ctx: &mut RenderingContext, rect: Rect) -> Result<(), backend::Error>;
}
