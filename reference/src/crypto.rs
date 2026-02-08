//! Cryptographic primitives
//! See spec/TRIP-SPEC.md Section 9 for details

/// Generate random bytes
pub fn random_bytes(len: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut bytes = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

/// Generate random nonce (16 bytes)
pub fn random_nonce() -> [u8; 16] {
    let mut nonce = [0u8; 16];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

