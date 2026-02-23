// trip-verifier/src/criticality.rs
//
// The Criticality Engine
// =======================
//
// Orchestrator for all TRIP analyses. Takes a verified
// breadcrumb chain and produces a CriticalityResult
// containing all statistical exponents and scores needed
// for PoH Certificate generation.
//
// This is the RATS Verifier logic.

use crate::chain::BreadcrumbChain;
use crate::psd::{self, PsdResult};
use crate::levy::{self, LevyResult};
use crate::hamiltonian::{
    self, BehavioralProfile, ChainHamiltonianResult,
    HamiltonianWeights,
};
use crate::error::{TripError, Result};

/// Minimum breadcrumbs required for meaningful analysis.
/// Per TRIP spec Section 6.4 (Convergence Analysis):
/// - 64 minimum for PSD
/// - 200+ for confident classification
pub const MIN_BREADCRUMBS_PSD: usize = 64;
pub const MIN_BREADCRUMBS_CONFIDENT: usize = 200;

/// Configuration for the Criticality Engine.
#[derive(Debug, Clone)]
pub struct CriticalityConfig {
    /// Hamiltonian component weights
    pub weights: HamiltonianWeights,
    /// Minimum displacement threshold for Lévy fitting (km)
    pub levy_x_min: f64,
    /// Alpha range for biological classification
    pub alpha_min: f64,
    pub alpha_max: f64,
    /// Beta range for human Lévy flight
    pub beta_min: f64,
    pub beta_max: f64,
}

impl Default for CriticalityConfig {
    fn default() -> Self {
        Self {
            weights: HamiltonianWeights::default(),
            levy_x_min: 0.01,  // 10 meters
            alpha_min: 0.30,
            alpha_max: 0.80,
            beta_min: 0.80,
            beta_max: 1.20,
        }
    }
}

/// Complete result from the Criticality Engine.
/// This contains everything needed for PoH Certificate generation.
#[derive(Debug)]
pub struct CriticalityResult {
    /// PSD scaling exponent analysis
    pub psd: PsdResult,

    /// Lévy flight parameters
    pub levy: LevyResult,

    /// Per-breadcrumb Hamiltonian scoring
    pub hamiltonian: ChainHamiltonianResult,

    /// Overall trust score [0, 100]
    pub trust_score: f64,

    /// Confidence in the classification [0, 1]
    pub confidence: f64,

    /// Number of breadcrumbs analyzed
    pub chain_length: usize,

    /// Is this identity classified as human?
    pub is_human: bool,

    /// Summary of what contributed to the decision
    pub verdict: Verdict,
}

/// Human-readable verdict breakdown.
#[derive(Debug)]
pub struct Verdict {
    pub psd_pass: bool,
    pub levy_pass: bool,
    pub hamiltonian_pass: bool,
    pub confidence_sufficient: bool,
    pub summary: String,
}

/// The Criticality Engine.
pub struct CriticalityEngine {
    config: CriticalityConfig,
}

impl CriticalityEngine {
    pub fn new(config: CriticalityConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(CriticalityConfig::default())
    }

    /// Evaluate a breadcrumb chain and produce a CriticalityResult.
    ///
    /// This is the main entry point for the Verifier.
    pub fn evaluate(&self, chain: &BreadcrumbChain) -> Result<CriticalityResult> {
        if chain.len() < MIN_BREADCRUMBS_PSD {
            return Err(TripError::InsufficientBreadcrumbs {
                got: chain.len(),
                need: MIN_BREADCRUMBS_PSD,
            });
        }

        // --- 1. PSD Analysis ---
        let displacement_km = chain.displacement_series();
        let interval_seconds = chain.interval_series();
        let psd_result = psd::compute_psd_from_chain(&displacement_km, &interval_seconds)?;

        // --- 2. Lévy Flight Fitting ---
        let levy_result = levy::fit_levy(&displacement_km, self.config.levy_x_min)?;

        // --- 3. Build Behavioral Profile ---
        let profile = BehavioralProfile::from_chain(chain);

        // --- 4. Hamiltonian Evaluation ---
        let hamiltonian_result = hamiltonian::evaluate_hamiltonian(
            chain,
            &profile,
            &self.config.weights,
        );

        // --- 5. Compute Trust Score ---
        let (trust_score, confidence, is_human, verdict) = self.compute_verdict(
            &psd_result,
            &levy_result,
            &hamiltonian_result,
            chain.len(),
        );

        Ok(CriticalityResult {
            psd: psd_result,
            levy: levy_result,
            hamiltonian: hamiltonian_result,
            trust_score,
            confidence,
            chain_length: chain.len(),
            is_human,
            verdict,
        })
    }

