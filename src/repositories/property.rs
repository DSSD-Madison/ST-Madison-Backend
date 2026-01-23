pub mod duckdb;
pub mod error;

use crate::models::{Property, PropertyWithHistory, TaxRecord};
use error::PropertyRepositoryError;

pub trait PropertyRepository {
    fn get_property_with_history(
        &self,
        address: impl AsRef<str>,
    ) -> Result<PropertyWithHistory, PropertyRepositoryError>;

    fn get_tax_records(
        &self,
        parcel_id: impl AsRef<str>,
    ) -> Result<Vec<TaxRecord>, PropertyRepositoryError>;
}
