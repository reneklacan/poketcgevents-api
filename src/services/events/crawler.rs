use anyhow::{Context, anyhow};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use csv::ReaderBuilder;
use sea_orm::{Database, DatabaseConnection, Set};
use std::{collections::HashMap, mem, path::Path, str::FromStr};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    entities::{
        events::{self, EventKind},
        organizers,
    },
    persistence::{events_repository, organizers_repository},
};

#[derive(Debug, serde::Deserialize)]
struct EventCsvRecord {
    #[serde(rename = "type")]
    kind: String,
    name: String,
    shop: String,
    #[serde(rename = "street_adress")]
    street_address: String,
    state: String,
    city: String,
    #[serde(rename = "country_code")]
    country_code: String,
    #[serde(rename = "pokemon_url")]
    pokemon_event_slug: String,
    guid: String,
    latitude: String,
    longitude: String,
    #[serde(rename = "when")]
    happening_at: String,
    league: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OrganizerKey {
    name: String,
    address: String,
    city: String,
    area: String,
    country: String,
}

#[derive(Debug, Clone)]
struct OrganizerValues {
    name: String,
    address: String,
    city: String,
    area: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

pub async fn call() -> Result<(), anyhow::Error> {
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let db = Database::connect(database_url)
        .await
        .context("failed to connect to database")?;

    let offset = FixedOffset::east_opt(0).ok_or_else(|| anyhow!("failed to build UTC offset"))?;
    let now = Utc::now().with_timezone(&offset);

    let events_path = Path::new("data/events.csv");
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(events_path)
        .context("failed to open data/events.csv")?;

    let mut organizer_cache = load_existing_organizers(&db).await?;
    let mut event_models: Vec<events::ActiveModel> = Vec::new();
    let mut total_events = 0usize;

    for record_result in reader.deserialize::<EventCsvRecord>() {
        let record = match record_result {
            Ok(r) => r,
            Err(err) => {
                warn!(error = %err, "skipping malformed CSV row");
                continue;
            }
        };

        let organizer_id = match ensure_organizer(&db, &mut organizer_cache, &record, now).await {
            Ok(id) => id,
            Err(err) => {
                warn!(error = %err, "skipping row due to organizer error");
                continue;
            }
        };

        let happening_at = match parse_datetime(&record.happening_at, offset) {
            Ok(dt) => dt,
            Err(err) => {
                warn!(
                    error = %err,
                    when = %record.happening_at,
                    "skipping row due to invalid datetime"
                );
                continue;
            }
        };

        let guid = match Uuid::parse_str(record.guid.trim()) {
            Ok(g) => g,
            Err(err) => {
                warn!(error = %err, guid = %record.guid, "skipping row due to invalid GUID");
                continue;
            }
        };

        event_models.push(build_event_model(
            &record,
            organizer_id,
            guid,
            happening_at,
            now,
        )?);

        if event_models.len() >= 100 {
            let chunk = mem::take(&mut event_models);
            let chunk_len = chunk.len();
            events_repository::upsert(&db, chunk)
                .await
                .context("failed to upsert events chunk")?;
            total_events += chunk_len;
        }
    }

    if !event_models.is_empty() {
        let chunk_len = event_models.len();
        events_repository::upsert(&db, event_models)
            .await
            .context("failed to upsert final events chunk")?;
        total_events += chunk_len;
    }

    info!(total_events, "Upserted events from CSV");

    Ok(())
}

fn build_event_model(
    record: &EventCsvRecord,
    organizer_id: i32,
    guid: Uuid,
    happening_at: DateTime<FixedOffset>,
    now: DateTime<FixedOffset>,
) -> Result<events::ActiveModel, anyhow::Error> {
    let league = parse_optional_i32(&record.league);

    Ok(events::ActiveModel {
        id: Default::default(),
        organizer_id: Set(organizer_id),
        kind: Set(EventKind::from_str(&record.kind.trim())?),
        name: Set(record.name.trim().to_string()),
        pokemon_event_slug: Set(record.pokemon_event_slug.trim().to_string()),
        guid: Set(guid),
        league: Set(league),
        happening_at: Set(happening_at),
        created_at: Set(now),
        updated_at: Set(now),
    })
}

async fn ensure_organizer(
    db: &DatabaseConnection,
    cache: &mut HashMap<OrganizerKey, organizers::Model>,
    record: &EventCsvRecord,
    now: DateTime<FixedOffset>,
) -> Result<i32, anyhow::Error> {
    let values = organizer_values_from_record(record);
    let key = build_organizer_key(&values);

    if let Some(existing) = cache.get(&key) {
        return Ok(existing.id);
    }

    let model = organizers::ActiveModel {
        id: Default::default(),
        name: Set(values.name.clone()),
        address: Set(values.address.clone()),
        city: Set(values.city.clone()),
        area: Set(values.area.clone()),
        country: Set(values.country.clone()),
        latitude: Set(values.latitude),
        longitude: Set(values.longitude),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let inserted = organizers_repository::insert(db, model)
        .await
        .context("failed to insert organizer")?;

    cache.insert(key.clone(), inserted.clone());

    Ok(inserted.id)
}

fn build_organizer_key(values: &OrganizerValues) -> OrganizerKey {
    OrganizerKey {
        name: normalize_key(&values.name),
        address: normalize_key(&values.address),
        city: normalize_key(&values.city),
        area: normalize_key(&values.area),
        country: normalize_key(&values.country),
    }
}

fn organizer_values_from_record(record: &EventCsvRecord) -> OrganizerValues {
    OrganizerValues {
        name: normalize(&record.shop, "Unknown organizer"),
        address: normalize(&record.street_address, "Unknown address"),
        city: normalize(&record.city, "Unknown city"),
        area: normalize(&record.state, "Unknown area"),
        country: normalize(&record.country_code, "XX"),
        latitude: parse_f64(&record.latitude).unwrap_or(0.0),
        longitude: parse_f64(&record.longitude).unwrap_or(0.0),
    }
}

fn organizer_values_from_model(model: &organizers::Model) -> OrganizerValues {
    OrganizerValues {
        name: model.name.clone(),
        address: model.address.clone(),
        city: model.city.clone(),
        area: model.area.clone(),
        country: model.country.clone(),
        latitude: model.latitude,
        longitude: model.longitude,
    }
}

fn parse_f64(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

fn parse_optional_i32(value: &str) -> Option<i32> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<i32>().ok()
}

fn parse_datetime(
    value: &str,
    offset: FixedOffset,
) -> Result<DateTime<FixedOffset>, anyhow::Error> {
    let trimmed = value.trim();
    let naive = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S")
        .with_context(|| format!("invalid datetime format: {trimmed}"))?;

    Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(
        naive, offset,
    ))
}

fn normalize(value: &str, fallback: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return fallback.to_string();
    }
    trimmed.to_string()
}

fn normalize_key(value: &str) -> String {
    normalize(value, "").to_ascii_lowercase()
}

async fn load_existing_organizers(
    db: &DatabaseConnection,
) -> Result<HashMap<OrganizerKey, organizers::Model>, anyhow::Error> {
    let mut map = HashMap::new();
    let organizers = organizers_repository::all(db)
        .await
        .context("failed to load existing organizers")?;

    for organizer in organizers {
        let values = organizer_values_from_model(&organizer);
        let key = build_organizer_key(&values);

        map.entry(key).or_insert(organizer);
    }

    Ok(map)
}
