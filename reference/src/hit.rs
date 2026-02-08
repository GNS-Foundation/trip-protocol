//! Human Identity Tag (HIT) - 128-bit compact identifier
//!
//! The HIT is derived from the Human Identity (public key) using SHA-256:
//!
//! ```text
//! HIT = SHA-256(HI)[0:16]
//! ```
//!
//! HITs are used for:
//! - Compact routing identifiers (16 bytes vs 32)
//! - Protocol message headers
//! - Peer lookup tables
//! - HIP (RFC 7401) interoperability

use crate::identity::PublicKey;
use crate::error::{Error, Result};
use sha2::{Sha256, Digest};
use std::fmt;

/// Size of HIT in bytes
pub const HIT_SIZE: usize = 16;

/// Human Identity Tag - 128-bit identifier derived from public key
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hit([u8; HIT_SIZE]);

impl Hit {
    /// Create HIT from raw bytes
    pub fn from_bytes(bytes: [u8; HIT_SIZE]) -> Self {
        Self(bytes)
    }

    /// Create HIT from byte slice
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != HIT_SIZE {
            return Err(Error::InvalidHitLength);
        }
        let mut bytes = [0u8; HIT_SIZE];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Derive HIT from a public key
    ///
    /// HIT = SHA-256(PublicKey)[0:16]
    pub fn from_public_key(public_key: &PublicKey) -> Self {
        let hash = Sha256::digest(public_key.as_bytes());
        let mut bytes = [0u8; HIT_SIZE];
        bytes.copy_from_slice(&hash[..HIT_SIZE]);
        Self(bytes)
    }

    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8; HIT_SIZE] {
        &self.0
    }

    /// Convert to hex string (32 characters)
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let bytes = hex::decode(hex_str).map_err(|_| Error::InvalidHex)?;
        Self::from_slice(&bytes)
    }

    /// Get short display (first 8 hex chars)
    pub fn short(&self) -> String {
        self.to_hex()[..8].to_string()
    }

    /// Check if this HIT was derived from the given public key
    pub fn matches(&self, public_key: &PublicKey) -> bool {
        let derived = Self::from_public_key(public_key);
        self.0 == derived.0
    }
}

impl fmt::Debug for Hit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hit({})", self.short())
    }
}

impl fmt::Display for Hit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl AsRef<[u8]> for Hit {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; HIT_SIZE]> for Hit {
    fn from(bytes: [u8; HIT_SIZE]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Hit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Hit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_hex(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::Identity;

    #[test]
    fn test_hit_size() {
        let id = Identity::generate();
        let hit = id.hit();
        assert_eq!(hit.as_bytes().len(), 16);
    }

    #[test]
    fn test_hit_hex_roundtrip() {
        let id = Identity::generate();
        let hit = id.hit();
        
        let hex = hit.to_hex();
        assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
        
        let hit2 = Hit::from_hex(&hex).unwrap();
        assert_eq!(hit, hit2);
    }

    #[test]
    fn test_hit_matches() {
        let id = Identity::generate();
        let hit = id.hit();
        
        assert!(hit.matches(id.public_key()));
        
        // Different identity should not match
        let id2 = Identity::generate();
        assert!(!hit.matches(id2.public_key()));
    }

    #[test]
    fn test_hit_deterministic() {
        let seed = [1u8; 32];
        let id1 = Identity::from_seed(&seed);
        let id2 = Identity::from_seed(&seed);
        
        assert_eq!(id1.hit(), id2.hit());
    }

    #[test]
    fn test_hit_from_slice_wrong_size() {
        assert!(Hit::from_slice(&[0u8; 8]).is_err());
        assert!(Hit::from_slice(&[0u8; 32]).is_err());
        assert!(Hit::from_slice(&[0u8; 16]).is_ok());
    }

    #[test]
    fn test_known_vector() {
        // Test vector from spec
        let public_key_hex = "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
        let public_key = PublicKey::from_hex(public_key_hex).unwrap();
        let hit = Hit::from_public_key(&public_key);
        
        // HIT should be first 16 bytes of SHA-256
        let full_hash = Sha256::digest(public_key.as_bytes());
        assert_eq!(hit.as_bytes(), &full_hash[..16]);
    }
}
