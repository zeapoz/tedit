use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    backend::{self, RenderingBackend},
    prompt::{Prompt, PromptResponse, PromptStatus},
    renderer::{Rect, Renderable, RenderingContext},
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
    fn render(
        &self,
        _ctx: &RenderingContext,
        rect: Rect,
        backend: &mut RenderingBackend,
    ) -> Result<(), backend::Error> {
        backend.move_cursor(rect.col, rect.row)?;
        backend.clear_line()?;

        let message = format!("{} [y/n] ", self.message);
        backend.write(&message)?;
        Ok(())
    }
}
