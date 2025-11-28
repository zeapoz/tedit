use std::time::{Duration, Instant};

mod widget;

use crate::editor::ui::{
    component::{
        Component, RenderingContext,
        status_bar::widget::{CursorWidget, FileWidget, MessageWidget, ModeWidget},
    },
    geometry::{anchor::Anchor, rect::Rect},
    theme::highlight_group::HL_UI_STATUSBAR,
    viewport::Viewport,
    widget::container::{Alignment, ContainerBuilder},
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
        let style = ctx.theme.resolve(&HL_UI_STATUSBAR);
        let left_container = ContainerBuilder::default()
            .with_child(ModeWidget::new(ctx))
            .with_child(FileWidget::new(ctx))
            .build()
            .with_whitespace_separator(1);
        // TODO: Make this expand.
        let center_container = ContainerBuilder::default()
            .with_child(MessageWidget::new(ctx))
            .with_alignment(Alignment::Center)
            .build();
        let right_container = ContainerBuilder::default()
            .with_child(CursorWidget::new(ctx))
            .with_alignment(Alignment::Right)
            .build();

        // Main widget container.
        let widget = ContainerBuilder::default()
            .with_width(Some(viewport.width()))
            .with_alignment(Alignment::SpaceEvenly)
            .with_child(left_container)
            .with_child(center_container)
            .with_child(right_container)
            .with_style(style)
            .build();
        viewport.put_widget(0, widget);
    }
}
