// trip-verifier/src/psd.rs
//
// Power Spectral Density (PSD) Analysis
// ======================================
//
// The spectral signature of human mobility: real humans produce
// 1/f^α pink noise with α ∈ [0.30, 0.80]. This is the
// fingerprint of Self-Organized Criticality — the statistical
// physics that governs biological movement.
//
// - White noise (α ≈ 0): bots, random walk generators
// - Pink noise (α ∈ [0.30, 0.80]): biological systems at criticality
// - Brown noise (α ≈ 2): GPS replays, scripted movement
//
// Implementation:
// 1. Extract displacement time series from breadcrumb chain
// 2. Apply Welch's method (segmented, windowed FFT)
// 3. Compute PSD via |FFT|²
// 4. Fit α via linear regression in log-log space
//
// References:
// - Parisi (2021), Nobel Prize — scale-free correlations
// - Maczák et al. (2024) — spectral analysis of GPS trajectories
// - Vadai et al. (2019) — fluctuations in daily motion

use rustfft::{FftPlanner, num_complex::Complex};
use crate::error::{TripError, Result};

/// Result of PSD analysis on a displacement time series.
#[derive(Debug, Clone)]
pub struct PsdResult {
    /// The PSD scaling exponent α.
    /// Human range: [0.30, 0.80] (pink noise)
    /// Bot/random: ≈ 0 (white noise)
    /// Replay: ≈ 2 (brown noise)
    pub alpha: f64,

    /// R² of the log-log fit (goodness of fit).
    /// Higher = more confident in α estimate.
    pub r_squared: f64,

    /// Number of frequency bins used in the fit.
    pub num_bins: usize,

    /// The raw PSD values (frequency, power) for diagnostics.
    pub spectrum: Vec<(f64, f64)>,

    /// Classification based on α range.
    pub classification: PsdClassification,
}

/// Classification of the PSD scaling exponent per TRIP spec Table 3.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PsdClassification {
    /// α < 0.10 — White noise (bots, random walk)
    WhiteNoise,
    /// α ∈ [0.10, 0.30) — Borderline, insufficient criticality
    Borderline,
    /// α ∈ [0.30, 0.80] — Pink noise, biological criticality
    Biological,
    /// α ∈ (0.80, 1.50] — Strong correlations, possibly suspicious
    StrongCorrelation,
    /// α > 1.50 — Brown noise (replay, scripted)
    BrownNoise,
}

impl PsdClassification {
    pub fn from_alpha(alpha: f64) -> Self {
        match alpha {
            a if a < 0.10 => Self::WhiteNoise,
            a if a < 0.30 => Self::Borderline,
            a if a <= 0.80 => Self::Biological,
            a if a <= 1.50 => Self::StrongCorrelation,
            _ => Self::BrownNoise,
        }
    }

    pub fn is_human(&self) -> bool {
        matches!(self, Self::Biological)
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::WhiteNoise => "white_noise",
            Self::Borderline => "borderline",
            Self::Biological => "biological",
            Self::StrongCorrelation => "strong_correlation",
            Self::BrownNoise => "brown_noise",
        }
    }
}

/// Compute the PSD scaling exponent α from a displacement time series.
///
/// Uses Welch's method:
/// 1. Divide the signal into overlapping segments
/// 2. Apply a Hann window to each segment
/// 3. Compute FFT and average the periodograms
/// 4. Fit α via log-log linear regression
///
/// # Arguments
/// * `displacements` — displacement magnitudes (km) between consecutive breadcrumbs
/// * `dt_mean` — mean time interval between samples (seconds), used for frequency axis
///
/// # Returns
/// `PsdResult` with α, R², and diagnostic info.
pub fn compute_psd(displacements: &[f64], dt_mean: f64) -> Result<PsdResult> {
    let n = displacements.len();

    if n < 32 {
        return Err(TripError::PsdError(
            format!("Need at least 32 displacements, got {n}")
        ));
    }

    // --- Step 1: Remove mean (center the signal) ---
    let mean = displacements.iter().sum::<f64>() / n as f64;
    let centered: Vec<f64> = displacements.iter().map(|&x| x - mean).collect();

    // --- Step 2: Welch's method parameters ---
    // Segment length: largest power of 2 that fits at least 4 segments
    let segment_len = optimal_segment_length(n);
    let overlap = segment_len / 2; // 50% overlap
    let step = segment_len - overlap;

    // --- Step 3: Compute windowed periodograms ---
    let hann_window = hann(segment_len);
    let window_power: f64 = hann_window.iter().map(|w| w * w).sum::<f64>() / segment_len as f64;

    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(segment_len);

    let mut avg_psd = vec![0.0f64; segment_len / 2 + 1];
    let mut n_segments = 0;

    let mut start = 0;
    while start + segment_len <= n {
        // Extract segment and apply window
        let mut buffer: Vec<Complex<f64>> = centered[start..start + segment_len]
            .iter()
            .zip(hann_window.iter())
            .map(|(&x, &w)| Complex::new(x * w, 0.0))
            .collect();

        // FFT in-place
        fft.process(&mut buffer);

        // Accumulate |FFT|² (one-sided PSD)
        for (i, psd_bin) in avg_psd.iter_mut().enumerate() {
            let mag_sq = buffer[i].norm_sqr();
            // Double non-DC, non-Nyquist bins for one-sided spectrum
            let scale = if i == 0 || i == segment_len / 2 { 1.0 } else { 2.0 };
            *psd_bin += scale * mag_sq / (segment_len as f64 * window_power);
        }

        n_segments += 1;
        start += step;
    }

    if n_segments == 0 {
        return Err(TripError::PsdError("No complete segments".to_string()));
    }

    // Average over segments
    for bin in &mut avg_psd {
        *bin /= n_segments as f64;
    }

    // --- Step 4: Build frequency axis ---
    let fs = 1.0 / dt_mean; // sampling frequency in Hz
    let df = fs / segment_len as f64;
    let spectrum: Vec<(f64, f64)> = (1..avg_psd.len()) // skip DC
        .map(|i| (i as f64 * df, avg_psd[i]))
        .filter(|&(_, p)| p > 0.0) // skip zero-power bins
        .collect();

    if spectrum.len() < 4 {
        return Err(TripError::PsdError(
            "Too few non-zero frequency bins for fitting".to_string()
        ));
    }

    // --- Step 5: Log-log linear regression to find α ---
    // PSD(f) ∝ 1/f^α  →  log(PSD) = -α·log(f) + c
    let log_f: Vec<f64> = spectrum.iter().map(|&(f, _)| f.ln()).collect();
    let log_p: Vec<f64> = spectrum.iter().map(|&(_, p)| p.ln()).collect();

    let (slope, _intercept, r_squared) = linear_regression(&log_f, &log_p);
    let alpha = -slope; // PSD ∝ f^(-α), so slope = -α

    let classification = PsdClassification::from_alpha(alpha);

    Ok(PsdResult {
        alpha,
        r_squared,
        num_bins: spectrum.len(),
        spectrum,
        classification,
    })
}

