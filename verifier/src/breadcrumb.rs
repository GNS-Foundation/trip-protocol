// trip-verifier/src/breadcrumb.rs
//
// Breadcrumb: the atomic unit of TRIP Evidence.
// Matches the JSON structure produced by the Flutter BreadcrumbBlock.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single breadcrumb — signed attestation of spatiotemporal presence.
/// This is what arrives from the Attester (mobile device).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breadcrumb {
    pub index: u64,
    pub identity_public_key: String,
    pub timestamp: DateTime<Utc>,
    pub location_cell: String,       // H3 hex string
    pub location_resolution: u8,     // H3 resolution (typically 10)
    pub context_digest: String,      // SHA-256 of sensor context
    pub previous_hash: Option<String>,
    pub meta_flags: MetaFlags,
    pub signature: String,           // Ed25519 hex signature
    pub block_hash: String,          // SHA-256 of block content + signature
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaFlags {
    #[serde(default)]
    pub battery: Option<i32>,
    #[serde(default = "default_sampling")]
    pub sampling: String,
    #[serde(default = "default_unknown")]
    pub state: String,
    #[serde(default = "default_unknown")]
    pub network: String,
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub manual: bool,
}

fn default_sampling() -> String { "normal".to_string() }
fn default_unknown() -> String { "unknown".to_string() }

impl Breadcrumb {
    /// Extract the H3 cell index as u64 for geospatial computations
    pub fn h3_cell(&self) -> Option<u64> {
        u64::from_str_radix(&self.location_cell, 16).ok()
    }

    /// Unix timestamp in seconds (for time series)
    pub fn unix_seconds(&self) -> f64 {
        self.timestamp.timestamp() as f64
    }
}

/// Displacement between two consecutive breadcrumbs.
/// The fundamental observable for PSD and Lévy analysis.
#[derive(Debug, Clone)]
pub struct Displacement {
    pub dt_seconds: f64,        // time interval
    pub distance_km: f64,       // great-circle distance
    pub from_cell: String,
    pub to_cell: String,
    pub timestamp: DateTime<Utc>,
}

/// Compute displacements from an ordered breadcrumb chain.
/// Uses H3 cell centers for distance calculation (privacy-preserving:
/// we never need raw GPS, only the quantized cells).
pub fn compute_displacements(breadcrumbs: &[Breadcrumb]) -> Vec<Displacement> {
    if breadcrumbs.len() < 2 {
        return Vec::new();
    }

    let mut displacements = Vec::with_capacity(breadcrumbs.len() - 1);

    for pair in breadcrumbs.windows(2) {
        let b0 = &pair[0];
        let b1 = &pair[1];

        let dt = (b1.unix_seconds() - b0.unix_seconds()).max(0.001);

        // Convert H3 cells to lat/lon centers for distance
        let dist = h3_cell_distance_km(&b0.location_cell, &b1.location_cell);

        displacements.push(Displacement {
            dt_seconds: dt,
            distance_km: dist,
            from_cell: b0.location_cell.clone(),
            to_cell: b1.location_cell.clone(),
            timestamp: b1.timestamp,
        });
    }

    displacements
}

/// Haversine distance between two H3 cell centers, in km.
/// Falls back to 0.0 if cells can't be parsed.
pub fn h3_cell_distance_km(cell_a: &str, cell_b: &str) -> f64 {
    let (lat_a, lon_a) = match h3_cell_to_latlon(cell_a) {
        Some(c) => c,
        None => return 0.0,
    };
    let (lat_b, lon_b) = match h3_cell_to_latlon(cell_b) {
        Some(c) => c,
        None => return 0.0,
    };
    haversine_km(lat_a, lon_a, lat_b, lon_b)
}

/// Convert H3 hex string to (lat, lon) center coordinates.
/// Uses the h3o crate.
fn h3_cell_to_latlon(hex_str: &str) -> Option<(f64, f64)> {
    let index = u64::from_str_radix(hex_str, 16).ok()?;
    let cell = h3o::CellIndex::try_from(index).ok()?;
    let ll = h3o::LatLng::from(cell);
    Some((ll.lat(), ll.lng()))
}

/// Haversine great-circle distance in kilometers.
fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0; // Earth radius in km
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    R * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_rome_to_naples() {
        // Rome: 41.9028, 12.4964
        // Naples: 40.8518, 14.2681
        let d = haversine_km(41.9028, 12.4964, 40.8518, 14.2681);
        assert!((d - 190.0).abs() < 10.0, "Rome-Naples should be ~190km, got {d}");
    }

    #[test]
    fn test_haversine_same_point() {
        let d = haversine_km(41.9028, 12.4964, 41.9028, 12.4964);
        assert!(d < 0.001);
    }
}
