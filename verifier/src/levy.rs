// trip-verifier/src/levy.rs
//
// Truncated Lévy Flight Analysis
// ================================
//
// Human displacement follows a truncated power-law distribution:
//   P(Δr) ∝ Δr^(-1-β) · exp(-Δr/κ)
//
// Where:
//   β ∈ [0.8, 1.2]  — Lévy exponent (human mobility)
//   κ               — truncation distance (individual range, km)
//
// This module fits β and κ from observed displacements using
// Maximum Likelihood Estimation (MLE) on the truncated Pareto
// distribution.
//
// Reference: González, Hidalgo, Barabási (2008), "Understanding
// individual human mobility patterns", Nature 453.

use crate::error::{TripError, Result};

/// Result of Lévy flight fitting.
#[derive(Debug, Clone)]
pub struct LevyResult {
    /// Lévy exponent β.
    /// Human range: [0.8, 1.2]
    pub beta: f64,

    /// Truncation distance κ (km).
    /// Represents the individual's characteristic mobility range.
    pub kappa_km: f64,

    /// Kolmogorov-Smirnov statistic (goodness of fit).
    /// Lower = better fit. Typically < 0.1 for good fits.
    pub ks_statistic: f64,

    /// Number of displacements used in the fit.
    pub n_samples: usize,

    /// Classification
    pub classification: LevyClassification,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LevyClassification {
    /// β < 0.5 — Too concentrated (possibly stationary bot)
    TooConcentrated,
    /// β ∈ [0.5, 0.8) — Borderline, limited mobility
    Borderline,
    /// β ∈ [0.8, 1.2] — Human Lévy flight
    HumanLevy,
    /// β ∈ (1.2, 1.8] — High mobility, possibly vehicular
    HighMobility,
    /// β > 1.8 — Ballistic (robot, drone, scripted)
    Ballistic,
}

impl LevyClassification {
    pub fn from_beta(beta: f64) -> Self {
        match beta {
            b if b < 0.5 => Self::TooConcentrated,
            b if b < 0.8 => Self::Borderline,
            b if b <= 1.2 => Self::HumanLevy,
            b if b <= 1.8 => Self::HighMobility,
            _ => Self::Ballistic,
        }
    }

    pub fn is_human(&self) -> bool {
        matches!(self, Self::HumanLevy)
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::TooConcentrated => "too_concentrated",
            Self::Borderline => "borderline",
            Self::HumanLevy => "human_levy",
            Self::HighMobility => "high_mobility",
            Self::Ballistic => "ballistic",
        }
    }
}

/// Fit a truncated power-law (Lévy) distribution to displacement data.
///
/// Uses a two-step approach:
/// 1. Estimate β from the power-law regime via Hill estimator
/// 2. Estimate κ from the exponential tail truncation
///
/// # Arguments
/// * `displacements` — displacement magnitudes in km (must be > 0)
/// * `x_min` — minimum displacement threshold for fitting (km).
///             Smaller displacements are noise from H3 quantization.
///             Default: 0.01 km (10 meters)
pub fn fit_levy(displacements: &[f64], x_min: f64) -> Result<LevyResult> {
    // Filter to displacements above threshold
    let mut valid: Vec<f64> = displacements.iter()
        .filter(|&&d| d > x_min && d.is_finite())
        .copied()
        .collect();

    if valid.len() < 20 {
        return Err(TripError::LevyFitError(
            format!("Need at least 20 displacements above x_min={x_min}km, got {}", valid.len())
        ));
    }

    valid.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = valid.len();

    // --- Step 1: Hill estimator for β ---
    // β_hill = n / Σ ln(x_i / x_min)
    let sum_log: f64 = valid.iter()
        .map(|&x| (x / x_min).ln())
        .sum();

    if sum_log <= 0.0 {
        return Err(TripError::LevyFitError(
            "All displacements equal to x_min".to_string()
        ));
    }

    let beta_hill = n as f64 / sum_log;

    // --- Step 2: Estimate κ via MLE grid search ---
    // For a truncated power law P(x) ∝ x^(-1-β) · exp(-x/κ),
    // we find κ that maximizes the log-likelihood.
    let kappa = estimate_kappa(&valid, beta_hill, x_min);

    // --- Step 3: Kolmogorov-Smirnov goodness of fit ---
    let ks = ks_test_truncated_pareto(&valid, beta_hill, kappa, x_min);

    let classification = LevyClassification::from_beta(beta_hill);

    Ok(LevyResult {
        beta: beta_hill,
        kappa_km: kappa,
        ks_statistic: ks,
        n_samples: n,
        classification,
    })
}

/// Convenience: fit Lévy with default x_min = 0.01 km
pub fn fit_levy_default(displacements: &[f64]) -> Result<LevyResult> {
    fit_levy(displacements, 0.01)
}

// ========================================================================
// Internal helpers
// ========================================================================

