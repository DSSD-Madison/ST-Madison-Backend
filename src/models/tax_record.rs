use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRecord {
    pub tax_year: i32,
    pub assessed_value_land: f64,
    pub assessed_value_improvement: f64,
    pub total_assessed_value: f64,
    pub county_tax: f64,
    pub city_tax: f64,
    pub school_tax: f64,
    pub matc_tax: f64,
    pub gross_tax: f64,
    pub net_tax: f64,
}
