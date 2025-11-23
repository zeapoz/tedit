use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    prompt::{Prompt, PromptResponse, PromptStatus},
    ui::{
        component::{Component, RenderingContext},
        frame::{Line, Span},
        geometry::{anchor::Anchor, rect::Rect},
        theme::highlight_group::HL_UI_OVERLAY,
        viewport::Viewport,
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
        let message = format!("{} [y/n] ", self.message);
        viewport.put_line(
            0,
            Line::new(viewport.width(), vec![Span::new(&message)]).with_style(style),
        );
    }
}
