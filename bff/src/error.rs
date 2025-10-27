use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::HttpClient(_) => (StatusCode::BAD_GATEWAY, "External service error"),
            AppError::Env(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            AppError::Json(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error"),
        };

        let body = Json(json!({
            "error": error_message,
            "details": self.to_string()
        }));

        (status, body).into_response()
    }
}
