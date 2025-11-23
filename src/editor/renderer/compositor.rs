use std::cell::RefCell;

use crate::editor::{
    Mode,
    command_palette::CommandPalette,
    geometry::rect::Rect,
    prompt::PromptManager,
    ui::{
        component::{
            Component, RenderingContext, pane_manager::PaneManagerView, status_bar::StatusBar,
        },
        frame::{Cell, Frame},
        theme::highlight_group::HL_UI,
        viewport::Viewport,
    },
};

// TODO: Store each Renderable here once view model separation is complete.
/// A compositor that organizes the rendering of multiple objects on the terminal.
#[derive(Debug, Default, Clone)]
pub struct Compositor {
    pane_manager_view: PaneManagerView,
    status_bar: StatusBar,
}

impl Compositor {
    /// Composes a frame from the given context.
    pub fn compose_frame(
        &mut self,
        ctx: &RenderingContext,
        prompt_manager: &mut PromptManager,
        command_palette: &mut CommandPalette,
    ) -> Frame {
        let editor_view = ctx.editor_view;
        let frame = RefCell::new(Frame::new(editor_view.width, editor_view.height));

        // Fill the background with the default color.
        let mut editor_viewport = Viewport::new(
            Rect::new(0, 0, editor_view.width, editor_view.height),
            &frame,
        );
        editor_viewport.fill(Cell::default().with_style(ctx.theme.resolve(&HL_UI)));

        // Render the views.
        self.pane_manager_view.render(
            ctx,
            Viewport::new(self.pane_manager_view.rect(editor_view), &frame),
        );
        self.status_bar.render(
            ctx,
            Viewport::new(self.status_bar.rect(editor_view), &frame),
        );

        if let Some(active) = prompt_manager.active_prompt.as_mut() {
            active
                .prompt
                .render(ctx, Viewport::new(active.prompt.rect(editor_view), &frame));
        } else if ctx.mode == Mode::Command {
            command_palette.render(
                ctx,
                Viewport::new(command_palette.rect(editor_view), &frame),
            );
        }

        // Update the cursor position based on its screen position in the pane manager view.
        let cursor_position = self
            .pane_manager_view
            .get_active_cursor_screen_position(&ctx.pane_manager);
        let mut frame = frame.into_inner();
        frame.set_cursor_position(cursor_position);
        frame
    }
}
