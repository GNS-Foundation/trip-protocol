// trip-verifier/src/hamiltonian.rs
//
// The Six-Component Hamiltonian
// ==============================
//
// Each breadcrumb is scored against the identity's learned
// behavioral profile across six dimensions. The total energy
// H quantifies how anomalous a breadcrumb is.
//
// H = w1·H_spatial + w2·H_temporal + w3·H_kinetic
//   + w4·H_flock + w5·H_contextual + w6·H_structure
//
// Low H = consistent with profile (trustworthy)
// High H = anomalous (suspicious)
//
// Default weights from TRIP spec Table 6:
//   spatial=0.25, temporal=0.20, kinetic=0.15,
//   flock=0.15, contextual=0.15, structure=0.10

use crate::breadcrumb::Breadcrumb;
use crate::chain::BreadcrumbChain;
use std::collections::HashMap;

/// Component weights for the Hamiltonian.
#[derive(Debug, Clone)]
pub struct HamiltonianWeights {
    pub spatial: f64,
    pub temporal: f64,
    pub kinetic: f64,
    pub flock: f64,
    pub contextual: f64,
    pub structure: f64,
}

impl Default for HamiltonianWeights {
    fn default() -> Self {
        Self {
            spatial: 0.25,
            temporal: 0.20,
            kinetic: 0.15,
            flock: 0.15,
            contextual: 0.15,
            structure: 0.10,
        }
    }
}

/// Result of Hamiltonian evaluation for a single breadcrumb.
#[derive(Debug, Clone)]
pub struct HamiltonianScore {
    pub index: u64,
    pub h_spatial: f64,
    pub h_temporal: f64,
    pub h_kinetic: f64,
    pub h_flock: f64,
    pub h_contextual: f64,
    pub h_structure: f64,
    pub h_total: f64,
    pub alert_level: AlertLevel,
}

/// Result of Hamiltonian evaluation for the entire chain.
#[derive(Debug, Clone)]
pub struct ChainHamiltonianResult {
    pub scores: Vec<HamiltonianScore>,
    pub mean_energy: f64,
    pub max_energy: f64,
    pub alert_count: AlertCounts,
}

#[derive(Debug, Clone, Default)]
pub struct AlertCounts {
    pub green: usize,
    pub yellow: usize,
    pub orange: usize,
    pub red: usize,
}

/// Alert levels per TRIP spec Table 7.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertLevel {
    /// H < 0.3 — Normal behavior
    Green,
    /// H ∈ [0.3, 0.6) — Unusual but plausible
    Yellow,
    /// H ∈ [0.6, 0.8) — Suspicious
    Orange,
    /// H ≥ 0.8 — Anomalous
    Red,
}

impl AlertLevel {
    pub fn from_energy(h: f64) -> Self {
        match h {
            e if e < 0.3 => Self::Green,
            e if e < 0.6 => Self::Yellow,
            e if e < 0.8 => Self::Orange,
            _ => Self::Red,
        }
    }
}

/// Behavioral profile learned from the trajectory history.
/// Built incrementally as breadcrumbs are processed.
pub struct BehavioralProfile {
    /// Histogram of H3 cells visited (cell_hex → visit count)
    pub cell_histogram: HashMap<String, u32>,
    /// Anchor cells: cells with > 5% of total visits
    pub anchor_cells: Vec<String>,
    /// Mean displacement between consecutive breadcrumbs (km)
    pub mean_displacement_km: f64,
    /// Std deviation of displacement
    pub std_displacement_km: f64,
    /// Hourly activity profile: hour (0-23) → fraction of breadcrumbs
    pub hourly_profile: [f64; 24],
    /// Mean time interval between breadcrumbs (seconds)
    pub mean_interval_seconds: f64,
    /// Std deviation of intervals
    pub std_interval_seconds: f64,
    /// Transition probabilities between top cells
    pub transition_matrix: HashMap<(String, String), f64>,
}

