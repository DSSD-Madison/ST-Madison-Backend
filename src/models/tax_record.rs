use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRecord {
    pub tax_year: i32,
    pub assessed_value_land: Decimal,
    pub assessed_value_improvement: Decimal,
    pub total_assessed_value: Decimal,
    pub county_tax: Decimal,
    pub city_tax: Decimal,
    pub school_tax: Decimal,
    pub matc_tax: Decimal,
    pub gross_tax: Decimal,
    pub net_tax: Decimal,
}
