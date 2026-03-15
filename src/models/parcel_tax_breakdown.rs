use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParcelTaxBreakdown {
    pub parcel_address: String,
    pub tax_year: i32,
    pub county_tax: f64,
    pub school_tax: f64,
    pub city_tax: f64,
    pub matc_tax: f64,
}