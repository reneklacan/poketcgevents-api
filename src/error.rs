use axum::{http::StatusCode, response::IntoResponse, response::Response};

#[derive(Debug, thiserror::Error)]
pub struct ApiError {
    pub status_code: StatusCode,
    #[allow(dead_code)]
    pub error: anyhow::Error,
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

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error,
        }
    }
}

impl From<sea_orm::error::DbErr> for ApiError {
    fn from(error: sea_orm::error::DbErr) -> Self {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error: anyhow::Error::from(error),
        }
    }
}
