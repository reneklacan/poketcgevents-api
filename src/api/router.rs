use anyhow::Context;
use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::handlers;
use super::openapi::ApiDoc;

pub async fn call() -> Result<(), anyhow::Error> {
    let app = Router::new()
        .route("/", get(homepage))
        .route("/about", get(about))
        .route("/contact", post(contact))
        .route("/debug/crawler", post(handlers::debug::crawler));

    let openapi_config = utoipa_swagger_ui::Config::default()
        .display_operation_id(true)
        .display_request_duration(true);

    let app = app.merge(
        SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
            .config(openapi_config.clone()),
    );

    let port: u16 = std::env::var("PORT")
        .unwrap_or("4400".to_string())
        .parse()
        .unwrap_or(4400);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .context("Failed to start server")?;

    Ok(())
}
// Homepage handler
async fn homepage() -> &'static str {
    "Welcome to My Rust Website!"
}
// About page handler
async fn about() -> &'static str {
    "This is the About Page of the Rust Website."
}
// Contact form handler
#[derive(Deserialize)]
struct ContactForm {
    name: String,
    message: String,
}
#[derive(Serialize)]
struct ResponseMessage {
    status: String,
    message: String,
}
async fn contact(Json(payload): Json<ContactForm>) -> Json<ResponseMessage> {
    Json(ResponseMessage {
        status: "success".to_string(),
        message: format!("Thanks for reaching out, {}!", payload.name),
    })
}
