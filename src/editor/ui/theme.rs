use std::collections::HashMap;

use serde::Deserialize;

use crate::editor::ui::{style::Style, theme::highlight_group::HighlightGroup};

pub mod highlight_group;
pub mod registry;

#[derive(Deserialize, Debug, Clone)]
pub struct ThemeEntry {
    #[serde(flatten)]
    pub style: Style,
    pub parent: Option<HighlightGroup>,
}

impl ThemeEntry {
    /// Merges another theme entry into this one. Conflicting values will be overwritten.
    pub fn merge_onto(&mut self, other: Self) {
        // Don't merge if this entry has a parent.
        if self.parent.is_some() {
            return;
        } else if other.parent.is_some() {
            self.parent = other.parent;
        }
        self.style = other.style.force_applied(self.style);
    }
}

// A theme that has optional inheritence.
#[derive(Deserialize, Debug, Clone)]
pub struct RawTheme {
    pub name: String,
    pub inherits: Option<String>,
    pub groups: HashMap<HighlightGroup, ThemeEntry>,
}

impl From<RawTheme> for Theme {
    fn from(value: RawTheme) -> Self {
        Theme {
            groups: value.groups,
        }
    }
}

// A resolved theme ready for use in the editor.
#[derive(Debug, Clone)]
pub struct Theme {
    pub groups: HashMap<HighlightGroup, ThemeEntry>,
}

impl Theme {
    /// Resolves the style for the given highlight group.
    pub fn resolve(&self, group: &HighlightGroup) -> Style {
        let mut style = Style::default();
        let mut current = Some(group);

        while let Some(name) = current {
            if let Some(entry) = self.groups.get(name) {
                style.apply(entry.style);
                current = entry.parent.as_ref();
            } else {
                break;
            }
        }
        style
    }

    /// Merges this theme over another theme.
    pub fn merge_onto(&mut self, other: &Theme) {
        for (k, v) in other.groups.clone() {
            self.groups
                .entry(k)
                .and_modify(|entry| entry.merge_onto(v.clone()))
                .or_insert(v);
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        let groups = highlight_group::all_highlight_groups()
            .into_iter()
            .collect();
        Self { groups }
    }
}
