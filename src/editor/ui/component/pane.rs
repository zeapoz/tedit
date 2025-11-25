use crate::editor::{
    pane::{Pane, cursor::Cursor},
    ui::{
        component::{RenderingContext, gutter::Gutter},
        geometry::{point::Point, rect::Rect},
        theme::highlight_group::HL_UI_PANE,
        viewport::Viewport,
        widget::{container::ContainerBuilder, span::Span},
    },
};

/// A basic pane layout that organizes panes into equally-sized bars.
#[derive(Debug, Default, Clone)]
pub struct BarsLayout {
    pub rects: Vec<Rect>,
}

impl BarsLayout {
    /// Calculate the layout confitguration based on the number of panes and the size of the pane
    /// manager rectangle.
    pub fn calculate_layout(num_panes: usize, rect: Rect) -> BarsLayout {
        Self {
            rects: rect.split_vertically_n(num_panes),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PaneView {
    /// The rect.
    pub rect: Rect,
    /// The gutter of the pane.
    pub gutter: Gutter,
    /// The column offset of the viewport.
    pub col_offset: usize,
    /// The row offset of the viewport.
    pub row_offset: usize,
    /// The width of the viewport.
    pub width: usize,
    /// The height of the viewport.
    pub height: usize,
}

impl PaneView {
    /// Scroll the viewport to the given cursor such that the cursor is visible. Returns
    /// `true` if the viewport was scrolled.
    pub fn scroll_to_cursor(&mut self, cursor: &Cursor) -> bool {
        let mut scrolled = false;

        // Vertical scrolling.
        if cursor.row() < self.row_offset {
            self.row_offset = cursor.row();
            scrolled = true;
        } else if cursor.row() >= self.row_offset.saturating_add(self.height) {
            self.row_offset = cursor.row() - self.height + 1;
            scrolled = true;
        }

        // Horizontal scrolling.
        if cursor.col() < self.col_offset {
            self.col_offset = cursor.col();
            scrolled = true;
        } else if cursor.col() >= self.col_offset.saturating_add(self.width) {
            self.col_offset = cursor.col() - self.width + 1;
            scrolled = true;
        }

        scrolled
    }

    /// Scrolls the viewport vertically by the given offset.
    pub fn scroll_vertically(&mut self, offset: isize) {
        if offset.is_positive() {
            self.row_offset = self.row_offset.saturating_add(offset as usize);
        } else if offset.is_negative() {
            self.row_offset = self.row_offset.saturating_sub(offset.unsigned_abs());
        }
    }

    /// Updates the viewport to match the given dimensions.
    pub fn update_size(&mut self, rect: Rect) {
        let (_gutter, buffer) = rect.split_vertically_exact(self.gutter.width());
        self.rect = rect;
        self.width = buffer.width;
        self.height = buffer.height;
    }

    /// Return the width of the viewport.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Return the height of the viewport.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a point coordinate relative to the viewport and the gutter.
    pub fn coord_to_screen(&self, Point { mut col, mut row }: Point) -> Point {
        col = col.saturating_sub(self.col_offset) + self.gutter.width();
        row = row.saturating_sub(self.row_offset);
        Point::new(col, row)
    }

    /// Returns the visible rows of the pane.
    pub fn visible_rows(&self, pane: &Pane) -> Vec<String> {
        let mut rows = Vec::with_capacity(self.height());

        let start_row = self.row_offset;
        for row_idx in 0..self.height() {
            let buffer_row = start_row + row_idx;
            let row_content = pane
                .buffer
                .read()
                .unwrap()
                .row(buffer_row)
                .map(|r| r.chars_in_range(self.col_offset, self.width))
                .unwrap_or_default();
            rows.push(row_content);
        }

        rows
    }

    /// Renders the pane view.
    pub fn render(&mut self, ctx: &RenderingContext, pane: &Pane, mut viewport: Viewport) {
        self.scroll_to_cursor(&pane.cursor);

        self.gutter.update_width(pane.buffer_lines());
        let (gutter_viewport, mut buffer_viewport) =
            viewport.split_horizontally_exact(self.gutter.width());

        // Render the gutter.
        self.gutter
            .render(ctx, pane, self.row_offset, gutter_viewport);

        // Render the buffer content.
        let rows = self.visible_rows(pane);
        let style = ctx.theme.resolve(&HL_UI_PANE);
        for (i, row) in rows.iter().enumerate() {
            let span = Span::new(row);
            let widget = ContainerBuilder::default()
                .with_width(Some(buffer_viewport.width()))
                .with_child(span)
                .with_style(style)
                .build();
            buffer_viewport.put_widget(i, widget);
        }
    }
}
