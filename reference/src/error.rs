//! Error types for TRIP protocol

use thiserror::Error;

/// TRIP protocol errors
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid key length
    #[error("invalid key length")]
    InvalidKeyLength,

    /// Invalid HIT length
    #[error("invalid HIT length (expected 16 bytes)")]
    InvalidHitLength,

    /// Invalid hex encoding
    #[error("invalid hex encoding")]
    InvalidHex,

    /// Invalid handle format
    #[error("invalid handle format: {0}")]
    InvalidHandle(String),

    /// Invalid signature
    #[error("invalid signature")]
    InvalidSignature,

    /// Signature verification failed
    #[error("signature verification failed")]
    SignatureVerificationFailed,

    /// Invalid message format
    #[error("invalid message format")]
    InvalidMessageFormat,

    /// Unknown message type
    #[error("unknown message type: {0}")]
    UnknownMessageType(u8),

    /// Invalid state transition
    #[error("invalid state transition")]
    InvalidStateTransition,

    /// Trust level insufficient
    #[error("trust level insufficient: required {required}, got {actual}")]
    InsufficientTrust { required: u8, actual: u8 },

    /// Proof verification failed
    #[error("proof verification failed: {0}")]
    ProofVerificationFailed(String),

    /// Session not found
    #[error("session not found")]
    SessionNotFound,

    /// Session expired
    #[error("session expired")]
    SessionExpired,

    /// Rate limit exceeded
    #[error("rate limit exceeded")]
    RateLimitExceeded,

    /// Replay attack detected
    #[error("replay attack detected")]
    ReplayDetected,

    /// Decryption failed
    #[error("decryption failed")]
    DecryptionFailed,

    /// Encryption failed
    #[error("encryption failed")]
    EncryptionFailed,

    /// Invalid breadcrumb
    #[error("invalid breadcrumb: {0}")]
    InvalidBreadcrumb(String),

    /// Invalid epoch
    #[error("invalid epoch: {0}")]
    InvalidEpoch(String),

    /// Invalid trajectory
    #[error("invalid trajectory: {0}")]
    InvalidTrajectory(String),

    /// IO error
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for TRIP operations
pub type Result<T> = std::result::Result<T, Error>;

/// Protocol error codes (for wire format)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// Success (no error)
    Success = 0x00,
    /// Invalid message format
    InvalidFormat = 0x01,
    /// Invalid signature
    InvalidSignature = 0x02,
    /// Unknown HIT
    UnknownHit = 0x03,
    /// Trust level insufficient
    InsufficientTrust = 0x04,
    /// Proof verification failed
    ProofFailed = 0x05,
    /// Session not found
    SessionNotFound = 0x06,
    /// Session expired
    SessionExpired = 0x07,
    /// Rate limit exceeded
    RateLimited = 0x08,
    /// Replay detected
    ReplayDetected = 0x09,
    /// Decryption failed
    DecryptionFailed = 0x0A,
    /// Invalid state transition
    InvalidState = 0x0B,
    /// Resource exhausted
    ResourceExhausted = 0x0C,
    /// Not registered
    NotRegistered = 0x0D,
    /// Handle already claimed
    HandleTaken = 0x0E,
    /// Payment failed
    PaymentFailed = 0x0F,
    /// Unknown error
    Unknown = 0xFF,
}

impl From<&Error> for ErrorCode {
    fn from(err: &Error) -> Self {
        match err {
            Error::InvalidKeyLength | Error::InvalidHitLength | Error::InvalidMessageFormat => {
                ErrorCode::InvalidFormat
            }
            Error::InvalidSignature | Error::SignatureVerificationFailed => {
                ErrorCode::InvalidSignature
            }
            Error::InsufficientTrust { .. } => ErrorCode::InsufficientTrust,
            Error::ProofVerificationFailed(_) => ErrorCode::ProofFailed,
            Error::SessionNotFound => ErrorCode::SessionNotFound,
            Error::SessionExpired => ErrorCode::SessionExpired,
            Error::RateLimitExceeded => ErrorCode::RateLimited,
            Error::ReplayDetected => ErrorCode::ReplayDetected,
            Error::DecryptionFailed => ErrorCode::DecryptionFailed,
            Error::InvalidStateTransition => ErrorCode::InvalidState,
            _ => ErrorCode::Unknown,
        }
    }
}
