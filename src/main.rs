use lambda_http::{run, Error};
use system_calls::app;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,system_calls=debug".into()),
        )
        .json()
        .with_target(false)
        .without_time()
        .init();

    let state = app::AppState::from_env().await;
    run(app::router(state)).await
}
