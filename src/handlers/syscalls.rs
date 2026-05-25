use axum::extract::{Path, State};
use axum::Json;

use crate::app::AppState;
use crate::domain::Syscall;
use crate::error::{AppError, Result};

fn parse_syscall_number(number: &str) -> Result<u32> {
    number
        .parse::<u32>()
        .map_err(|_| AppError::BadRequest("number must be a non-negative 32-bit integer".into()))
}

pub async fn list(
    State(state): State<AppState>,
    Path((os, arch)): Path<(String, String)>,
) -> Result<Json<Vec<Syscall>>> {
    let syscalls = state.repo.list(&os, &arch).await?;
    Ok(Json(syscalls))
}

pub async fn get_by_name(
    State(state): State<AppState>,
    Path((os, arch, name)): Path<(String, String, String)>,
) -> Result<Json<Syscall>> {
    state
        .repo
        .get_by_name(&os, &arch, &name)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}

pub async fn get_by_number(
    State(state): State<AppState>,
    Path((os, arch, number)): Path<(String, String, String)>,
) -> Result<Json<Syscall>> {
    let number = parse_syscall_number(&number)?;

    state
        .repo
        .get_by_number(&os, &arch, number)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::parse_syscall_number;
    use crate::error::AppError;

    #[test]
    fn parses_valid_number() {
        assert_eq!(parse_syscall_number("123").unwrap(), 123);
    }

    #[test]
    fn rejects_negative_number() {
        match parse_syscall_number("-1").unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "number must be a non-negative 32-bit integer")
            }
            err => panic!("unexpected error: {err:?}"),
        }
    }

    #[test]
    fn rejects_number_larger_than_u32() {
        match parse_syscall_number("4294967296").unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "number must be a non-negative 32-bit integer")
            }
            err => panic!("unexpected error: {err:?}"),
        }
    }
}
