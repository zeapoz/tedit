use crate::editor::ui::{
    component::RenderingContext,
    frame::Cell,
    style::Style,
    widget::{
        Widget,
        container::{Container, ContainerBuilder},
        separator::WhitespaceSeparator,
        span::Span,
    },
};

/// A widget that displays the current mode.
pub struct ModeWidget {
    container: Container,
}

impl ModeWidget {
    pub fn new(ctx: &RenderingContext) -> Self {
        let style = ctx.theme.resolve(ctx.mode.into());
        Self {
            container: ContainerBuilder::default()
                .with_child(WhitespaceSeparator::default())
                .with_child(Span::new(&ctx.mode.to_string()))
                .with_child(WhitespaceSeparator::default())
                .with_style(style)
                .build(),
        }
    }
}

impl Widget for ModeWidget {
    fn as_cells(&mut self) -> Vec<Cell> {
        self.container.as_cells()
    }

    fn width(&self) -> usize {
        self.container.width()
    }

    fn set_width(&mut self, width: Option<usize>) {
        self.container.set_width(width);
    }

    fn set_style(&mut self, style: Style) {
        self.container.set_style(style);
    }
}

/// A widget that displays information about the current file.
pub struct FileWidget {
    container: Container,
}

impl FileWidget {
    pub fn new(ctx: &RenderingContext) -> Self {
        let active_pane = ctx.pane_manager.active();
        let file_name = active_pane.file_name();
        let style = if active_pane.is_dirty() {
            Style::new().bold().underline()
        } else {
            Style::new().bold()
        };

        Self {
            container: ContainerBuilder::default()
                .with_child(Span::new(&file_name))
                .with_style(style)
                .build(),
        }
    }
}

impl Widget for FileWidget {
    fn as_cells(&mut self) -> Vec<Cell> {
        self.container.as_cells()
    }

    fn width(&self) -> usize {
        self.container.width()
    }

    fn set_width(&mut self, width: Option<usize>) {
        self.container.set_width(width);
    }

    fn set_style(&mut self, style: Style) {
        self.container.set_style(style);
    }
}

/// A widget that displays the current message.
pub struct MessageWidget {
    container: Container,
}

impl MessageWidget {
    pub fn new(ctx: &RenderingContext) -> Self {
        // TODO: Style based on message type.
        let message = ctx
            .status_message
            .as_ref()
            .map(|m| m.text().to_string())
            .unwrap_or_default();

        Self {
            container: ContainerBuilder::default()
                .with_child(Span::new(&message))
                .build(),
        }
    }
}

impl Widget for MessageWidget {
    fn as_cells(&mut self) -> Vec<Cell> {
        self.container.as_cells()
    }

    fn width(&self) -> usize {
        self.container.width()
    }

    fn set_width(&mut self, width: Option<usize>) {
        self.container.set_width(width);
    }

    fn set_style(&mut self, style: Style) {
        self.container.set_style(style);
    }
}

/// A widget that displays the cursor position.
pub struct CursorWidget {
    container: Container,
}

impl CursorWidget {
    pub fn new(ctx: &RenderingContext) -> Self {
        let (cursor_col, cursor_row) = ctx.pane_manager.active().cursor_position();
        let cursor_position = format!("{}:{}", cursor_row + 1, cursor_col + 1);
        Self {
            container: ContainerBuilder::default()
                .with_child(Span::new(&cursor_position))
                .build(),
        }
    }
}

impl Widget for CursorWidget {
    fn as_cells(&mut self) -> Vec<Cell> {
        self.container.as_cells()
    }

    fn width(&self) -> usize {
        self.container.width()
    }

    fn set_width(&mut self, width: Option<usize>) {
        self.container.set_width(width);
    }

    fn set_style(&mut self, style: Style) {
        self.container.set_style(style);
    }
}
