use crate::editor::{
    command::{Command, CommandArgs, CommandRegistry},
    renderer::{
        Renderable, RenderingContext,
        frame::{Line, Span},
        style::Style,
        viewport::Viewport,
    },
};

/// Information about a command.
#[derive(Debug, Clone, Copy)]
pub struct CommandInfo {
    /// The name of the command.
    pub name: &'static str,
    /// A description of the command.
    pub description: &'static str,
}

impl From<&dyn Command> for CommandInfo {
    fn from(value: &dyn Command) -> Self {
        CommandInfo {
            name: value.name(),
            description: value.description(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandPalette {
    /// The current command query. The query should be a space-seperated list beginning with the
    /// name of the command to execute and ending with the arguments to pass to the command.
    query: String,
    /// The index of the currently selected command.
    selected_index: usize,
    /// A list of filtered commands based on the current query.
    filtered_commands: Vec<CommandInfo>,
    /// All commands that are available.
    commands: Vec<CommandInfo>,
}

impl CommandPalette {
    /// The prefix to render in the query prompt.
    const QUERY_PREIFX: &str = "> ";

    /// Returns a new command palette populated with all commands registered in the given
    /// [`CommandRegistry`].
    pub fn new(registry: &CommandRegistry) -> Self {
        let commands: Vec<_> = registry
            .get_all_commands()
            .map(|c| CommandInfo::from(c.as_ref()))
            .collect();
        let filtered_commands = commands.clone();

        Self {
            query: String::new(),
            selected_index: 0,
            filtered_commands,
            commands,
        }
    }

    /// Returns the name of the currently selected command.
    pub fn get_selected_command(&self) -> Option<&CommandInfo> {
        self.filtered_commands.get(self.selected_index)
    }

    pub fn get_filtered_command(&self, index: usize) -> Option<&CommandInfo> {
        self.filtered_commands.get(index)
    }

    /// Returns the name of the command extracted from the current query.
    pub fn command_query(&self) -> &str {
        self.query.split_whitespace().next().unwrap_or_default()
    }

    /// Parses the current query into a command name and its arguments and returns it.
    pub fn parse_query(&self) -> (String, CommandArgs) {
        let command = self
            .get_selected_command()
            .map(|c| c.name.to_string())
            .unwrap_or(self.query.clone());

        let args = self.query.split_whitespace().skip(1).collect::<Vec<_>>();
        let command_args = CommandArgs::new(args);
        (command, command_args)
    }

    /// Updates the list of filtered commands based on the current query. Uses substring matching
    /// to filter commands.
    pub fn update_filtered_commands(&mut self) {
        let command_query = self.command_query();
        self.filtered_commands = self
            .commands
            .iter()
            .filter(|c| c.name.contains(command_query))
            .cloned()
            .collect();

        // Make sure the selected index is still valid.
        let last_command_index = self.filtered_commands.len().saturating_sub(1);
        self.selected_index = self.selected_index.min(last_command_index);
    }

    /// Inserts a character into the current query.
    pub fn insert_char(&mut self, c: char) {
        self.query.push(c);
        self.update_filtered_commands();
    }

    /// Deletes a character from the current query.
    pub fn delete_char(&mut self) {
        self.query.pop();
        self.update_filtered_commands();
    }

    /// Updates the query to the given string without updating the filtered commands.
    pub fn set_query(&mut self, query: &str) {
        self.query = query.to_string();
    }

    /// Autocompletes the current query to the currently selected command.
    pub fn autocomplete(&mut self) {
        if let Some(command) = self.get_selected_command() {
            self.query = command.name.to_string();
        }
    }

    /// Autocompletes the current query to the currently selected command, or selects the next
    /// index if the query is already autocompleted.
    pub fn autocomplete_or_next(&mut self) {
        let Some(selected_command) = self.get_selected_command().map(|c| c.name) else {
            return;
        };
        if self.query == selected_command {
            self.select_next_command();
        } else {
            self.autocomplete();
        }
    }

    /// Selects the next command in the command palette, wrapping around at the end.
    pub fn select_next_command(&mut self) {
        let next_index = (self.selected_index + 1) % self.filtered_commands.len();
        if let Some(selected_command) = self.get_filtered_command(next_index).map(|c| c.name) {
            self.set_query(selected_command);
            self.selected_index = next_index;
        }
    }

    /// Selects the previous command in the command palette, wraoping around at the start. Returns
    /// the name of the selected command.
    pub fn select_prev_command(&mut self) {
        let prev_index = if self.selected_index == 0 {
            self.filtered_commands.len().saturating_sub(1)
        } else {
            self.selected_index.saturating_sub(1)
        };

        if let Some(selected_command) = self.get_filtered_command(prev_index).map(|c| c.name) {
            self.set_query(selected_command);
            self.selected_index = prev_index;
        }
    }

    /// Clears the current query, then resets the selected index and the filtered commands.
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.update_filtered_commands();
        self.selected_index = 0;
    }
}

impl Renderable for CommandPalette {
    fn render(&self, _ctx: &RenderingContext, mut viewport: Viewport) {
        // Render the query prompt.
        let text = format!("{}{}", Self::QUERY_PREIFX, self.query);
        viewport.put_line(
            viewport.height().saturating_sub(1),
            Line::new(viewport.width(), vec![Span::new(&text)]),
        );

        // Render the command list above the query prompt.
        for i in 0..self.filtered_commands.len() {
            let command = self.filtered_commands.get(i);
            if let Some(command) = command {
                let row = viewport.height().saturating_sub(i + 2);

                // TODO: Show description somwhere, maybe in the status bar.
                let style = if i == self.selected_index {
                    Style::new().bold()
                } else {
                    Style::default()
                };
                viewport.put_line(
                    row,
                    Line::new(
                        viewport.width(),
                        vec![Span::new(command.name).with_style(style)],
                    ),
                );
            }
        }
    }
}
