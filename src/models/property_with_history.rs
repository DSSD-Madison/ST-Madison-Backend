use serde::{Deserialize, Serialize};

use super::{Property, TaxRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyWithHistory {
    pub property: Property,
    pub tax_records: Vec<TaxRecord>,
}
