use chrono::{Duration, Utc};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::Connections;
use crate::entities::events::EventKind;
use crate::entities::{events, organizers};
use crate::error::ApiError;
use crate::persistence::organizers_repository;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsSearchFilters {
    pub country: Option<String>,
    pub city: Option<String>,
    pub area: Option<String>,
    pub organizer_id: Option<i32>,
    pub kind: Option<EventKind>,
    pub state: Option<EventState>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EventState {
    Upcoming,
    Past,
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
    let EventsSearchFilters {
        city,
        area,
        kind,
        country,
        organizer_id,
        state,
    } = request.filters;
    let page = Ord::max(request.page, 1);
    let page_size = Ord::min(request.page_size, 100);

    let query = events::Entity::find()
        .join(JoinType::InnerJoin, events::Relation::Organizers.def())
        .apply_if(country, |query, country| {
            query.filter(organizers::Column::Country.eq(country))
        })
        .apply_if(organizer_id, |query, organizer_id| {
            query.filter(events::Column::OrganizerId.eq(organizer_id))
        })
        .apply_if(city, |query, city| {
            query.filter(organizers::Column::City.eq(city))
        })
        .apply_if(area, |query, area| {
            query.filter(organizers::Column::Area.eq(area))
        })
        .apply_if(kind, |query, kind| {
            query.filter(events::Column::Kind.eq(kind))
        })
        .apply_if(state, |query, state| match state {
            EventState::Upcoming => query
                .filter(events::Column::HappeningAt.gt(Utc::now() - Duration::hours(8)))
                .order_by(events::Column::HappeningAt, Order::Asc),
            EventState::Past => query
                .filter(events::Column::HappeningAt.lt(Utc::now() - Duration::hours(8)))
                .order_by(events::Column::HappeningAt, Order::Desc),
        });

    let events = query
        .clone()
        .limit(Some(page_size))
        .offset(page.saturating_sub(1) * page_size)
        .all(&conns.db)
        .await?;

    let organizer_ids: Vec<i32> = events.iter().map(|event| event.organizer_id).collect();
    let organizers_map = organizers_repository::map_by_ids(&conns.db, organizer_ids).await?;

    let total = if page <= 1 && (events.len() as u64) < page_size {
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
        page,
        page_size,
    })
}
