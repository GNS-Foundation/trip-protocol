use std::env;
use std::fs;
use std::process;

use trip_verifier::breadcrumb::Breadcrumb;
use trip_verifier::chain::BreadcrumbChain;
use trip_verifier::criticality::CriticalityEngine;
use trip_verifier::certificate::PoHCertificate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&"--verbose".to_string());
    let file_path = args.iter()
        .filter(|a| !a.starts_with('-') && *a != &args[0])
        .next();

    let file_path = match file_path {
        Some(p) => p.clone(),
        None => {
            eprintln!("Usage: analyze [--verbose] <chain_export.json>");
            process::exit(1);
        }
    };

    println!("Loading chain from: {}", file_path);
    let json_str = match fs::read_to_string(&file_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Error reading file: {e}"); process::exit(1); }
    };

    let breadcrumbs: Vec<Breadcrumb> = match serde_json::from_str(&json_str) {
        Ok(b) => b,
        Err(e) => { eprintln!("Error parsing JSON: {e}"); process::exit(1); }
    };

    println!("Loaded {} breadcrumbs", breadcrumbs.len());
    if breadcrumbs.is_empty() { eprintln!("Empty chain."); process::exit(1); }

    println!("\n=== Chain Verification ===");
    let chain = match BreadcrumbChain::from_breadcrumbs(breadcrumbs) {
        Ok(c) => c,
        Err(e) => { eprintln!("Chain verification FAILED: {e}"); process::exit(1); }
    };

    let id = &chain.identity;
    let id_short = if id.len() > 16 { format!("{}...{}", &id[..8], &id[id.len()-8..]) } else { id.clone() };

    println!("  Identity:     {}", id_short);
    println!("  Breadcrumbs:  {}", chain.len());
    println!("  Unique cells: {}", chain.unique_cells());
    println!("  Duration:     {:.1} hours", chain.duration_seconds() / 3600.0);
    println!("  Chain hash:   {}...", &chain.head_hash()[..16.min(chain.head_hash().len())]);

    let displacements = chain.displacement_series();
    let intervals = chain.interval_series();

    if !displacements.is_empty() {
        let mean_disp = displacements.iter().sum::<f64>() / displacements.len() as f64;
        let max_disp = displacements.iter().cloned().fold(0.0f64, f64::max);
        let mean_int = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let total_dist: f64 = displacements.iter().sum();

        println!("\n=== Displacement Statistics ===");
        println!("  Total distance:     {:.2} km", total_dist);
        println!("  Mean displacement:  {:.4} km ({:.1} m)", mean_disp, mean_disp * 1000.0);
        println!("  Max displacement:   {:.4} km ({:.1} m)", max_disp, max_disp * 1000.0);
        println!("  Mean interval:      {:.0} seconds ({:.1} min)", mean_int, mean_int / 60.0);
        println!("  Non-zero moves:     {} / {}",
            displacements.iter().filter(|&&d| d > 0.001).count(),
            displacements.len()
        );
    }

    println!("\n=== Criticality Engine ===");
    let engine = CriticalityEngine::with_defaults();

    match engine.evaluate(&chain) {
        Ok(result) => {
            println!("\n  --- PSD Analysis ---");
            println!("  alpha = {:.4}  ({})", result.psd.alpha, result.psd.classification.label());
            println!("  R2    = {:.4}", result.psd.r_squared);
            println!("  Bins:   {}", result.psd.num_bins);
            println!("  Human [0.30, 0.80] -> {}",
                if result.psd.classification.is_human() { "PASS" } else { "FAIL" });

            println!("\n  --- Levy Flight ---");
            println!("  beta  = {:.4}  ({})", result.levy.beta, result.levy.classification.label());
            println!("  kappa = {:.2} km", result.levy.kappa_km);
            println!("  KS    = {:.4}", result.levy.ks_statistic);
            println!("  Human [0.80, 1.20] -> {}",
                if result.levy.classification.is_human() { "PASS" } else { "FAIL" });

            println!("\n  --- Hamiltonian ---");
            println!("  Mean energy:  {:.4}", result.hamiltonian.mean_energy);
            println!("  Max energy:   {:.4}", result.hamiltonian.max_energy);
            println!("  Green:{} Yellow:{} Orange:{} Red:{}",
                result.hamiltonian.alert_count.green,
                result.hamiltonian.alert_count.yellow,
                result.hamiltonian.alert_count.orange,
                result.hamiltonian.alert_count.red);

            println!("\n  === VERDICT ===");
            println!("  Trust Score:  {:.1} / 100", result.trust_score);
            println!("  Confidence:   {:.1}%", result.confidence * 100.0);
            println!("  Result:       {}", if result.is_human { "HUMAN" } else { "NOT VERIFIED" });
            println!("\n  {}", result.verdict.summary);

            // Save certificate
            let cert = PoHCertificate::from_criticality_result(
                &result, chain.identity.clone(),
                "0".repeat(64), chain.unique_cells(),
                chain.head_hash().to_string(), 3600,
            );
            if let Ok(json) = cert.to_json() {
                let cert_path = file_path.replace(".json", "_poh.json");
                let _ = fs::write(&cert_path, &json);
                println!("\n  Certificate: {cert_path}");
            }
        }
        Err(e) => {
            eprintln!("\nCriticality Engine error: {e}");
            eprintln!("Need at least 64 breadcrumbs for analysis.");
            process::exit(1);
        }
    }
}
