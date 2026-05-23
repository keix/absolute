use axum::extract::{Path, State};
use axum::Json;

use crate::app::AppState;
use crate::domain::RegisterConvention;
use crate::error::{AppError, Result};

pub async fn get(
    State(state): State<AppState>,
    Path((os, arch, instruction)): Path<(String, String, String)>,
) -> Result<Json<RegisterConvention>> {
    state
        .repo
        .get_register_convention(&os, &arch, &instruction)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}
