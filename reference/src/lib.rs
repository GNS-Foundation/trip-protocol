//! # TRIP Protocol Reference Implementation
//!
//! This crate provides a reference implementation of the Trajectory Routing Identity Protocol (TRIP).
//!
//! TRIP enables secure, authenticated communication between humans by separating identity
//! from network location, with Sybil resistance through Proof-of-Trajectory.
//!
//! ## Quick Start
//!
//! ```rust
//! use trip_protocol::{Identity, Hit, Handle};
//!
//! // Generate a new identity
//! let identity = Identity::generate();
//!
//! // Get the Human Identity Tag (HIT)
//! let hit = identity.hit();
//! println!("HIT: {}", hit.to_hex());
//!
//! // Get Stellar address for payments
//! #[cfg(feature = "stellar")]
//! {
//!     let stellar_addr = identity.stellar_address();
//!     println!("Stellar: {}", stellar_addr);
//! }
//! ```
//!
//! ## Features
//!
//! - `std` (default): Enable standard library support
//! - `serde`: Enable serialization/deserialization
//! - `stellar`: Enable Stellar address derivation
//!
//! ## Protocol Overview
//!
//! TRIP defines:
//! - **Human Identity (HI)**: Ed25519 public key as stable identifier
//! - **Human Identity Tag (HIT)**: 128-bit hash for compact routing
//! - **@handle**: Human-readable names bound to identity
//! - **Proof-of-Trajectory**: Sybil resistance through physical movement
//! - **Trust Levels**: Progressive access based on trajectory history

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

extern crate alloc;

pub mod identity;
pub mod hit;
pub mod handle;
pub mod handshake;
pub mod session;
pub mod messages;
pub mod trust;
pub mod trajectory;
pub mod crypto;
pub mod error;

// Re-exports
pub use identity::{Identity, PublicKey, PrivateKey};
pub use hit::Hit;
pub use handle::Handle;
pub use handshake::{Handshake, HandshakeState};
pub use session::Session;
pub use messages::{Message, MessageType};
pub use trust::{TrustLevel, TrustProof};
pub use trajectory::{Breadcrumb, Epoch};
pub use error::{Error, Result};

/// Protocol version
pub const PROTOCOL_VERSION: u8 = 0x01;

/// Size of Human Identity (Ed25519 public key) in bytes
pub const HI_SIZE: usize = 32;

/// Size of Human Identity Tag in bytes
pub const HIT_SIZE: usize = 16;

/// Size of Ed25519 signature in bytes
pub const SIGNATURE_SIZE: usize = 64;

/// Maximum handle length
pub const MAX_HANDLE_LENGTH: usize = 20;

/// Minimum breadcrumbs per epoch
pub const MIN_BREADCRUMBS_PER_EPOCH: usize = 100;

/// Minimum interval between breadcrumbs (seconds)
pub const MIN_BREADCRUMB_INTERVAL_SECS: u64 = 600; // 10 minutes

/// Maximum interval between breadcrumbs (seconds)
pub const MAX_BREADCRUMB_INTERVAL_SECS: u64 = 86400; // 24 hours

/// H3 resolution for location cells
pub const H3_RESOLUTION: u8 = 7; // ~5km² cells

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::identity::{Identity, PublicKey};
    pub use crate::hit::Hit;
    pub use crate::handle::Handle;
    pub use crate::trust::TrustLevel;
    pub use crate::trajectory::{Breadcrumb, Epoch};
    pub use crate::error::{Error, Result};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_constants() {
        assert_eq!(HI_SIZE, 32);
        assert_eq!(HIT_SIZE, 16);
        assert_eq!(SIGNATURE_SIZE, 64);
    }
}
