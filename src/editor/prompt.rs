use crossterm::event::KeyEvent;

use crate::editor::{
    self, Editor,
    geometry::{point::Point, rect::Rect},
    prompt::{confirm::ConfirmPrompt, search::SearchPrompt},
    ui::{
        component::{Component, RenderingContext},
        viewport::Viewport,
    },
};

pub mod confirm;
pub mod search;

/// A trait for defining prompts.
pub trait Prompt: Clone + Component {
    /// Handles an input event and returns a [`PromptStatus`] indicating whether the prompt should
    /// return or continue.
    fn process_key(&mut self, event: &KeyEvent) -> PromptStatus;

    /// Returns an action to be executed when the prompts state has changed.
    fn on_changed(&mut self) -> PromptAction {
        PromptAction::None
    }
}

/// Defines all types of prompts.
#[derive(Debug, Clone)]
pub enum PromptType {
    Confirm(ConfirmPrompt),
    Search(SearchPrompt),
}

impl PromptType {
    /// Processes an input event and returns a [`PromptStatus`] indicating whether the prompt
    /// should return or continue.
    pub fn process_key(&mut self, event: &KeyEvent) -> PromptStatus {
        match self {
            Self::Confirm(prompt) => prompt.process_key(event),
            Self::Search(prompt) => prompt.process_key(event),
        }
    }

    /// Returns an action to be executed when the prompts state has changed.
    pub fn on_changed(&mut self) -> PromptAction {
        match self {
            Self::Confirm(prompt) => prompt.on_changed(),
            Self::Search(prompt) => prompt.on_changed(),
        }
    }

    /// Calculates the area of the prompt.
    pub fn rect(&self, parent: Rect) -> Rect {
        match self {
            Self::Confirm(prompt) => prompt.rect(parent),
            Self::Search(prompt) => prompt.rect(parent),
        }
    }

    /// Calls the types render method.
    pub fn render(&mut self, ctx: &RenderingContext, viewport: Viewport) {
        match self {
            Self::Confirm(prompt) => prompt.render(ctx, viewport),
            Self::Search(prompt) => prompt.render(ctx, viewport),
        }
    }
}

/// A callback that is called when the prompt is done.
pub type PromptCallback = dyn FnOnce(&mut Editor, PromptResponse) -> Result<(), editor::Error>;

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
    MoveCursor(Point),
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
    pub prompt: PromptType,
    /// The callback to execute when the prompt is done.
    pub callback: Box<PromptCallback>,
}

#[derive(Default)]
pub struct PromptManager {
    pub active_prompt: Option<ActivePrompt>,
}

impl PromptManager {
    /// Shows a new prompt.
    pub fn show_prompt<F>(&mut self, prompt: PromptType, callback: F)
    where
        F: FnMut(&mut Editor, PromptResponse) -> Result<(), editor::Error> + 'static,
    {
        self.active_prompt = Some(ActivePrompt {
            prompt,
            callback: Box::new(callback),
        })
    }
}