/// Estimate κ via maximum likelihood on a grid.
/// κ is the distance at which the power-law is truncated by
/// an exponential cutoff. For humans, this represents their
/// characteristic travel range.
fn estimate_kappa(sorted_data: &[f64], beta: f64, x_min: f64) -> f64 {
    let x_max = sorted_data.last().copied().unwrap_or(100.0);

    // Search over a grid of κ values
    let mut best_kappa = x_max;
    let mut best_ll = f64::NEG_INFINITY;

    // Logarithmic grid from x_min to 10 * x_max
    let n_grid = 100;
    let log_min = x_min.ln();
    let log_max = (10.0 * x_max).ln();

    for i in 0..n_grid {
        let kappa = (log_min + (log_max - log_min) * i as f64 / n_grid as f64).exp();

        let ll = log_likelihood_truncated_pareto(sorted_data, beta, kappa, x_min);

        if ll > best_ll {
            best_ll = ll;
            best_kappa = kappa;
        }
    }

    best_kappa
}

/// Log-likelihood of a truncated Pareto distribution.
/// P(x | β, κ, x_min) ∝ x^(-1-β) · exp(-x/κ)
fn log_likelihood_truncated_pareto(
    data: &[f64],
    beta: f64,
    kappa: f64,
    x_min: f64,
) -> f64 {
    // Normalization constant (numerical integration)
    let z = normalization_constant(beta, kappa, x_min);
    if z <= 0.0 || !z.is_finite() {
        return f64::NEG_INFINITY;
    }

    let log_z = z.ln();

    data.iter()
        .map(|&x| {
            (-1.0 - beta) * x.ln() - x / kappa - log_z
        })
        .sum()
}

/// Normalization constant for the truncated Pareto:
/// Z = ∫_{x_min}^{∞} x^(-1-β) · exp(-x/κ) dx
/// Computed via numerical quadrature (trapezoidal rule).
fn normalization_constant(beta: f64, kappa: f64, x_min: f64) -> f64 {
    // Integrate from x_min to x_min + 20*kappa (practically infinity)
    let x_max = x_min + 20.0 * kappa;
    let n_steps = 1000;
    let dx = (x_max - x_min) / n_steps as f64;

    let mut integral = 0.0;
    for i in 0..=n_steps {
        let x = x_min + dx * i as f64;
        let f = x.powf(-1.0 - beta) * (-x / kappa).exp();
        let weight = if i == 0 || i == n_steps { 0.5 } else { 1.0 };
        integral += weight * f;
    }

    integral * dx
}

/// Kolmogorov-Smirnov test: max|F_empirical - F_theoretical|
fn ks_test_truncated_pareto(
    sorted_data: &[f64],
    beta: f64,
    kappa: f64,
    x_min: f64,
) -> f64 {
    let n = sorted_data.len() as f64;
    let z_total = normalization_constant(beta, kappa, x_min);

    if z_total <= 0.0 {
        return 1.0;
    }

    let mut max_diff = 0.0f64;

    for (i, &x) in sorted_data.iter().enumerate() {
        let empirical = (i + 1) as f64 / n;

        // Theoretical CDF: F(x) = 1 - Z(x)/Z(x_min)
        let z_tail = normalization_constant(beta, kappa, x);
        let theoretical = 1.0 - z_tail / z_total;

        let diff = (empirical - theoretical).abs();
        max_diff = max_diff.max(diff);
    }

    max_diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_levy_fit_synthetic_power_law() {
        // Generate synthetic power-law data with known β ≈ 1.0
        let mut rng = rand::thread_rng();
        let x_min = 0.01;
        let beta_true = 1.0;

        let data: Vec<f64> = (0..500)
            .map(|_| {
                // Inverse CDF method for Pareto: x = x_min * u^(-1/β)
                let u: f64 = rng.gen_range(0.001..1.0);
                x_min * u.powf(-1.0 / beta_true)
            })
            .collect();

        let result = fit_levy(&data, x_min).unwrap();
        assert!(
            (result.beta - beta_true).abs() < 0.3,
            "Expected β ≈ {beta_true}, got {}",
            result.beta
        );
    }

    #[test]
    fn test_insufficient_displacements() {
        let data = vec![0.1; 5];
        assert!(fit_levy(&data, 0.01).is_err());
    }

    #[test]
    fn test_classification_ranges() {
        assert_eq!(LevyClassification::from_beta(0.3), LevyClassification::TooConcentrated);
        assert_eq!(LevyClassification::from_beta(0.6), LevyClassification::Borderline);
        assert_eq!(LevyClassification::from_beta(1.0), LevyClassification::HumanLevy);
        assert_eq!(LevyClassification::from_beta(1.5), LevyClassification::HighMobility);
        assert_eq!(LevyClassification::from_beta(2.0), LevyClassification::Ballistic);
    }
}
