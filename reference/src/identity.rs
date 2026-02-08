//! Human Identity (HI) - Ed25519 keypairs as identity roots
//!
//! The Human Identity is the canonical identifier in TRIP. It is an Ed25519 public key
//! from which all other identifiers are derived:
//!
//! - **HIT**: SHA-256(HI)[0:16]
//! - **Stellar Address**: StrKey encoding
//! - **Facets**: HKDF-derived child keys

use crate::hit::Hit;
use crate::error::{Error, Result};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};

/// Ed25519 public key (Human Identity)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PublicKey([u8; 32]);

impl PublicKey {
    /// Create from raw bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create from byte slice
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != 32 {
            return Err(Error::InvalidKeyLength);
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Derive HIT from this public key
    pub fn hit(&self) -> Hit {
        Hit::from_public_key(self)
    }

    /// Convert to hex string
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

    /// Get the Stellar address for this public key
    #[cfg(feature = "stellar")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stellar")))]
    pub fn stellar_address(&self) -> String {
        stellar_strkey::ed25519::PublicKey(self.0).to_string()
    }
}

impl std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PublicKey({}...)", self.short())
    }
}

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Ed25519 private key (kept secure)
pub struct PrivateKey {
    signing_key: SigningKey,
}

impl PrivateKey {
    /// Generate a new random private key
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self { signing_key }
    }

    /// Create from seed bytes (32 bytes)
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(seed);
        Self { signing_key }
    }

    /// Get the corresponding public key
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.signing_key.verifying_key().to_bytes())
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.signing_key.sign(message).to_bytes()
    }

    /// Get raw seed bytes (SENSITIVE - use with caution)
    pub fn to_seed(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
}

impl Clone for PrivateKey {
    fn clone(&self) -> Self {
        Self::from_seed(&self.signing_key.to_bytes())
    }
}

/// Complete identity (keypair + optional metadata)
pub struct Identity {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl Identity {
    /// Generate a new random identity
    pub fn generate() -> Self {
        let private_key = PrivateKey::generate();
        let public_key = private_key.public_key();
        Self { private_key, public_key }
    }

    /// Create from seed bytes
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let private_key = PrivateKey::from_seed(seed);
        let public_key = private_key.public_key();
        Self { private_key, public_key }
    }

    /// Get the public key (Human Identity)
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Get the Human Identity Tag
    pub fn hit(&self) -> Hit {
        self.public_key.hit()
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.private_key.sign(message)
    }

    /// Verify a signature (static method)
    pub fn verify(public_key: &PublicKey, message: &[u8], signature: &[u8; 64]) -> bool {
        let verifying_key = match VerifyingKey::from_bytes(&public_key.0) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let sig = match Signature::from_bytes(signature) {
            Ok(s) => s,
            Err(_) => return false,
        };
        verifying_key.verify(message, &sig).is_ok()
    }

    /// Get Stellar address for payments
    #[cfg(feature = "stellar")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stellar")))]
    pub fn stellar_address(&self) -> String {
        self.public_key.stellar_address()
    }

    /// Derive a facet identity
    pub fn derive_facet(&self, facet_name: &str) -> Identity {
        use hkdf::Hkdf;

        let hk = Hkdf::<Sha256>::new(None, &self.private_key.to_seed());
        let info = format!("facet:{}", facet_name);
        let mut facet_seed = [0u8; 32];
        hk.expand(info.as_bytes(), &mut facet_seed)
            .expect("HKDF expand failed");
        
        Identity::from_seed(&facet_seed)
    }
}

impl Clone for Identity {
    fn clone(&self) -> Self {
        Self {
            private_key: self.private_key.clone(),
            public_key: self.public_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_generation() {
        let id = Identity::generate();
        assert_eq!(id.public_key().as_bytes().len(), 32);
    }

    #[test]
    fn test_sign_verify() {
        let id = Identity::generate();
        let message = b"Hello, TRIP!";
        let signature = id.sign(message);
        
        assert!(Identity::verify(id.public_key(), message, &signature));
        assert!(!Identity::verify(id.public_key(), b"wrong message", &signature));
    }

    #[test]
    fn test_hit_derivation() {
        let id = Identity::generate();
        let hit = id.hit();
        assert_eq!(hit.as_bytes().len(), 16);
        
        // HIT should match public key
        assert!(hit.matches(id.public_key()));
    }

    #[test]
    fn test_facet_derivation() {
        let id = Identity::generate();
        let work_facet = id.derive_facet("work");
        let home_facet = id.derive_facet("home");
        
        // Facets should be different
        assert_ne!(work_facet.public_key().as_bytes(), home_facet.public_key().as_bytes());
        
        // Same facet name should give same key
        let work_facet_2 = id.derive_facet("work");
        assert_eq!(work_facet.public_key().as_bytes(), work_facet_2.public_key().as_bytes());
    }

    #[test]
    fn test_from_seed_deterministic() {
        let seed = [42u8; 32];
        let id1 = Identity::from_seed(&seed);
        let id2 = Identity::from_seed(&seed);
        
        assert_eq!(id1.public_key().as_bytes(), id2.public_key().as_bytes());
    }
}
