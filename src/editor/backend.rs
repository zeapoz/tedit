use std::io::{Write, stdout};

use crossterm::{
    cursor, event, execute,
    terminal::{self, ClearType},
};

use crate::editor::Result;

#[derive(Debug)]
pub struct TerminalBackend;

impl TerminalBackend {
    /// Initializes the terminal backend.
    pub fn initialize() -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            stdout(),
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All),
        )?;
        Ok(Self)
    }

    /// Deinitializes the terminal backend.
    pub fn deinitialize(&self) -> Result<()> {
        execute!(
            stdout(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Clears the current line.
    pub fn clear_line(&self) -> Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// Clears the terminal viewport.
    pub fn clear_all(&self) -> Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        Ok(())
    }

    /// Writes text to the terminal.
    pub fn write(&self, s: &str) -> Result<()> {
        write!(stdout(), "{s}")?;
        Ok(())
    }

    /// Flushes the terminal output.
    pub fn flush(&self) -> Result<()> {
        stdout().flush()?;
        Ok(())
    }

    /// Updates the cursor position on screen.
    pub fn move_cursor(&self, col: u16, row: u16) -> Result<()> {
        execute!(stdout(), cursor::MoveTo(col, row))?;
        Ok(())
    }

    /// Hides the cursor.
    pub fn hide_cursor(&self) -> Result<()> {
        execute!(stdout(), cursor::Hide)?;
        Ok(())
    }

    /// Shows the cursor.
    pub fn show_cursor(&self) -> Result<()> {
        execute!(stdout(), cursor::Show)?;
        Ok(())
    }

    /// Returns the size of the terminal viewport.
    pub fn size(&self) -> Result<(u16, u16)> {
        let size = crossterm::terminal::size()?;
        Ok(size)
    }
}
