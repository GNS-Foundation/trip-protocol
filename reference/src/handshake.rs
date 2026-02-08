//! Base Exchange (Handshake) - Secure session establishment
//!
//! The TRIP handshake is a 4-way exchange similar to HIP's Base Exchange,
//! but using trajectory trust instead of computational puzzles.

use crate::identity::PublicKey;
use crate::hit::Hit;
use crate::trust::TrustLevel;

/// Handshake state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    /// Initial state, no handshake in progress
    Unassociated,
    /// I1 sent, waiting for R1
    I1Sent,
    /// R1 sent (responder), waiting for I2
    R1Sent,
    /// I2 sent, waiting for R2
    I2Sent,
    /// R2 sent (responder), session establishing
    R2Sent,
    /// Session established
    Established,
    /// Closing
    Closing,
}

/// Handshake context
pub struct Handshake {
    state: HandshakeState,
    local_hit: Hit,
    remote_hit: Option<Hit>,
    requested_trust: TrustLevel,
    granted_trust: Option<TrustLevel>,
    // Ephemeral keys for key exchange
    local_ephemeral: Option<[u8; 32]>,
    remote_ephemeral: Option<[u8; 32]>,
    // Nonces
    initiator_nonce: Option<[u8; 16]>,
    responder_nonce: Option<[u8; 16]>,
}

impl Handshake {
    /// Create new handshake as initiator
    pub fn new_initiator(local_hit: Hit, requested_trust: TrustLevel) -> Self {
        Self {
            state: HandshakeState::Unassociated,
            local_hit,
            remote_hit: None,
            requested_trust,
            granted_trust: None,
            local_ephemeral: None,
            remote_ephemeral: None,
            initiator_nonce: None,
            responder_nonce: None,
        }
    }

    /// Create new handshake as responder
    pub fn new_responder(local_hit: Hit) -> Self {
        Self {
            state: HandshakeState::Unassociated,
            local_hit,
            remote_hit: None,
            requested_trust: TrustLevel::Anonymous,
            granted_trust: None,
            local_ephemeral: None,
            remote_ephemeral: None,
            initiator_nonce: None,
            responder_nonce: None,
        }
    }

    /// Get current state
    pub fn state(&self) -> HandshakeState {
        self.state
    }

    /// Check if handshake is complete
    pub fn is_established(&self) -> bool {
        self.state == HandshakeState::Established
    }
}

// TODO: Implement I1, R1, I2, R2 message generation and processing
