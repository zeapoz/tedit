use crate::editor::{
    pane::Pane,
    ui::{
        component::RenderingContext,
        theme::highlight_group::{HL_UI_PANE_GUTTER, HL_UI_PANE_GUTTER_CURSOR},
        viewport::Viewport,
        widget::{
            container::{Alignment, ContainerBuilder},
            span::Span,
        },
    },
};

// TODO: Make this adapt to the current buffer/be configurable.
/// The width of the gutter.
const GUTTER_WIDTH: usize = 7;

#[derive(Debug, Clone, Copy)]
pub struct Gutter {
    width: usize,
}

impl Default for Gutter {
    fn default() -> Self {
        Self {
            width: GUTTER_WIDTH,
        }
    }
}

impl Gutter {
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    /// Returns the width of the gutter.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Renders the gutter.
    pub fn render(
        &self,
        ctx: &RenderingContext,
        pane: &Pane,
        row_offset: usize,
        mut viewport: Viewport,
    ) {
        let cursor_row = pane.cursor_position().1;
        // TODO: Don't display numbers exceeding the buffer height.
        for row in 0..viewport.height() {
            let pane_row = row_offset + row;
            let s = format!(
                "{:>width$}",
                pane_row.saturating_add(1),
                // TODO: Get the number of digits from the buffer.
                width = self.width.saturating_sub(3)
            );

            let style = if cursor_row == pane_row {
                ctx.theme.resolve(&HL_UI_PANE_GUTTER_CURSOR)
            } else {
                ctx.theme.resolve(&HL_UI_PANE_GUTTER)
            };

            let span = Span::new(&s);
            let widget = ContainerBuilder::default()
                .with_width(Some(viewport.width()))
                .with_child(span)
                .with_alignment(Alignment::Center)
                .with_style(style)
                .build();
            viewport.put_widget(row, widget);
        }
    }
}
