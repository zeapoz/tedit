use serde::Deserialize;
use std::sync::{Arc, LazyLock};

use crate::editor::{
    Color,
    ui::theme::{Style, ThemeEntry},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HighlightGroup(Arc<str>);

impl HighlightGroup {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self(Arc::from(name.as_ref()))
    }

    /// Returns the name of this highlight group.
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for HighlightGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(HighlightGroup::new(s))
    }
}

// A macro to generate highlight groups.
macro_rules! highlight_groups {
    (
        $(
            ($id:ident => $name:expr, $style:expr $(, parent: $parent:expr)?)
        ),* $(,)?
    ) => {
        $(
            pub static $id: LazyLock<HighlightGroup> =
                LazyLock::new(|| HighlightGroup::new($name));
        )*

        pub fn all_highlight_groups() -> Vec<(HighlightGroup, ThemeEntry)> {
            vec![
                $(
                    (
                        HighlightGroup::new($name),
                        ThemeEntry {
                            style: $style,
                            parent: None $( .or(Some(HighlightGroup::new($parent))) )?,
                        }
                    )
                ),*
            ]
        }
    };
}

// Default theme colors.
const FG_0: Color = Color::rgb(220, 220, 220);
const FG_1: Color = Color::rgb(140, 140, 140);
const BG_0: Color = Color::rgb(30, 30, 30);
const BG_1: Color = Color::rgb(40, 40, 40);
const BG_2: Color = Color::rgb(50, 50, 50);
const GREEN: Color = Color::rgb(100, 200, 0);
const ORANGE: Color = Color::rgb(255, 100, 0);
const RED: Color = Color::rgb(255, 0, 0);

highlight_groups! {
    // Base UI Colors.
    (HL_UI => "ui", Style::new().bg(BG_0).fg(FG_0)),
    // Status bar.
    (HL_UI_STATUSBAR => "ui.statusbar", Style::new().bg(BG_1), parent: "ui"),
    (HL_UI_STATUSBAR_MODE_INSERT => "ui.statusbar.mode.insert", Style::new().bg(GREEN).fg(BG_0).bold(), parent: "ui.statusbar"),
    (HL_UI_STATUSBAR_MODE_COMMAND => "ui.statusbar.mode.command", Style::new().bg(ORANGE).fg(BG_0).bold(), parent: "ui.statusbar"),
    (HL_UI_STATUSBAR_MESSAGE_ERROR => "ui.statusbar.message.error", Style::new().bg(RED).fg(BG_0).bold(), parent: "ui.statusbar"),
    // Pane.
    (HL_UI_PANE => "ui.pane", Style::default(), parent: "ui"),
    (HL_UI_PANE_GUTTER => "ui.pane.gutter", Style::new().bg(BG_1).fg(FG_1), parent: "ui.pane"),
    (HL_UI_PANE_GUTTER_CURSOR => "ui.pane.gutter.cursor", Style::new().fg(GREEN).bold(), parent: "ui.pane.gutter"),
    // Overlay layers.
    (HL_UI_OVERLAY => "ui.overlay", Style::new().bg(BG_2), parent: "ui"),
    (HL_UI_COMMAND_PROMPT => "ui.overlay.command_prompt", Style::default(), parent: "ui.overlay"),
    (HL_UI_COMMAND_PROMPT_SELECTED => "ui.overlay.command_prompt.selected", Style::new().fg(ORANGE).bold(), parent: "ui.overlay.command_prompt"),
}
