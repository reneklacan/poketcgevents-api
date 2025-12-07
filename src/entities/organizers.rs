use chrono::{DateTime, FixedOffset};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "organizers")]
#[schema(as = Organizer)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub address: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub city: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub area: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub country: String,
    #[sea_orm(column_type = "Double", nullable)]
    pub latitude: f64,
    #[sea_orm(column_type = "Double", nullable)]
    pub longitude: f64,
    #[sea_orm(column_type = "Text", nullable)]
    pub timezone: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    #[sea_orm(has_many)]
    #[schema(ignore)]
    #[serde(skip)]
    pub events: HasMany<super::events::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
