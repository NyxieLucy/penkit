use crate::commands::{Category, Command, get_commands};

pub struct App {
    pub module: Option<String>,
    pub target: Option<String>,
    pub output_file: Option<String>,
    pub final_command: Option<String>,
    pub selected_category: usize,
    pub selected_command: usize,
    pub categories: Vec<Category>,
    pub commands: Vec<&'static Command>,
    pub input_mode: bool,
    pub input_buffer: String,
    pub input_label: String,
    pub param_values: std::collections::HashMap<String, String>,
    pub current_param_index: usize,
    pub show_help: bool,
    pub wants_run: bool,
    pub sudo_mode: bool,
}

impl App {
    pub fn new(module: Option<String>, target: Option<String>, output: Option<String>) -> Self {
        let categories = Category::all();
        let initial_category = if let Some(ref m) = module {
            Category::from_str(m).unwrap_or(Category::Recon)
        } else {
            Category::Recon
        };

        let cat_idx = categories.iter().position(|c| c == &initial_category).unwrap_or(0);
        let commands: Vec<&'static Command> = get_commands(&initial_category).iter().collect();

        let mut app = Self {
            module,
            target: target.clone(),
            output_file: output,
            final_command: None,
            selected_category: cat_idx,
            selected_command: 0,
            categories,
            commands,
            input_mode: false,
            input_buffer: String::new(),
            input_label: String::new(),
            param_values: std::collections::HashMap::new(),
            current_param_index: 0,
            show_help: false,
            wants_run: false,
            sudo_mode: false,
        };

        if let Some(t) = target {
            app.param_values.insert("target".to_string(), t);
        }

        app
    }

    pub fn current_command(&self) -> Option<&'static Command> {
        self.commands.get(self.selected_command).copied()
    }

    pub fn next_category(&mut self) {
        if self.selected_category < self.categories.len() - 1 {
            self.selected_category += 1;
            self.update_commands();
        }
    }

    pub fn prev_category(&mut self) {
        if self.selected_category > 0 {
            self.selected_category -= 1;
            self.update_commands();
        }
    }

    pub fn next_command(&mut self) {
        if self.selected_command < self.commands.len().saturating_sub(1) {
            self.selected_command += 1;
        }
    }

    pub fn prev_command(&mut self) {
        if self.selected_command > 0 {
            self.selected_command -= 1;
        }
    }

    fn update_commands(&mut self) {
        let cat = &self.categories[self.selected_category];
        self.commands = get_commands(cat).iter().collect();
        self.selected_command = 0;
        self.final_command = None;
        self.param_values.clear();
        if let Some(ref t) = self.target {
            self.param_values.insert("target".to_string(), t.clone());
        }
    }

    pub fn start_input(&mut self) {
        if let Some(cmd) = self.current_command() {
            if cmd.params.is_empty() {
                self.generate_command();
                return;
            }
            self.current_param_index = 0;
            self.input_mode = true;
            self.input_buffer.clear();
            let param = &cmd.params[0];
            self.input_label = format!(
                "{} (default: {})",
                param.label,
                param.default.unwrap_or("none")
            );
            if let Some(default) = param.default {
                self.input_buffer = default.to_string();
            } else if let Some(existing) = self.param_values.get(param.key) {
                self.input_buffer = existing.clone();
            }
        }
    }

    pub fn submit_input(&mut self) {
        if let Some(cmd) = self.current_command() {
            if !self.input_buffer.is_empty() {
                let key = cmd.params[self.current_param_index].key.to_string();
                self.param_values.insert(key, self.input_buffer.clone());
            }

            self.current_param_index += 1;
            if self.current_param_index >= cmd.params.len() {
                self.input_mode = false;
                self.generate_command();
            } else {
                self.input_buffer.clear();
                let param = &cmd.params[self.current_param_index];
                self.input_label = format!(
                    "{} (default: {})",
                    param.label,
                    param.default.unwrap_or("none")
                );
                if let Some(default) = param.default {
                    self.input_buffer = default.to_string();
                } else if let Some(existing) = self.param_values.get(param.key) {
                    self.input_buffer = existing.clone();
                }
            }
        }
    }

    pub fn generate_command(&mut self) {
        if let Some(cmd) = self.current_command() {
            let mut result = cmd.template.to_string();
            for param in cmd.params {
                let val = self
                    .param_values
                    .get(param.key)
                    .cloned()
                    .or(param.default.map(|d| d.to_string()))
                    .unwrap_or_default();
                result = result.replace(&format!("{{{}}}", param.key), &val);
            }
            self.final_command = Some(result);
        }
    }
}
