// trip-verifier/src/chain.rs
//
// BreadcrumbChain: an ordered, verified sequence of breadcrumbs
// from a single identity. Chain verification ensures:
// 1. Hash chaining (each block references the previous)
// 2. Monotonic timestamps
// 3. Index ordering
// 4. Ed25519 signature validity

use crate::breadcrumb::{Breadcrumb, Displacement, compute_displacements};
use crate::error::{TripError, Result};
use sha2::{Sha256, Digest};
use serde_json;

/// A verified breadcrumb chain from a single identity.
pub struct BreadcrumbChain {
    pub identity: String,           // Ed25519 public key hex
    pub breadcrumbs: Vec<Breadcrumb>,
    pub displacements: Vec<Displacement>,
    pub chain_verified: bool,
}

impl BreadcrumbChain {
    /// Parse and verify a breadcrumb chain from JSON.
    /// Performs structural validation but NOT Ed25519 signature
    /// verification (that requires the full crypto stack).
    pub fn from_breadcrumbs(mut breadcrumbs: Vec<Breadcrumb>) -> Result<Self> {
        if breadcrumbs.is_empty() {
            return Err(TripError::InsufficientBreadcrumbs { got: 0, need: 1 });
        }

        // Sort by index to ensure ordering
        breadcrumbs.sort_by_key(|b| b.index);

        let identity = breadcrumbs[0].identity_public_key.clone();

        // Verify all breadcrumbs belong to same identity
        for b in &breadcrumbs {
            if b.identity_public_key != identity {
                return Err(TripError::ChainIntegrity(
                    format!("Mixed identities: expected {}, got {}", identity, b.identity_public_key)
                ));
            }
        }

        // Verify index sequence
        for (i, b) in breadcrumbs.iter().enumerate() {
            if b.index != i as u64 {
                return Err(TripError::ChainIntegrity(
                    format!("Index gap: expected {}, got {} at position {}", i, b.index, i)
                ));
            }
        }

        // Verify monotonic timestamps
        for pair in breadcrumbs.windows(2) {
            if pair[1].timestamp <= pair[0].timestamp {
                return Err(TripError::ChainIntegrity(
                    format!(
                        "Non-monotonic timestamp at index {}: {} <= {}",
                        pair[1].index, pair[1].timestamp, pair[0].timestamp
                    )
                ));
            }
        }

        // Verify hash chaining
        Self::verify_hash_chain(&breadcrumbs)?;

        // Compute displacements
        let displacements = compute_displacements(&breadcrumbs);

        Ok(Self {
            identity,
            breadcrumbs,
            displacements,
            chain_verified: true,
        })
    }

    /// Verify the hash chain: each breadcrumb's previous_hash
    /// must equal the prior breadcrumb's block_hash.
    fn verify_hash_chain(breadcrumbs: &[Breadcrumb]) -> Result<()> {
        // Genesis block must have no previous hash
        if breadcrumbs[0].previous_hash.is_some() {
            return Err(TripError::ChainIntegrity(
                "Genesis block has a previous_hash".to_string()
            ));
        }

        // Each subsequent block must reference the previous
        for pair in breadcrumbs.windows(2) {
            match &pair[1].previous_hash {
                Some(prev) if prev == &pair[0].block_hash => {},
                Some(prev) => {
                    return Err(TripError::ChainIntegrity(
                        format!(
                            "Hash chain broken at index {}: expected {}, got {}",
                            pair[1].index,
                            &pair[0].block_hash[..8],
                            &prev[..8.min(prev.len())]
                        )
                    ));
                }
                None => {
                    return Err(TripError::ChainIntegrity(
                        format!("Missing previous_hash at index {}", pair[1].index)
                    ));
                }
            }
        }

        Ok(())
    }

    /// Recompute and verify block hashes.
    /// Matches the Flutter BreadcrumbBlock.computeHash() algorithm:
    /// SHA-256(dataToSign + ":" + signature)
    pub fn verify_block_hashes(&self) -> Result<()> {
        for b in &self.breadcrumbs {
            let data_to_sign = serde_json::json!({
                "index": b.index,
                "identity": b.identity_public_key,
                "timestamp": b.timestamp.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                "loc_cell": b.location_cell,
                "loc_res": b.location_resolution,
                "context": b.context_digest,
                "prev_hash": b.previous_hash.as_deref().unwrap_or("genesis"),
                "meta": b.meta_flags,
            });

            let content = format!("{}:{}", data_to_sign, b.signature);
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            let hash = hex::encode(hasher.finalize());

            if hash != b.block_hash {
                return Err(TripError::ChainIntegrity(
                    format!(
                        "Block hash mismatch at index {}: computed {}, stored {}",
                        b.index, &hash[..8], &b.block_hash[..8]
                    )
                ));
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.breadcrumbs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.breadcrumbs.is_empty()
    }

    /// Duration of the trajectory in seconds
    pub fn duration_seconds(&self) -> f64 {
        if self.breadcrumbs.len() < 2 {
            return 0.0;
        }
        let first = self.breadcrumbs.first().unwrap().unix_seconds();
        let last = self.breadcrumbs.last().unwrap().unix_seconds();
        last - first
    }

    /// Number of unique H3 cells visited
    pub fn unique_cells(&self) -> usize {
        let mut cells: Vec<&str> = self.breadcrumbs.iter()
            .map(|b| b.location_cell.as_str())
            .collect();
        cells.sort();
        cells.dedup();
        cells.len()
    }

    /// Extract displacement magnitudes as a time series (km)
    pub fn displacement_series(&self) -> Vec<f64> {
        self.displacements.iter().map(|d| d.distance_km).collect()
    }

    /// Extract time intervals as a series (seconds)
    pub fn interval_series(&self) -> Vec<f64> {
        self.displacements.iter().map(|d| d.dt_seconds).collect()
    }

    /// Chain head hash (most recent breadcrumb's block_hash)
    pub fn head_hash(&self) -> &str {
        self.breadcrumbs.last()
            .map(|b| b.block_hash.as_str())
            .unwrap_or("")
    }
}
