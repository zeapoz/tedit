use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    prompt::{Prompt, PromptResponse, PromptStatus},
    ui::{
        component::{Component, RenderingContext},
        geometry::{anchor::Anchor, rect::Rect},
        theme::highlight_group::HL_UI_OVERLAY,
        viewport::Viewport,
        widget::{container::ContainerBuilder, span::Span},
    },
};

#[derive(Debug, Clone)]
pub struct ConfirmPrompt {
    message: String,
}

impl ConfirmPrompt {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Prompt for ConfirmPrompt {
    fn process_key(&mut self, event: &KeyEvent) -> PromptStatus {
        match event.code {
            KeyCode::Enter => PromptStatus::Done(PromptResponse::Yes),
            KeyCode::Char('y') => PromptStatus::Done(PromptResponse::Yes),
            KeyCode::Char('n') => PromptStatus::Done(PromptResponse::No),
            KeyCode::Esc => PromptStatus::Done(PromptResponse::Cancel),
            _ => PromptStatus::Pending,
        }
    }
}

impl Component for ConfirmPrompt {
    fn rect(&self, parent: Rect) -> Rect {
        Rect::new(0, 0, parent.width, 1)
            .anchored_on(parent, Anchor::BottomLeft)
            .offset(0, -1)
    }

    fn render(&mut self, ctx: &RenderingContext, mut viewport: Viewport) {
        let style = ctx.theme.resolve(&HL_UI_OVERLAY);
        let message_str = format!("{} [y/n] ", self.message);

        let span = Span::new(&message_str);
        let widget = ContainerBuilder::default()
            .with_width(Some(viewport.width()))
            .with_child(span)
            .with_style(style).build();
        viewport.put_widget(0, widget);
    }
}
