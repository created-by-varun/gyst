use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::config::Config;

const SYSTEM_PROMPT: &str = r#"You are a Git command suggestion assistant. Given a natural language description of what the user wants to do, suggest the appropriate Git command(s).

Rules:
1. Always provide clear, concise commands
2. Include a brief explanation of what each command does
3. If multiple steps are needed, number them
4. If there are alternative approaches, mention them
5. Include any relevant flags or options that might be helpful
6. Warn about any potential risks or things to be careful about

Format your response as:
COMMAND: <the command>
EXPLANATION: <brief explanation>
NOTE: <optional notes/warnings>
"#;

#[derive(Debug, Serialize)]
struct CommandRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    system: String,
    messages: Vec<CommandMessage>,
}

#[derive(Debug, Serialize)]
struct CommandMessage {
    role: String,
    content: Vec<CommandContent>,
}

#[derive(Debug, Serialize)]
struct CommandContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct CommandResponse {
    content: Vec<CommandResponseContent>,
}

#[derive(Debug, Deserialize)]
struct CommandResponseContent {
    text: String,
}

pub struct CommandSuggester {
    client: reqwest::Client,
    config: Config,
}

impl CommandSuggester {
    pub fn new(config: Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    pub async fn suggest(&self, description: &str) -> Result<String> {
        let api_key = self.config.get_api_key()
            .ok_or_else(|| anyhow::anyhow!("API key not found. Please set it using 'gyst config --api-key <key>'"))?;

        let request = CommandRequest {
            model: "claude-3-5-haiku-20241022".to_string(),
            max_tokens: 500,
            temperature: 0.2,  // Lower temperature for more focused suggestions
            system: SYSTEM_PROMPT.to_string(),
            messages: vec![CommandMessage {
                role: "user".to_string(),
                content: vec![CommandContent {
                    content_type: "text".to_string(),
                    text: description.to_string(),
                }],
            }],
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?
            .json::<CommandResponse>()
            .await?;

        Ok(response.content[0].text.clone())
    }
}
