use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use crossterm::event::{Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use thiserror::Error;

use crate::editor::{
    backend::EditorBackend,
    buffer::{BufferEntry, manager::BufferManager},
    command::{CommandRegistry, register_commands},
    command_palette::CommandPalette,
    config::Config,
    keymap::Keymap,
    pane::{cursor::CursorMovement, manager::PaneManager},
    prompt::{
        PromptAction, PromptManager, PromptResponse, PromptStatus, PromptType,
        confirm::ConfirmPrompt,
    },
    renderer::{Renderer, compositor::Compositor},
    ui::{
        component::{
            RenderingContext,
            status_bar::{Message, MessageType},
        },
        geometry::{point::Point, rect::Rect},
        style::Color,
        theme::{
            Theme,
            highlight_group::{
                HL_UI_STATUSBAR_MODE_COMMAND, HL_UI_STATUSBAR_MODE_INSERT, HighlightGroup,
            },
            registry::ThemeRegistry,
        },
    },
};

pub mod backend;
mod buffer;
pub mod command;
mod command_palette;
pub mod config;
mod keymap;
mod pane;
mod prompt;
mod renderer;
pub mod ui;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BufferError(#[from] buffer::Error),
    #[error(transparent)]
    BackendError(#[from] backend::Error),
    #[error(transparent)]
    ThemeRegistryError(#[from] ui::theme::registry::Error),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// A mode for editing text.
    #[default]
    Insert,
    /// A mode for running commands.
    Command,
}

impl From<Mode> for &HighlightGroup {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Insert => &HL_UI_STATUSBAR_MODE_INSERT,
            Mode::Command => &HL_UI_STATUSBAR_MODE_COMMAND,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Mode::Insert => "INS",
            Mode::Command => "CMD",
        };
        write!(f, "{s}")
    }
}

pub struct Editor {
    /// The buffer manager.
    buffer_manager: BufferManager,
    /// The pane manager.
    pane_manager: PaneManager,
    /// The editor backend.
    backend: EditorBackend,
    /// The renderer.
    renderer: Renderer,
    /// The compositor.
    compositor: Compositor,
    /// The command registry.
    command_registry: CommandRegistry,
    /// The command palette.
    command_palette: CommandPalette,
    /// The mapping from key to command.
    keymap: Keymap,
    /// The prompt manager.
    prompt_manager: PromptManager,
    /// The theme registry for loading and managing themes.
    theme_registry: ThemeRegistry,
    /// The current theme.
    theme: Arc<Theme>,
    // TODO: Make this into new editor state struct.
    /// The editor configuration.
    pub config: Config,
    /// The current mode.
    pub mode: Mode,
    /// An optional message to display in the status bar.
    pub status_message: Option<Message>,
    /// Whether the editor should quit.
    pub should_quit: bool,
}

impl Editor {
    /// Returns a new editor.
    pub fn new<P: AsRef<Path>>(
        files: Option<Vec<P>>,
        config_path: Option<PathBuf>,
    ) -> Result<Self> {
        let renderer = Renderer::initialize()?;
        let backend = EditorBackend;

        let mut status_message = None;

        // Try to load the configuration.
        let config = Config::load(config_path).unwrap_or_else(|e| {
            let err_message = Message::new(&format!(
                "Failed to load configuration, using default configuration: {e}"
            ))
            .with_type(MessageType::Error);
            status_message = Some(err_message);
            Config::default()
        });

        // Open a buffer via the buffer manager.
        let mut buffer_manager = BufferManager::default();
        let buffers = if let Some(paths) = files {
            paths
                .into_iter()
                .map(|path| -> Result<BufferEntry> {
                    let buffer = buffer_manager.open_new_or_existing_file(path)?;
                    Ok(buffer)
                })
                .collect::<Result<_>>()?
        } else {
            vec![buffer_manager.open_empty_file()]
        };

        let mode = Mode::default();
        let compositor = Compositor::default();

        // Initialize commands and keybindings.
        let mut command_registry = CommandRegistry::new();
        register_commands(&mut command_registry);

        let command_palette = CommandPalette::new(&command_registry);
        let prompt_manager = PromptManager::default();

        // Create a new pane and add it to the pane manager.
        let mut pane_manager = PaneManager::new();
        for buffer in buffers {
            pane_manager.open_pane(buffer);
        }

        // Try to load a theme, otherwise fallback to the default.
        let mut theme_registry = ThemeRegistry::default();
        theme_registry.load_builtin_themes()?;

        let theme = if let Some(ref name) = config.editor.theme {
            match theme_registry.themes.get(name) {
                Some(theme) => theme.clone(),
                None => {
                    status_message = Some(
                        Message::new(&format!("Theme not found: {name}"))
                            .with_type(MessageType::Error),
                    );
                    theme_registry.get_default_theme()
                }
            }
        } else {
            theme_registry.get_default_theme()
        };

        Ok(Self {
            buffer_manager,
            pane_manager,
            backend,
            renderer,
            compositor,
            command_registry,
            command_palette,
            keymap: Keymap::default(),
            prompt_manager,
            theme_registry,
            theme,
            mode,
            status_message,
            should_quit: false,
            config,
        })
    }

    /// Opens a new file and loads its contents into the buffer manager and the pane manager.
    pub fn open_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let buffer = self.buffer_manager.open_new_or_existing_file(path)?;
        self.pane_manager.open_pane(buffer);
        Ok(())
    }

    /// Runs the editor main loop.
    pub fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.update()?;
            self.render()?;

            let event = self.backend.read_event()?;

            // Handle prompt input first.
            if self.prompt_manager.active_prompt.is_some() {
                self.handle_prompt_input(event);
                continue;
            }

            match self.mode {
                Mode::Insert => self.handle_insert_mode_input(event),
                Mode::Command => self.handle_command_mode_input(event),
            };

            // Only quit if there is no active prompt.
            if self.should_quit && self.prompt_manager.active_prompt.is_none() {
                break;
            }
        }
        self.exit()
    }

    /// Handles event input in insert mode.
    pub fn handle_insert_mode_input(&mut self, event: Event) {
        match event {
            Event::Key(event) => {
                if let Some(command) = self.keymap.get(&event).cloned() {
                    if let Err(err) = command.execute(self) {
                        self.show_err_message(&err.to_string());
                    }
                } else if let KeyCode::Char(c) = event.code {
                    // TODO: Replace by a command.
                    self.pane_manager.active_mut().insert_char(c);
                }
            }
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: _column,
                row: _row,
                ..
            }) => {
                // TODO: Implement a UI layer to map screen coordinates to components, so we can
                // redirect mouse events to the correct component.
                // self.pane_manager
                //     .active_mut()
                //     .click(column as usize, row as usize);
            }
            _ => {}
        }
    }

    /// Handles event input in command mode.
    pub fn handle_command_mode_input(&mut self, event: Event) {
        if let Event::Key(event) = event {
            // TODO: Implement command mode keybindings as commands with context (mode).
            match event.code {
                KeyCode::Esc => self.exit_command_mode(),
                KeyCode::Enter => {
                    let command_name = self.command_palette.command_query();
                    match self.command_palette.parse_query(&self.command_registry) {
                        Some(Ok(command)) => {
                            if let Err(err) = command.execute(self) {
                                self.show_err_message(&err.to_string());
                            }
                        }
                        Some(Err(e)) => self.show_err_message(&e.to_string()),
                        None => {
                            self.show_err_message(&format!("No such command found: {command_name}"))
                        }
                    }
                    self.exit_command_mode();
                }
                KeyCode::Tab => self.command_palette.autocomplete_or_next(),
                KeyCode::Char(c) => self.command_palette.insert_char(c),
                KeyCode::Down | KeyCode::BackTab => self.command_palette.select_prev_command(),
                KeyCode::Up => self.command_palette.select_next_command(),
                KeyCode::Backspace => self.command_palette.delete_char(),
                _ => {}
            }
        }
    }

    fn handle_prompt_input(&mut self, event: Event) {
        if let Event::Key(key) = event
            && let Some(active) = self.prompt_manager.active_prompt.as_mut()
        {
            let status = active.prompt.process_key(&key);
            match status {
                PromptStatus::Pending => {}
                PromptStatus::Changed => {
                    let action = active.prompt.on_changed();
                    if let PromptAction::MoveCursor(Point { col, row }) = action {
                        self.pane_manager
                            .active_mut()
                            .move_cursor(CursorMovement::Position(col, row));
                    }
                }
                PromptStatus::Done(response) => {
                    let active = self.prompt_manager.active_prompt.take().unwrap();
                    if let Err(err) = (active.callback)(self, response) {
                        self.show_err_message(&err.to_string());
                    }
                }
            }
        }
    }

    /// Shows a message in the status bar.
    pub fn show_message(&mut self, s: &str) {
        let message = Message::new(s);
        self.status_message = Some(message);
    }

    /// Shows an error message in the status bar.
    pub fn show_err_message(&mut self, s: &str) {
        let message = Message::new(s).with_type(MessageType::Error);
        self.status_message = Some(message);
    }

    /// Exits command mode and cleans up the stored query.
    pub fn exit_command_mode(&mut self) {
        self.command_palette.clear_query();
        self.mode = Mode::Insert;
    }

    /// Saves the active buffer.
    pub fn save_active_buffer<P: AsRef<Path>>(&mut self, path: Option<P>) -> Result<()> {
        let path = path.map(|p| p.as_ref().to_path_buf());

        // It a path was given, attempt to save the buffer to that path, prompting to overwrite if
        // the file already exists. Otherwise, save the buffer to the current path.
        if let Some(path) = path {
            // TODO: Use eyre to handle errors instead of long matches.
            if let Err(buffer::Error::SaveError(buffer::SaveError::FileAlreadyExists(_))) =
                self.pane_manager.active_mut().save_as(&path, false)
            {
                self.prompt_manager.show_prompt(
                    PromptType::Confirm(ConfirmPrompt::new(
                        "File already exists, do you want to overwrite it?",
                    )),
                    move |editor, response| {
                        if response == PromptResponse::Yes {
                            editor.pane_manager.active_mut().save_as(&path, true)?;
                        }
                        Ok(())
                    },
                )
            }
        } else {
            self.pane_manager.active_mut().save()?;
        }

        Ok(())
    }

    /// Closes the active pane. If the pane is dirty, prompts the user to save the pane
    /// before closing it.
    pub fn close_active_pane(&mut self) -> Result<()> {
        // If there are multiple panes with the same buffer id, only close the active pane.
        let active_buffer_id = self.pane_manager.active().buffer_id();
        if !self.pane_manager.is_unique(active_buffer_id) {
            self.pane_manager.close_active();
            return Ok(());
        }

        self.close_buffer(active_buffer_id)
    }

    /// Closes the buffer with the given id prompting the user to save the buffer if it is dirty.
    pub fn close_buffer(&mut self, id: usize) -> Result<()> {
        if !self.pane_manager.active().is_dirty() {
            self.pane_manager.close_active();
            self.buffer_manager.close(id);
            return Ok(());
        }

        self.prompt_manager.show_prompt(
            PromptType::Confirm(ConfirmPrompt::new(
                "Buffer contains unsaved changes, do you want to save before closing it?",
            )),
            move |editor, response| {
                match response {
                    PromptResponse::Yes => {
                        editor.save_active_buffer(None::<&str>)?;
                        editor.pane_manager.close_active();
                        editor.buffer_manager.close(id);
                    }
                    PromptResponse::No => {
                        editor.pane_manager.close_active();
                        editor.buffer_manager.close(id);
                    }
                    _ => return Ok(()),
                };
                Ok(())
            },
        );
        Ok(())
    }

    /// Exits the editor.
    pub fn exit(&mut self) -> Result<()> {
        self.renderer.deinitialize()?;
        Ok(())
    }

    /// Updates the state of the editor.
    pub fn update(&mut self) -> Result<()> {
        // Check if the message has timed out. If so, clear it.
        if let Some(message) = &self.status_message
            && message.timed_out()
        {
            self.status_message = None;
        }

        Ok(())
    }

    /// Creates a new rendering context from the editor and calls the renderer.
    pub fn render(&mut self) -> Result<()> {
        let (width, height) = self.backend.size()?;
        let editor_view = Rect::new(0, 0, width, height);
        let rendering_context = RenderingContext::new(&*self, editor_view);
        let frame = self.compositor.compose_frame(
            &rendering_context,
            &mut self.prompt_manager,
            &mut self.command_palette,
        );
        self.renderer.render(frame)?;
        Ok(())
    }
}
