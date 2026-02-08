# Implementing TRIP

This guide helps developers implement the TRIP protocol in their language of choice.

## Overview

A TRIP implementation consists of these core components:

```
┌─────────────────────────────────────────────────────────────┐
│                    TRIP Implementation                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Identity   │  │   Messages   │  │   Session    │      │
│  │   (HI, HIT)  │  │ (wire format)│  │ (encrypted)  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Handshake   │  │    Trust     │  │  Trajectory  │      │
│  │(Base Exchange)│ │  (levels)    │  │ (breadcrumbs)│      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Minimum Viable Implementation

To claim TRIP compliance, you MUST implement:

### 1. Identity (Required)

```
Human Identity (HI):
  - Ed25519 public key (32 bytes)
  - Ed25519 private key (32 bytes)
  - Signature generation and verification

Human Identity Tag (HIT):
  - HIT = SHA-256(HI)[0:16]
  - 16 bytes (128 bits)
```

**Test:** Validate against `test-vectors/identity.json`

### 2. Messages (Required)

Implement the common header:

```
Header (56 bytes):
  - version: u8
  - msg_type: u8
  - flags: u16
  - sender_hit: [16]byte
  - recipient_hit: [16]byte
  - msg_id: u64
  - timestamp: u64
  - length: u32

+ Payload (variable)
+ Signature (64 bytes)
```

Minimum message types:
- I1, R1, I2, R2 (handshake)
- DATA, ACK (session)
- ERROR

**Test:** Validate against `test-vectors/messages.json`

### 3. Handshake (Required)

Implement the 4-way Base Exchange:

```
I1 → (initiator sends: HIT, HI, requested_trust, nonce)
R1 ← (responder sends: HI, challenge, DH_ephemeral)
I2 → (initiator sends: proof, DH_ephemeral)
R2 ← (responder sends: session_id, granted_trust)
```

State machine:
```
UNASSOCIATED → I1_SENT → I2_SENT → ESTABLISHED
UNASSOCIATED → R1_SENT → R2_SENT → ESTABLISHED
```

**Test:** Validate against `test-vectors/handshake.json`

### 4. Session Encryption (Required)

After handshake, use:
- Algorithm: ChaCha20-Poly1305
- Key derivation: HKDF-SHA256
- Nonce: 96-bit, unique per message

**Test:** Validate against `test-vectors/encryption.json`

## Optional Components

### Handle Resolution

If supporting `@handle` names:
- Validate format: `@[a-z0-9_]{1,20}`
- Implement lookup protocol

### Trust Levels

If supporting trust verification:
- Implement epoch validation
- Implement breadcrumb chain verification
- Implement trust computation

### Mobility

If supporting mobility:
- Implement UPDATE messages
- Implement locator management

### Payments

If supporting Stellar integration:
- Derive Stellar address from HI
- Implement PAY_* messages

## Cryptographic Requirements

### MUST Use

| Algorithm | Purpose | Reference |
|-----------|---------|-----------|
| Ed25519 | Signatures | RFC 8032 |
| X25519 | Key exchange | RFC 7748 |
| ChaCha20-Poly1305 | Encryption | RFC 8439 |
| SHA-256 | Hashing | FIPS 180-4 |
| HKDF-SHA256 | Key derivation | RFC 5869 |

### MUST NOT

- Use RSA (except for legacy interop)
- Use SHA-1 for security purposes
- Use ECB mode for encryption
- Reuse nonces

## Interoperability Checklist

Before claiming compliance:

- [ ] Pass all relevant test vectors
- [ ] Implement common message header correctly
- [ ] Handle unknown message types gracefully
- [ ] Verify signatures before processing messages
- [ ] Use big-endian byte order for multi-byte integers
- [ ] Use UTF-8 for strings
- [ ] Handle errors with appropriate error codes

## Recommended Libraries

### Rust
- `ed25519-dalek`: Ed25519 signatures
- `x25519-dalek`: X25519 key exchange
- `chacha20poly1305`: Encryption
- `sha2`: SHA-256

### JavaScript/TypeScript
- `@noble/ed25519`: Ed25519
- `@noble/curves`: X25519
- `chacha20-poly1305`: Encryption

### Go
- `golang.org/x/crypto/ed25519`
- `golang.org/x/crypto/curve25519`
- `golang.org/x/crypto/chacha20poly1305`

### Python
- `cryptography`: All primitives
- `pynacl`: Ed25519, X25519

## Testing Your Implementation

### 1. Run Test Vectors

```bash
# Example for your implementation
your-trip-impl test --vectors test-vectors/
```

### 2. Interop Testing

Test against the reference implementation:

```bash
# Start reference responder
cd reference && cargo run --example responder

# Run your initiator against it
your-trip-impl connect localhost:8765
```

### 3. Fuzz Testing

Consider fuzzing your message parser:

```bash
cargo fuzz run message_parser  # Rust example
```

## Common Mistakes

1. **Wrong HIT derivation**: Must be first 16 bytes of SHA-256, not last
2. **Endianness**: All integers are big-endian on the wire
3. **Nonce reuse**: Each message MUST have a unique nonce
4. **Signature scope**: Sign header + payload, not just payload
5. **Timing attacks**: Use constant-time comparison for signatures

## Getting Help

- Open an issue on GitHub
- Join Discord: [discord.gg/gns-foundation](https://discord.gg/gns-foundation)
- Email: protocol@gns.foundation

## Registering Your Implementation

Once complete, submit a PR to add your implementation to the README:

```markdown
| Language | Repository | Maintainer | Status |
|----------|------------|------------|--------|
| YourLang | github.com/you/trip-yourlang | @you | Complete |
```

---

Good luck! We're excited to see TRIP implementations in every language.
