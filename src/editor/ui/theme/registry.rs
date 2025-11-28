use std::{collections::HashMap, fs, io, path::Path, sync::Arc};

use thiserror::Error;

use crate::editor::ui::theme::{RawTheme, Theme};

pub const DEFAULT_THEME_NAME: &'static str = "default";

const KANAGAWA_THEME: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/themes/kanagawa.toml"));

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("could not parse theme: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone)]
pub struct ThemeRegistry {
    pub themes: HashMap<String, Arc<Theme>>,
}

impl ThemeRegistry {
    /// Loads a new theme at the given path into the registry.
    pub fn load_theme_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let toml = fs::read_to_string(path)?;
        self.parse_and_load_theme(&toml)
    }

    /// Parses a TOML string and loads the theme into the registry.
    pub fn parse_and_load_theme(&mut self, toml: &str) -> Result<(), Error> {
        let raw: RawTheme = toml::from_str(&toml).map_err(|e| Error::ParseError(e.to_string()))?;
        let name = raw.name.clone();

        // Merge with parent if inherits is specified.
        let theme = if let Some(ref inherits) = raw.inherits {
            if let Some(parent) = self.themes.get(inherits) {
                let mut theme: Theme = raw.into();
                theme.merge_onto(parent);
                theme
            } else {
                return Err(Error::ParseError(format!(
                    "could not find parent theme: {inherits}"
                )));
            }
        } else {
            raw.into()
        };

        self.themes.insert(name, Arc::new(theme));
        Ok(())
    }

    /// Loads all builtin themes into the registry.
    pub fn load_builtin_themes(&mut self) -> Result<(), Error> {
        self.parse_and_load_theme(KANAGAWA_THEME)
    }

    /// Returns the default theme.
    pub fn get_default_theme(&self) -> Arc<Theme> {
        self.themes.get(DEFAULT_THEME_NAME).unwrap().clone()
    }

    /// Returns a list of all loaded themes.
    pub fn list_themes(&self) -> Vec<String> {
        self.themes.keys().map(|k| k.to_string()).collect()
    }
}

impl Default for ThemeRegistry {
    fn default() -> Self {
        let mut themes = HashMap::default();
        themes.insert(DEFAULT_THEME_NAME.to_string(), Arc::new(Theme::default()));
        Self { themes }
    }
}
