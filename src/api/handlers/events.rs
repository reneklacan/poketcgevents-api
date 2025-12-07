use axum::{Extension, Json};

use crate::Connections;
use crate::error::ApiError;
use crate::services::events::search_service::{self, EventsSearchRequest, EventsSearchResponse};

#[utoipa::path(
    post,
    tag = "Events",
    path = "/events/search",
    operation_id = "search",
    request_body = EventsSearchRequest,
    responses(
        (status = OK, body = EventsSearchResponse),
    ),
)]
pub async fn search(
    Extension(conns): Extension<Connections>,
    Json(request): Json<EventsSearchRequest>,
) -> Result<Json<EventsSearchResponse>, ApiError> {
    Ok(Json(search_service::search(&conns, request).await?))
}
