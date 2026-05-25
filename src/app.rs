use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::handlers;
use crate::repository::{SyscallRepository, SyscallStore};

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn SyscallStore>,
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{router, AppState};
    use async_trait::async_trait;
    use axum::body::{to_bytes, Body};
    use http::{Request, StatusCode};
    use serde_json::json;
    use tower::util::ServiceExt;

    use crate::domain::{RegisterConvention, Syscall};
    use crate::error::Result;
    use crate::repository::SyscallStore;

    struct StubStore {
        syscall_by_number: Option<Syscall>,
    }

    #[async_trait]
    impl SyscallStore for StubStore {
        async fn get_by_name(&self, _os: &str, _arch: &str, _name: &str) -> Result<Option<Syscall>> {
            panic!("get_by_name should not be called in these tests")
        }

        async fn get_by_number(
            &self,
            _os: &str,
            _arch: &str,
            _number: u32,
        ) -> Result<Option<Syscall>> {
            Ok(self.syscall_by_number.clone())
        }

        async fn list(&self, _os: &str, _arch: &str) -> Result<Vec<Syscall>> {
            panic!("list should not be called in these tests")
        }

        async fn get_register_convention(
            &self,
            _os: &str,
            _arch: &str,
            _instruction: &str,
        ) -> Result<Option<RegisterConvention>> {
            panic!("get_register_convention should not be called in these tests")
        }
    }

    fn app_with_store(store: StubStore) -> axum::Router {
        router(AppState {
            repo: Arc::new(store),
        })
    }

    fn sample_syscall() -> Syscall {
        Syscall {
            os: "linux".into(),
            arch: "x86_64".into(),
            number: 1,
            abi: "sysv".into(),
            name: "write".into(),
            entry: "sys_write".into(),
            man_url: Some("https://man7.org/linux/man-pages/man2/write.2.html".into()),
        }
    }

    #[tokio::test]
    async fn get_by_number_rejects_invalid_number_with_bad_request_json() {
        let response = app_with_store(StubStore {
            syscall_by_number: None,
        })
        .oneshot(
            Request::builder()
                .uri("/v1/linux/x86_64/syscalls/number/-1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
            json!({ "error": "number must be a non-negative 32-bit integer" })
        );
    }

    #[tokio::test]
    async fn get_by_number_returns_not_found_when_repository_misses() {
        let response = app_with_store(StubStore {
            syscall_by_number: None,
        })
        .oneshot(
            Request::builder()
                .uri("/v1/linux/x86_64/syscalls/number/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
            json!({ "error": "not found" })
        );
    }

    #[tokio::test]
    async fn get_by_number_returns_syscall_json_when_found() {
        let response = app_with_store(StubStore {
            syscall_by_number: Some(sample_syscall()),
        })
        .oneshot(
            Request::builder()
                .uri("/v1/linux/x86_64/syscalls/number/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
            json!({
                "os": "linux",
                "arch": "x86_64",
                "number": 1,
                "abi": "sysv",
                "name": "write",
                "entry": "sys_write",
                "man_url": "https://man7.org/linux/man-pages/man2/write.2.html"
            })
        );
    }
}
