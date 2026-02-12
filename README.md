# TRIP: Trajectory-based Recognition of Identity Proof

**IETF Internet-Draft** | [Datatracker](https://datatracker.ietf.org/doc/draft-ayerbe-trip-protocol/) | [IPR Disclosure #7153](https://datatracker.ietf.org/ipr/7153/)

TRIP is a decentralized protocol for establishing that an online identity corresponds to a physical entity moving through the real world — without biometrics, without a central authority, and without exposing precise location data.

## How It Works

A TRIP-enabled device collects **breadcrumbs** — Ed25519-signed, spatially quantized (H3 hexagonal grid), temporally ordered attestations of physical presence. Over time, the spatiotemporal diversity of this trajectory becomes progressively harder to fabricate.

### The Criticality Engine

TRIP evaluates the **statistical physics** of movement trajectories:

- **Parisi Factor (PSD α):** Real human movement produces 1/f^α pink noise with α ∈ [0.30, 0.80]. Bots produce white noise (α ≈ 0). Replays produce brown noise (α ≈ 2). Only biological systems at criticality produce the characteristic pink noise signature.

- **Barabási Mobility Constants:** Human displacement follows truncated Lévy flights with ~93% predictability. TRIP learns each identity's mobility profile and detects deviations.

- **Six-Component Hamiltonian:** Each breadcrumb is scored against the identity's behavioral profile across six dimensions:

| Component | Detects |
|-----------|---------|
| H_spatial | Teleportation / impossible jumps |
| H_temporal | Wrong location at wrong time |
| H_kinetic | Improbable anchor transitions |
| H_flock | Movement against local human flow |
| H_contextual | GPS injection (GPS moves, phone is flat) |
| H_structure | Chain integrity and timing regularity |

### Proof-of-Humanity Certificate

The protocol generates a compact attestation containing **only statistical exponents** (α, β, κ) — no GPS coordinates, no cell identifiers, no raw location data. A relying party learns that the identity moves like a biological organism, not where it has been.

## IETF Status

| Document | Status |
|----------|--------|
| [draft-ayerbe-trip-protocol-02](https://datatracker.ietf.org/doc/draft-ayerbe-trip-protocol/) | Individual I-D, active |
| [draft-ayerbe-trip-protocol-01](drafts/draft-ayerbe-trip-protocol-01.txt) | Superseded |
| [IPR Disclosure #7153](https://datatracker.ietf.org/ipr/7153/) | US Provisional Patent 63/948,788 (RAND terms) |

### Changes in -02

- **RATS Architecture mapping** — full alignment with RFC 9334 roles (Attester, Verifier, Relying Party, Endorser)
- **Replay protection** — nonce-bound active verification for all Attestation Results
- **Accessibility analysis** — impact assessment for users with limited mobility
- **Passive mode flagged for removal** — to be removed entirely in -03

### RATS Alignment

TRIP maps to the [RATS Architecture (RFC 9334)](https://www.rfc-editor.org/rfc/rfc9334.html):

| RATS Role | TRIP Component |
|-----------|---------------|
| Attester | TRIP-enabled device |
| Evidence | Breadcrumbs and epoch records |
| Verifier | Criticality Engine |
| Attestation Results | PoH Certificate and trust score |
| Relying Party | Any service accepting PoH Certificates |
| Endorser | Anchor nodes providing countersignatures |

## Repository Structure

```
trip-protocol/
├── drafts/                          # IETF Internet-Draft files (xml2rfc v3)
│   ├── draft-ayerbe-trip-protocol-02.xml
│   ├── draft-ayerbe-trip-protocol-02.txt
│   ├── draft-ayerbe-trip-protocol-02.html
│   ├── draft-ayerbe-trip-protocol-01.xml
│   ├── draft-ayerbe-trip-protocol-01.txt
│   └── draft-ayerbe-trip-protocol-01.html
├── ietf/                            # IETF submission files
│   └── draft-ayerbe-trip-protocol-02.xml
├── spec/                            # Protocol specifications
│   ├── TRIP-SPEC.md                 # Core protocol specification
│   ├── MESSAGES.md                  # Message format specification
│   ├── TRUST.md                     # Trust scoring model
│   └── TRUST-DYNAMICS.md            # Parisi trust dynamics framework
├── reference/                       # Reference implementation (Rust)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                   # Library root
│       ├── identity.rs              # Ed25519 identity (HI/HIT)
│       ├── hit.rs                   # HIT derivation and validation
│       ├── handle.rs                # GNS handle resolution
│       ├── handshake.rs             # 4-message handshake protocol
│       ├── session.rs               # Encrypted session management
│       ├── messages.rs              # Wire format encoding
│       ├── trajectory.rs            # Breadcrumb chain operations
│       ├── trust.rs                 # Criticality Engine scoring
│       ├── crypto.rs                # Cryptographic primitives
│       └── error.rs                 # Error types
├── test-vectors/                    # Test data for interoperability
│   └── identity.json                # HIT derivation test vectors
├── CONTRIBUTING.md                  # How to contribute
├── IMPLEMENTING.md                  # Implementation guide
├── LICENSE
└── README.md
```

## Building the Draft

```bash
# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install xml2rfc
pip install xml2rfc

# Generate text output
xml2rfc --v3 drafts/draft-ayerbe-trip-protocol-02.xml --text

# Generate HTML output
xml2rfc --v3 drafts/draft-ayerbe-trip-protocol-02.xml --html
```

## Key References

- Parisi, G. (2021). Nobel Prize in Physics — scale-free correlations in complex systems
- Cavagna, A. et al. (2010). "Scale-free correlations in starling flocks." *PNAS*
- Ballerini, M. et al. (2008). "Interaction ruling animal collective behavior depends on topological rather than metric distance." *PNAS*
- González, M.C., Hidalgo, C.A., Barabási, A.-L. (2008). "Understanding individual human mobility patterns." *Nature*
- Song, C. et al. (2010). "Limits of Predictability in Human Mobility." *Science*

## Author

**Camilo Ayerbe Posada**
[ULISSY s.r.l.](https://ulissy.com), Rome, Italy
camilo@ulissy.com

## License

This repository contains IETF Internet-Draft materials subject to [BCP 78](https://www.rfc-editor.org/info/bcp78) and the IETF Trust's Legal Provisions. See [LICENSE](LICENSE) for details.

---

*"The flock is the identity. The murmuration is the trust."*
