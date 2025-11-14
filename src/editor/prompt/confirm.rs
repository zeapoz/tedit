use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    Result,
    backend::TerminalBackend,
    prompt::{Prompt, PromptResponse, PromptStatus},
};

#[derive(Debug)]
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
    fn handle_input(&self, event: &KeyEvent) -> PromptStatus {
        match event.code {
            KeyCode::Enter => PromptStatus::Done(PromptResponse::Yes),
            KeyCode::Char('y') => PromptStatus::Done(PromptResponse::Yes),
            KeyCode::Char('n') => PromptStatus::Done(PromptResponse::No),
            KeyCode::Esc => PromptStatus::Done(PromptResponse::Cancel),
            _ => PromptStatus::Pending,
        }
    }

    fn render(&self, backend: &TerminalBackend) -> Result<()> {
        /// The row to render the prompt to. Counted from the bottom of the terminal.
        const RENDER_ROW: u16 = 2;

        let (_, rows) = backend.size()?;

        backend.move_cursor(0, rows - RENDER_ROW)?;
        backend.clear_line()?;

        let message = format!("{} [y/n] ", self.message);
        backend.write(&message)?;
        Ok(())
    }
}
