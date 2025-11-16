use std::io::{self, Stdout, Write};

use crossterm::{
    cursor, event, queue,
    terminal::{self, ClearType},
};

pub type Error = io::Error;

pub type Result<T> = std::result::Result<T, Error>;

// TODO: Convert into implementor of trait.
#[derive(Debug)]
pub struct TerminalBackend {
    stdout: Stdout,
}

impl TerminalBackend {
    /// Initializes the terminal backend.
    pub fn initialize() -> Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        queue!(
            stdout,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All),
        )?;
        Ok(Self { stdout })
    }

    /// Deinitializes the terminal backend.
    pub fn deinitialize(&mut self) -> Result<()> {
        queue!(
            self.stdout,
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Returns the size of the terminal viewport.
    pub fn size(&self) -> Result<(usize, usize)> {
        let (cols, rows) = crossterm::terminal::size()?;
        Ok((cols as usize, rows as usize))
    }

    /// Clears the current line.
    pub fn clear_line(&mut self) -> Result<()> {
        queue!(self.stdout, terminal::Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// Clears the terminal viewport.
    pub fn clear_all(&mut self) -> Result<()> {
        queue!(self.stdout, terminal::Clear(ClearType::All))?;
        Ok(())
    }

    /// Writes text to the terminal.
    pub fn write(&mut self, s: &str) -> Result<()> {
        // TODO: Instead of writing directly, render to a frame buffer and diff with previous frame.
        write!(self.stdout, "{s}")?;
        Ok(())
    }

    /// Flushes the terminal output.
    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    /// Updates the cursor position on screen.
    pub fn move_cursor(&mut self, col: usize, row: usize) -> Result<()> {
        let col = col.min(u16::MAX as usize) as u16;
        let row = row.min(u16::MAX as usize) as u16;
        queue!(self.stdout, cursor::MoveTo(col, row))?;
        Ok(())
    }

    /// Hides the cursor.
    pub fn hide_cursor(&mut self) -> Result<()> {
        queue!(self.stdout, cursor::Hide)?;
        Ok(())
    }

    /// Shows the cursor.
    pub fn show_cursor(&mut self) -> Result<()> {
        queue!(self.stdout, cursor::Show)?;
        Ok(())
    }
}
