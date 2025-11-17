use crate::editor::{
    Editor, Mode,
    backend::{self, RenderingBackend},
    command_palette::CommandPalette,
    document::Document,
    gutter::Gutter,
    prompt::PromptType,
    status_bar::StatusBar,
};

pub use rect::Rect;

pub mod compositor;
pub mod layout;
pub mod rect;

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
    fn render(
        &self,
        ctx: &RenderingContext,
        rect: Rect,
        backend: &mut RenderingBackend,
    ) -> Result<(), backend::Error>;
}
