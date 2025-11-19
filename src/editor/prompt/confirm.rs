use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    backend::{self, RenderingBackend},
    prompt::{Prompt, PromptResponse, PromptStatus},
    renderer::{Rect, Renderable, RenderingContext, frame::Span, viewport::Viewport},
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

impl Renderable for ConfirmPrompt {
    fn render(&self, _ctx: &RenderingContext, mut viewport: Viewport<'_>) {
        let message = format!("{} [y/n] ", self.message);
        viewport.put_span(0, 0, Span::new(&message));
    }
}
