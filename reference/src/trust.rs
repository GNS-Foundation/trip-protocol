//! Trust levels and verification
//! See spec/TRUST.md for details

/// Trust level (0-4)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Anonymous = 0,
    Verified = 1,
    Established = 2,
    Trusted = 3,
    Vouched = 4,
}

impl Default for TrustLevel {
    fn default() -> Self {
        Self::Anonymous
    }
}

/// Proof for trust verification
pub enum TrustProof {
    None,
    Epoch { epoch_count: u32 },
    Trajectory { breadcrumb_count: u32 },
    Vouch { voucher_trust: TrustLevel },
}

