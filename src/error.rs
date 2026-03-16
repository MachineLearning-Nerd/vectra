use thiserror::Error;

#[derive(Debug, Error)]
pub enum VectraError {
    #[error("Vectors must have the same dimension: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Vector must not be empty")]
    EmptyVector,

    #[error("Zero vector cannot be used with cosine similarity")]
    ZeroVector,

    #[error("Vector ID {0} not found")]
    NotFound(u64),

    #[error("K must be greater than 0")]
    InvalidK,
}
