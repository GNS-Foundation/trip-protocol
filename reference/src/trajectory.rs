//! Trajectory, breadcrumbs, and epochs
//! See spec/TRAJECTORY.md for details

use crate::identity::PublicKey;

/// Location breadcrumb
pub struct Breadcrumb {
    pub owner: PublicKey,
    pub index: u64,
    pub timestamp: u64,
    pub cell: u64,        // H3 cell index
    pub context: [u8; 32], // Sensor context hash
    pub previous: [u8; 32], // Previous breadcrumb hash
    pub signature: [u8; 64],
}

/// Collection of breadcrumbs forming an epoch
pub struct Epoch {
    pub owner: PublicKey,
    pub breadcrumbs: Vec<Breadcrumb>,
    pub merkle_root: [u8; 32],
    pub start_time: u64,
    pub end_time: u64,
    pub signature: [u8; 64],
}

