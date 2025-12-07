mod api;
mod connections;
mod entities;
mod error;
mod logging;
mod persistence;
mod services;

pub use connections::Connections;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    connections::migrate().await?;
    logging::setup();
    api::router::call().await
}
