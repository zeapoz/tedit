use std::time::{Duration, Instant};

use crate::editor::ui::{
    component::{Component, RenderingContext},
    geometry::{anchor::Anchor, rect::Rect},
    style::Style,
    theme::highlight_group::HL_UI_STATUSBAR,
    viewport::Viewport,
    widget::{
        Widget,
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

    /// Updates the state of the status bar.
    pub fn update(&mut self) {}
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
        let cursor_position = format!("{}:{}", cursor_row + 1, cursor_col + 1);

        let message = ctx
            .status_message
            .as_ref()
            .map(|m| m.text())
            .unwrap_or_default();

        let left = Container::new(vec![
            Span::new(&mode).with_style(mode_style),
            Span::new(&file).with_style(file_style),
        ]);

        let center = Container::new(vec![Span::new(message)]).with_alignment(Alignment::Center);

        let right =
            Container::new(vec![Span::new(&cursor_position)]).with_alignment(Alignment::Right);

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
