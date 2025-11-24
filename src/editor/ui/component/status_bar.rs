use std::time::{Duration, Instant};

use crate::editor::ui::{
    component::{Component, RenderingContext},
    geometry::{anchor::Anchor, rect::Rect},
    style::Style,
    text::{Alignment, Line, Section, Span},
    theme::highlight_group::HL_UI_STATUSBAR,
    viewport::Viewport,
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
        let mode = ctx.mode.to_string();
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

        let left = Section::new(vec![
            Span::new(&mode)
                .with_style(mode_style)
                .with_width(mode.len().saturating_add(2))
                .with_alignment(Alignment::Center),
            Span::new(&file).with_style(file_style),
        ])
        .with_alignment(Alignment::Left)
        .with_whitespace_separator();

        let center = Section::new(vec![Span::new(message)])
            .with_alignment(Alignment::Center)
            .with_whitespace_separator();

        let right = Section::new(vec![Span::new(&cursor_position)])
            .with_alignment(Alignment::Right)
            .with_whitespace_separator();

        let line = Line::new(viewport.width())
            .with_section(left)
            .with_section(center)
            .with_section(right)
            .with_style(style);
        viewport.put_line(0, line);
    }
}
