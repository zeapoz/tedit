use crate::editor::{
    Editor, Mode,
    backend::{self, RenderingBackend},
    command_palette::CommandPalette,
    document::Document,
    gutter::Gutter,
    prompt::PromptType,
    renderer::{frame::Frame, layout::Layout, viewport::Viewport},
    status_bar::StatusBar,
};

pub use rect::Rect;

pub mod compositor;
pub mod frame;
pub mod layout;
pub mod rect;
pub mod viewport;

// TODO: Make this cheaper to create. Instead of cloning everything, just clone the state needed
// for rendering.
/// A context for rendering objects.
pub struct RenderingContext {
    pub mode: Mode,
    pub gutter: Gutter,
    pub document: Document,
    pub status_bar: StatusBar,
    pub prompt: Option<PromptType>,
    pub command_palette: CommandPalette,
}

impl<'a> From<&'a Editor> for RenderingContext {
    fn from(value: &'a Editor) -> Self {
        let prompt = value
            .prompt_manager
            .active_prompt
            .as_ref()
            .map(|active| active.prompt.clone());

        Self {
            mode: value.mode,
            gutter: value.gutter,
            document: value.document_manager.active().clone(),
            status_bar: value.status_bar.clone(),
            prompt,
            command_palette: value.command_palette.clone(),
        }
    }
}

/// A trait for types that can be rendered to the terminal.
pub trait Renderable {
    /// Renders the object to the terminal.
    fn render(&self, ctx: &RenderingContext, viewport: Viewport);
}

/// Responsible for rendering frames to the terminal.
#[derive(Debug)]
pub struct Renderer {
    backend: RenderingBackend,
}

impl Renderer {
    /// Initializes a new compositor.
    pub fn initialize() -> Result<Self, backend::Error> {
        let backend = RenderingBackend::initialize()?;
        Ok(Self { backend })
    }

    /// deinitialize the compositor.
    pub fn deinitialize(&mut self) -> Result<(), backend::Error> {
        self.backend.deinitialize()
    }

    /// Renders the editor to the terminal.
    pub fn render(&mut self, frame: &Frame) -> Result<(), backend::Error> {
        self.backend.hide_cursor()?;
        self.backend.move_cursor(0, 0)?;
        self.backend.clear_all()?;

        for (row, cells) in frame.rows().enumerate() {
            self.backend.move_cursor(0, row)?;
            for cell in cells {
                self.backend.write_char(cell.char)?;
            }
        }

        if let Some((col, row)) = frame.cursor_position() {
            self.backend.move_cursor(col, row)?;
            self.backend.show_cursor()?;
        }

        self.backend.show_cursor()?;
        self.backend.flush()?;

        Ok(())
    }
}
