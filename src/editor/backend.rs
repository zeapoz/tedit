use std::io::stdout;

use crossterm::{
    cursor, event, execute,
    terminal::{self, ClearType},
};

use crate::editor::Result;

#[derive(Debug)]
pub struct TerminalBackend;

impl TerminalBackend {
    /// Initializes the terminal backend.
    pub fn initialize() -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(
            std::io::stdout(),
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All),
        )?;
        Ok(())
    }

    /// Deinitializes the terminal backend.
    pub fn deinitialize() -> Result<()> {
        execute!(
            std::io::stdout(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Clears the terminal viewport.
    pub fn clear() -> Result<()> {
        execute!(std::io::stdout(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    /// Updates the cursor position on screen.
    pub fn move_cursor(row: u16, col: u16) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(row, col))?;
        Ok(())
    }

    /// Returns the size of the terminal viewport.
    pub fn size() -> Result<(u16, u16)> {
        let size = crossterm::terminal::size()?;
        Ok(size)
    }
}
