# TRIP Trust Dynamics: The Parisi Framework

## Overview

This document describes the mathematical framework underpinning TRIP's trust model, grounded in Giorgio Parisi's Nobel Prize-winning work on scale-free correlations in complex systems.

## The Starling-to-TRIP Mapping

| Starling Flock | TRIP Network |
|----------------|--------------|
| Bird | Entity (human, device) |
| Position r_i(t) | Trajectory τ_i(t) |
| Velocity v_i(t) | Trust gradient ∇T_i(t) |
| k=6-7 nearest neighbors | k=7 peer attestations |
| Alignment interaction | Trust reinforcement |
| Flock coherence | Network-wide Sybil resistance |
| Predator detection | Attack detection |

**Fundamental insight:** Local peer attestations (k≈7) create global trust correlations, just as local neighbor tracking creates global flock coherence.

## The Trust Hamiltonian

Borrowing from Parisi's spin glass formalism:

```
H[{T_i}] = -Σ⟨i,j⟩ J_ij T_i T_j - Σ_i h_i T_i + Σ_i λ_i(t) T_i
```

Where:
- `J_ij`: Peer attestation coupling (positive if j attested i)
- `h_i`: External field from anchor attestations
- `λ_i(t)`: Time-dependent decay potential

The system evolves toward **minimum energy** (maximum trust coherence).

## Phase Transitions

The TRIP network exhibits phase transitions:

**Ordered Phase (High Trust Density):**
- Many peer interactions
- Trust propagates efficiently
- Sybil attacks easily detected (isolated clusters)

**Disordered Phase (Low Trust Density):**
- Few peer interactions
- Trust is localized
- Network more vulnerable

**Critical percolation threshold:**

```
p_c = 1/⟨k⟩ ≈ 1/7 ≈ 0.143
```

Above p_c: Trust percolates, Sybils isolated.
Below p_c: Network fragments, Sybils can dominate.

## Scale-Free Correlations

Following Parisi's analysis of starling correlations:

```
C(r) ~ r^{-(d-2+η)}
```

Trust influence propagates across the ENTIRE network, not just locally. This is not analogy — it is the same mathematics.

## Trust Decay

Following Parisi's work on relaxation in disordered systems:

```
T(t) = T_0 · exp(-γt + γτ_0(1 - e^{-t/τ_0}))
```

Where:
- `τ_0` = 30 days (human decay constant)
- `γ` = decay rate

Inactive entities fade from the network naturally.

## Implementation Constants

```
PARISI_K = 7                    # Topological neighbor count
TAU_0_HUMAN = 30 days           # Human decay constant
PERCOLATION_THRESHOLD = 0.143   # Critical network density
ALPHA_BIOLOGICAL = [0.30, 0.80] # PSD exponent range for humans
```

## References

1. Parisi, G. (2021). Nobel Prize Lecture: "Multiple equilibria"
2. Cavagna, A., et al. (2010). "Scale-free correlations in starling flocks." PNAS
3. Ballerini, M., et al. (2008). "Interaction ruling animal collective behavior depends on topological rather than metric distance." PNAS
4. González, M.C., Hidalgo, C.A., Barabási, A.-L. (2008). "Understanding individual human mobility patterns." Nature
5. Song, C., et al. (2010). "Limits of Predictability in Human Mobility." Science

---

*"Each starling tracks only 6-7 nearest neighbors, yet the correlation length spans the entire flock."*

*"Each TRIP entity attests only to local encounters, yet the trust network spans the entire system."*

**"The flock is the identity. The murmuration is the trust."**
