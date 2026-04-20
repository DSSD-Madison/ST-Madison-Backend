use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::ParcelResponse,
    repositories::property::{PropertyRepository, duckdb::DuckDbPropertyRepository},
    state::AppState,
};

pub async fn get_parcel_by_address(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<ParcelResponse>, (StatusCode, String)> {
    let repo = DuckDbPropertyRepository::new(state.db.clone());

    let property_with_history = repo
        .get_property_with_history(&address)
        .map_err(|e| (StatusCode::NOT_FOUND, format!("Property not found: {e:?}")))?;

    Ok(Json(ParcelResponse::from(property_with_history)))
}
