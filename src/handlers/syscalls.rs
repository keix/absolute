use axum::extract::{Path, State};
use axum::Json;

use crate::app::AppState;
use crate::domain::Syscall;
use crate::error::{AppError, Result};

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
    Path((os, arch, number)): Path<(String, String, u32)>,
) -> Result<Json<Syscall>> {
    state
        .repo
        .get_by_number(&os, &arch, number)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}
