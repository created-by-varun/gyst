use crate::git::StagedChanges;
use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Response structures
#[derive(Debug, Deserialize)]
struct CommitResponse {
    message: String,
}

#[derive(Debug, Deserialize)]
struct SuggestionsResponse {
    suggestions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CommandResponse {
    suggestion: String,
}

// Request structures
#[derive(Debug, Serialize)]
struct CommitRequest {
    changes: StagedChanges,
    diff: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u8>,
}

#[derive(Debug, Serialize)]
struct CommandRequest {
    description: String,
}

pub struct ServerClient {
    client: Client,
}

impl ServerClient {
    pub fn new(_config: crate::config::Config) -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn get_server_url(&self) -> String {
        // Use a fixed server URL
        "http://127.0.0.1:8080".to_string()
    }

    pub async fn generate_message(&self, changes: &StagedChanges, diff: &str) -> Result<String> {
        let server_url = self.get_server_url();
        let url = format!("{}/api/commit", server_url);

        let request = CommitRequest {
            changes: changes.clone(),
            diff: diff.to_string(),
            count: None,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to server")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Server error: {}", error_text));
        }

        let commit_response: CommitResponse = response
            .json()
            .await
            .context("Failed to parse server response")?;

        Ok(commit_response.message)
    }

    pub async fn generate_suggestions(
        &self,
        changes: &StagedChanges,
        diff: &str,
        count: u8,
    ) -> Result<Vec<String>> {
        let server_url = self.get_server_url();
        let url = format!("{}/api/commit/suggestions", server_url);

        let request = CommitRequest {
            changes: changes.clone(),
            diff: diff.to_string(),
            count: Some(count),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to server")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Server error: {}", error_text));
        }

        let suggestions_response: SuggestionsResponse = response
            .json()
            .await
            .context("Failed to parse server response")?;

        Ok(suggestions_response.suggestions)
    }

    pub async fn suggest_command(&self, description: &str) -> Result<String> {
        let server_url = self.get_server_url();
        let url = format!("{}/api/command", server_url);

        let request = CommandRequest {
            description: description.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to server")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Server error: {}", error_text));
        }

        let command_response: CommandResponse = response
            .json()
            .await
            .context("Failed to parse server response")?;

        Ok(command_response.suggestion)
    }

    pub async fn health_check(&self) -> Result<bool> {
        let server_url = self.get_server_url();
        let url = format!("{}/api/health", server_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to server")?;

        Ok(response.status().is_success())
    }
}
