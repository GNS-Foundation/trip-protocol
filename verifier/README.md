# trip-verifier

**TRIP Protocol Criticality Engine — Rust Implementation**

This crate implements the RATS Verifier role for the TRIP protocol. It receives breadcrumb Evidence from an Attester (mobile device), evaluates trajectory statistics using the Criticality Engine, and produces Proof-of-Humanity (PoH) Certificates as Attestation Results.

## Architecture

```
Phone (Attester)                    Verifier (this crate)
┌──────────────┐                   ┌─────────────────────────────┐
│ Breadcrumbs  │──── Evidence ────▶│ chain.rs     → verify chain │
│ (H3 + Ed25519│                   │ psd.rs       → compute α    │
│  signed)     │                   │ levy.rs      → fit β, κ     │
│              │                   │ hamiltonian  → 6-component H│
│              │◀── Challenge ─────│ criticality  → orchestrate  │
│              │──── Response ────▶│ certificate  → PoH cert     │
└──────────────┘                   │ verification → nonce flow   │
                                   └──────────────┬──────────────┘
                                                   │
                                                   ▼
                                   Relying Party gets PoH Certificate
                                   (α, β, κ, trust_score — NO location data)
```

## Modules

| Module | Lines | What it does |
|--------|-------|-------------|
| `breadcrumb.rs` | 149 | Breadcrumb struct, displacement computation, H3→lat/lon |
| `chain.rs` | 193 | Chain verification (hashing, ordering, monotonicity) |
| `psd.rs` | 345 | **Power Spectral Density** — Welch's method FFT → α exponent |
| `levy.rs` | 298 | **Lévy flight fitting** — Hill estimator → β, κ parameters |
| `hamiltonian.rs` | 462 | **Six-component Hamiltonian** — per-breadcrumb anomaly scoring |
| `criticality.rs` | 271 | **Criticality Engine** — orchestrates all analyses → verdict |
| `certificate.rs` | 248 | PoH Certificate generation (CBOR + JSON) |
| `verification.rs` | 189 | Active Verification Protocol (nonce challenge/response) |
| `error.rs` | 38 | Error types |

## Quick Start

```rust
use trip_verifier::{BreadcrumbChain, CriticalityEngine, PoHCertificate};

// 1. Parse breadcrumbs from JSON
let breadcrumbs: Vec<Breadcrumb> = serde_json::from_str(&json_data)?;

// 2. Verify the chain
let chain = BreadcrumbChain::from_breadcrumbs(breadcrumbs)?;

// 3. Run the Criticality Engine
let engine = CriticalityEngine::with_defaults();
let result = engine.evaluate(&chain)?;

// 4. Generate PoH Certificate
let cert = PoHCertificate::from_criticality_result(
    &result,
    chain.identity.clone(),
    verifier_public_key,
    chain.unique_cells(),
    chain.head_hash().to_string(),
    3600, // valid for 1 hour
);

println!("α = {:.3}, β = {:.3}, trust = {:.0}, human = {}",
    result.psd.alpha,
    result.levy.beta,
    result.trust_score,
    result.is_human
);
```

## Build & Test

```bash
cargo build
cargo test
```

## Classification Ranges

| Parameter | Range | Meaning |
|-----------|-------|---------|
| α (PSD) | [0.30, 0.80] | Pink noise — biological criticality |
| α < 0.10 | | White noise — bots/random |
| α > 1.50 | | Brown noise — replay/scripted |
| β (Lévy) | [0.80, 1.20] | Human mobility pattern |
| H (mean) | < 0.40 | Normal behavioral profile |

## Dependencies

- `rustfft` — FFT for PSD computation
- `h3o` — H3 geospatial indexing
- `ciborium` — CBOR encoding for certificates
- `ed25519-dalek` — signature verification
- `sha2` — chain hash verification

## Status

This is the reference Verifier implementation for TRIP Protocol draft-03.
Production deployment will run as a standalone service behind the
GNS backend (Node.js), accessible via HTTP API.
