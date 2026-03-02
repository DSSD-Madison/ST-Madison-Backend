use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParcelAssessmentModel {
    pub current_land_value: Option<i64>,
    pub current_improvement_value: Option<i64>,
    pub current_total_value: Option<i64>,
    pub net_taxes: Option<f64>,
    pub lot_size: Option<f64>,
}
