use chrono::{DateTime, FixedOffset};
use sea_orm::{ActiveValue, IntoActiveValue, entity::prelude::*};
// use sea_orm_typed_id::define_id;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
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
    // #[sea_orm(column_type = "Text", nullable)]
    pub kind: EventKind,
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

#[derive(
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Clone,
    EnumIter,
    ToSchema,
    EnumString,
    FromJsonQueryResult,
)]
pub enum EventKind {
    #[strum(serialize = "League Challenge VG")]
    #[serde(rename = "League Challenge VG")]
    LeagueChallengeVG,
    #[strum(serialize = "League Cup VG")]
    #[serde(rename = "League Cup VG")]
    LeagueCupVG,
    #[strum(serialize = "GO Challenge")]
    #[serde(rename = "GO Challenge")]
    GoChallenge,
    #[strum(serialize = "League Cup")]
    #[serde(rename = "League Cup")]
    LeagueCup,
    #[strum(serialize = "Pre Release")]
    #[serde(rename = "Pre Release")]
    PreRelease,
    #[strum(serialize = "GO Cup")]
    #[serde(rename = "GO Cup")]
    GoCup,
    #[strum(serialize = "League Challenge")]
    #[serde(rename = "League Challenge")]
    LeagueChallenge,
    #[serde(untagged)]
    #[strum(default)]
    Other(String),
}

impl IntoActiveValue<EventKind> for EventKind {
    fn into_active_value(self) -> ActiveValue<EventKind> {
        ActiveValue::Set(self)
    }
}

impl ActiveEnum for EventKind {
    // The macro attribute `rs_type` is being pasted here
    type Value = String;
    type ValueVec = Vec<String>;

    // By default, the name of Rust enum in camel case if `enum_name` was not provided explicitly
    fn name() -> DynIden {
        "EventKind".to_string().into()
    }

    // Map Rust enum variants to corresponding `num_value` or `string_value`
    fn to_value(&self) -> Self::Value {
        match self {
            Self::LeagueChallengeVG => "League Challenge VG",
            Self::LeagueCupVG => "League Cup VG",
            Self::GoChallenge => "GO Challenge",
            Self::LeagueCup => "League Cup",
            Self::PreRelease => "Pre Release",
            Self::GoCup => "GO Cup",
            Self::LeagueChallenge => "League Challenge",
            Self::Other(s) => s,
        }
        .to_owned()
    }

    // Map `num_value` or `string_value` to corresponding Rust enum variants
    fn try_from_value(v: &Self::Value) -> Result<Self, DbErr> {
        match v.as_ref() {
            "League Challenge VG" => Ok(Self::LeagueChallengeVG),
            "League Cup VG" => Ok(Self::LeagueCupVG),
            "GO Challenge" => Ok(Self::GoChallenge),
            "League Cup" => Ok(Self::LeagueCup),
            "Pre Release" => Ok(Self::PreRelease),
            "GO Cup" => Ok(Self::GoCup),
            "League Challenge" => Ok(Self::LeagueChallenge),
            _ => Ok(Self::Other(v.clone())),
        }
    }

    // The macro attribute `db_type` is being pasted here
    fn db_type() -> ColumnDef {
        ColumnType::String(StringLen::None).def()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_event_kind_serialize() {
        assert_eq!(
            EventKind::from_str("League Challenge VG"),
            Ok(EventKind::LeagueChallengeVG)
        );
        assert_eq!(
            EventKind::from_str("League Cup VG"),
            Ok(EventKind::LeagueCupVG)
        );
        assert_eq!(
            EventKind::from_str("GO Challenge"),
            Ok(EventKind::GoChallenge)
        );
        assert_eq!(EventKind::from_str("League Cup"), Ok(EventKind::LeagueCup));
        assert_eq!(
            EventKind::from_str("Pre Release"),
            Ok(EventKind::PreRelease)
        );
        assert_eq!(EventKind::from_str("GO Cup"), Ok(EventKind::GoCup));
        assert_eq!(
            EventKind::from_str("League Challenge"),
            Ok(EventKind::LeagueChallenge)
        );
        assert_eq!(
            EventKind::from_str("Other"),
            Ok(EventKind::Other("Other".to_string()))
        );
    }
}
