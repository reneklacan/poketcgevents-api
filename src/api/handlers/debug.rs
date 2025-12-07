use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use tracing::error;

use crate::services::events::crawler;

#[derive(Debug, thiserror::Error)]
pub struct ApiError {
    status_code: StatusCode,
    error: anyhow::Error,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            serde_json::json!({ "error": self.to_string() }).to_string(),
        )
            .into_response()
    }
}

pub async fn crawler() -> Result<impl IntoResponse, ApiError> {
    crawler::call().await.map_err(|err| {
        error!(error = %err, "crawler failed");
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error: err,
        }
    })?;

    Ok(Json(serde_json::json!({ "message": "crawler finished" })))
}
