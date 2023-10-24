/// Result alias for `Error`
pub type Result<T> = std::result::Result<T, Error>;

/// Core error variants possible when interacting with the blockchain
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] crate::db::DatabaseError),

    #[error("{0}")]
    Custom(std::string::String),
}
