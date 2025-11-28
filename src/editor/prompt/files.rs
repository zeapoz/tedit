use crossterm::event::{KeyCode, KeyEvent};
use ignore::WalkBuilder;
use std::path::Path;
use std::path::PathBuf;

use crate::editor::ui::geometry::anchor::Anchor;
use crate::editor::{
    prompt::{Prompt, PromptResponse, PromptStatus},
    ui::{
        component::{Component, RenderingContext},
        geometry::rect::Rect,
        theme::highlight_group::{
            HL_UI_COMMAND_PROMPT, HL_UI_COMMAND_PROMPT_SELECTED, HL_UI_OVERLAY,
        },
        viewport::Viewport,
        widget::{container::ContainerBuilder, span::Span},
    },
};

/// Reads the contents of a directory recursively, excluding files ignored by `.gitignore`.
fn read_dir_recursively<P: AsRef<Path>>(
    path: P,
    files: &mut Vec<PathBuf>,
) -> Result<(), ignore::Error> {
    let walker = WalkBuilder::new(path).git_ignore(true).build();
    for entry in walker {
        let entry_path = entry?.path().to_path_buf();
        if entry_path.is_file() {
            files.push(entry_path);
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct FilesPrompt {
    query: String,
    files: Vec<PathBuf>,
    filtered_files: Vec<PathBuf>,
    selected_index: usize,
}

impl FilesPrompt {
    const QUERY_PROMPT: &str = "Find file: ";
    const MAX_ENTRIES: usize = 20;

    pub fn new(dir: Option<&str>) -> Result<Self, ignore::Error> {
        let dir = dir.unwrap_or(".");

        let mut files = Vec::new();
        read_dir_recursively(dir, &mut files)?;
        files.sort();
        let filtered_files = files.clone();

        Ok(Self {
            query: String::new(),
            files,
            filtered_files,
            selected_index: 0,
        })
    }

    /// Filters the files based on the query.
    fn filter_files(&mut self) {
        if self.query.is_empty() {
            self.filtered_files = self.files.clone();
        } else {
            self.filtered_files = self
                .files
                .iter()
                .filter(|path| {
                    path.to_str()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&self.query.to_lowercase())
                })
                .cloned()
                .collect();
        }
        self.selected_index = 0;
    }
}

impl Prompt for FilesPrompt {
    fn process_key(&mut self, event: &KeyEvent) -> PromptStatus {
        match event.code {
            KeyCode::Esc => PromptStatus::Done(PromptResponse::Cancel),
            KeyCode::Enter => {
                if let Some(selected_file) = self.filtered_files.get(self.selected_index) {
                    return PromptStatus::Done(PromptResponse::File(selected_file.clone()));
                }
                PromptStatus::Done(PromptResponse::Cancel)
            }
            KeyCode::Char(c) => {
                self.query.push(c);
                self.filter_files();
                PromptStatus::Changed
            }
            KeyCode::Backspace => {
                if !self.query.is_empty() {
                    self.query.pop();
                    self.filter_files();
                }
                PromptStatus::Changed
            }
            KeyCode::Down => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                PromptStatus::Changed
            }
            KeyCode::Up => {
                if !self.filtered_files.is_empty()
                    && self.selected_index < self.filtered_files.len() - 1
                {
                    self.selected_index += 1;
                }
                PromptStatus::Changed
            }
            _ => PromptStatus::Pending,
        }
    }
}

impl Component for FilesPrompt {
    fn render(&mut self, ctx: &RenderingContext, mut viewport: Viewport) {
        let style = ctx.theme.resolve(&HL_UI_OVERLAY);
        let text_style = ctx.theme.resolve(&HL_UI_COMMAND_PROMPT);
        let focused_style = ctx.theme.resolve(&HL_UI_COMMAND_PROMPT_SELECTED);

        let query_str = format!("{}{}", Self::QUERY_PROMPT, self.query);
        let query_span = Span::new(&query_str).with_style(style);

        let query_container = ContainerBuilder::default()
            .with_child(query_span)
            .with_width(Some(viewport.width()))
            .with_style(style)
            .build();
        viewport.put_widget(viewport.height().saturating_sub(1), query_container);

        for (i, file) in self
            .filtered_files
            .iter()
            .enumerate()
            .take(viewport.height() - 1)
        {
            let row = viewport.height().saturating_sub(i + 2);

            let file_name = file
                .to_str()
                .unwrap_or("[invalid file name]")
                .trim_start_matches("./");
            let span_style = if i == self.selected_index {
                focused_style
            } else {
                text_style
            };
            let span = Span::new(file_name).with_style(span_style);
            let container = ContainerBuilder::default()
                .with_child(span)
                .with_width(Some(viewport.width()))
                .with_style(if i == self.selected_index {
                    focused_style
                } else {
                    style
                })
                .build();
            viewport.put_widget(row, container);
        }
    }

    fn rect(&self, parent: Rect) -> Rect {
        let height = (self.filtered_files.len() + 1)
            .min(Self::MAX_ENTRIES)
            .max(1);

        Rect::new(0, 0, parent.width, height)
            .anchored_on(parent, Anchor::BottomLeft)
            .offset(0, -1)
    }
}