impl BehavioralProfile {
    /// Build a behavioral profile from a verified chain.
    pub fn from_chain(chain: &BreadcrumbChain) -> Self {
        let n = chain.breadcrumbs.len();

        // Cell histogram
        let mut cell_histogram: HashMap<String, u32> = HashMap::new();
        for b in &chain.breadcrumbs {
            *cell_histogram.entry(b.location_cell.clone()).or_insert(0) += 1;
        }

        // Anchor cells (> 5% of visits)
        let threshold = (n as f64 * 0.05).ceil() as u32;
        let anchor_cells: Vec<String> = cell_histogram.iter()
            .filter(|(_, &count)| count >= threshold)
            .map(|(cell, _)| cell.clone())
            .collect();

        // Displacement statistics
        let displacements = chain.displacement_series();
        let mean_displacement_km = if displacements.is_empty() {
            0.0
        } else {
            displacements.iter().sum::<f64>() / displacements.len() as f64
        };
        let std_displacement_km = std_dev(&displacements, mean_displacement_km);

        // Hourly profile
        let mut hour_counts = [0u32; 24];
        for b in &chain.breadcrumbs {
            let hour = b.timestamp.hour() as usize;
            hour_counts[hour] += 1;
        }
        let mut hourly_profile = [0.0f64; 24];
        for (i, &count) in hour_counts.iter().enumerate() {
            hourly_profile[i] = count as f64 / n.max(1) as f64;
        }

        // Interval statistics
        let intervals = chain.interval_series();
        let mean_interval_seconds = if intervals.is_empty() {
            0.0
        } else {
            intervals.iter().sum::<f64>() / intervals.len() as f64
        };
        let std_interval_seconds = std_dev(&intervals, mean_interval_seconds);

        // Transition matrix (cell_i → cell_j counts, normalized)
        let mut transitions: HashMap<(String, String), u32> = HashMap::new();
        let mut from_counts: HashMap<String, u32> = HashMap::new();
        for pair in chain.breadcrumbs.windows(2) {
            let from = pair[0].location_cell.clone();
            let to = pair[1].location_cell.clone();
            *transitions.entry((from.clone(), to)).or_insert(0) += 1;
            *from_counts.entry(from).or_insert(0) += 1;
        }
        let transition_matrix: HashMap<(String, String), f64> = transitions.into_iter()
            .map(|((from, to), count)| {
                let total = *from_counts.get(&from).unwrap_or(&1);
                ((from, to), count as f64 / total as f64)
            })
            .collect();

        Self {
            cell_histogram,
            anchor_cells,
            mean_displacement_km,
            std_displacement_km,
            hourly_profile,
            mean_interval_seconds,
            std_interval_seconds,
            transition_matrix,
        }
    }
}

/// Evaluate the six-component Hamiltonian for every breadcrumb
/// in the chain, given a behavioral profile.
pub fn evaluate_hamiltonian(
    chain: &BreadcrumbChain,
    profile: &BehavioralProfile,
    weights: &HamiltonianWeights,
) -> ChainHamiltonianResult {
    let mut scores = Vec::with_capacity(chain.len());
    let mut alert_count = AlertCounts::default();

    for (i, breadcrumb) in chain.breadcrumbs.iter().enumerate() {
        let prev = if i > 0 { Some(&chain.breadcrumbs[i - 1]) } else { None };

        let h_spatial = compute_h_spatial(breadcrumb, prev, profile);
        let h_temporal = compute_h_temporal(breadcrumb, profile);
        let h_kinetic = compute_h_kinetic(breadcrumb, prev, profile);
        let h_flock = compute_h_flock(breadcrumb); // placeholder
        let h_contextual = compute_h_contextual(breadcrumb, prev);
        let h_structure = compute_h_structure(breadcrumb, prev, profile);

        let h_total = weights.spatial * h_spatial
            + weights.temporal * h_temporal
            + weights.kinetic * h_kinetic
            + weights.flock * h_flock
            + weights.contextual * h_contextual
            + weights.structure * h_structure;

        let alert_level = AlertLevel::from_energy(h_total);
        match alert_level {
            AlertLevel::Green => alert_count.green += 1,
            AlertLevel::Yellow => alert_count.yellow += 1,
            AlertLevel::Orange => alert_count.orange += 1,
            AlertLevel::Red => alert_count.red += 1,
        }

        scores.push(HamiltonianScore {
            index: breadcrumb.index,
            h_spatial,
            h_temporal,
            h_kinetic,
            h_flock,
            h_contextual,
            h_structure,
            h_total,
            alert_level,
        });
    }

    let mean_energy = if scores.is_empty() {
        0.0
    } else {
        scores.iter().map(|s| s.h_total).sum::<f64>() / scores.len() as f64
    };
    let max_energy = scores.iter()
        .map(|s| s.h_total)
        .fold(0.0f64, f64::max);

    ChainHamiltonianResult {
        scores,
        mean_energy,
        max_energy,
        alert_count,
    }
}

// ========================================================================
// Component implementations
// ========================================================================

/// H_spatial: Displacement anomaly.
/// Detects teleportation / impossible jumps.
/// Energy = normalized distance from mean displacement.
fn compute_h_spatial(
    current: &Breadcrumb,
    prev: Option<&Breadcrumb>,
    profile: &BehavioralProfile,
) -> f64 {
    let prev = match prev {
        Some(p) => p,
        None => return 0.0, // genesis breadcrumb
    };

    let dist = crate::breadcrumb::h3_cell_distance_km(
        &prev.location_cell,
        &current.location_cell,
    );

    if profile.std_displacement_km < 0.001 {
        return 0.0;
    }

    // Z-score clamped to [0, 1]
    let z = ((dist - profile.mean_displacement_km) / profile.std_displacement_km).abs();
    sigmoid(z, 3.0)  // sigmoid with inflection at z=3
}

