use serde::{Deserialize, Serialize};

use super::PropertyWithHistory;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParcelResponse {
    pub assessments: AssessmentsData,
    pub land_efficiency: LandEfficiencyData,
    pub trends: TrendsData,
    pub tax_breakdown: TaxBreakdownData,
    pub property_details: PropertyDetailsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentsData {
    pub land_value: String,
    pub improvement_value: String,
    pub total_assessed_value: String,
    pub net_taxes: String,
    pub lot_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LandEfficiencyData {
    pub land_value_per_sqft: String,
    pub net_taxes_per_sqft: String,
    pub land_share_of_property: String,
    pub land_value_alignment_index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendsData {
    pub years: Vec<i32>,
    pub effective_tax_rate: Vec<f64>,
    pub net_taxes: Vec<f64>,
    pub assessed_value: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxBreakdownData {
    pub years: Vec<i32>,
    pub sources: Vec<TaxSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSource {
    pub label: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDetailsData {
    pub property_class: String,
    pub property_use: String,
    pub year_built: Option<i32>,
    pub bedrooms: Option<f64>,
    pub full_baths: Option<f64>,
    pub half_baths: Option<f64>,
    pub total_living_area: Option<String>,
    pub home_style: Option<String>,
    pub multi_story: Option<bool>,
}

// --- formatting helpers ---

fn format_with_commas(n: i64) -> String {
    let s = n.abs().to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn format_currency(value: f64) -> String {
    format!("${}", format_with_commas(value.round() as i64))
}

fn format_sqft(value: f64) -> String {
    format!("{} sqft", format_with_commas(value.round() as i64))
}

fn format_currency_per_sqft(value: f64) -> String {
    format!("${:.2}/sqft", value)
}

// --- conversion ---

impl From<PropertyWithHistory> for ParcelResponse {
    fn from(pwh: PropertyWithHistory) -> Self {
        let property = &pwh.property;

        // Records from DB arrive newest-first; reverse for chronological order.
        let mut records = pwh.tax_records.clone();
        records.reverse();

        let latest = records.last();

        // Assessments — from most recent record
        let assessments = AssessmentsData {
            land_value: latest
                .map(|r| format_currency(r.assessed_value_land))
                .unwrap_or_else(|| "-".to_string()),
            improvement_value: latest
                .map(|r| format_currency(r.assessed_value_improvement))
                .unwrap_or_else(|| "-".to_string()),
            total_assessed_value: latest
                .map(|r| format_currency(r.total_assessed_value))
                .unwrap_or_else(|| "-".to_string()),
            net_taxes: latest
                .map(|r| format_currency(r.net_tax))
                .unwrap_or_else(|| "-".to_string()),
            lot_size: property
                .lot_size
                .map(|s| format_sqft(s))
                .unwrap_or_else(|| "-".to_string()),
        };

        // Land efficiency — computed from latest record + lot size
        let land_efficiency = match (latest, property.lot_size) {
            (Some(r), Some(lot_sqft)) if lot_sqft > 0.0 => LandEfficiencyData {
                land_value_per_sqft: format_currency_per_sqft(
                    r.assessed_value_land / lot_sqft,
                ),
                net_taxes_per_sqft: format_currency_per_sqft(r.net_tax / lot_sqft),
                land_share_of_property: if r.total_assessed_value > 0.0 {
                    format!(
                        "{:.1}%",
                        (r.assessed_value_land / r.total_assessed_value) * 100.0
                    )
                } else {
                    "-".to_string()
                },
                land_value_alignment_index: if r.total_assessed_value > 0.0 {
                    format!(
                        "{:.4}",
                        r.assessed_value_land / r.total_assessed_value
                    )
                } else {
                    "-".to_string()
                },
            },
            _ => LandEfficiencyData {
                land_value_per_sqft: "-".to_string(),
                net_taxes_per_sqft: "-".to_string(),
                land_share_of_property: "-".to_string(),
                land_value_alignment_index: "-".to_string(),
            },
        };

        // Trends — one value per year, chronological
        let years: Vec<i32> = records.iter().map(|r| r.tax_year).collect();
        let effective_tax_rate: Vec<f64> = records
            .iter()
            .map(|r| {
                if r.total_assessed_value > 0.0 {
                    (r.net_tax / r.total_assessed_value * 10000.0).round() / 10000.0
                } else {
                    0.0
                }
            })
            .collect();
        let net_taxes: Vec<f64> = records.iter().map(|r| r.net_tax).collect();
        let assessed_value: Vec<f64> =
            records.iter().map(|r| r.total_assessed_value).collect();

        let trends = TrendsData {
            years,
            effective_tax_rate,
            net_taxes,
            assessed_value,
        };

        // Tax breakdown — by source across years
        let breakdown_years: Vec<i32> = records.iter().map(|r| r.tax_year).collect();
        let sources = vec![
            TaxSource {
                label: "City".to_string(),
                values: records.iter().map(|r| r.city_tax).collect(),
            },
            TaxSource {
                label: "School".to_string(),
                values: records.iter().map(|r| r.school_tax).collect(),
            },
            TaxSource {
                label: "County".to_string(),
                values: records.iter().map(|r| r.county_tax).collect(),
            },
            TaxSource {
                label: "MATC".to_string(),
                values: records.iter().map(|r| r.matc_tax).collect(),
            },
        ];

        let tax_breakdown = TaxBreakdownData {
            years: breakdown_years,
            sources,
        };

        // Property details
        let property_details = PropertyDetailsData {
            property_class: property.property_class.clone(),
            property_use: property.property_use.clone(),
            year_built: None,
            bedrooms: property.bedrooms,
            full_baths: property.full_baths,
            half_baths: property.half_baths,
            total_living_area: property
                .total_living_area
                .map(|a| format_sqft(a)),
            home_style: None,
            multi_story: None,
        };

        ParcelResponse {
            assessments,
            land_efficiency,
            trends,
            tax_breakdown,
            property_details,
        }
    }
}
