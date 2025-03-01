use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub commit: CommitConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub api_key: String,
    #[serde(default = "default_model")]
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GitConfig {
    #[serde(default = "default_max_diff_size")]
    pub max_diff_size: usize,
    #[serde(default = "default_protected_branches")]
    pub protected_branches: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommitConfig {
    #[serde(default = "default_commit_template")]
    pub template: String,
    #[serde(default = "default_max_subject_length")]
    pub max_subject_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_use_server")]
    pub use_server: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            use_server: true,
        }
    }
}

fn default_model() -> String {
    "claude-haiku".to_string()
}

fn default_max_diff_size() -> usize {
    1000
}

fn default_protected_branches() -> Vec<String> {
    vec!["main".to_string(), "master".to_string()]
}

fn default_commit_template() -> String {
    "conventional".to_string()
}

fn default_max_subject_length() -> usize {
    72
}

fn default_use_server() -> bool {
    true
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Config::get_config_path()?;

        if !config_path.exists() {
            return Ok(Config {
                ai: AiConfig {
                    provider: "anthropic".to_string(),
                    api_key: String::new(),
                    model: "claude-3-5-haiku-20241022".to_string(),
                },
                git: GitConfig::default(),
                commit: CommitConfig::default(),
                server: ServerConfig::default(),
            });
        }

        let contents = fs::read_to_string(&config_path).context("Failed to read config file")?;

        toml::from_str(&contents).context("Failed to parse config file")
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Config::get_config_path()?;

        // Ensure the directory exists
        if let Some(dir) = config_path.parent() {
            fs::create_dir_all(dir).context("Failed to create config directory")?;
        }

        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, contents).context("Failed to write config file")?;

        Ok(())
    }

    pub fn set_api_key(&mut self, api_key: String) -> Result<()> {
        self.ai.api_key = api_key;
        self.save()
    }

    pub fn get_api_key(&self) -> Option<&str> {
        if self.ai.api_key.is_empty() {
            None
        } else {
            Some(&self.ai.api_key)
        }
    }

    pub fn set_use_server(&mut self, use_server: bool) -> Result<()> {
        self.server.use_server = use_server;
        self.save()
    }

    pub fn use_server(&self) -> bool {
        self.server.use_server
    }

    fn get_config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Failed to determine home directory")?;
        Ok(home.join(".gyst").join("config.toml"))
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str("\nAI Configuration:\n");
        output.push_str(&format!("  Provider: {}\n", self.ai.provider));
        output.push_str(&format!("  Model: {}\n", self.ai.model));
        output.push_str(&format!(
            "  API Key: {}\n",
            if self.ai.api_key.is_empty() {
                "<not set>".to_string()
            } else {
                "********".to_string()
            }
        ));

        output.push_str("\nGit Configuration:\n");
        output.push_str(&format!(
            "  Max Diff Size: {} lines\n",
            self.git.max_diff_size
        ));
        output.push_str("  Protected Branches:\n");
        for branch in &self.git.protected_branches {
            output.push_str(&format!("    - {}\n", branch));
        }

        output.push_str("\nCommit Configuration:\n");
        output.push_str(&format!("  Template: {}\n", self.commit.template));
        output.push_str(&format!(
            "  Max Subject Length: {} characters\n",
            self.commit.max_subject_length
        ));

        output.push_str("\nServer Configuration:\n");
        output.push_str(&format!("  Use Server: {}\n", self.server.use_server));

        output
    }
}
