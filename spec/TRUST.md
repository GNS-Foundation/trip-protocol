# TRIP Trust Level System

**Version:** 1.0.0-draft

This document specifies the trust level system used in TRIP for progressive access control and Sybil resistance.

---

## 1. Overview

TRIP uses a progressive trust system based on **Proof-of-Trajectory**. Unlike computational puzzles (HIP) or centralized verification, trust is earned through physical-world presence over time.

### Design Goals

1. **Sybil Resistance:** Make creating fake identities economically infeasible
2. **Progressive Access:** Grant capabilities as trust increases
3. **Decentralized:** No central authority required
4. **Physical Anchoring:** Trust rooted in real-world movement

---

## 2. Trust Levels

### 2.1 Level Definitions

| Level | Name | Requirement | Time to Achieve |
|-------|------|-------------|-----------------|
| 0 | Anonymous | Valid keypair | Instant |
| 1 | Verified | 1+ published epoch | ~1-2 weeks |
| 2 | Established | 10+ published epochs | ~2-3 months |
| 3 | Trusted | 100+ published epochs | ~2 years |
| 4 | Vouched | Active vouch from L3+ | Instant (with vouch) |

### 2.2 Epoch Requirements

An **epoch** is a collection of at least 100 breadcrumbs:

```
Epoch = {
    owner: HI,                    // Human Identity
    breadcrumbs: [Breadcrumb],    // ≥ 100 breadcrumbs
    merkle_root: Hash,            // Root of breadcrumb tree
    start_time: Timestamp,
    end_time: Timestamp,
    signature: Signature          // Signed by owner
}

Breadcrumb = {
    owner: HI,
    index: u64,
    timestamp: Timestamp,
    cell: H3Cell,                 // Location (quantized)
    context: Hash,                // Sensor digest
    previous: Hash,               // Chain link
    signature: Signature
}
```

### 2.3 Timing Constraints

- **Minimum breadcrumb interval:** 10 minutes
- **Maximum breadcrumb interval:** 24 hours
- **Minimum epoch duration:** ~17 hours (100 × 10 min)
- **Typical epoch duration:** 1-2 weeks

---

## 3. Privilege Matrix

### 3.1 Capabilities by Level

| Capability | L0 | L1 | L2 | L3 | L4 |
|------------|----|----|----|----|----| 
| Receive messages | ✓ | ✓ | ✓ | ✓ | ✓ |
| Receive payments | ✓ | ✓ | ✓ | ✓ | ✓ |
| Send messages | ✗ | ✓† | ✓ | ✓ | ✓ |
| Send payments | ✗ | ✓ | ✓ | ✓ | ✓ |
| Request payments | ✗ | ✗ | ✓ | ✓ | ✓ |
| Claim temp handle | ✗ | ✓ | ✓ | ✓ | ✓ |
| Claim perm handle | ✗ | ✗ | ✓ | ✓ | ✓ |
| Create facets | ✗ | ✗ | ✓ | ✓ | ✓ |
| Vouch for others | ✗ | ✗ | ✗ | ✓ | ✗ |
| Run relay node | ✗ | ✗ | ✗ | ✓ | ✗ |
| Protocol governance | ✗ | ✗ | ✗ | ✓ | ✗ |

† = Rate limited

### 3.2 Rate Limits by Level

| Level | Messages/min | Connections/hour | Bytes/day |
|-------|--------------|------------------|-----------|
| 0 | 0 | 1 | 0 |
| 1 | 10 | 10 | 10 MB |
| 2 | 60 | 60 | 100 MB |
| 3 | 600 | Unlimited | 1 GB |
| 4 | 60 | 60 | 100 MB |

---

## 4. Trust Computation

### 4.1 Algorithm

```python
def compute_trust_level(identity: Identity) -> TrustLevel:
    # Count valid published epochs
    epoch_count = count_valid_epochs(identity)
    
    # Check for active vouches
    vouches = get_active_vouches(identity)
    
    # Vouch grants Level 4 immediately
    for vouch in vouches:
        if vouch.voucher_trust >= 3 and vouch.is_valid():
            return TrustLevel.VOUCHED  # Level 4
    
    # Epoch-based progression
    if epoch_count >= 100:
        return TrustLevel.TRUSTED      # Level 3
    elif epoch_count >= 10:
        return TrustLevel.ESTABLISHED  # Level 2
    elif epoch_count >= 1:
        return TrustLevel.VERIFIED     # Level 1
    else:
        return TrustLevel.ANONYMOUS    # Level 0
```

