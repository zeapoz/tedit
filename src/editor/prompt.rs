use crossterm::event::KeyEvent;

use crate::editor::{Editor, Result, backend::TerminalBackend};

pub mod confirm;
pub mod search;

/// A trait for defining prompts.
pub trait Prompt {
    /// Handles an input event and returns a [`PromptStatus`] indicating whether the prompt should
    /// return or continue.
    fn process_key(&mut self, event: &KeyEvent) -> PromptStatus;

    /// Returns an action to be when the prompts state has changed.
    fn on_changed(&mut self) -> PromptAction {
        PromptAction::None
    }

    /// Renders the prompt to the terminal.
    fn render(&self, backend: &TerminalBackend) -> Result<()>;
}

/// A callback that is called when the prompt is done.
pub type PromptCallback = dyn FnOnce(&mut Editor, PromptResponse) -> Result<()>;

/// Represents the available responses of a promp.
#[derive(Debug, Clone, PartialEq)]
pub enum PromptResponse {
    Yes,
    No,
    Cancel,
    Text(String),
}

/// An action that can be returned by the prompt to be handled by the editor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromptAction {
    None,
    MoveCursor { col: usize, row: usize },
}

/// Represents the status of a prompt.
#[derive(Debug, Clone, PartialEq)]
pub enum PromptStatus {
    /// Emitted when the prompt is done.
    Done(PromptResponse),
    /// Emitted when the prompt input is changed.
    Changed,
    /// Emitted as long as the prompt is still active.
    Pending,
}

pub struct ActivePrompt {
    pub prompt: Box<dyn Prompt>,
    /// The callback to execute when the prompt is done.
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
