// trip-verifier/src/certificate.rs
//
// Proof-of-Humanity (PoH) Certificate
// =====================================
//
// The Attestation Result in RATS terms. Contains only
// statistical exponents — no raw location data.
//
// CBOR encoding per TRIP spec Section 10, Table 8:
// {
//   0: identity_key,       (bstr .size 32)
//   1: alpha,              (float)
//   2: beta,               (float)
//   3: kappa,              (float)
//   4: trust_score,        (uint)
//   5: confidence,         (float)
//   6: chain_length,       (uint)
//   7: unique_cells,       (uint)
//   8: mean_hamiltonian,   (float)
//   9: verifier_key,       (bstr .size 32)
//  10: issued_at,          (uint, Unix seconds)
//  11: valid_seconds,      (uint)
//  12: nonce,              (bstr .size 16) [Active Verification]
//  13: chain_head_hash,    (bstr .size 32) [Active Verification]
//  14: verifier_signature, (bstr .size 64)
// }

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::criticality::CriticalityResult;
use crate::error::{TripError, Result};

/// PoH Certificate — the Attestation Result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoHCertificate {
    /// Ed25519 public key of the identity (Attester)
    pub identity_key: String,     // hex, 64 chars

    /// PSD scaling exponent
    pub alpha: f64,

    /// Lévy exponent
    pub beta: f64,

    /// Truncation distance (km)
    pub kappa: f64,

    /// Trust score [0, 100]
    pub trust_score: f64,

    /// Classification confidence [0, 1]
    pub confidence: f64,

    /// Number of breadcrumbs in the evaluated chain
    pub chain_length: u64,

    /// Number of unique H3 cells visited
    pub unique_cells: u64,

    /// Mean Hamiltonian energy
    pub mean_hamiltonian: f64,

    /// Ed25519 public key of the Verifier
    pub verifier_key: String,     // hex, 64 chars

    /// Issuance timestamp
    pub issued_at: DateTime<Utc>,

    /// Validity duration in seconds
    pub valid_seconds: u64,

    /// Relying Party nonce (Active Verification)
    pub nonce: Option<Vec<u8>>,   // 16 bytes

    /// Chain head hash at time of verification
    pub chain_head_hash: Option<String>, // hex, 64 chars

    /// Ed25519 signature by the Verifier over fields 0-13
    pub verifier_signature: Option<String>, // hex, 128 chars
}

impl PoHCertificate {
    /// Create a certificate from a CriticalityResult.
    ///
    /// # Arguments
    /// * `result` — output of the Criticality Engine
    /// * `identity_key` — Attester's Ed25519 public key hex
    /// * `verifier_key` — Verifier's Ed25519 public key hex
    /// * `unique_cells` — number of unique H3 cells
    /// * `chain_head_hash` — hash of the most recent breadcrumb
    /// * `valid_seconds` — certificate validity duration
    pub fn from_criticality_result(
        result: &CriticalityResult,
        identity_key: String,
        verifier_key: String,
        unique_cells: usize,
        chain_head_hash: String,
        valid_seconds: u64,
    ) -> Self {
        Self {
            identity_key,
            alpha: result.psd.alpha,
            beta: result.levy.beta,
            kappa: result.levy.kappa_km,
            trust_score: result.trust_score,
            confidence: result.confidence,
            chain_length: result.chain_length as u64,
            unique_cells: unique_cells as u64,
            mean_hamiltonian: result.hamiltonian.mean_energy,
            verifier_key,
            issued_at: Utc::now(),
            valid_seconds,
            nonce: None,
            chain_head_hash: Some(chain_head_hash),
            verifier_signature: None,
        }
    }

    /// Set the Active Verification nonce (from Relying Party).
    pub fn with_nonce(mut self, nonce: Vec<u8>) -> Self {
        self.nonce = Some(nonce);
        self
    }

