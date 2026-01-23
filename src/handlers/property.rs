use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    models::PropertyWithHistory,
    repositories::property::{PropertyRepository, duckdb::DuckDbPropertyRepository},
    state::AppState,
};

pub async fn get_property_by_address(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<PropertyWithHistory>, String> {
    let repo = DuckDbPropertyRepository::new(state.db.clone());

    let property = repo
        .get_property_with_history(&address)
        .map_err(|e| format!("Failed  to fetch property: {:?}", e))?;

    Ok(Json(property))
}