/// Compute PSD from a BreadcrumbChain's displacement series.
/// Convenience function that handles the displacement extraction.
pub fn compute_psd_from_chain(
    displacement_km: &[f64],
    interval_seconds: &[f64],
) -> Result<PsdResult> {
    if displacement_km.len() != interval_seconds.len() {
        return Err(TripError::PsdError(
            "Displacement and interval arrays must be same length".to_string()
        ));
    }

    let dt_mean = interval_seconds.iter().sum::<f64>() / interval_seconds.len() as f64;
    compute_psd(displacement_km, dt_mean)
}

// ========================================================================
// Internal helpers
// ========================================================================

/// Hann window: w(n) = 0.5 * (1 - cos(2π·n / (N-1)))
fn hann(size: usize) -> Vec<f64> {
    let n = size as f64;
    (0..size)
        .map(|i| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (n - 1.0)).cos()))
        .collect()
}

/// Find optimal segment length: largest power of 2 such that
/// we get at least 3 segments with 50% overlap.
fn optimal_segment_length(total_samples: usize) -> usize {
    let mut seg = 64; // minimum
    while seg * 2 <= total_samples / 2 {
        seg *= 2;
    }
    seg.max(32)
}

/// Simple linear regression: y = slope·x + intercept
/// Returns (slope, intercept, r_squared)
fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x.iter().map(|a| a * a).sum();
    let sum_y2: f64 = y.iter().map(|a| a * a).sum();

    let denom = n * sum_x2 - sum_x * sum_x;
    if denom.abs() < f64::EPSILON {
        return (0.0, 0.0, 0.0);
    }

    let slope = (n * sum_xy - sum_x * sum_y) / denom;
    let intercept = (sum_y - slope * sum_x) / n;

    // R² = 1 - SS_res / SS_tot
    let y_mean = sum_y / n;
    let ss_tot = sum_y2 - n * y_mean * y_mean;
    let ss_res: f64 = x.iter().zip(y.iter())
        .map(|(&xi, &yi)| {
            let pred = slope * xi + intercept;
            (yi - pred).powi(2)
        })
        .sum();

    let r_squared = if ss_tot.abs() > f64::EPSILON {
        1.0 - ss_res / ss_tot
    } else {
        0.0
    };

    (slope, intercept, r_squared)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    /// White noise should produce α ≈ 0
    #[test]
    fn test_white_noise_alpha() {
        let mut rng = rand::thread_rng();
        let signal: Vec<f64> = (0..1024)
            .map(|_| rng.gen_range(0.0..1.0))
            .collect();

        let result = compute_psd(&signal, 300.0).unwrap();
        assert!(
            result.alpha.abs() < 0.30,
            "White noise α should be near 0, got {}",
            result.alpha
        );
        assert_eq!(result.classification, PsdClassification::WhiteNoise);
    }

    /// Brown noise (cumulative sum of white noise) should produce α ≈ 2
    #[test]
    fn test_brown_noise_alpha() {
        let mut rng = rand::thread_rng();
        let mut signal = vec![0.0f64; 1024];
        for i in 1..1024 {
            signal[i] = signal[i - 1] + rng.gen_range(-1.0..1.0);
        }

        let result = compute_psd(&signal, 300.0).unwrap();
        assert!(
            result.alpha > 1.5,
            "Brown noise α should be > 1.5, got {}",
            result.alpha
        );
        assert_eq!(result.classification, PsdClassification::BrownNoise);
    }

    /// Regression fit quality
    #[test]
    fn test_linear_regression_perfect() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // y = 2x
        let (slope, intercept, r2) = linear_regression(&x, &y);
        assert!((slope - 2.0).abs() < 0.001);
        assert!(intercept.abs() < 0.001);
        assert!((r2 - 1.0).abs() < 0.001);
    }

    /// Hann window properties
    #[test]
    fn test_hann_window() {
        let w = hann(64);
        assert_eq!(w.len(), 64);
        assert!(w[0] < 0.01);        // starts near zero
        assert!(w[63] < 0.01);       // ends near zero
        assert!((w[32] - 1.0).abs() < 0.01); // peak at center
    }

    /// Minimum sample check
    #[test]
    fn test_insufficient_samples() {
        let signal = vec![1.0; 16];
        let result = compute_psd(&signal, 300.0);
        assert!(result.is_err());
    }
}
