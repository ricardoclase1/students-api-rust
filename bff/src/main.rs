use axum::{
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod config;
mod error;
mod handlers;
mod middleware;

use config::AppConfig;
use handlers::notify_email;
use middleware::request_id;

#[derive(Deserialize, Serialize, Debug, validator::Validate)]
pub struct EmailRequest {
    #[validate(email)]
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct EmailResponse {
    pub request_id: String,
    pub status: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    // Build application with routes
    let app = Router::new()
        .route("/notify/email", post(notify_email))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(axum::middleware::from_fn(request_id))
        .with_state(config);

    // Run server
    let port = std::env::args().nth(2).unwrap_or_else(|| "3000".to_string()).parse::<u16>().unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("BFF server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
