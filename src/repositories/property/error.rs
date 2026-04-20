#[derive(Debug)]
pub enum PropertyRepositoryError {
    NotFound,
    Database(String),
    RowMapping(String),
}

impl std::fmt::Display for PropertyRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "property not found"),
            Self::Database(msg) => write!(f, "database error: {msg}"),
            Self::RowMapping(msg) => write!(f, "row mapping error: {msg}"),
        }
    }
}
