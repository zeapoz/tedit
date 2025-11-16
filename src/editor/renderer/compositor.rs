use crate::editor::{
    Mode, backend,
    command_palette::CommandPalette,
    document::Document,
    gutter::Gutter,
    prompt::PromptManager,
    renderer::{Rect, Renderable, RenderingContext},
    status_bar::StatusBar,
};

/// A compositor that organizes the rendering of multiple objects on the terminal.
pub struct Compositor<'a> {
    pub gutter: &'a Gutter,
    pub document: &'a Document,
    pub status_bar: &'a StatusBar,
    pub prompt_manager: &'a PromptManager,
    pub command_palette: &'a CommandPalette,
}

impl Compositor<'_> {
    /// Renders all components in the compositor.
    pub fn render(&self, ctx: &mut RenderingContext) -> Result<(), backend::Error> {
        let (width, height) = ctx.backend.size()?;

        let gutter_rect = Rect::new(
            0,
            0,
            self.gutter.width(),
            height.saturating_sub(self.status_bar.height()),
        );
        self.gutter.render(ctx, gutter_rect)?;

        let document_rect = Rect::new(
            self.gutter.width(),
            0,
            width.saturating_sub(self.gutter.width()),
            height.saturating_sub(self.status_bar.height()),
        );
        self.document.render(ctx, document_rect)?;

        if let Some(active) = &self.prompt_manager.active_prompt {
            let prompt_rect = Rect::new(0, height.saturating_sub(2), width, 1);
            active.prompt.render(ctx, prompt_rect)?;
        } else if *ctx.mode == Mode::Command {
            let command_palette_rect = Rect::new(0, 0, width, height.saturating_sub(1));
            self.command_palette.render(ctx, command_palette_rect)?;
        }

        let status_bar_rect = Rect::new(0, height.saturating_sub(1), width, 1);
        self.status_bar.render(ctx, status_bar_rect)?;

        // Render the cursor last to ensure it is on top of everything else.
        let cursor_position = self.document.cursor_position();
        ctx.backend.move_cursor(
            cursor_position.0.saturating_add(self.gutter.width()),
            cursor_position
                .1
                .saturating_sub(self.document.viewport_row_offset()),
        )?;

        Ok(())
    }
}
