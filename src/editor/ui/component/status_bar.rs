use std::time::{Duration, Instant};

use crate::editor::ui::{
    component::{Component, RenderingContext},
    geometry::{anchor::Anchor, rect::Rect},
    style::Style,
    theme::highlight_group::HL_UI_STATUSBAR,
    viewport::Viewport,
    widget::{
        container::{Alignment, Container},
        span::Span,
    },
};

#[derive(Debug, Clone)]
pub struct Message {
    /// The content of the message.
    content: String,
    /// The time when the message was set.
    set_time: Instant,
    /// The duration for which the message should be displayed.
    duration: Duration,
}

impl Message {
    const DEFAULT_MESSAGE_TIMEOUT: Duration = Duration::from_secs(5);

    /// Creates a new message with the given content. By default, the messages duration will be set
    /// to [`DEFAULT_MESSAGE_TIMEOUT`].
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            set_time: Instant::now(),
            duration: Self::DEFAULT_MESSAGE_TIMEOUT,
        }
    }

    /// Sets the duration for which the message should be displayed.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Returns the content of the message.
    pub fn text(&self) -> &str {
        &self.content
    }

    /// Returns true if the message has timed out.
    pub fn timed_out(&self) -> bool {
        self.set_time.elapsed() > self.duration
    }
}

#[derive(Debug, Clone)]
pub struct StatusBar {
    /// The height of the status bar.
    height: usize,
}

impl StatusBar {
    const DEFAULT_HEIGHT: usize = 1;

    /// Returns the height of the status bar.
    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            height: Self::DEFAULT_HEIGHT,
        }
    }
}

impl Component for StatusBar {
    fn rect(&self, parent: Rect) -> Rect {
        Rect::new(0, 0, parent.width, self.height).anchored_on(parent, Anchor::BottomLeft)
    }

    fn render(&mut self, ctx: &RenderingContext, mut viewport: Viewport) {
        // TODO make each displayed thing its own widget module.
        let style = ctx.theme.resolve(&HL_UI_STATUSBAR);
        let mode = format!(" {} ", ctx.mode);
        let mode_style = ctx.theme.resolve(&ctx.mode.into());

        let active_pane = ctx.pane_manager.active();
        let file = active_pane.file_name();
        let file_style = if active_pane.is_dirty() {
            Style::new().bold().underline()
        } else {
            Style::new().bold()
        };

        let (cursor_col, cursor_row) = active_pane.cursor_position();
        let cursor_position_str = format!("{}:{}", cursor_row + 1, cursor_col + 1);

        let message = ctx
            .status_message
            .as_ref()
            .map(|m| m.text().to_string())
            .unwrap_or_default();

        // Left container.
        let mode_span = Span::new(&mode).with_style(mode_style);
        let file_span = Span::new(&file).with_style(file_style);

        let left = Container::default()
            .with_child(mode_span)
            .with_child(file_span);

        // Center container.
        let message_span = Span::new(&message);

        let center = Container::default()
            .with_child(message_span)
            .with_alignment(Alignment::Center);

        // Right container.
        let cursor_span = Span::new(&cursor_position_str);

        let right = Container::default()
            .with_child(cursor_span)
            .with_alignment(Alignment::Right);

        // Main widget container.
        let widget = Container::default()
            .with_width(Some(viewport.width()))
            .with_alignment(Alignment::SpaceEvenly)
            .with_child(left)
            .with_child(center)
            .with_child(right)
            .with_style(style);
        viewport.put_widget(0, widget);
    }
}
