use crate::editor::{
    backend::{self, RenderingBackend},
    renderer::{Renderable, RenderingContext, layout::Layout},
};

/// A compositor that organizes the rendering of multiple objects on the terminal.
#[derive(Debug)]
pub struct Compositor {
    backend: RenderingBackend,
}

impl Compositor {
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
    pub fn render(&mut self, ctx: RenderingContext, layout: &Layout) -> Result<(), backend::Error> {
        self.backend.hide_cursor()?;
        self.backend.move_cursor(0, 0)?;
        self.backend.clear_all()?;

        self.render_components(ctx, layout)?;

        self.backend.show_cursor()?;
        self.backend.flush()?;

        Ok(())
    }

    /// Renders all componenets in the context to the terminal using the given layout.
    pub fn render_components(
        &mut self,
        ctx: RenderingContext,
        layout: &Layout,
    ) -> Result<(), backend::Error> {
        ctx.gutter.render(&ctx, layout.gutter, &mut self.backend)?;
        ctx.document
            .render(&ctx, layout.document, &mut self.backend)?;

        if let Some(rect) = layout.prompt
            && let Some(prompt) = &ctx.prompt
        {
            prompt.render(&ctx, rect, &mut self.backend)?;
        } else if let Some(rect) = layout.command_palette {
            ctx.command_palette
                .render(&ctx, rect, &mut self.backend)?;
        }

        ctx.status_bar
            .render(&ctx, layout.status_bar, &mut self.backend)?;

        // Render the cursor last to ensure it is on top of everything else.
        let cursor_position = ctx.document.cursor_position();
        self.backend.move_cursor(
            cursor_position.0.saturating_add(ctx.gutter.width()),
            cursor_position
                .1
                .saturating_sub(ctx.document.viewport_row_offset()),
        )?;

        Ok(())
    }
}
