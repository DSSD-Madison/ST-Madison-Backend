use std::sync::{Arc, Mutex};

use duckdb::Connection;

use crate::models::ParcelAssessmentModel;

mod error;
pub use error::ParcelAssessmentRepositoryError;

pub struct DuckDbParcelAssessmentRepository {
    db: Arc<Mutex<Connection>>,
}

impl DuckDbParcelAssessmentRepository {
    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db }
    }

    pub fn get_parcel_assessment(
        &self,
        parcel_id: impl AsRef<str>,
    ) -> Result<Vec<ParcelAssessmentModel>, ParcelAssessmentRepositoryError> {
        let parcel_id = parcel_id.as_ref();
        let conn = self
            .db
            .lock()
            .map_err(|e| ParcelAssessmentRepositoryError::Database(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                r#"SELECT
                    current_land_value,
                    current_improvement_value,
                    current_total_value,
                    net_taxes,
                    lot_size
                FROM silver.parcels
                WHERE parcel_id = ?"#,
            )
            .map_err(|e| ParcelAssessmentRepositoryError::Database(e.to_string()))?;

        let records = stmt
            .query_map([parcel_id], |row| {
                Ok(ParcelAssessmentModel {
                    current_land_value: row.get(0)?,
                    current_improvement_value: row.get(1)?,
                    current_total_value: row.get(2)?,
                    net_taxes: row.get(3)?,
                    lot_size: row.get(4)?,
                })
            })
            .map_err(|e| ParcelAssessmentRepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ParcelAssessmentRepositoryError::RowMapping(e.to_string()))?;

        Ok(records)
    }
}