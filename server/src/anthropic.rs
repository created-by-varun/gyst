use crate::error::ServerError;
use anyhow::Result;
use log::info;
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use std::env;

// System prompts from original implementation
const COMMIT_SYSTEM_PROMPT: &str = r#"You are an AI assistant that helps developers write clear and meaningful git commit messages.
Follow these rules:
1. Use the conventional commit format: <type>(<scope>): <description>
2. Keep the subject line under 72 characters
3. Use the imperative mood ("add" not "added")
4. Don't end the subject line with a period
5. Focus on WHY and WHAT, not HOW
6. If there are breaking changes, add BREAKING CHANGE: in the body

Types: feat, fix, docs, style, refactor, perf, test, chore, ci, build

Return ONLY the commit message, without any prefixes or explanations."#;

const COMMAND_SYSTEM_PROMPT: &str = r#"You are a Git command suggestion assistant. Given a natural language description of what the user wants to do, suggest the appropriate Git command(s).

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

// Request and response structures for the server API
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitRequest {
    pub changes: StagedChanges,
    pub diff: String,
    pub count: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StagedChanges {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub deleted: Vec<String>,
    pub renamed: Vec<(String, String)>,
    pub stats: DiffStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffStats {
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRequest {
    pub description: String,
}

// Anthropic API structures
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Serialize)]
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

// Helper function to get API key from environment
fn get_api_key() -> Result<String, ServerError> {
    env::var("ANTHROPIC_API_KEY").map_err(|_| ServerError::MissingApiKey)
}

// Helper function to get model from environment or use default
fn get_model() -> String {
    env::var("ANTHROPIC_MODEL").unwrap_or_else(|_| "claude-3-5-haiku-20241022".to_string())
}

// Clean commit message function from original implementation
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

// Function to generate a single commit message
pub async fn generate_commit_message(req: &CommitRequest) -> Result<String, ServerError> {
    let suggestions = generate_commit_suggestions(req, 1).await?;
    Ok(suggestions.into_iter().next().unwrap_or_default())
}

// Function to generate multiple commit message suggestions
pub async fn generate_commit_suggestions(
    req: &CommitRequest,
    count: u8,
) -> Result<Vec<String>, ServerError> {
    let api_key = get_api_key()?;
    let model = get_model();
    let client = Client::new();

    let mut prompt = String::new();
    prompt.push_str("Here are the changes to commit:\n\n");

    // Add file changes summary
    if !req.changes.added.is_empty() {
        prompt.push_str("Added files:\n");
        for file in &req.changes.added {
            prompt.push_str(&format!("  + {}\n", file));
        }
    }

    if !req.changes.modified.is_empty() {
        prompt.push_str("\nModified files:\n");
        for file in &req.changes.modified {
            prompt.push_str(&format!("  * {}\n", file));
        }
    }

    if !req.changes.deleted.is_empty() {
        prompt.push_str("\nDeleted files:\n");
        for file in &req.changes.deleted {
            prompt.push_str(&format!("  - {}\n", file));
        }
    }

    if !req.changes.renamed.is_empty() {
        prompt.push_str("\nRenamed files:\n");
        for (old, new) in &req.changes.renamed {
            prompt.push_str(&format!("  {} -> {}\n", old, new));
        }
    }

    // Add the diff
    prompt.push_str("\nHere's the detailed diff:\n");
    prompt.push_str(&req.diff);

    prompt.push_str("\nPlease generate a commit message following the conventional commit format.");

    let mut suggestions = Vec::new();

    for i in 0..count {
        info!("Generating commit suggestion {}/{}", i + 1, count);

        let request = AnthropicRequest {
            model: model.clone(),
            max_tokens: 200,
            temperature: 0.7, // Increased temperature for more varied suggestions
            system: COMMIT_SYSTEM_PROMPT.to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: vec![AnthropicContent {
                    content_type: "text".to_string(),
                    text: prompt.clone(),
                }],
            }],
        };

        let mut headers = HeaderMap::new();
        let header_value = HeaderValue::from_str(&api_key)
            .map_err(ServerError::InvalidHeaderValue)?;
        headers.insert("x-api-key", header_value);
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| ServerError::HttpClientError(e))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .map_err(|e| ServerError::HttpClientError(e))?;
            return Err(ServerError::AnthropicError(error_text));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| ServerError::HttpClientError(e))?;

        let anthropic_response: AnthropicResponse =
            serde_json::from_str(&response_text).map_err(|e| ServerError::SerializationError(e))?;

        let message = anthropic_response
            .content
            .into_iter()
            .find(|c| c.content_type == "text")
            .map(|c| c.text)
            .ok_or_else(|| ServerError::ParseError("No text content in response".to_string()))?;

        suggestions.push(clean_commit_message(&message));
    }

    Ok(suggestions)
}

// Function to suggest git commands
pub async fn suggest_command(req: &CommandRequest) -> Result<String, ServerError> {
    let api_key = get_api_key()?;
    let model = get_model();
    let client = Client::new();

    let request = AnthropicRequest {
        model,
        max_tokens: 500,
        temperature: 0.2, // Lower temperature for more focused suggestions
        system: COMMAND_SYSTEM_PROMPT.to_string(),
        messages: vec![AnthropicMessage {
            role: "user".to_string(),
            content: vec![AnthropicContent {
                content_type: "text".to_string(),
                text: req.description.clone(),
            }],
        }],
    };

    let mut headers = HeaderMap::new();
    let header_value = HeaderValue::from_str(&api_key)
        .map_err(ServerError::InvalidHeaderValue)?;
    headers.insert("x-api-key", header_value);
    headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| ServerError::HttpClientError(e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .map_err(|e| ServerError::HttpClientError(e))?;
        return Err(ServerError::AnthropicError(error_text));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| ServerError::HttpClientError(e))?;

    let anthropic_response: AnthropicResponse =
        serde_json::from_str(&response_text).map_err(|e| ServerError::SerializationError(e))?;

    let suggestion = anthropic_response
        .content
        .into_iter()
        .find(|c| c.content_type == "text")
        .map(|c| c.text)
        .ok_or_else(|| ServerError::ParseError("No text content in response".to_string()))?;

    Ok(suggestion)
}
