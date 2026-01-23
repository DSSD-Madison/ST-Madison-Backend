#[derive(Debug)]
pub enum PropertyRepositoryError {
    NotFound,
    Database(String),
    RowMapping(String),
}
