use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger, ResponseError};
use dotenv::dotenv;
use log::info;
use serde::Serialize;
use std::env;

mod anthropic;
mod error;

use anthropic::{CommandRequest, CommitRequest};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
struct CommitResponse {
    message: String,
}

#[derive(Serialize)]
struct SuggestionsResponse {
    suggestions: Vec<String>,
}

#[derive(Serialize)]
struct CommandResponse {
    suggestion: String,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn generate_commit(req: web::Json<CommitRequest>) -> impl Responder {
    match anthropic::generate_commit_message(&req).await {
        Ok(message) => HttpResponse::Ok().json(CommitResponse { message }),
        Err(e) => e.error_response(),
    }
}

async fn generate_commit_suggestions(req: web::Json<CommitRequest>) -> impl Responder {
    let count = req.count.unwrap_or(3);
    match anthropic::generate_commit_suggestions(&req, count).await {
        Ok(suggestions) => HttpResponse::Ok().json(SuggestionsResponse { suggestions }),
        Err(e) => e.error_response(),
    }
}

async fn suggest_command(req: web::Json<CommandRequest>) -> impl Responder {
    match anthropic::suggest_command(&req).await {
        Ok(suggestion) => HttpResponse::Ok().json(CommandResponse { suggestion }),
        Err(e) => e.error_response(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{}:{}", host, port);

    info!("Starting server at http://{}", server_url);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
                    .route("/commit", web::post().to(generate_commit))
                    .route(
                        "/commit/suggestions",
                        web::post().to(generate_commit_suggestions),
                    )
                    .route("/command", web::post().to(suggest_command)),
            )
    })
    .bind(server_url)?
    .run()
    .await
}
