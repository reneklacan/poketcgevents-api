mod api;
mod entities;
mod logging;
mod persistence;
mod services;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    logging::setup();
    api::router::call().await
}
