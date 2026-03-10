use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    models::ParcelAssessmentModel,
    repositories::parcel_assessment::DuckDbParcelAssessmentRepository, state::AppState,
};

pub async fn get_parcel_assessment_by_id(
    State(state): State<AppState>,
    Path(parcel_id): Path<String>,
) -> Result<Json<Vec<ParcelAssessmentModel>>, String> {
    let repo = DuckDbParcelAssessmentRepository::new(state.db.clone());

    let assessment = repo
        .get_parcel_assessment(&parcel_id)
        .map_err(|e| format!("Failed  to fetch property: {:?}", e))?;

    Ok(Json(assessment))
}
