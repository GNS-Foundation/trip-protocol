// trip-verifier/src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TripError {
    #[error("Chain verification failed: {0}")]
    ChainIntegrity(String),

    #[error("Signature verification failed at breadcrumb {index}")]
    SignatureInvalid { index: u64 },

    #[error("Insufficient breadcrumbs: got {got}, need at least {need}")]
    InsufficientBreadcrumbs { got: usize, need: usize },

    #[error("PSD computation failed: {0}")]
    PsdError(String),

    #[error("Lévy fit failed: {0}")]
    LevyFitError(String),

    #[error("Invalid H3 cell: {0}")]
    InvalidH3Cell(String),

    #[error("Nonce mismatch in active verification")]
    NonceMismatch,

    #[error("Verification deadline expired")]
    DeadlineExpired,

    #[error("Certificate encoding error: {0}")]
    CertificateError(String),

    #[error("Deserialization error: {0}")]
    DeserializeError(String),
}

pub type Result<T> = std::result::Result<T, TripError>;
