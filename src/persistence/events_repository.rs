use sea_orm::*;
use sea_query::OnConflict;

use crate::entities::events;

pub async fn upsert(
    db: &DatabaseConnection,
    models: Vec<events::ActiveModel>,
) -> Result<(), anyhow::Error> {
    let on_conflict = OnConflict::columns([events::Column::Guid])
        .update_columns(vec![
            events::Column::Name,
            events::Column::PokemonEventSlug,
            events::Column::Kind,
            events::Column::League,
            events::Column::HappeningAt,
            events::Column::UpdatedAt,
        ])
        .to_owned();

    let insert_result = events::Entity::insert_many(models)
        .on_conflict(on_conflict)
        .exec(db)
        .await;

    match insert_result {
        Ok(_) => Ok(()),
        Err(sea_orm::error::DbErr::RecordNotInserted) => Ok(()),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}
