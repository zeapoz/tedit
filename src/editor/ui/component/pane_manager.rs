use crate::editor::{
    geometry::{point::Point, rect::Rect},
    pane::manager::PaneManager,
    ui::{
        component::{
            Component, RenderingContext,
            pane::{BarsLayout, PaneView},
        },
        viewport::Viewport,
    },
};

#[derive(Debug, Default, Clone)]
pub struct PaneManagerView {
    pub rect: Rect,
    pub pane_views: Vec<PaneView>,
}

impl PaneManagerView {
    /// Syncs the views with the pane manager.
    pub fn sync_panes(&mut self, manager: &PaneManager, rect: Rect) {
        let num_panes = manager.num_panes();

        // Ensure we have enough pane views.
        while self.pane_views.len() < num_panes {
            self.pane_views.push(PaneView::default());
        }
        while self.pane_views.len() > num_panes {
            self.pane_views.pop();
        }

        // Update the rects based on layout.
        let layout = BarsLayout::calculate_layout(num_panes, rect);
        for (view, rect) in self.pane_views.iter_mut().zip(layout.rects.iter()) {
            view.update_size(*rect);
        }
    }

    /// Returns the screen position of the active pane's cursor.
    pub fn get_active_cursor_screen_position(&self, manager: &PaneManager) -> Point {
        let active_index = manager.active_pane();
        let active_view = self.pane_views[active_index];
        let local_cursor_position = manager.active().cursor_position();

        let Point { mut col, mut row } = active_view.coord_to_screen(local_cursor_position.into());
        col += active_view.rect.col + self.rect.col;
        row += active_view.rect.row + self.rect.row;
        Point::new(col, row)
    }
}

impl Component for PaneManagerView {
    fn rect(&self, editor_view: Rect) -> Rect {
        Rect::new(
            0,
            0,
            editor_view.width,
            editor_view.height.saturating_sub(1),
        )
    }

    fn render(&mut self, ctx: &RenderingContext, mut viewport: Viewport) {
        self.rect = viewport.rect();
        self.sync_panes(&ctx.pane_manager, self.rect);

        for (pane, pane_view) in ctx.pane_manager.iter().zip(self.pane_views.iter_mut()) {
            let pane_viewport = viewport.sub_rect(pane_view.rect).unwrap();
            pane_view.render(ctx, pane, pane_viewport);
        }
    }
}
