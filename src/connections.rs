use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone, Debug)]
pub struct Connections {
    pub db: DatabaseConnection,
}

pub async fn build() -> Result<Connections, anyhow::Error> {
    Ok(Connections {
        db: db_connection().await?,
    })
}

pub async fn db_connection() -> Result<DatabaseConnection, anyhow::Error> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let min_connections = std::env::var("DATABASE_CONNECTIONS_MIN")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(if cfg!(debug_assertions) { 1 } else { 20 });
    let max_connections = std::env::var("DATABASE_CONNECTIONS_MAX")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(200);

    let connect_options = ConnectOptions::new(url)
        .set_schema_search_path("public")
        .min_connections(min_connections)
        .max_connections(max_connections)
        .connect_timeout(Duration::from_secs(3))
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .to_owned();

    let db = Database::connect(connect_options).await?;

    Ok(db)
}
