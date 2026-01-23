use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub site_parcel_id: String,
    pub parcel_address: String,
    pub property_class: String,
    pub property_use: String,
    pub area_name: String,
    pub alder_district_name: String,
    pub bedrooms: Option<f64>,
    pub full_baths: Option<f64>,
    pub half_baths: Option<f64>,
    pub total_living_area: Option<f64>,
    pub lot_size: Option<f64>,
    pub current_value_2025: f64,
}
