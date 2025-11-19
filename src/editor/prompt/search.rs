use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    backend::{self, RenderingBackend},
    document::Document,
    prompt::{Prompt, PromptAction, PromptResponse, PromptStatus},
    renderer::{Rect, Renderable, RenderingContext, frame::Span, viewport::Viewport},
};

#[derive(Debug, Clone)]
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
}

impl Renderable for SearchPrompt {
    fn render(&self, _ctx: &RenderingContext, mut viewport: Viewport<'_>) {
        let message = format!("search: {}", self.query);
        viewport.put_span(0, 0, Span::new(&message));
    }
}
