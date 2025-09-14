use std::sync::LazyLock;

use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize)]
pub struct Config {
    pub prompt: Prompt,
    pub generator: Generator,
}

#[derive(Deserialize)]
pub struct Prompt {
    pub template: String,
}

#[derive(Deserialize)]
pub struct Generator {
    pub command: String,
    pub args: Vec<String>,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    from_str(include_str!("../assets/config.toml")).expect("Failed to parse embedded config.toml")
});

impl Prompt {
    pub fn format_with_diff(&self, diff_content: &str) -> String {
        self.template.replace("{diff_content}", diff_content)
    }
}