    /// Encode the certificate to CBOR bytes (fields 0-13, for signing).
    pub fn to_cbor_signable(&self) -> Result<Vec<u8>> {
        use ciborium::Value;

        let mut map = Vec::new();

        // 0: identity_key
        let id_bytes = hex::decode(&self.identity_key)
            .map_err(|e| TripError::CertificateError(format!("Invalid identity hex: {e}")))?;
        map.push((Value::Integer(0.into()), Value::Bytes(id_bytes)));

        // 1: alpha
        map.push((Value::Integer(1.into()), Value::Float(self.alpha)));

        // 2: beta
        map.push((Value::Integer(2.into()), Value::Float(self.beta)));

        // 3: kappa
        map.push((Value::Integer(3.into()), Value::Float(self.kappa)));

        // 4: trust_score
        map.push((Value::Integer(4.into()), Value::Integer((self.trust_score as i64).into())));

        // 5: confidence
        map.push((Value::Integer(5.into()), Value::Float(self.confidence)));

        // 6: chain_length
        map.push((Value::Integer(6.into()), Value::Integer((self.chain_length as i64).into())));

        // 7: unique_cells
        map.push((Value::Integer(7.into()), Value::Integer((self.unique_cells as i64).into())));

        // 8: mean_hamiltonian
        map.push((Value::Integer(8.into()), Value::Float(self.mean_hamiltonian)));

        // 9: verifier_key
        let vk_bytes = hex::decode(&self.verifier_key)
            .map_err(|e| TripError::CertificateError(format!("Invalid verifier hex: {e}")))?;
        map.push((Value::Integer(9.into()), Value::Bytes(vk_bytes)));

        // 10: issued_at (Unix seconds)
        map.push((Value::Integer(10.into()), Value::Integer((self.issued_at.timestamp() as i64).into())));

        // 11: valid_seconds
        map.push((Value::Integer(11.into()), Value::Integer((self.valid_seconds as i64).into())));

        // 12: nonce (if present)
        if let Some(ref nonce) = self.nonce {
            map.push((Value::Integer(12.into()), Value::Bytes(nonce.clone())));
        }

        // 13: chain_head_hash (if present)
        if let Some(ref hash) = self.chain_head_hash {
            let hash_bytes = hex::decode(hash)
                .map_err(|e| TripError::CertificateError(format!("Invalid hash hex: {e}")))?;
            map.push((Value::Integer(13.into()), Value::Bytes(hash_bytes)));
        }

        let cbor_value = Value::Map(map);
        let mut buf = Vec::new();
        ciborium::into_writer(&cbor_value, &mut buf)
            .map_err(|e| TripError::CertificateError(format!("CBOR encode error: {e}")))?;

        Ok(buf)
    }

    /// Encode the full certificate (including signature) to CBOR.
    pub fn to_cbor(&self) -> Result<Vec<u8>> {
        let signable = self.to_cbor_signable()?;

        // For the full certificate, we'd add field 14 (signature)
        // This is a simplified version; full implementation would
        // reconstruct the map with the signature field.
        // For now, return the signable portion.
        Ok(signable)
    }

    /// Encode to JSON for API responses.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| TripError::CertificateError(format!("JSON encode error: {e}")))
    }

    /// Is this certificate still valid?
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        let expires_at = self.issued_at + chrono::Duration::seconds(self.valid_seconds as i64);
        now < expires_at
    }

    /// Is this an Active Verification certificate (has nonce)?
    pub fn is_active_verification(&self) -> bool {
        self.nonce.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_validity() {
        let cert = PoHCertificate {
            identity_key: "a".repeat(64),
            alpha: 0.55,
            beta: 1.0,
            kappa: 50.0,
            trust_score: 75.0,
            confidence: 0.85,
            chain_length: 300,
            unique_cells: 42,
            mean_hamiltonian: 0.15,
            verifier_key: "b".repeat(64),
            issued_at: Utc::now(),
            valid_seconds: 3600,
            nonce: Some(vec![0u8; 16]),
            chain_head_hash: Some("c".repeat(64)),
            verifier_signature: None,
        };

        assert!(cert.is_valid());
        assert!(cert.is_active_verification());
    }
}
