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

#[derive(Debug, Clone, Copy)]
pub struct Gutter {
    width: usize,
}

impl Default for Gutter {
    fn default() -> Self {
        Self {
            width: Self::GUTTER_PADDING,
        }
    }
}

impl Gutter {
    // TODO: Gutter padding should be configurable.
    /// The minimum width of the gutter.
    const GUTTER_PADDING: usize = 4;
    const END_OF_BUFFER_MARKER: &'static str = "~";

    pub fn new(width: usize) -> Self {
        Self { width }
    }

    /// Returns the width of the gutter.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Updates the width to be at least as wide as the digits of `buffer_lines`.
    pub fn update_width(&mut self, buffer_lines: usize) {
        let digits = buffer_lines
            .to_string()
            .len()
            .saturating_add(Self::GUTTER_PADDING);
        self.width = self.width.max(digits);
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
        let buffer_lines = pane.buffer_lines();
        for row in 0..viewport.height() {
            let pane_row = row_offset + row;
            let line_number = if pane_row < buffer_lines {
                pane_row.saturating_add(1).to_string()
            } else if pane_row == buffer_lines.saturating_sub(1) {
                Self::END_OF_BUFFER_MARKER.to_string()
            } else {
                "".to_string()
            };

            let s = format!(
                "{:>width$}",
                line_number,
                width = self.width.saturating_sub(self.width / 2)
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
