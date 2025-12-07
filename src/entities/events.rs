use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;
// use sea_orm_typed_id::define_id;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// define_id!(EventId);

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "events")]
#[schema(as = Event)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub organizer_id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub kind: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub pokemon_event_slug: String,
    #[sea_orm(unique)]
    pub guid: Uuid,
    pub league: Option<i32>,
    pub happening_at: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    #[sea_orm(
        belongs_to,
        from = "organizer_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    #[schema(ignore)]
    #[serde(skip)]
    pub organizer: HasOne<super::organizers::Entity>,
    #[sea_orm(has_many)]
    #[schema(ignore)]
    #[serde(skip)]
    pub user_subscription_notifications: HasMany<super::user_subscription_notifications::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
