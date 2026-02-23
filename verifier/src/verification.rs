// trip-verifier/src/verification.rs
//
// Active Verification Protocol
// ==============================
//
// Implements the background-check attestation topology:
//
// 1. Relying Party → Verifier: {identity_key, nonce}
// 2. Verifier → Attester: {nonce, challenge_timestamp, deadline}
// 3. Attester → Verifier: {nonce_echo, chain_head_hash, signature}
// 4. Verifier → Relying Party: {PoH Certificate with nonce binding}
//
// The nonce binding prevents replay of certificates across
// different Relying Party contexts.

use chrono::{DateTime, Utc, Duration};
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::error::{TripError, Result};

/// Default deadline for attester to respond (seconds).
pub const DEFAULT_DEADLINE_SECONDS: u64 = 30;

/// Step 1: Relying Party's verification request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub identity_key: String,  // Ed25519 public key hex of the Attester
    pub nonce: Vec<u8>,        // 16-byte random nonce from the RP
}

impl VerificationRequest {
    /// Create a new request with a random nonce.
    pub fn new(identity_key: String) -> Self {
        let mut nonce = vec![0u8; 16];
        rand::thread_rng().fill(&mut nonce[..]);
        Self { identity_key, nonce }
    }

    /// Create a request with a specific nonce (for testing).
    pub fn with_nonce(identity_key: String, nonce: Vec<u8>) -> Self {
        Self { identity_key, nonce }
    }
}

/// Step 2: Verifier's challenge to the Attester.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivenessChallenge {
    pub nonce: Vec<u8>,                // Echo of RP's nonce
    pub challenge_timestamp: DateTime<Utc>,
    pub response_deadline_seconds: u64,
}

impl LivenessChallenge {
    pub fn from_request(request: &VerificationRequest) -> Self {
        Self {
            nonce: request.nonce.clone(),
            challenge_timestamp: Utc::now(),
            response_deadline_seconds: DEFAULT_DEADLINE_SECONDS,
        }
    }

    pub fn deadline(&self) -> DateTime<Utc> {
        self.challenge_timestamp + Duration::seconds(self.response_deadline_seconds as i64)
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.deadline()
    }
}

/// Step 3: Attester's response to the challenge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivenessResponse {
    pub nonce_echo: Vec<u8>,          // Must match the challenge nonce
    pub chain_head_hash: String,      // Current chain head (hex)
    pub response_timestamp: DateTime<Utc>,
    pub current_breadcrumb_index: u64,
    pub ed25519_signature: String,    // Signature over the response (hex)
}

/// Active Verification session state (held by the Verifier).
pub struct VerificationSession {
    pub request: VerificationRequest,
    pub challenge: LivenessChallenge,
    pub state: SessionState,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    /// Challenge sent, waiting for Attester response
    AwaitingResponse,
    /// Response received, running Criticality Engine
    Evaluating,
    /// Certificate produced
    Complete,
    /// Deadline expired or verification failed
    Failed(String),
}

impl VerificationSession {
    /// Create a new session from a Relying Party request.
    pub fn new(request: VerificationRequest) -> Self {
        let challenge = LivenessChallenge::from_request(&request);
        Self {
            request,
            challenge,
            state: SessionState::AwaitingResponse,
            created_at: Utc::now(),
        }
    }

    /// Validate the Attester's liveness response.
    pub fn validate_response(&mut self, response: &LivenessResponse) -> Result<()> {
        // Check deadline
        if self.challenge.is_expired() {
            self.state = SessionState::Failed("Deadline expired".to_string());
            return Err(TripError::DeadlineExpired);
        }

        // Check nonce match
        if response.nonce_echo != self.challenge.nonce {
            self.state = SessionState::Failed("Nonce mismatch".to_string());
            return Err(TripError::NonceMismatch);
        }

        // TODO: Verify Ed25519 signature over the response
        // using the identity_key from the original request.
        // Requires: ed25519_dalek signature verification.

        self.state = SessionState::Evaluating;
        Ok(())
    }

    /// Mark the session as complete.
    pub fn complete(&mut self) {
        self.state = SessionState::Complete;
    }

    /// Mark the session as failed.
    pub fn fail(&mut self, reason: String) {
        self.state = SessionState::Failed(reason);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_flow() {
        // Step 1: RP creates request
        let request = VerificationRequest::new("abc123".to_string());
        assert_eq!(request.nonce.len(), 16);

        // Step 2: Verifier creates session and challenge
        let mut session = VerificationSession::new(request.clone());
        assert_eq!(session.state, SessionState::AwaitingResponse);
        assert!(!session.challenge.is_expired());

        // Step 3: Attester responds (correct nonce)
        let response = LivenessResponse {
            nonce_echo: session.challenge.nonce.clone(),
            chain_head_hash: "deadbeef".repeat(8),
            response_timestamp: Utc::now(),
            current_breadcrumb_index: 500,
            ed25519_signature: "sig".to_string(),
        };

        assert!(session.validate_response(&response).is_ok());
        assert_eq!(session.state, SessionState::Evaluating);
    }

    #[test]
    fn test_nonce_mismatch() {
        let request = VerificationRequest::new("abc123".to_string());
        let mut session = VerificationSession::new(request);

        let bad_response = LivenessResponse {
            nonce_echo: vec![0u8; 16], // wrong nonce
            chain_head_hash: "deadbeef".repeat(8),
            response_timestamp: Utc::now(),
            current_breadcrumb_index: 500,
            ed25519_signature: "sig".to_string(),
        };

        assert!(session.validate_response(&bad_response).is_err());
    }
}
