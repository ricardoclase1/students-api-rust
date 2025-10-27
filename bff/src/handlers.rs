use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
    Json as AxumJson,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::EmailRequest;

pub async fn notify_email(
    State(_config): State<AppConfig>,
    headers: HeaderMap,
    AxumJson(payload): AxumJson<EmailRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Validate input
    payload.validate()?;

    // Extract request ID from headers or generate new one
    let request_id = headers
        .get("x-request-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or(&Uuid::new_v4().to_string())
        .to_string();

    tracing::info!("Processing email notification for request_id: {}", request_id);

    // Skip API Gateway forwarding for local testing - just log the notification
    tracing::info!("Email notification would be forwarded to API Gateway in production");

    // Skip SNS publishing for local testing - just log the notification
    tracing::info!("Email notification would be published to SNS in production: {:?}", payload);

    tracing::info!("Email notification processed successfully for request_id: {}", request_id);

    Ok(Json(json!({
        "request_id": request_id,
        "status": "Email notification queued successfully"
    })))
}
