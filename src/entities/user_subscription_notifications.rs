use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_subscription_notifications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique_key = "idx_user_subscription_notifications_subscription_id_event_id")]
    pub user_subscription_id: i32,
    #[sea_orm(unique_key = "idx_user_subscription_notifications_subscription_id_event_id")]
    pub event_id: i32,
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(
        belongs_to,
        from = "event_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub event: HasOne<super::events::Entity>,
    #[sea_orm(
        belongs_to,
        from = "user_subscription_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub user_subscription: HasOne<super::user_subscriptions::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