/// H_temporal: Rhythm anomaly.
/// Detects wrong location at wrong time.
/// Energy based on how unusual this hour is for this identity.
fn compute_h_temporal(
    current: &Breadcrumb,
    profile: &BehavioralProfile,
) -> f64 {
    let hour = current.timestamp.hour() as usize;
    let hour_activity = profile.hourly_profile[hour];

    // If this hour has very low historical activity, it's unusual
    if hour_activity < 0.001 {
        return 0.8; // unusual but not impossible
    }

    // Low activity → high energy
    1.0 - (hour_activity * 24.0).min(1.0) // normalize: if uniform, each hour = 1/24
}

/// H_kinetic: Transition anomaly.
/// Detects improbable anchor transitions.
/// Energy based on how unlikely the cell-to-cell transition is.
fn compute_h_kinetic(
    current: &Breadcrumb,
    prev: Option<&Breadcrumb>,
    profile: &BehavioralProfile,
) -> f64 {
    let prev = match prev {
        Some(p) => p,
        None => return 0.0,
    };

    let key = (prev.location_cell.clone(), current.location_cell.clone());
    match profile.transition_matrix.get(&key) {
        Some(&prob) if prob > 0.0 => {
            // Higher probability → lower energy
            // -log2(prob) normalized to [0, 1]
            let surprise = -prob.log2();
            sigmoid(surprise, 5.0) // transitions seen < 1/32 of the time → high energy
        }
        _ => {
            // Never-before-seen transition
            0.7 // suspicious but might be exploring new area
        }
    }
}

/// H_flock: Topological alignment.
/// Detects movement against local human flow.
///
/// NOTE: Full implementation requires cross-identity data
/// (other TRIP users in the same area). For single-identity
/// verification, this returns a neutral 0.0.
/// TODO: Implement when multi-user data is available.
fn compute_h_flock(_current: &Breadcrumb) -> f64 {
    0.0 // neutral until flock data is available
}

/// H_contextual: Sensor cross-correlation.
/// Detects GPS injection (GPS moves, but IMU says phone is flat).
/// Uses context digest differences as a proxy.
fn compute_h_contextual(
    current: &Breadcrumb,
    prev: Option<&Breadcrumb>,
) -> f64 {
    let _prev = match prev {
        Some(p) => p,
        None => return 0.0,
    };

    // The context digest is a hash of h3_cell + IMU + wifi + cellular.
    // If GPS moved but context digest is suspiciously similar,
    // it suggests GPS spoofing.
    //
    // Full implementation compares raw sensor correlations.
    // With hashed digests, we can only check "changed vs didn't change":
    // If location changed but context didn't → suspicious.
    if current.location_cell != _prev.location_cell
        && current.context_digest == _prev.context_digest
    {
        0.6 // moved but context identical → suspicious
    } else {
        0.0
    }
}

/// H_structure: Chain structural integrity.
/// Detects timing regularity anomalies.
/// Perfectly regular intervals suggest automation;
/// no intervals suggest replay.
fn compute_h_structure(
    current: &Breadcrumb,
    prev: Option<&Breadcrumb>,
    profile: &BehavioralProfile,
) -> f64 {
    let prev = match prev {
        Some(p) => p,
        None => return 0.0,
    };

    let dt = (current.unix_seconds() - prev.unix_seconds()).max(0.001);

    if profile.std_interval_seconds < 0.001 {
        return 0.0;
    }

    // Z-score of interval
    let z = ((dt - profile.mean_interval_seconds) / profile.std_interval_seconds).abs();

    // Extremely regular intervals (z ≈ 0 for all breadcrumbs) are
    // themselves suspicious. But for single-breadcrumb scoring,
    // we just flag individually outlying intervals.
    sigmoid(z, 3.0)
}

// ========================================================================
// Helpers
// ========================================================================

/// Sigmoid function: maps x to [0, 1] with inflection at midpoint.
/// Used to smoothly clamp anomaly scores.
fn sigmoid(x: f64, midpoint: f64) -> f64 {
    1.0 / (1.0 + (-2.0 * (x - midpoint)).exp())
}

/// Standard deviation helper
fn std_dev(values: &[f64], mean: f64) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let variance = values.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / (values.len() - 1) as f64;
    variance.sqrt()
}

use chrono::Timelike;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        // At midpoint, sigmoid ≈ 0.5
        assert!((sigmoid(3.0, 3.0) - 0.5).abs() < 0.01);
        // Well below midpoint → near 0
        assert!(sigmoid(0.0, 3.0) < 0.01);
        // Well above midpoint → near 1
        assert!(sigmoid(6.0, 3.0) > 0.99);
    }

    #[test]
    fn test_alert_levels() {
        assert_eq!(AlertLevel::from_energy(0.1), AlertLevel::Green);
        assert_eq!(AlertLevel::from_energy(0.4), AlertLevel::Yellow);
        assert_eq!(AlertLevel::from_energy(0.7), AlertLevel::Orange);
        assert_eq!(AlertLevel::from_energy(0.9), AlertLevel::Red);
    }

    #[test]
    fn test_default_weights_sum_to_one() {
        let w = HamiltonianWeights::default();
        let sum = w.spatial + w.temporal + w.kinetic + w.flock + w.contextual + w.structure;
        assert!((sum - 1.0).abs() < 0.001);
    }
}
