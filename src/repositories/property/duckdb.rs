use std::sync::{Arc, Mutex};

use duckdb::Connection;
use rust_decimal::Decimal;

use crate::models::Property;
use crate::models::PropertyWithHistory;
use crate::models::TaxRecord;

use super::PropertyRepository;
use super::PropertyRepositoryError;

pub struct DuckDbPropertyRepository {
    db: Arc<Mutex<Connection>>,
}

impl DuckDbPropertyRepository {
    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db }
    }
}

impl PropertyRepository for DuckDbPropertyRepository {
    fn get_property_with_history(
        &self,
        address: impl AsRef<str>,
    ) -> Result<PropertyWithHistory, PropertyRepositoryError> {
        let address = address.as_ref();
        let property = self.get_property(address)?;
        let tax_records = self.get_tax_records(&property.site_parcel_id)?;

        Ok(PropertyWithHistory {
            property,
            tax_records,
        })
    }

    fn get_tax_records(
        &self,
        parcel_id: impl AsRef<str>,
    ) -> Result<Vec<TaxRecord>, PropertyRepositoryError> {
        let parcel_id = parcel_id.as_ref();
        let conn = self
            .db
            .lock()
            .map_err(|e| PropertyRepositoryError::Database(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                r#"SELECT tax_year, assessed_value_land, assessed_value_improvement,
                      total_assessed_value, county_tax, city_tax, school_tax,
                      matc_tax, gross_tax, net_tax
               FROM silver.tax_roll
               WHERE parcel_id = ?
               ORDER BY tax_year DESC"#,
            )
            .map_err(|e| PropertyRepositoryError::Database(e.to_string()))?;

        let records = stmt
            .query_map([parcel_id], |row| {
                Ok(TaxRecord {
                    tax_year: row.get(0)?,
                    assessed_value_land: row.get(1)?,
                    assessed_value_improvement: row.get(2)?,
                    total_assessed_value: row.get(3)?,
                    county_tax: row.get(4)?,
                    city_tax: row.get(5)?,
                    school_tax: row.get(6)?,
                    matc_tax: row.get(7)?,
                    gross_tax: row.get(8)?,
                    net_tax: row.get(9)?,
                })
            })
            .map_err(|e| PropertyRepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| PropertyRepositoryError::RowMapping(e.to_string()))?;

        Ok(records)
    }
}

impl DuckDbPropertyRepository {
    fn get_property(&self, address: &str) -> Result<Property, PropertyRepositoryError> {
        let conn = self
            .db
            .lock()
            .map_err(|e| PropertyRepositoryError::Database(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                r#"SELECT site_parcel_id, parcel_address, property_class, property_use,
                      area_name, alder_district_name, bedrooms, full_baths, half_baths,
                      total_living_area, lot_size, current_total_value
               FROM gold.sites
               WHERE parcel_address = ?"#,
            )
            .map_err(|e| PropertyRepositoryError::Database(e.to_string()))?;

        let property = stmt
            .query_row([address], |row| {
                Ok(Property {
                    site_parcel_id: row.get(0)?,
                    parcel_address: row.get(1)?,
                    property_class: row.get(2)?,
                    property_use: row.get(3)?,
                    area_name: row.get(4)?,
                    alder_district_name: row.get(5)?,
                    bedrooms: row.get(6)?,
                    full_baths: row.get(7)?,
                    half_baths: row.get(8)?,
                    total_living_area: row.get(9)?,
                    lot_size: row.get(10)?,
                    current_value_2025: row.get(11)?,
                })
            })
            .map_err(|_| PropertyRepositoryError::NotFound)?;

        Ok(property)
    }
}
