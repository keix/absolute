use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::handlers;
use crate::repository::SyscallRepository;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<SyscallRepository>,
}

impl AppState {
    pub async fn from_env() -> Self {
        let config = Config::from_env();

        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .load()
            .await;

        let mut ddb_builder = aws_sdk_dynamodb::config::Builder::from(&aws_config);
        if let Some(endpoint) = &config.ddb_endpoint {
            ddb_builder = ddb_builder.endpoint_url(endpoint);
        }
        let ddb = aws_sdk_dynamodb::Client::from_conf(ddb_builder.build());

        let repo = SyscallRepository::new(ddb, config.table_name);

        Self {
            repo: Arc::new(repo),
        }
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/v1/:os/:arch/syscalls", get(handlers::syscalls::list))
        .route(
            "/v1/:os/:arch/syscalls/:name",
            get(handlers::syscalls::get_by_name),
        )
        .route(
            "/v1/:os/:arch/syscalls/number/:number",
            get(handlers::syscalls::get_by_number),
        )
        .route(
            "/v1/:os/:arch/registers/:instruction",
            get(handlers::registers::get),
        )
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
