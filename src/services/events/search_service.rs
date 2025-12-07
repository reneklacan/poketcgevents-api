use sea_orm::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::Connections;
use crate::entities::{events, organizers};
use crate::error::ApiError;
use crate::persistence::organizers_repository;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsSearchFilters {
    pub country: Option<String>,
    pub organizer_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsSearchRequest {
    pub filters: EventsSearchFilters,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsSearchResponse {
    pub events: Vec<EventFull>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventFull {
    pub event: events::Model,
    pub organizer: organizers::Model,
}

pub async fn search(
    conns: &Connections,
    request: EventsSearchRequest,
) -> Result<EventsSearchResponse, ApiError> {
    let query = events::Entity::find()
        .join(JoinType::InnerJoin, events::Relation::Organizers.def())
        .apply_if(request.filters.country, |query, country| {
            query.filter(organizers::Column::Country.eq(country))
        })
        .apply_if(request.filters.organizer_id, |query, organizer_id| {
            query.filter(events::Column::OrganizerId.eq(organizer_id))
        });

    let events = query
        .clone()
        .limit(Some(request.page_size))
        .offset((request.page - 1) * request.page_size)
        .all(&conns.db)
        .await?;

    let organizer_ids: Vec<i32> = events.iter().map(|event| event.organizer_id).collect();

    let organizers_map = organizers_repository::map_by_ids(&conns.db, organizer_ids).await?;

    let total = if request.page <= 1 && (events.len() as u64) < request.page_size {
        events.len() as u64
    } else {
        query.count(&conns.db).await?
    };

    Ok(EventsSearchResponse {
        events: events
            .into_iter()
            .filter_map(|event| {
                organizers_map
                    .get(&event.organizer_id)
                    .map(|organizer| EventFull {
                        event,
                        organizer: organizer.clone(),
                    })
            })
            .collect(),
        total,
        page: request.page,
        page_size: request.page_size,
    })
}
