use axum::{Json, extract::State};

use crate::{
    models::LandEfficiencyMetrics,
    repositories::land_efficiency::DuckDbLandEfficiencyRepository,
    state::AppState,
};

pub async fn get_land_efficiency_metrics(
    State(state): State<AppState>,
) -> Result<Json<Vec<LandEfficiencyMetrics>>, String> {
    let repo = DuckDbLandEfficiencyRepository::new(state.db.clone());

    let metrics = repo
        .get_land_efficiency_metrics()
        .map_err(|e| format!("Failed to fetch land efficiency metrics: {:?}", e))?;

    Ok(Json(metrics))
}