use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{
    repositories::property::{PropertyRepository, duckdb::DuckDbPropertyRepository},
    state::AppState,
};

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn search_addresses(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let q = params.q.trim().to_string();
    if q.len() < 2 {
        return Ok(Json(vec![]));
    }

    let repo = DuckDbPropertyRepository::new(state.db.clone());
    let results = repo
        .search_addresses(&q)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results))
}
