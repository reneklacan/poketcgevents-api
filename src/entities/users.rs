use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub username: Option<String>,
    #[sea_orm(column_type = "Text", nullable, unique)]
    pub discord_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable, unique)]
    pub google_id: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(
        belongs_to,
        from = "discord_id",
        to = "discord_id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub discord_user: HasOne<super::discord_users::Entity>,
    #[sea_orm(
        belongs_to,
        from = "google_id",
        to = "google_id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub google_user: HasOne<super::google_users::Entity>,
    #[sea_orm(has_many)]
    pub user_subscriptions: HasMany<super::user_subscriptions::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
