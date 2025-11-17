use crate::editor::{
    Mode,
    backend::{self, TerminalBackend},
    command_palette::CommandPalette,
    document::Document,
    gutter::Gutter,
    prompt::PromptManager,
    renderer::{Rect, Renderable, RenderingContext},
    status_bar::StatusBar,
};

/// Represents the layout of the editor, the container of all other components.
#[derive(Debug)]
pub struct Layout {
    pub gutter: Rect,
    pub document: Rect,
    pub status_bar: Rect,
    pub prompt: Option<Rect>,
    pub command_palette: Option<Rect>,
}

/// A compositor that organizes the rendering of multiple objects on the terminal.
pub struct Compositor<'a> {
    pub gutter: &'a Gutter,
    pub document: Option<&'a Document>,
    pub status_bar: &'a StatusBar,
    pub prompt_manager: &'a PromptManager,
    pub command_palette: &'a CommandPalette,
}

impl Compositor<'_> {
    pub fn calculate_layout(&self, mode: &Mode, backend: &TerminalBackend) -> Layout {
        let (width, height) = backend.size().unwrap_or((0, 0));
        let gutter_rect = Rect::new(
            0,
            0,
            self.gutter.width(),
            height.saturating_sub(self.status_bar.height()),
        );

        let document_rect = Rect::new(
            self.gutter.width(),
            0,
            width.saturating_sub(self.gutter.width()),
            height.saturating_sub(self.status_bar.height()),
        );

        let status_bar_rect = Rect::new(0, height.saturating_sub(1), width, 1);

        let prompt_rect = if self.prompt_manager.active_prompt.is_some() {
            Some(Rect::new(0, height.saturating_sub(2), width, 1))
        } else {
            None
        };

        let command_palette_rect =
            (*mode == Mode::Command).then(|| Rect::new(0, 0, width, height.saturating_sub(1)));

        Layout {
            gutter: gutter_rect,
            document: document_rect,
            status_bar: status_bar_rect,
            prompt: prompt_rect,
            command_palette: command_palette_rect,
        }
    }

    /// Renders all components in the compositor.
    pub fn render(
        &self,
        ctx: &mut RenderingContext,
        layout: &Layout,
    ) -> Result<(), backend::Error> {
        self.gutter.render(ctx, layout.gutter)?;

        if let Some(document) = self.document {
            document.render(ctx, layout.document)?;
        } else {
            // TODO: Render a welcome message.
        }

        if let Some(prompt_rect) = layout.prompt
            && let Some(active) = &self.prompt_manager.active_prompt
        {
            active.prompt.render(ctx, prompt_rect)?;
        } else if let Some(command_palette_rect) = layout.command_palette {
            self.command_palette.render(ctx, command_palette_rect)?;
        }

        self.status_bar.render(ctx, layout.status_bar)?;

        // Render the cursor last to ensure it is on top of everything else.
        if let Some(document) = self.document {
            let cursor_position = document.cursor_position();
            ctx.backend.move_cursor(
                cursor_position.0.saturating_add(self.gutter.width()),
                cursor_position
                    .1
                    .saturating_sub(document.viewport_row_offset()),
            )?;
        }

        Ok(())
    }
}
