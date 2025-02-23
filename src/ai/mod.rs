use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::git::StagedChanges;
use reqwest::header::HeaderValue;

const SYSTEM_PROMPT: &str = r#"You are an AI assistant that helps developers write clear and meaningful git commit messages.
Follow these rules:
1. Use the conventional commit format: <type>(<scope>): <description>
2. Keep the subject line under 72 characters
3. Use the imperative mood ("add" not "added")
4. Don't end the subject line with a period
5. Focus on WHY and WHAT, not HOW
6. If there are breaking changes, add BREAKING CHANGE: in the body

Types: feat, fix, docs, style, refactor, perf, test, chore, ci, build

Return ONLY the commit message, without any prefixes or explanations."#;

#[derive(Debug, Serialize, Clone)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize, Clone)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Serialize, Clone)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponseContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

pub struct CommitMessageGenerator {
    config: Config,
    client: reqwest::Client,
}

impl CommitMessageGenerator {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn generate_message(&self, changes: &StagedChanges, diff: &str) -> Result<String> {
        let suggestions = self.generate_suggestions(changes, diff, 1).await?;
        Ok(suggestions.into_iter().next().unwrap())
    }

    fn clean_commit_message(message: &str) -> String {
        // Remove any prefixes like "Based on the changes..."
        if let Some(feat_idx) = message.find("feat") {
            message[feat_idx..].trim().to_string()
        } else if let Some(fix_idx) = message.find("fix") {
            message[fix_idx..].trim().to_string()
        } else if let Some(docs_idx) = message.find("docs") {
            message[docs_idx..].trim().to_string()
        } else if let Some(style_idx) = message.find("style") {
            message[style_idx..].trim().to_string()
        } else if let Some(refactor_idx) = message.find("refactor") {
            message[refactor_idx..].trim().to_string()
        } else if let Some(perf_idx) = message.find("perf") {
            message[perf_idx..].trim().to_string()
        } else if let Some(test_idx) = message.find("test") {
            message[test_idx..].trim().to_string()
        } else if let Some(chore_idx) = message.find("chore") {
            message[chore_idx..].trim().to_string()
        } else if let Some(ci_idx) = message.find("ci") {
            message[ci_idx..].trim().to_string()
        } else if let Some(build_idx) = message.find("build") {
            message[build_idx..].trim().to_string()
        } else {
            message.trim().to_string()
        }
    }

    pub async fn generate_suggestions(&self, changes: &StagedChanges, diff: &str, count: u8) -> Result<Vec<String>> {
        let api_key = self.config.get_api_key()
            .ok_or_else(|| anyhow!("API key not set. Use 'gyst config --api-key <key>' to set it."))?;

        let mut prompt = String::new();
        prompt.push_str("Here are the changes to commit:\n\n");
        
        // Add file changes summary
        if !changes.added.is_empty() {
            prompt.push_str("Added files:\n");
            for file in &changes.added {
                prompt.push_str(&format!("  + {}\n", file));
            }
        }
        
        if !changes.modified.is_empty() {
            prompt.push_str("\nModified files:\n");
            for file in &changes.modified {
                prompt.push_str(&format!("  * {}\n", file));
            }
        }
        
        if !changes.deleted.is_empty() {
            prompt.push_str("\nDeleted files:\n");
            for file in &changes.deleted {
                prompt.push_str(&format!("  - {}\n", file));
            }
        }
        
        if !changes.renamed.is_empty() {
            prompt.push_str("\nRenamed files:\n");
            for (old, new) in &changes.renamed {
                prompt.push_str(&format!("  {} -> {}\n", old, new));
            }
        }

        // Add the diff
        prompt.push_str("\nHere's the detailed diff:\n");
        prompt.push_str(diff);
        
        prompt.push_str("\nPlease generate a commit message following the conventional commit format.");

        let mut suggestions = Vec::new();
        
        for _ in 0..count {
            let request = AnthropicRequest {
                model: "claude-3-5-haiku-20241022".to_string(),
                max_tokens: 200,
                temperature: 0.0,
                system: SYSTEM_PROMPT.to_string(),
                messages: vec![AnthropicMessage {
                    role: "user".to_string(),
                    content: vec![AnthropicContent {
                        content_type: "text".to_string(),
                        text: prompt.clone(),
                    }],
                }],
            };

            let response = self.client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", HeaderValue::from_str(&api_key)?)
                .header("anthropic-version", HeaderValue::from_static("2023-06-01"))
                .header("Content-Type", HeaderValue::from_static("application/json"))
                .json(&request)
                .send()
                .await
                .context("Failed to send request to Anthropic")?;

            let response_text = response.text().await?;

            let anthropic_response: AnthropicResponse = serde_json::from_str(&response_text)
                .context("Failed to parse Anthropic response")?;

            let message = anthropic_response.content.into_iter()
                .find(|c| c.content_type == "text")
                .map(|c| c.text)
                .ok_or_else(|| anyhow!("No text content in response"))?;

            suggestions.push(Self::clean_commit_message(&message));
        }

        Ok(suggestions)
    }
}