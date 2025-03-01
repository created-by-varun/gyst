use actix_web::{HttpResponse, ResponseError};
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    
    #[error("Missing API key")]
    MissingApiKey,
    
    #[error("Anthropic API error: {0}")]
    AnthropicError(String),
    
    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl From<anyhow::Error> for ServerError {
    fn from(err: anyhow::Error) -> Self {
        ServerError::InternalError(format!("{}", err))
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::ApiError(_) => HttpResponse::BadGateway().json(serde_json::json!({
                "error": self.to_string()
            })),
            ServerError::ParseError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            ServerError::ConfigError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            ServerError::InternalError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            ServerError::InvalidHeaderValue(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            ServerError::MissingApiKey => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Server configuration error: Missing API key"
                }))
            }
            ServerError::AnthropicError(msg) => {
                HttpResponse::BadGateway().json(serde_json::json!({
                    "error": format!("Anthropic API error: {}", msg)
                }))
            }
            ServerError::HttpClientError(_) => {
                HttpResponse::BadGateway().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            ServerError::SerializationError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
        }
    }
}
