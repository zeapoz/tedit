use std::io::{self, Write, stdout};

use crossterm::{
    cursor, event, execute,
    terminal::{self, ClearType},
};

pub type Error = io::Error;

pub type Result<T> = std::result::Result<T, Error>;

// TODO: Convert into implementor of trait.
#[derive(Debug)]
pub struct TerminalBackend;

// TODO: Queue terminal operatitons instead of executing.
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
    pub fn move_cursor(&self, col: usize, row: usize) -> Result<()> {
        let col = col.min(u16::MAX as usize) as u16;
        let row = row.min(u16::MAX as usize) as u16;
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
    pub fn size(&self) -> Result<(usize, usize)> {
        let (cols, rows) = crossterm::terminal::size()?;
        Ok((cols as usize, rows as usize))
    }
}
