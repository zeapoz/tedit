use std::io::{self, Stdout, Write};

use crossterm::{
    cursor,
    event::{self, Event},
    queue,
    style::{self, Attribute},
    terminal::{self, ClearType},
};

use crate::editor::renderer::style::{Color, FontIntensity, ResolvedStyle};

pub type Error = io::Error;

pub type Result<T> = std::result::Result<T, Error>;

// TODO: Convert into implementor of trait.
/// The backend for handling input and terminal size.
#[derive(Debug)]
pub struct EditorBackend;

impl EditorBackend {
    /// Returns the size of the terminal viewport.
    pub fn size(&self) -> Result<(usize, usize)> {
        let (cols, rows) = terminal::size()?;
        Ok((cols as usize, rows as usize))
    }

    /// Reads and returns an event from the backend.
    pub fn read_event(&self) -> Result<Event> {
        event::read()
    }
}

/// The backend for rendering to the terminal.
#[derive(Debug)]
pub struct RenderingBackend {
    stdout: Stdout,
}

impl RenderingBackend {
    /// Initializes the terminal backend.
    pub fn initialize() -> Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        queue!(
            stdout,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture,
            cursor::MoveTo(0, 0),
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

    /// Writes a character to the terminal.
    pub fn write_char(&mut self, c: char) -> Result<()> {
        write!(self.stdout, "{c}")?;
        Ok(())
    }

    /// Writes text to the terminal.
    pub fn write(&mut self, s: &str) -> Result<()> {
        write!(self.stdout, "{s}")?;
        Ok(())
    }

    /// Sets the bold style.
    pub fn set_style(&mut self, style: ResolvedStyle) -> Result<()> {
        queue!(
            self.stdout,
            style::SetForegroundColor(style.fg.into()),
            style::SetBackgroundColor(style.bg.into()),
        )?;

        self.write(&style.to_string())?;
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

impl From<Color> for crossterm::style::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Reset => style::Color::Reset,
            Color::Black => style::Color::Black,
            Color::DarkGrey => style::Color::DarkGrey,
            Color::Red => style::Color::Red,
            Color::DarkRed => style::Color::DarkRed,
            Color::Green => style::Color::Green,
            Color::DarkGreen => style::Color::DarkGreen,
            Color::Yellow => style::Color::Yellow,
            Color::DarkYellow => style::Color::DarkYellow,
            Color::Blue => style::Color::Blue,
            Color::DarkBlue => style::Color::DarkBlue,
            Color::Magenta => style::Color::Magenta,
            Color::DarkMagenta => style::Color::DarkMagenta,
            Color::Cyan => style::Color::Cyan,
            Color::DarkCyan => style::Color::DarkCyan,
            Color::White => style::Color::White,
            Color::Grey => style::Color::Grey,
            Color::Rgb { r, g, b } => style::Color::Rgb { r, g, b },
            Color::AnsiValue(v) => style::Color::AnsiValue(v),
        }
    }
}

impl From<FontIntensity> for Attribute {
    fn from(value: FontIntensity) -> Self {
        match value {
            FontIntensity::Normal => Attribute::NormalIntensity,
            FontIntensity::Bold => Attribute::Bold,
            FontIntensity::Dim => Attribute::Dim,
        }
    }
}

// TODO: Queue as a command instead.
#[allow(clippy::to_string_trait_impl)]
impl ToString for ResolvedStyle {
    fn to_string(&self) -> String {
        let mut s = String::new();

        let intensity: Attribute = self.intensity.into();
        s.push_str(&intensity.to_string());

        if self.underline {
            s.push_str(&Attribute::Underlined.to_string());
        } else {
            s.push_str(&Attribute::NoUnderline.to_string());
        }
        s
    }
}
