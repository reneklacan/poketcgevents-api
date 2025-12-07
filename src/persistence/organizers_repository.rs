use sea_orm::*;
use std::collections::HashMap;

use crate::entities::organizers;

pub async fn all_by_ids(
    db: &DatabaseConnection,
    ids: Vec<i32>,
) -> Result<Vec<organizers::Model>, anyhow::Error> {
    organizers::Entity::find()
        .filter(organizers::Column::Id.is_in(ids))
        .all(db)
        .await
        .map_err(anyhow::Error::from)
}

pub async fn map_by_ids(
    db: &DatabaseConnection,
    ids: Vec<i32>,
) -> Result<HashMap<i32, organizers::Model>, anyhow::Error> {
    let organizers = all_by_ids(db, ids).await?;
    Ok(organizers
        .into_iter()
        .map(|organizer| (organizer.id, organizer))
        .collect())
}

pub async fn all(db: &DatabaseConnection) -> Result<Vec<organizers::Model>, anyhow::Error> {
    organizers::Entity::find()
        .all(db)
        .await
        .map_err(anyhow::Error::from)
}

pub async fn insert(
    db: &DatabaseConnection,
    model: organizers::ActiveModel,
) -> Result<organizers::Model, anyhow::Error> {
    model.insert(db).await.map_err(anyhow::Error::from)
}