### 4.2 Epoch Validation

```python
def is_valid_epoch(epoch: Epoch) -> bool:
    # Must have minimum breadcrumbs
    if len(epoch.breadcrumbs) < 100:
        return False
    
    # Verify merkle root
    if compute_merkle_root(epoch.breadcrumbs) != epoch.merkle_root:
        return False
    
    # Verify epoch signature
    if not verify(epoch.signature, epoch.merkle_root, epoch.owner):
        return False
    
    # Verify each breadcrumb
    for i, bc in enumerate(epoch.breadcrumbs):
        if not is_valid_breadcrumb(bc, epoch.owner):
            return False
        
        # Verify chain linkage
        if i > 0:
            expected_prev = hash(epoch.breadcrumbs[i-1])
            if bc.previous != expected_prev:
                return False
    
    # Verify timing constraints
    for i in range(1, len(epoch.breadcrumbs)):
        time_diff = epoch.breadcrumbs[i].timestamp - epoch.breadcrumbs[i-1].timestamp
        if time_diff < MIN_INTERVAL:
            return False
        if time_diff > MAX_INTERVAL:
            return False
    
    # Verify spatial consistency (speed check)
    if not is_spatially_consistent(epoch.breadcrumbs):
        return False
    
    return True
```

### 4.3 Spatial Consistency

```python
MAX_SPEED_KMH = 1000  # Maximum plausible speed (airplane)

def is_spatially_consistent(breadcrumbs: List[Breadcrumb]) -> bool:
    for i in range(1, len(breadcrumbs)):
        bc1 = breadcrumbs[i-1]
        bc2 = breadcrumbs[i]
        
        # Calculate distance between H3 cells
        distance_km = h3_distance_km(bc1.cell, bc2.cell)
        
        # Calculate time difference
        time_hours = (bc2.timestamp - bc1.timestamp) / 3600000
        
        # Check speed
        if time_hours > 0:
            speed_kmh = distance_km / time_hours
            if speed_kmh > MAX_SPEED_KMH:
                return False
    
    return True
```

---

## 5. Vouching System

### 5.1 Vouch Structure

```
Vouch = {
    voucher: HI,           // Who is vouching (must be L3+)
    vouchee: HI,           // Who is being vouched for
    timestamp: Timestamp,  // When vouch was issued
    expires: Timestamp,    // Vouch expiration
    reason: String,        // Optional reason (max 140 chars)
    signature: Signature   // Signed by voucher
}
```

### 5.2 Vouch Rules

1. **Voucher must be Level 3+**
2. **Maximum active vouches:** 10 per voucher
3. **Vouch duration:** Maximum 1 year, renewable
4. **Vouch is revocable** by voucher at any time
5. **Vouch does not stack:** Multiple vouches don't increase level beyond 4
6. **Voucher reputation:** Vouch abuse may affect voucher's standing

### 5.3 Vouch Verification

```python
def is_valid_vouch(vouch: Vouch) -> bool:
    # Check voucher trust level
    voucher_trust = compute_trust_level(vouch.voucher)
    if voucher_trust < 3:
        return False
    
    # Check expiration
    if vouch.expires < current_time():
        return False
    
    # Check not revoked
    if is_revoked(vouch):
        return False
    
    # Verify signature
    message = encode(vouch.vouchee, vouch.timestamp, vouch.expires)
    if not verify(vouch.signature, message, vouch.voucher):
        return False
    
    # Check voucher's active vouch count
    if count_active_vouches(vouch.voucher) > MAX_VOUCHES:
        return False
    
    return True
```

---

## 6. Sybil Cost Analysis

### 6.1 Cost Model

Creating a legitimate TRIP identity requires:

