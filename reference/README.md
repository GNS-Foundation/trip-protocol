# TRIP Reference Implementation

This is the reference implementation of the TRIP protocol in Rust.

## Status

⚠️ **Work in Progress** - This implementation is for testing and validation purposes.

| Component | Status |
|-----------|--------|
| Identity (HI, HIT) | ✅ Complete |
| Handle | ✅ Complete |
| Handshake | 🔄 In Progress |
| Session | 🔄 In Progress |
| Messages | 📋 Skeleton |
| Trust | 📋 Skeleton |
| Trajectory | 📋 Skeleton |
| Crypto | 📋 Skeleton |

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Usage

```rust
use trip_protocol::{Identity, Hit, Handle};

// Generate a new identity
let identity = Identity::generate();

// Get identifiers
let public_key = identity.public_key();
let hit = identity.hit();

println!("Public Key: {}", public_key);
println!("HIT: {}", hit);

// Sign a message
let message = b"Hello, TRIP!";
let signature = identity.sign(message);

// Verify signature
assert!(Identity::verify(public_key, message, &signature));

// Derive facet identity
let work_identity = identity.derive_facet("work");
```

## Features

- `std` (default): Standard library support
- `serde`: Serialization/deserialization
- `stellar`: Stellar address derivation

```bash
# Build with all features
cargo build --all-features
```

## Test Vectors

Validate against protocol test vectors:

```bash
cargo test --test vectors
```

## License

Apache License 2.0
