use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    pane::Pane,
    prompt::{Prompt, PromptAction, PromptResponse, PromptStatus},
    renderer::{
        Renderable, RenderingContext,
        frame::{Line, Span},
        viewport::Viewport,
    },
};

#[derive(Debug, Clone)]
pub struct SearchPrompt {
    query: String,
    // TODO: Should not be copied.
    /// The pane to search within.
    pane: Pane,
}

impl SearchPrompt {
    pub fn new(pane: Pane) -> Self {
        Self {
            query: String::new(),
            pane,
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
        if let Some((col, row)) = self.pane.find_next(&self.query) {
            PromptAction::MoveCursor { col, row }
        } else {
            PromptAction::None
        }
    }
}

impl Renderable for SearchPrompt {
    fn render(&self, _ctx: &RenderingContext, mut viewport: Viewport) {
        let message = format!("search: {}", self.query);
        viewport.put_line(0, Line::new(viewport.width(), vec![Span::new(&message)]));
    }
}
