use crate::editor::{
    Editor, Mode, backend::EditorBackend, prompt::PromptManager, renderer::Rect,
    status_bar::StatusBar,
};

/// The context for the layout of the editor.
pub struct LayoutContext<'a> {
    pub mode: &'a Mode,
    pub status_bar: &'a StatusBar,
    pub prompt_manager: &'a PromptManager,
    pub backend: &'a EditorBackend,
}

impl<'a> LayoutContext<'a> {
    pub fn new(
        mode: &'a Mode,
        status_bar: &'a StatusBar,
        prompt_manager: &'a PromptManager,
        backend: &'a EditorBackend,
    ) -> Self {
        Self {
            mode,
            status_bar,
            prompt_manager,
            backend,
        }
    }
}

impl<'a> From<&'a Editor> for LayoutContext<'a> {
    fn from(value: &'a Editor) -> Self {
        Self {
            mode: &value.mode,
            status_bar: &value.status_bar,
            prompt_manager: &value.prompt_manager,
            backend: &value.backend,
        }
    }
}

/// Represents the layout of the editor, the container of all other components.
#[derive(Debug)]
pub struct Layout {
    pub width: usize,
    pub height: usize,
    pub pane_manager: Rect,
    pub status_bar: Rect,
    pub prompt: Option<Rect>,
    pub command_palette: Option<Rect>,
}

impl Layout {
    /// Calculate the layout of the editor from the given context.
    pub fn calculate(ctx: &LayoutContext) -> Layout {
        let (width, height) = ctx.backend.size().unwrap_or((0, 0));

        let pane_manager = Rect::new(0, 0, width, height.saturating_sub(ctx.status_bar.height()));
        let status_bar = Rect::new(0, height.saturating_sub(1), width, 1);

        let prompt = if ctx.prompt_manager.active_prompt.is_some() {
            Some(Rect::new(0, height.saturating_sub(2), width, 1))
        } else {
            None
        };

        let command_palette =
            (*ctx.mode == Mode::Command).then(|| Rect::new(0, 0, width, height.saturating_sub(1)));

        Layout {
            width,
            height,
            pane_manager,
            status_bar,
            prompt,
            command_palette,
        }
    }
}