    /// Compute the final verdict from individual analyses.
    fn compute_verdict(
        &self,
        psd: &PsdResult,
        levy: &LevyResult,
        hamiltonian: &ChainHamiltonianResult,
        chain_length: usize,
    ) -> (f64, f64, bool, Verdict) {
        // PSD check: α in biological range?
        let psd_pass = psd.alpha >= self.config.alpha_min
            && psd.alpha <= self.config.alpha_max
            && psd.r_squared >= 0.5;

        // Lévy check: β in human range?
        let levy_pass = levy.beta >= self.config.beta_min
            && levy.beta <= self.config.beta_max
            && levy.ks_statistic < 0.15;

        // Hamiltonian check: low mean energy, few red alerts?
        let red_fraction = hamiltonian.alert_count.red as f64
            / hamiltonian.scores.len().max(1) as f64;
        let hamiltonian_pass = hamiltonian.mean_energy < 0.4
            && red_fraction < 0.05;

        // Confidence: increases with chain length
        // Per TRIP spec convergence analysis:
        // 64 → 0.3 confidence, 200 → 0.7, 500+ → 0.95
        let confidence = convergence_confidence(chain_length);
        let confidence_sufficient = confidence >= 0.5;

        // Trust score [0, 100]:
        // 40% from PSD (scaled by how close α is to center of range)
        // 25% from Lévy
        // 25% from Hamiltonian
        // 10% from chain length / confidence
        let psd_score = if psd_pass {
            let center = (self.config.alpha_min + self.config.alpha_max) / 2.0;
            let range = (self.config.alpha_max - self.config.alpha_min) / 2.0;
            let distance = (psd.alpha - center).abs() / range;
            (1.0 - distance) * psd.r_squared
        } else {
            0.0
        };

        let levy_score = if levy_pass {
            let center = (self.config.beta_min + self.config.beta_max) / 2.0;
            let range = (self.config.beta_max - self.config.beta_min) / 2.0;
            let distance = (levy.beta - center).abs() / range;
            (1.0 - distance) * (1.0 - levy.ks_statistic)
        } else {
            0.0
        };

        let ham_score = if hamiltonian_pass {
            1.0 - hamiltonian.mean_energy
        } else {
            (0.4 - hamiltonian.mean_energy).max(0.0) / 0.4
        };

        let trust_score = (
            40.0 * psd_score
            + 25.0 * levy_score
            + 25.0 * ham_score
            + 10.0 * confidence
        ).clamp(0.0, 100.0);

        let is_human = psd_pass && levy_pass && hamiltonian_pass && confidence_sufficient;

        let summary = format!(
            "PSD α={:.3} ({}), Lévy β={:.3} ({}), H_mean={:.3} ({}), confidence={:.2} ({}). {}",
            psd.alpha, if psd_pass { "PASS" } else { "FAIL" },
            levy.beta, if levy_pass { "PASS" } else { "FAIL" },
            hamiltonian.mean_energy, if hamiltonian_pass { "PASS" } else { "FAIL" },
            confidence, if confidence_sufficient { "PASS" } else { "FAIL" },
            if is_human { "HUMAN" } else { "NOT VERIFIED" },
        );

        let verdict = Verdict {
            psd_pass,
            levy_pass,
            hamiltonian_pass,
            confidence_sufficient,
            summary,
        };

        (trust_score, confidence, is_human, verdict)
    }
}

/// Confidence as a function of chain length.
/// Models the convergence of statistical estimators:
///   c(n) = 1 - exp(-n / τ)
/// where τ = 200 (characteristic convergence length)
fn convergence_confidence(chain_length: usize) -> f64 {
    let tau = 200.0;
    1.0 - (-(chain_length as f64) / tau).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_confidence() {
        let c64 = convergence_confidence(64);
        let c200 = convergence_confidence(200);
        let c500 = convergence_confidence(500);

        assert!(c64 > 0.25 && c64 < 0.40, "64 breadcrumbs: {c64}");
        assert!(c200 > 0.60 && c200 < 0.70, "200 breadcrumbs: {c200}");
        assert!(c500 > 0.90, "500 breadcrumbs: {c500}");
    }
}
