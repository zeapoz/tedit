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
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}

impl Color {
    /// Returns a color from an rgb value.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// Returns a color from a hex string.
    pub fn hex(s: &str) -> Self {
        let s = s.trim_start_matches('#');
        let r = u8::from_str_radix(&s[0..2], 16).unwrap();
        let g = u8::from_str_radix(&s[2..4], 16).unwrap();
        let b = u8::from_str_radix(&s[4..6], 16).unwrap();
        Self::Rgb { r, g, b }
    }
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

    /// Applies the given style to this style. Only unset values in the current style will get
    /// overwritten by the given style.
    pub fn apply(&mut self, other: Self) {
        self.fg = self.fg.or(other.fg);
        self.bg = self.bg.or(other.bg);
        self.intensity = self.intensity.or(other.intensity);
        self.underline = self.underline.or(other.underline);
    }

    /// Applies the given style to this style and overwrites all set values from the given style.
    pub fn force_apply(&mut self, other: Self) {
        self.fg = other.fg.or(self.bg);
        self.bg = other.bg.or(self.bg);
        self.intensity = other.intensity.or(self.intensity);
        self.underline = other.underline.or(self.underline);
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