| Resource | Minimum | Typical |
|----------|---------|---------|
| Device | 1 smartphone | 1 smartphone |
| Time | 17 hours | 1-2 weeks |
| Physical presence | Required | Required |
| Movement | ~10km | ~100km |

### 6.2 Attack Cost Estimation

To create **N** Sybil identities:

| N | Devices | Humans | Time | Est. Cost (USD) |
|---|---------|--------|------|-----------------|
| 1 | 1 | 1 | 2 weeks | ~$100 |
| 10 | 10 | 10 | 2 weeks | ~$1,000 |
| 100 | 100 | 100 | 2 weeks | ~$10,000 |
| 1000 | 1000 | 1000 | 2 weeks | ~$100,000+ |

**Key insight:** Attack cost scales linearly with N, and cannot be parallelized on a single device.

### 6.3 Comparison with HIP

| Metric | HIP | TRIP |
|--------|-----|------|
| Identity creation time | ~1ms | ~2 weeks |
| Identity creation cost | $0 | ~$100 |
| Parallelizable? | Yes (CPU) | No (physical) |
| 1000 Sybils cost | ~$0 | >$100,000 |
| Requires hardware? | No | Yes (phone + GPS) |

---

## 7. Trust Level Transitions

### 7.1 State Diagram

```
                    ┌─────────────┐
                    │  Anonymous  │
                    │   Level 0   │
                    └──────┬──────┘
                           │ Publish 1st epoch
                           ▼
                    ┌─────────────┐
                    │  Verified   │
                    │   Level 1   │
                    └──────┬──────┘
                           │ Publish 10th epoch
                           ▼
                    ┌─────────────┐
                    │ Established │◄────────────┐
                    │   Level 2   │             │
                    └──────┬──────┘             │
                           │ Publish 100th      │ Receive vouch
                           │ epoch              │ from L3+
                           ▼                    │
                    ┌─────────────┐      ┌──────┴──────┐
                    │   Trusted   │      │   Vouched   │
                    │   Level 3   │      │   Level 4   │
                    └─────────────┘      └─────────────┘
                           │
                           │ Can vouch for others
                           ▼
                    ┌─────────────┐
                    │  Vouchee    │
                    │  → Level 4  │
                    └─────────────┘
```

### 7.2 Downgrade Conditions

Trust can be **reduced** if:

1. **Epochs invalidated:** If past epochs are found to be fraudulent
2. **Vouch revoked:** Level 4 → Level based on epoch count
3. **Extended inactivity:** No new epochs for 2+ years may trigger review

---

## 8. Privacy Considerations

### 8.1 Location Privacy

- Breadcrumbs use **H3 cells** (resolution 7 = ~5km²), not GPS coordinates
- Sensor context is **hashed**, not raw data
- Exact timing can be **jittered** within tolerance

### 8.2 Trust Level Privacy

- Trust level is **public** (necessary for protocol operation)
- Epoch count is **public**
- Specific trajectory data is **not required** for verification
  - Only merkle proofs needed, not full breadcrumb history

---

## 9. Implementation Notes

### 9.1 Caching

Implementations SHOULD cache:
- Computed trust levels (with TTL)
- Epoch validity checks
- Vouch validity checks

### 9.2 Rate Limiting

Implementations MUST:
- Enforce rate limits per trust level
- Track limits per HIT, not per session
- Handle limit exceeded gracefully (ERROR 0x08)

### 9.3 Trust Proof Formats

Three proof types for handshake:

**Epoch Proof:**
```
{
    epoch_id: String,
    merkle_root: [32]byte,
    epoch_count: u32,
    merkle_proof: [MerkleNode],
    signature: [64]byte
}
```

**Trajectory Proof:**
```
{
    breadcrumbs: [Breadcrumb],  // Recent breadcrumbs
    total_count: u32            // Total breadcrumb count
}
```

**Vouch Proof:**
```
{
    vouch: Vouch,
    voucher_epoch_count: u32    // Proves voucher is L3+
}
```

---

## 10. Test Vectors

See [test-vectors/trust.json](../test-vectors/trust.json) for:
- Trust level computation examples
- Epoch validation test cases
- Vouch verification test cases
- Spatial consistency checks
