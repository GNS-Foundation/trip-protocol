// trip-verifier/src/lib.rs
//
// TRIP Protocol Criticality Engine
// Verifier implementation per draft-ayerbe-trip-protocol-03
//
// This crate implements the RATS Verifier role: it receives
// breadcrumb Evidence from an Attester, evaluates trajectory
// statistics using the Criticality Engine, and produces
// Proof-of-Humanity (PoH) Certificates as Attestation Results.

pub mod breadcrumb;
pub mod chain;
pub mod psd;
pub mod levy;
pub mod hamiltonian;
pub mod criticality;
pub mod certificate;
pub mod verification;
pub mod error;

// Re-exports for convenience
pub use breadcrumb::Breadcrumb;
pub use chain::BreadcrumbChain;
pub use criticality::CriticalityEngine;
pub use certificate::PoHCertificate;
pub use error::TripError;
