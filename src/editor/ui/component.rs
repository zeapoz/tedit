use crate::editor::{
    Editor, Mode,
    geometry::rect::Rect,
    pane::manager::PaneManager,
    ui::{component::status_bar::Message, viewport::Viewport},
};

pub mod gutter;
pub mod pane;
pub mod pane_manager;
pub mod status_bar;

// TODO: Make this cheaper to create. Instead of cloning everything, just clone the state needed
// for rendering.
/// A context for rendering objects.
pub struct RenderingContext {
    pub mode: Mode,
    pub pane_manager: PaneManager,
    pub status_message: Option<Message>,
    pub editor_view: Rect,
}

impl RenderingContext {
    pub fn new(editor: &Editor, editor_view: Rect) -> Self {
        Self {
            mode: editor.mode,
            pane_manager: editor.pane_manager.clone(),
            status_message: editor.status_message.clone(),
            editor_view,
        }
    }
}

/// A trait for UI components.
pub trait Component {
    /// Returns the bounding box of the component.
    fn rect(&self, parent: Rect) -> Rect;

    /// Renders the object to the terminal.
    fn render(&mut self, ctx: &RenderingContext, viewport: Viewport);
}
