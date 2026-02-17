pub mod error;

use std::sync::{Arc, Mutex};

use duckdb::Connection;

use crate::models::LandEfficiencyMetrics;

use error::LandEfficiencyRepositoryError;

pub struct DuckDbLandEfficiencyRepository {
    db: Arc<Mutex<Connection>>,
}

impl DuckDbLandEfficiencyRepository {
    pub fn new(db: Arc<Mutex<Connection>>) -> Self {
        Self { db }
    }

    pub fn get_land_efficiency_metrics(
        &self,
    ) -> Result<Vec<LandEfficiencyMetrics>, LandEfficiencyRepositoryError> {
        let conn = self
            .db
            .lock()
            .map_err(|e| LandEfficiencyRepositoryError::Database(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                r#"SELECT
                    land_value_per_sqft_lot,
                    net_taxes_per_sqft_lot,
                    land_share_property,
                    land_value_alignment_index
                FROM silver.parcels"#,
            )
            .map_err(|e| LandEfficiencyRepositoryError::Database(e.to_string()))?;

        let records = stmt
            .query_map([], |row| {
                Ok(LandEfficiencyMetrics {
                    land_value_per_sqft: row.get(0)?,
                    net_taxes_per_sqft: row.get(1)?,
                    land_share_of_property: row.get(2)?,
                    land_value_alignment_index: row.get(3)?,
                })
            })
            .map_err(|e| LandEfficiencyRepositoryError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| LandEfficiencyRepositoryError::RowMapping(e.to_string()))?;

        Ok(records)
    }
}
