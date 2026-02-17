use thiserror::Error;

#[derive(Debug, Error)]
pub enum LandEfficiencyRepositoryError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Row mapping error: {0}")]
    RowMapping(String),
}
