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
    logging::setup();
    api::router::call().await
}
