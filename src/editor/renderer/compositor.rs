use crate::editor::renderer::{
    Renderable, RenderingContext, frame::Frame, layout::Layout, viewport::Viewport,
};

// TODO: Implement layers.
/// A compositor that organizes the rendering of multiple objects on the terminal.
#[derive(Debug)]
pub struct Compositor;

impl Compositor {
    /// Composes a frame from the given context and layout.
    pub fn compose_frame(ctx: &RenderingContext, layout: &Layout) -> Frame {
        let mut frame = Frame::new(layout.width, layout.height);

        let gutter_viewport = Viewport::new(layout.gutter, &mut frame);
        ctx.gutter.render(ctx, gutter_viewport);

        let document_viewport = Viewport::new(layout.document, &mut frame);
        ctx.document.render(ctx, document_viewport);

        if let Some(rect) = layout.prompt
            && let Some(prompt) = &ctx.prompt
        {
            let prompt_viewport = Viewport::new(rect, &mut frame);
            prompt.render(ctx, prompt_viewport);
        } else if let Some(rect) = layout.command_palette {
            let command_palette_viewport = Viewport::new(rect, &mut frame);
            ctx.command_palette.render(ctx, command_palette_viewport);
        }

        let status_bar_viewport = Viewport::new(layout.status_bar, &mut frame);
        ctx.status_bar.render(ctx, status_bar_viewport);

        let cursor_position = ctx.document.cursor_position();
        frame.set_cursor_position(
            cursor_position.0.saturating_add(ctx.gutter.width()),
            cursor_position
                .1
                .saturating_sub(ctx.document.viewport_row_offset()),
        );

        frame
    }
}
