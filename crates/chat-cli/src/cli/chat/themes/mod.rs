pub mod bash_parser;
pub mod renderer;
pub mod theme_manager;

#[cfg(test)]
mod cross_platform_test;
#[cfg(test)]
mod integration_test;

use std::collections::HashMap;

pub use theme_manager::ThemeManager;

/// Represents a bash-style theme for the CLI prompt
#[derive(Debug, Clone)]
pub struct BashTheme {
    pub name: String,
    pub prompt_template: String,
    pub variables: HashMap<String, String>,
    pub git_enabled: bool,
}

impl BashTheme {
    pub fn new(name: String) -> Self {
        Self {
            name,
            prompt_template: String::new(),
            variables: HashMap::new(),
            git_enabled: false,
        }
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }
}

/// Git information for prompt display
#[derive(Debug, Clone)]
pub struct GitInfo {
    pub branch: Option<String>,
    pub is_dirty: bool,
    pub is_repo: bool,
}

impl GitInfo {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            branch: None,
            is_dirty: false,
            is_repo: false,
        }
    }
}
