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
            stdout(),
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
            stdout(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Clears the terminal viewport.
    pub fn clear() -> Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    /// Updates the cursor position on screen.
    pub fn move_cursor(row: u16, col: u16) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(row, col))?;
        Ok(())
    }

    /// Hides the cursor.
    pub fn hide_cursor() -> Result<()> {
        execute!(stdout(), cursor::Hide)?;
        Ok(())
    }

    /// Shows the cursor.
    pub fn show_cursor() -> Result<()> {
        execute!(stdout(), cursor::Show)?;
        Ok(())
    }

    /// Returns the size of the terminal viewport.
    pub fn size() -> Result<(u16, u16)> {
        let size = crossterm::terminal::size()?;
        Ok(size)
    }
}
