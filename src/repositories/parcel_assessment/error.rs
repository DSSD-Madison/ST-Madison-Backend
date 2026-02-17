#[derive(Debug)]
pub enum ParcelAssessmentRepositoryError {
    NotFound,
    Database(String),
    RowMapping(String),
}
