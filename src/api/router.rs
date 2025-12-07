use anyhow::Context;
use axum::{
    Extension, Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::handlers;
use super::openapi::ApiDoc;
use crate::connections;

pub async fn call() -> Result<(), anyhow::Error> {
    let conns = connections::build().await?;

    let openapi_config = utoipa_swagger_ui::Config::default()
        .display_operation_id(true)
        .display_request_duration(true);

    let app = Router::new()
        .route("/", get(root))
        .route("/events/search", post(handlers::events::search))
        .route("/debug/crawler", post(handlers::debug::crawler))
        .layer(Extension(conns))
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
                .config(openapi_config),
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

async fn root() -> &'static str {
    ":)"
}
