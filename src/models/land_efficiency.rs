use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LandEfficiencyMetrics {
    pub land_value_per_sqft: Option<f64>,
    pub net_taxes_per_sqft: Option<f64>,
    pub land_share_of_property: Option<f64>,
    pub land_value_alignment_index: Option<f64>,
}
