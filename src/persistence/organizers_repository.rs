use sea_orm::*;

use crate::entities::organizers;

pub async fn all(
    db: &DatabaseConnection,
) -> Result<Vec<organizers::Model>, anyhow::Error> {
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

