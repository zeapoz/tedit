use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    Result,
    backend::TerminalBackend,
    document::Document,
    prompt::{Prompt, PromptAction, PromptResponse, PromptStatus},
};

#[derive(Debug)]
pub struct SearchPrompt {
    query: String,
    // TODO: Should not be copied.
    /// The document to search within.
    document: Document,
}

impl SearchPrompt {
    pub fn new(document: Document) -> Self {
        Self {
            query: String::new(),
            document,
        }
    }
}

impl Prompt for SearchPrompt {
    fn process_key(&mut self, event: &KeyEvent) -> PromptStatus {
        match event.code {
            KeyCode::Esc => PromptStatus::Done(PromptResponse::Text(self.query.to_string())),
            KeyCode::Enter => PromptStatus::Done(PromptResponse::Text(self.query.to_string())),
            KeyCode::Char(c) => {
                self.query.push(c);
                PromptStatus::Changed
            }
            KeyCode::Backspace => {
                self.query.pop();
                PromptStatus::Changed
            }
            _ => PromptStatus::Pending,
        }
    }

    fn on_changed(&mut self) -> PromptAction {
        if let Some((col, row)) = self.document.find_next(&self.query) {
            PromptAction::MoveCursor { col, row }
        } else {
            PromptAction::None
        }
    }

    fn render(&self, backend: &TerminalBackend) -> Result<()> {
        /// The row to render the prompt to. Counted from the bottom of the terminal.
        const RENDER_ROW: u16 = 2;

        let (_, rows) = backend.size()?;

        backend.move_cursor(0, rows - RENDER_ROW)?;
        backend.clear_line()?;

        let message = format!("search: {}", self.query);
        backend.write(&message)?;
        Ok(())
    }
}
