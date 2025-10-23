use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;
use sqlx::Error as SqlxError;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(SqlxError),
    JwtError(jsonwebtoken::errors::Error),
    PasswordHashError,
    Unauthorized,
    NotFound(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API Error") // Mensaje simple, el detalle está en ResponseError
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            ApiError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::JwtError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating token".to_string()),
            ApiError::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, "Error processing password".to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid credentials or token".to_string()),
            ApiError::NotFound(item) => (StatusCode::NOT_FOUND, format!("{} not found", item)),
        };
        HttpResponse::build(status).json(json!({"status": "error", "message": message}))
    }
}
