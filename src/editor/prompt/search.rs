use crossterm::event::{KeyCode, KeyEvent};

use crate::editor::{
    geometry::{anchor::Anchor, rect::Rect},
    pane::Pane,
    prompt::{Prompt, PromptAction, PromptResponse, PromptStatus},
    ui::{
        component::{Component, RenderingContext},
        frame::{Line, Span},
        theme::highlight_group::HL_UI_OVERLAY,
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
        if let Some(point) = self.pane.find_next(&self.query) {
            PromptAction::MoveCursor(point)
        } else {
            PromptAction::None
        }
    }
}

impl Component for SearchPrompt {
    fn rect(&self, parent: Rect) -> Rect {
        Rect::new(0, 0, parent.width, 1)
            .anchored_on(parent, Anchor::BottomLeft)
            .offset(0, -1)
    }

    fn render(&mut self, ctx: &RenderingContext, mut viewport: Viewport) {
        let style = ctx.theme.resolve(&HL_UI_OVERLAY);
        let message = format!("search: {}", self.query);
        viewport.put_line(
            0,
            Line::new(viewport.width(), vec![Span::new(&message)]).with_style(style),
        );
    }
}
