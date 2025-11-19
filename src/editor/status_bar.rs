use std::time::{Duration, Instant};

use crate::editor::renderer::{Renderable, RenderingContext, frame::Span, viewport::Viewport};

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
    /// An optional message to display in the status bar.
    message: Option<Message>,
}

impl StatusBar {
    const DEFAULT_HEIGHT: usize = 1;

    /// Returns the height of the status bar.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Sets the message to display in the status bar.
    pub fn set_message(&mut self, message: Message) {
        self.message = Some(message);
    }

    /// Updates the state of the status bar.
    pub fn update(&mut self) {
        // Check if the message has timed out. If so, clear it.
        if let Some(message) = &self.message
            && message.timed_out()
        {
            self.message = None;
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            height: Self::DEFAULT_HEIGHT,
            message: None,
        }
    }
}

impl Renderable for StatusBar {
    fn render(&self, ctx: &RenderingContext, mut viewport: Viewport<'_>) {
        let mode = ctx.mode.to_string();
        let file_name = ctx.document.file_name();

        // TODO: Bold style.
        let dirty_marker = if ctx.document.is_dirty() {
            "*".to_string()
        } else {
            " ".to_string()
        };

        let (cursor_col, cursor_row) = ctx.document.cursor_position();
        let cursor_position = format!("{}:{}", cursor_row + 1, cursor_col + 1);

        let status = format!(
            "{mode} {file_name}{dirty_marker} {cursor_position} {}",
            self.message.as_ref().map(|m| m.text()).unwrap_or_default()
        );

        viewport.put_span(0, 0, Span::new(&status));
    }
}
