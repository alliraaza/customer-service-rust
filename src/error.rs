use summer_web::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("customer {0} not found")]
    NotFound(i64),

    #[error("a customer with this email already exists")]
    Conflict,

    #[error("validation failed: {0}")]
    Validation(String),

    // Anything unexpected (DB down, bug). #[from] lets `?` convert anyhow errors.
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        AppError::Validation(e.to_string())
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(_)   => (StatusCode::NOT_FOUND, "NOT_FOUND", self.to_string()),
            AppError::Conflict      => (StatusCode::CONFLICT, "CONFLICT", self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", self.to_string()),
            AppError::Internal(e) => {
                // Log the real cause, but never leak it to the client.
                tracing::error!("internal error: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "unexpected error".to_string())
            }
        };
        (status, Json(json!({ "code": code, "message": message }))).into_response()
    }
}