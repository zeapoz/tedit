use std::cell::RefCell;

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
        let frame = RefCell::new(Frame::new(layout.width, layout.height));

        let pane_viewport = Viewport::new(layout.pane_manager, &frame);
        ctx.pane_manager.render(ctx, pane_viewport);

        if let Some(rect) = layout.prompt
            && let Some(prompt) = &ctx.prompt
        {
            let prompt_viewport = Viewport::new(rect, &frame);
            prompt.render(ctx, prompt_viewport);
        } else if let Some(rect) = layout.command_palette {
            let command_palette_viewport = Viewport::new(rect, &frame);
            ctx.command_palette.render(ctx, command_palette_viewport);
        }

        let status_bar_viewport = Viewport::new(layout.status_bar, &frame);
        ctx.status_bar.render(ctx, status_bar_viewport);

        let (cursor_col, cursor_row) = ctx.pane_manager.active_cursor_screen_position();
        let mut frame = frame.into_inner();
        frame.set_cursor_position(cursor_col, cursor_row);
        frame
    }
}
