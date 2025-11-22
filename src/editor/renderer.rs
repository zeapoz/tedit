use crate::editor::{
    Editor, Mode,
    backend::{self, RenderingBackend},
    command_palette::CommandPalette,
    pane::manager::PaneManager,
    prompt::PromptType,
    renderer::{
        frame::{Cell, Frame, diff::FrameDiff},
        viewport::Viewport,
    },
    status_bar::StatusBar,
};

pub mod compositor;
pub mod frame;
pub mod layout;
pub mod style;
pub mod viewport;

// TODO: Make this cheaper to create. Instead of cloning everything, just clone the state needed
// for rendering.
/// A context for rendering objects.
pub struct RenderingContext {
    pub mode: Mode,
    pub pane_manager: PaneManager,
    pub status_bar: StatusBar,
    pub prompt: Option<PromptType>,
    pub command_palette: CommandPalette,
}

impl<'a> From<&'a Editor> for RenderingContext {
    fn from(value: &'a Editor) -> Self {
        let prompt = value
            .prompt_manager
            .active_prompt
            .as_ref()
            .map(|active| active.prompt.clone());

        Self {
            mode: value.mode,
            pane_manager: value.pane_manager.clone(),
            status_bar: value.status_bar.clone(),
            prompt,
            command_palette: value.command_palette.clone(),
        }
    }
}

/// A trait for types that can be rendered to the terminal.
pub trait Renderable {
    /// Renders the object to the terminal.
    fn render(&self, ctx: &RenderingContext, viewport: Viewport);
}

/// Responsible for rendering frames to the terminal.
#[derive(Debug)]
pub struct Renderer {
    backend: RenderingBackend,
    last_frame: Option<Frame>,
}

impl Renderer {
    /// Initializes a new compositor.
    pub fn initialize() -> Result<Self, backend::Error> {
        let backend = RenderingBackend::initialize()?;
        Ok(Self {
            backend,
            last_frame: None,
        })
    }

    /// deinitialize the compositor.
    pub fn deinitialize(&mut self) -> Result<(), backend::Error> {
        self.backend.deinitialize()
    }

    /// Renders the editor to the terminal.
    pub fn render(&mut self, frame: Frame) -> Result<(), backend::Error> {
        self.backend.hide_cursor()?;
        self.backend.move_cursor(0, 0)?;

        // If there is a previous frame, diff the current frame with it and render the differing
        // rows. Otherwise, render the entire frame row by row.
        if let Some(last) = &self.last_frame {
            let diff = FrameDiff::compute(last, &frame);
            self.render_frame_diff(diff)?;
        } else {
            for (row, cells) in frame.rows().enumerate() {
                self.backend.move_cursor(0, row)?;
                for cell in cells {
                    self.render_cell(cell)?;
                }
            }
        }

        if let Some((col, row)) = frame.cursor_position() {
            self.backend.move_cursor(col, row)?;
            self.backend.show_cursor()?;
        }

        self.backend.show_cursor()?;
        self.backend.flush()?;

        self.last_frame = Some(frame);
        Ok(())
    }

    /// Renders the frame diff between the previous frame and the current frame.
    fn render_frame_diff(&mut self, diff: FrameDiff) -> Result<(), backend::Error> {
        let mut current_row = None;
        let mut last_col = 0;
        let mut buffer = Vec::new();
        for diff_cell in &diff.cells {
            if Some(diff_cell.row) != current_row {
                // Flush buffer if we moved to a new row.
                if !buffer.is_empty() {
                    for cell in &buffer {
                        self.render_cell(cell)?;
                    }
                    buffer.clear();
                }
                // Move cursor to start of the new row.
                self.backend.move_cursor(diff_cell.col, diff_cell.row)?;
                current_row = Some(diff_cell.row);
                last_col = diff_cell.col.saturating_sub(1);
            }

            if diff_cell.col > last_col + 1 {
                // Flush buffer if non-adjacent.
                if !buffer.is_empty() {
                    for cell in &buffer {
                        self.render_cell(cell)?;
                    }
                    buffer.clear();
                }
                self.backend.move_cursor(diff_cell.col, diff_cell.row)?;
            }

            buffer.push(*diff_cell.cell);
            last_col = diff_cell.col;
        }

        // Flush last buffer
        if !buffer.is_empty() {
            for cell in &buffer {
                self.render_cell(cell)?;
            }
        }
        Ok(())
    }

    /// Renders a single cell to the terminal.
    fn render_cell(&mut self, cell: &Cell) -> Result<(), backend::Error> {
        // TODO: Optimize calls to `set_style` by diffing with previous and only queuing the
        // changes.
        let style = cell.style.resolve();
        self.backend.set_style(style)?;
        self.backend.write_char(cell.char)?;
        Ok(())
    }
}
