use axum::{Json, http::StatusCode, response::IntoResponse};
use tracing::error;

use crate::error::ApiError;

use crate::services::events::crawler;
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
