use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "google_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub google_id: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    pub email_verified: Option<bool>,
    #[sea_orm(column_type = "Text", nullable)]
    pub first_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub last_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub profile_image_url: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

impl ActiveModelBehavior for ActiveModel {}
