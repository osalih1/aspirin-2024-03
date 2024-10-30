use thiserror::Error;

#[derive(Error, Debug)]
pub enum AspirinEatsError {
    /// Error when trying to parse a JSON string
    #[error("Parse error: {0}")]
    ParseError(String),
    /// Error when an invalid request is received
    #[error("Invalid request")]
    InvalidRequest,
    /// Error when a resource is not found
    #[error("Not found")]
    NotFound,
    /// Error when an unsupported HTTP method is used
    #[error("Method not allowed")]
    MethodNotAllowed,
    /// IO Error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Database Error (from rusqlite)
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
}
