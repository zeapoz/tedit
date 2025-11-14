use crossterm::event::KeyEvent;

use crate::editor::{Editor, Result, backend::TerminalBackend};

pub mod confirm;

/// A trait for defining prompts.
pub trait Prompt {
    /// Handles an input event and returns a [`PromptStatus`] indicating whether the prompt should
    /// return or continue.
    fn handle_input(&self, event: &KeyEvent) -> PromptStatus;

    /// Renders the prompt to the terminal.
    fn render(&self, backend: &TerminalBackend) -> Result<()>;
}

/// A callback that is called when the prompt is done.
pub type PromptCallback = dyn FnOnce(&mut Editor, PromptResponse) -> Result<()>;

/// Represents the available responses of a promp.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromptResponse {
    Yes,
    No,
    Cancel,
}

/// Represents the status of a prompt.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromptStatus {
    Done(PromptResponse),
    Pending,
}

pub struct ActivePrompt {
    pub prompt: Box<dyn Prompt>,
    pub callback: Box<PromptCallback>,
}

#[derive(Default)]
pub struct PromptManager {
    pub active_prompt: Option<ActivePrompt>,
}

impl PromptManager {
    /// Shows a new prompt.
    pub fn show_prompt<F>(&mut self, prompt: Box<dyn Prompt>, callback: F)
    where
        F: FnMut(&mut Editor, PromptResponse) -> Result<()> + 'static,
    {
        self.active_prompt = Some(ActivePrompt {
            prompt,
            callback: Box::new(callback),
        })
    }
}
