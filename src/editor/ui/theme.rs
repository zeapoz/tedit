use std::collections::HashMap;

use crate::editor::ui::{style::Style, theme::highlight_group::HighlightGroup};

pub mod highlight_group;

#[derive(Debug, Clone)]
pub struct ThemeEntry {
    pub style: Style,
    pub parent: Option<HighlightGroup>,
}

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

    /// Returns the fallback theme.
    pub fn fallback() -> Self {
        let groups = highlight_group::all_highlight_groups()
            .into_iter()
            .collect();
        Theme { groups }
    }
}
