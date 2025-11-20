/// A color in the terminal.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    #[default]
    Reset,
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
}

/// The font intensity.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FontIntensity {
    #[default]
    Normal,
    Bold,
    Dim,
}

/// The style of a single cell.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub intensity: Option<FontIntensity>,
    pub underline: Option<bool>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Sets the normal style.
    pub fn normal(mut self) -> Self {
        self.intensity = Some(FontIntensity::Normal);
        self
    }

    /// Sets the bold style.
    pub fn bold(mut self) -> Self {
        self.intensity = Some(FontIntensity::Bold);
        self
    }

    /// Sets the dim style.
    pub fn dim(mut self) -> Self {
        self.intensity = Some(FontIntensity::Dim);
        self
    }

    /// Sets the underline style.
    pub fn underline(mut self) -> Self {
        self.underline = Some(true);
        self
    }

    /// Merges the given style with this style. Only unset values in the current style will get
    /// overwritten by the given style.
    pub fn merge(mut self, other: Self) -> Self {
        self.fg = self.fg.or(other.fg);
        self.bg = self.bg.or(other.bg);
        self.intensity = self.intensity.or(other.intensity);
        self.underline = self.underline.or(other.underline);
        self
    }

    /// Resolves the style, replacing all `None` values with the default values.
    pub fn resolve(self) -> ResolvedStyle {
        ResolvedStyle {
            fg: self.fg.unwrap_or_default(),
            bg: self.bg.unwrap_or_default(),
            intensity: self.intensity.unwrap_or_default(),
            underline: self.underline.unwrap_or_default(),
        }
    }
}

/// A resolved style that can be used to render a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedStyle {
    pub fg: Color,
    pub bg: Color,
    pub intensity: FontIntensity,
    pub underline: bool,
}
