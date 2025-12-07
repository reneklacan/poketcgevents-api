use std::str::FromStr;
use tracing_subscriber::{
    Registry, filter::EnvFilter, fmt::writer::MakeWriterExt, layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn setup() {
    let level_str = std::env::var("RUST_LOG")
        .or_else(|_| std::env::var("OTEL_LOG_LEVEL"))
        .unwrap_or_else(|_| "debug".to_string());
    let level = tracing::Level::from_str(&level_str)
        .unwrap_or_else(|_| panic!("Invalid log level: {level_str}"));

    unsafe {
        std::env::set_var(
            "RUST_LOG",
            [
                level_str.as_str(),
                "h2=info",
                "handlebars::render=info",
                "hyper=error",
                "opentelemetry_sdk=info",
                "otel::tracing=trace",
                "otel=debug",
                "reqwest::connect=info",
                "rustls=error",
                "sea_orm::driver::sqlx_postgres=trace",
                "serenity::gateway::bridge::shard_runner=warn",
                "serenity::gateway::shard=warn",
                "sqlx_core::logger=info",
                "sqlx_postgres::options::pgpass=info",
                "tonic=info",
                "tower_http::trace=trace",
            ]
            .join(","),
        );
    }
    let env_filter = EnvFilter::from_default_env();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout.with_max_level(level))
        .map_event_format(|e| e.compact());
    let subscriber = Registry::default().with(fmt_layer).with(env_filter);
    subscriber.init();
}
