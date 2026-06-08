use crate::commands::{self, Category, Command};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    CategorySelect,
    CommandSelect,
    ParamInput,
    Preview,
}

pub struct App {
    // Navigation
    pub screen: Screen,
    pub categories: Vec<Category>,
    pub category_index: usize,

    // Commands for selected category
    pub commands: Vec<&'static Command>,
    pub command_index: usize,

    // Search
    pub search_query: String,
    pub search_active: bool,

    // Param filling
    pub current_param_index: usize,
    pub param_inputs: HashMap<String, String>,
    pub input_buffer: String,

    // Result
    pub resolved_command: String,
    pub final_command: Option<String>,
    pub output_file: Option<String>,

    // Pre-filled from CLI args
    pub preset_target: Option<String>,

    // Status bar message
    pub status_msg: String,
    pub status_is_error: bool,
}

impl App {
    pub fn new(
        module: Option<String>,
        target: Option<String>,
        output: Option<String>,
    ) -> Self {
        let categories = Category::all();

        // If a module was passed, jump straight to it
        let (screen, category_index) = if let Some(ref m) = module {
            let idx = categories
                .iter()
                .position(|c| Category::from_str(m).as_ref() == Some(c))
                .unwrap_or(0);
            (Screen::CommandSelect, idx)
        } else {
            (Screen::CategorySelect, 0)
        };

        let commands = commands::get_commands(&categories[category_index])
            .iter()
            .collect();

        Self {
            screen,
            categories,
            category_index,
            commands,
            command_index: 0,
            search_query: String::new(),
            search_active: false,
            current_param_index: 0,
            param_inputs: HashMap::new(),
            input_buffer: String::new(),
            resolved_command: String::new(),
            final_command: None,
            output_file: output,
            preset_target: target,
            status_msg: String::from("Use ↑↓ to navigate · Enter to select · / to search · q to quit"),
            status_is_error: false,
        }
    }

    pub fn select_category(&mut self) {
        self.commands = commands::get_commands(&self.categories[self.category_index])
            .iter()
            .collect();
        self.command_index = 0;
        self.search_query.clear();
        self.screen = Screen::CommandSelect;
        self.set_status("Enter to select command · Backspace to go back · / to search");
    }

    pub fn select_command(&mut self) {
        let cmd = self.filtered_commands()[self.command_index];
        self.param_inputs.clear();
        self.current_param_index = 0;
        self.input_buffer.clear();

        // Pre-fill target if provided
        if let Some(ref t) = self.preset_target.clone() {
            self.param_inputs.insert("target".to_string(), t.clone());
        }

        if cmd.params.is_empty() {
            // No params needed, jump straight to preview
            self.resolve_and_preview();
        } else {
            // Skip already pre-filled params
            self.advance_to_next_empty_param();
            if self.current_param_index >= cmd.params.len() {
                self.resolve_and_preview();
            } else {
                self.screen = Screen::ParamInput;
                self.prefill_default();
                self.set_status("Fill in the parameters · Enter to confirm · Esc to cancel");
            }
        }
    }

    pub fn current_command(&self) -> &'static Command {
        let filtered = self.filtered_commands_static();
        filtered[self.command_index]
    }

    pub fn filtered_commands(&self) -> Vec<&'static Command> {
        self.filtered_commands_static()
    }

    fn filtered_commands_static(&self) -> Vec<&'static Command> {
        if self.search_query.is_empty() {
            return self.commands.clone();
        }
        let q = self.search_query.to_lowercase();
        self.commands
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&q)
                    || c.description.to_lowercase().contains(&q)
                    || c.tags.iter().any(|t| t.contains(&q.as_str()))
            })
            .copied()
            .collect()
    }

    pub fn submit_param(&mut self) {
        let cmd = self.current_command();
        let param_key = cmd.params[self.current_param_index].key.to_string();
        let value = self.input_buffer.trim().to_string();

        if !value.is_empty() {
            self.param_inputs.insert(param_key, value);
        } else if let Some(default) = cmd.params[self.current_param_index].default {
            self.param_inputs.insert(param_key, default.to_string());
        }

        self.current_param_index += 1;
        self.input_buffer.clear();
        self.advance_to_next_empty_param();

        if self.current_param_index >= cmd.params.len() {
            self.resolve_and_preview();
        } else {
            self.prefill_default();
        }
    }

    fn advance_to_next_empty_param(&mut self) {
        let cmd = self.current_command();
        while self.current_param_index < cmd.params.len() {
            let key = cmd.params[self.current_param_index].key;
            if self.param_inputs.contains_key(key) {
                self.current_param_index += 1;
            } else {
                break;
            }
        }
    }

    fn prefill_default(&mut self) {
        let cmd = self.current_command();
        if self.current_param_index < cmd.params.len() {
            if let Some(default) = cmd.params[self.current_param_index].default {
                self.input_buffer = default.to_string();
            }
        }
    }

    pub fn resolve_and_preview(&mut self) {
        let cmd = self.current_command();
        self.resolved_command = commands::resolve_template(cmd.template, &self.param_inputs);
        self.screen = Screen::Preview;
        self.set_status("y = copy/use · e = edit params · Backspace = back · q = quit");
    }

    pub fn confirm_command(&mut self) {
        self.final_command = Some(self.resolved_command.clone());
    }

    pub fn go_back(&mut self) {
        self.screen = match self.screen {
            Screen::Preview     => Screen::ParamInput,
            Screen::ParamInput  => Screen::CommandSelect,
            Screen::CommandSelect => Screen::CategorySelect,
            Screen::CategorySelect => Screen::CategorySelect,
        };
        if self.screen == Screen::ParamInput {
            // Reset to first param
            self.current_param_index = 0;
            self.input_buffer.clear();
            self.param_inputs.clear();
            self.prefill_default();
        }
        self.set_status("Use ↑↓ to navigate · Enter to select · / to search · q to quit");
    }

    pub fn scroll_up(&mut self) {
        match self.screen {
            Screen::CategorySelect => {
                if self.category_index > 0 {
                    self.category_index -= 1;
                }
            }
            Screen::CommandSelect => {
                let len = self.filtered_commands().len();
                if len > 0 && self.command_index > 0 {
                    self.command_index -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn scroll_down(&mut self) {
        match self.screen {
            Screen::CategorySelect => {
                if self.category_index < self.categories.len() - 1 {
                    self.category_index += 1;
                }
            }
            Screen::CommandSelect => {
                let len = self.filtered_commands().len();
                if len > 0 && self.command_index < len - 1 {
                    self.command_index += 1;
                }
            }
            _ => {}
        }
    }

    pub fn set_status(&mut self, msg: &str) {
        self.status_msg = msg.to_string();
        self.status_is_error = false;
    }

    pub fn set_error(&mut self, msg: &str) {
        self.status_msg = msg.to_string();
        self.status_is_error = true;
    }
}
