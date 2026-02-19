# TrIP-NPA: Network Path Attestation via Trajectory Breadcrumbs

## A Companion Specification to draft-ayerbe-trip-protocol-03

**Authors:** C. Ayerbe Posada (ULISSY s.r.l.)  
**Status:** Technical Concept Note (Pre-Draft)  
**Date:** February 2026  
**Companion to:** draft-ayerbe-trip-protocol-03 (TRIP: Trajectory-based Recognition of Identity Proof)

---

## Abstract

This document describes TrIP-NPA (Network Path Attestation), a companion
specification to the Trajectory-based Recognition of Identity Proof (TRIP)
protocol [draft-ayerbe-trip-protocol-03]. Where the base TRIP specification
uses geospatial breadcrumbs to attest human identity through movement
patterns, TrIP-NPA extends the same cryptographic primitive to network
infrastructure: each hop in a packet's path appends a signed breadcrumb
to an optional protocol extension, creating an immutable micro-ledger of
the packet's trajectory through the network.

This shifts the network security model from **perimeter-based** ("trust the
firewall") to **trajectory-based** ("trust the path history"). The receiving
endpoint can verify not just *who* sent the packet, but *where it has been*
— enabling behavioral anomaly detection at the protocol level.

TrIP-NPA does not modify the base TRIP specification. It reuses TRIP's
cryptographic primitives (Ed25519 signatures, SHA-256 hash chains, CBOR
encoding), its RATS Architecture mapping (RFC 9334), and its Criticality
Engine framework, applying them to a new attestation domain.

---

## 1. Motivation: The Three Gaps in Current Network Security

### 1.1 What Perimeter Security Cannot See

Traditional IP security operates on a binary model: packets inside the
perimeter are trusted; packets outside are not. This creates three
fundamental weaknesses that no existing IETF work fully addresses:

| Weakness | Description | Current Mitigation | Gap |
|----------|-------------|-------------------|-----|
| **Identity is spoofable** | IP source addresses can be forged; TLS proves server identity but not network path | mTLS, RPKI | Proves endpoint, not journey |
| **Observability is external** | Anomaly detection requires separate infrastructure (IDS/IPS, SIEM) watching from outside | Prometheus, Datadog | Cannot be embedded in packet |
| **Lateral movement is invisible** | Past the perimeter, a compromised node reaches any other node without path accountability | Zero Trust (BeyondCorp) | Verifies each request, not the path between requests |

### 1.2 What IETF Already Has (and What's Missing)

| Working Group | What It Does | What It Lacks |
|--------------|-------------|---------------|
| **RATS** (RFC 9334) | Remote Attestation Procedures — attests *devices* | Cannot attest *paths* through devices |
| **NASR** (BoF, IETF 118+) | Network Attestation for Secure Routing — verifies node trustworthiness at enrollment | No *runtime behavioral* verification |
| **TPR** (draft-voit-rats-trusted-path-routing) | Builds trusted topologies from attested devices | Cannot prove a packet *actually followed* the trusted path |
| **SPRING/SRv6** | Segment Routing — defines explicit paths | Defines paths but cannot *attest* they were followed |

**The common gap:** All existing approaches verify the *infrastructure* ("is this
router trustworthy?"). None verify the *journey* ("did this packet follow an
expected behavioral pattern?").

### 1.3 The TrIP Insight Applied to Networks

TRIP (draft-ayerbe-trip-protocol-03) established the paradigm:

> *"You are your trajectory through the world."*

For human identity, this means: sustained physical movement through geographic
space, captured as cryptographically-signed breadcrumbs, produces an unfakeable
behavioral fingerprint.

TrIP-NPA applies the identical paradigm to network packets:

> *"A packet is its trajectory through the infrastructure."*

A packet's path through load balancers, sidecars, services, and databases IS
a trajectory. The behavioral pattern of that trajectory — its timing, its
sequence, its deviations from learned baselines — IS attestation.

---

## 2. Architectural Mapping to TRIP draft-03

TrIP-NPA reuses the three-layer model from draft-03, mapping each layer
to network infrastructure:

### 2.1 Layer Mapping

| Layer | Base TRIP (Human) | TrIP-NPA (Network) |
|-------|-------------------|---------------------|
| **Layer 1: Device Identity** | Smartphone Ed25519 keypair | Infrastructure node keypair (router, LB, sidecar, DB) |
| **Layer 2: Human/Service Identity** | Person holding device | Service identity (microservice, API gateway, function) |
| **Layer 3: Fleet/Topology Identity** | Organization namespace (org@) | Network topology / trust domain / service mesh |

### 2.2 RATS Role Mapping (Extending draft-03 Section 11)

| RATS Role | Base TRIP | TrIP-NPA |
|-----------|----------|----------|
| **Attester** | TRIP-enabled mobile device | Each infrastructure node that stamps the packet |
| **Evidence** | Geospatial breadcrumbs (H3 cells) | Network breadcrumbs (node identity + timestamp + hop) |
| **Verifier** | Criticality Engine | Network Trajectory Analyzer (NTA) |
| **Attestation Result** | PoH Certificate | Path Attestation Certificate (PAC) |
| **Relying Party** | Service consuming PoH | Destination service or security gateway |

### 2.3 Primitive Reuse from draft-03

TrIP-NPA reuses the following primitives **unchanged** from draft-03:

- **Ed25519 keypairs** (RFC 8032) for node identity and signature
- **SHA-256 hash chains** for breadcrumb linking
- **CBOR encoding** (RFC 8949) for wire format
- **Merkle trees** for epoch sealing
- **Deterministic CBOR** for reproducible signatures

New primitives introduced by TrIP-NPA:

- **NetworkBreadcrumb** — adapted from Breadcrumb (draft-03 Section 2) for hop attestation
- **PathTrajectory** — adapted from Trajectory for network path modeling
- **Network Hamiltonian** — adapted from the six-component Hamiltonian (draft-03 Section 8) for path anomaly detection

---

## 3. The Network Breadcrumb

### 3.1 Data Structure

A NetworkBreadcrumb is encoded as a CBOR map, following the same
deterministic encoding rules as draft-03 Section 2.3:

```
NetworkBreadcrumb = {
    0 => uint,              ; hop_index — sequential within this packet's journey
    1 => bstr .size 32,     ; node_identity — Ed25519 public key of this node
    2 => uint,              ; node_type — enumerated (see below)
    3 => uint,              ; timestamp_us — microsecond-precision Unix time
    4 => uint,              ; processing_us — microseconds spent at this node
    5 => bstr .size 32,     ; prev_hop_hash — SHA-256 of previous NetworkBreadcrumb
    6 => bstr .size 32,     ; context_digest — node health/state digest
    7 => bstr .size 64,     ; signature — Ed25519 over fields 0-6
}
```

### 3.2 Node Types

```
NodeType = &(
    origin:        0,       ; The initial sender (client, IoT device)
    gateway:       1,       ; API gateway, ingress controller
    loadbalancer:  2,       ; L4/L7 load balancer
    waf:           3,       ; Web Application Firewall
    sidecar:       4,       ; Service mesh sidecar (Envoy, Linkerd)
    service:       5,       ; Application microservice
    database:      6,       ; Database or cache layer
    queue:         7,       ; Message queue / event bus
    egress:        8,       ; Egress proxy / external gateway
    cdn:           9,       ; CDN edge node
)
```

### 3.3 Signature Production

Identical to draft-03 Section 2.3:

```
signable_payload = CBOR-Deterministic(fields[0..6])
signature        = Ed25519-Sign(node_private_key, signable_payload)
```

### 3.4 Chain Construction

Identical to draft-03 Section 2.4:

```
HopHash(NB)          = SHA-256(CBOR-Deterministic(NB[0..7]))
NB[i+1].field[5]     = HopHash(NB[i])
NB[0].field[5]       = SHA-256(origin_request_id)    ; Genesis: hash of request ID
```

### 3.5 Context Digest (Node Health)

Analogous to draft-03 Section 2.2, but for infrastructure state:

```
context_string = "cpu:" + cpu_load_bucket
               + "|mem:" + memory_utilization_bucket
               + "|conn:" + active_connections_bucket
               + "|err:" + error_rate_bucket
               + "|ver:" + software_version_hash

context_digest = SHA-256(UTF-8(context_string))
```

This binds ambient infrastructure state to the breadcrumb, enabling
detection of breadcrumbs stamped by a node under abnormal conditions.

---

## 4. Transport Binding: The TrIP-NPA Header Extension

TrIP-NPA defines transport bindings for three common protocols. Each
binding carries the breadcrumb chain as an optional extension:

### 4.1 HTTP Header (Application Layer)

```http
TrIP-NPA: base64url(CBOR-Array([NetworkBreadcrumb]))
TrIP-NPA-Origin: base64url(origin_ed25519_public_key)
TrIP-NPA-Hash: base64url(SHA-256(complete_breadcrumb_chain))
```

Each intermediary (reverse proxy, sidecar, API gateway) appends its
NetworkBreadcrumb to the CBOR array and updates TrIP-NPA-Hash.

### 4.2 gRPC Metadata (Service Mesh)

```
metadata key: "trip-npa-chain-bin"    (binary metadata)
metadata key: "trip-npa-origin-bin"   (binary metadata)
metadata key: "trip-npa-hash-bin"     (binary metadata)
```

### 4.3 IPv6 Extension Header (Network Layer)

For hardware-accelerated deployments (SmartNICs, P4 switches):

```
IPv6 Hop-by-Hop Option:
    Option Type:  TBD (to be assigned by IANA)
    Opt Data Len: variable
    Option Data:  CBOR-encoded NetworkBreadcrumb
```

### 4.4 Size Budget Analysis

| Component | Bytes | Notes |
|-----------|-------|-------|
| hop_index | 1 | uint, max 255 hops |
| node_identity | 32 | Ed25519 public key |
| node_type | 1 | enum |
| timestamp_us | 8 | uint64 |
| processing_us | 4 | uint32 |
| prev_hop_hash | 32 | SHA-256 |
| context_digest | 32 | SHA-256 |
| signature | 64 | Ed25519 |
| **CBOR overhead** | ~12 | map headers, keys |
| **Total per hop** | **~186 bytes** | |

For a typical 5-hop microservice path: **~930 bytes** of header overhead.
For comparison, a typical mTLS handshake exchanges 3,000-5,000 bytes.

---

## 5. The Formal Model (The Math)

### 5.1 Definitions

Let:

- **N** = {n₁, n₂, ..., nₖ} be the set of infrastructure nodes, each
  with Ed25519 keypair (skᵢ, pkᵢ)
- **P** = a packet/request traversing the network
- **B(P)** = {b₁, b₂, ..., bₖ} be the breadcrumb chain accumulated by P
- **T(P)** = the trajectory of P, defined as the ordered sequence of
  (node_type, timestamp, processing_time) tuples

### 5.2 Breadcrumb Construction

At each hop i, node nᵢ constructs breadcrumb bᵢ:

```
bᵢ = {
    i,                                          // hop index
    pkᵢ,                                        // node identity
    typeᵢ,                                      // node type
    tᵢ,                                         // timestamp (μs)
    Δtᵢ,                                        // processing time (μs)
    H(bᵢ₋₁),                                   // hash of previous breadcrumb
    ctx(nᵢ),                                    // context digest of node state
    σᵢ = Sign(skᵢ, CBOR(i, pkᵢ, typeᵢ, tᵢ, Δtᵢ, H(bᵢ₋₁), ctx(nᵢ)))
}
```

### 5.3 Chain Validity (Formal)

A breadcrumb chain B(P) = {b₁, ..., bₖ} is **valid** if and only if
ALL of the following hold:

```
∀ i ∈ [1, k]:
    (V1) Verify(pkᵢ, σᵢ, CBOR(bᵢ[0..6])) = true       // Signature validity
    (V2) bᵢ.hop_index = i                                // Contiguous sequence
    (V3) bᵢ.timestamp ≥ bᵢ₋₁.timestamp                  // Monotonic time
    (V4) bᵢ.prev_hop_hash = SHA-256(CBOR(bᵢ₋₁))         // Hash chain integrity
    (V5) pkᵢ ∈ TrustedNodes(topology)                    // Node is trusted
    (V6) bᵢ.timestamp - bᵢ₋₁.timestamp ≤ MaxHopLatency  // Timing plausibility
```

And for the genesis breadcrumb b₁:
```
    (V0) b₁.prev_hop_hash = SHA-256(request_id)          // Bound to request
    (V0') b₁.node_type = origin                           // Must start at origin
```

### 5.4 The Network Hamiltonian (Anomaly Detection)

Inspired by the six-component Hamiltonian in draft-03 Section 8, TrIP-NPA
defines a **four-component Network Hamiltonian** that scores path anomaly:

```
H_net = w₁·H_sequence + w₂·H_timing + w₃·H_topology + w₄·H_context
```

#### H_sequence: Path Sequence Anomaly

Using a learned Markov Transition Matrix M over node types:

```
M[typeᵢ][typeⱼ] = P(next_node_type = typeⱼ | current_node_type = typeᵢ)

H_sequence = Σᵢ -log(max(M[type(bᵢ)][type(bᵢ₊₁)], ε))
```

A request that goes gateway → service → database has low H_sequence.
A request that goes database → origin → CDN has high H_sequence.

*This directly parallels H_kinetic (draft-03 Section 8.3) which uses
the Markov Transition Matrix over anchor cells.*

#### H_timing: Latency Anomaly

For each hop, compare observed latency to the learned baseline:

```
μᵢⱼ = mean latency between node types i and j (learned)
σᵢⱼ = standard deviation of latency (learned)

H_timing = Σᵢ ((tᵢ₊₁ - tᵢ) - μᵢⱼ)² / (2·σᵢⱼ²)
```

Abnormally fast hops suggest bypassed processing.
Abnormally slow hops suggest interception or man-in-the-middle.

*This directly parallels H_temporal (draft-03 Section 8.2) which
detects circadian rhythm violations.*

#### H_topology: Structural Anomaly

Verify that the path matches a known valid topology graph:

```
G = (V, E) where V = trusted nodes, E = valid connections

H_topology = Σᵢ {
    0                   if (bᵢ, bᵢ₊₁) ∈ E       // Known valid edge
    λ_unknown           if bᵢ₊₁ ∈ V but edge unknown  // Known node, new path
    λ_untrusted         if bᵢ₊₁ ∉ V              // Unknown node entirely
}
```

*This parallels the H_flock component (draft-03 Section 8.4) which
detects misalignment with expected co-located trajectories.*

#### H_context: Node State Anomaly

Compare node health digest to expected baseline:

```
H_context = Σᵢ divergence(ctx(bᵢ), expected_ctx(nodeᵢ, timeᵢ))
```

A breadcrumb stamped by a node under abnormal load, running unexpected
software, or with anomalous error rates elevates H_context.

*This parallels H_contextual (draft-03 Section 8.5) which detects
sensor cross-correlation failures.*

### 5.5 Default Weights

| Component | Weight | Diagnostic Target |
|-----------|--------|-------------------|
| H_sequence | 0.30 | Impossible/unexpected path sequences |
| H_timing | 0.30 | Latency anomalies (MITM, bypass) |
| H_topology | 0.25 | Unknown nodes, unauthorized routes |
| H_context | 0.15 | Node health / compromised infrastructure |

### 5.6 Alert Classification

Following draft-03 Section 8.7 exactly:

| H_net Range | Level | Action |
|-------------|-------|--------|
| [0, baseline × 1.5) | **NOMINAL** | Accept, log |
| [baseline × 1.5, 3.0) | **ELEVATED** | Accept, flag for review |
| [3.0, 5.0) | **SUSPICIOUS** | Hold, require re-attestation |
| [5.0, ∞) | **CRITICAL** | Reject, alert SOC |

---

## 6. The ULissy DSL — Network Path Policy

ULissy (the domain-specific language for GNS Protocol development) provides
compile-time safety guarantees for network path policies. The following
extensions build on the ULissy type system defined in the ULissy Language
Whitepaper v0.1.

### 6.1 New Types

```ulissy
// ─── Network Breadcrumb (extends core Breadcrumb concept) ───

type NodeType = enum {
    .origin, .gateway, .loadbalancer, .waf,
    .sidecar, .service, .database, .queue, .egress, .cdn
}

type NetworkBreadcrumb {
    hopIndex:       Uint8
    nodeIdentity:   PublicKey           // Ed25519 PK of this node
    nodeType:       NodeType
    timestamp:      Moment              // Microsecond precision
    processingTime: Duration            // Time spent at this node
    prevHopHash:    Hash                // SHA-256 chain link
    contextDigest:  Hash                // Node health digest
    signature:      Signature           // Ed25519 proof

    // ─── Compile-time invariants (like Breadcrumb in draft-03) ───
    invariant timestamp > previous.timestamp
    invariant signature.valid(for: self, by: nodeIdentity)
    invariant hopIndex == previous.hopIndex + 1
    invariant prevHopHash == hash(previous)
}

type PathTrajectory = Chain<NetworkBreadcrumb>

type PathAttestation {
    originIdentity:  PublicKey           // Who sent the request
    trajectory:      PathTrajectory      // The breadcrumb chain
    trajectoryHash:  Hash                // SHA-256 of complete chain
    hopCount:        Uint8
    totalLatency:    Duration            // End-to-end
    anomalyScore:    Float               // H_net value

    computed isValid: Bool = trajectory.allSatisfy { crumb in
        crumb.signature.valid
        && crumb.prevHopHash == hash(crumb.previous)
        && crumb.timestamp > crumb.previous.timestamp
    }
}
```

### 6.2 Path Validation — Valid vs. Anomalous Chain

```ulissy
// ─── Core validation function ───

fn validatePathChain(chain: PathTrajectory) throws -> PathVerdict {

    // V1: Structural integrity (compile-time guaranteed by Chain<T>)
    guard chain.isContiguous else {
        throw PathError.brokenChain(at: chain.firstBreak)
    }

    // V2: Signature verification at every hop
    for crumb in chain {
        guard crumb.signature.valid(for: crumb, by: crumb.nodeIdentity) else {
            throw PathError.invalidSignature(hop: crumb.hopIndex,
                                              node: crumb.nodeIdentity)
        }
    }

    // V3: All nodes must be in trusted topology
    let untrusted = chain.filter { !topology.contains($0.nodeIdentity) }
    guard untrusted.isEmpty else {
        throw PathError.untrustedNode(nodes: untrusted.map(\.nodeIdentity))
    }

    // V4: Timing plausibility
    for (prev, curr) in chain.consecutivePairs {
        let hopLatency = curr.timestamp - prev.timestamp
        guard hopLatency >= 0.microseconds else {
            throw PathError.timeTravel(hop: curr.hopIndex)
        }
        guard hopLatency <= maxHopLatency else {
            throw PathError.excessiveLatency(hop: curr.hopIndex,
                                              observed: hopLatency)
        }
    }

    // V5: Compute Network Hamiltonian
    let H = computeNetworkHamiltonian(chain)

    return PathVerdict(
        valid: H.total < alertThreshold,
        anomalyScore: H.total,
        level: classifyAlert(H.total),
        components: H
    )
}
```

### 6.3 Network Hamiltonian in ULissy

```ulissy
// ─── The four-component Network Hamiltonian ───

type NetworkHamiltonian {
    sequence:  Float        // H_sequence: path order anomaly
    timing:    Float        // H_timing:   latency anomaly
    topology:  Float        // H_topology: structural anomaly
    context:   Float        // H_context:  node health anomaly

    computed total: Float =
        0.30 * sequence +
        0.30 * timing +
        0.25 * topology +
        0.15 * context
}

fn computeNetworkHamiltonian(chain: PathTrajectory) -> NetworkHamiltonian {

    // H_sequence: Markov transition probability
    let H_seq = chain.consecutivePairs.reduce(0.0) { sum, pair in
        let (prev, curr) = pair
        let prob = transitionMatrix[prev.nodeType][curr.nodeType]
        return sum + (-log(max(prob, 0.001)))
    }

    // H_timing: Latency deviation from learned baseline
    let H_time = chain.consecutivePairs.reduce(0.0) { sum, pair in
        let (prev, curr) = pair
        let observed = (curr.timestamp - prev.timestamp).microseconds
        let expected = baselineLatency[prev.nodeType][curr.nodeType]
        let stddev   = baselineStdDev[prev.nodeType][curr.nodeType]
        return sum + pow(Float(observed) - expected, 2) / (2 * pow(stddev, 2))
    }

    // H_topology: Unknown edges or nodes
    let H_topo = chain.consecutivePairs.reduce(0.0) { sum, pair in
        let (prev, curr) = pair
        if topologyGraph.hasEdge(from: prev.nodeIdentity,
                                   to: curr.nodeIdentity) {
            return sum + 0.0                    // Known valid edge
        } else if topologyGraph.hasNode(curr.nodeIdentity) {
            return sum + 1.0                    // Known node, unknown edge
        } else {
            return sum + 5.0                    // Unknown node entirely
        }
    }

    // H_context: Node health deviation
    let H_ctx = chain.reduce(0.0) { sum, crumb in
        let expected = expectedContext(node: crumb.nodeIdentity,
                                        at: crumb.timestamp)
        return sum + divergence(crumb.contextDigest, expected)
    }

    return NetworkHamiltonian(
        sequence: H_seq,
        timing:   H_time,
        topology: H_topo,
        context:  H_ctx
    )
}
```

### 6.4 Policy Rules — The ULissy Policy Engine

```ulissy
// ─── Declarative path policies ───

policy "api-gateway-required" {
    // Every request to internal services MUST pass through a gateway
    require chain.any { $0.nodeType == .gateway }
    severity: .critical
}

policy "no-direct-database" {
    // Database access MUST be preceded by a service hop
    require chain.consecutivePairs.none { pair in
        pair.0.nodeType != .service && pair.1.nodeType == .database
    }
    severity: .critical
}

policy "max-five-hops" {
    // No request should traverse more than 5 hops
    require chain.count <= 5
    severity: .elevated
}

policy "sidecar-present" {
    // Service mesh: every service hop must be preceded by its sidecar
    require chain.windows(2).allSatisfy { window in
        window[1].nodeType != .service || window[0].nodeType == .sidecar
    }
    severity: .suspicious
}

policy "timing-plausible" {
    // No hop faster than 10μs (suggests bypass) or slower than 5s
    require chain.consecutivePairs.allSatisfy { pair in
        let latency = pair.1.timestamp - pair.0.timestamp
        return latency >= 10.microseconds && latency <= 5.seconds
    }
    severity: .critical
}

policy "known-topology-only" {
    // All edges must exist in the registered topology graph
    require chain.consecutivePairs.allSatisfy { pair in
        topologyGraph.hasEdge(from: pair.0.nodeIdentity,
                                to: pair.1.nodeIdentity)
    }
    severity: .suspicious
}

// ─── Composite policy evaluation ───

fn evaluateAllPolicies(chain: PathTrajectory) -> [PolicyResult] {
    return policies.map { policy in
        PolicyResult(
            name: policy.name,
            passed: policy.evaluate(chain),
            severity: policy.severity
        )
    }
}
```

### 6.5 Real-time Path Monitoring in ULissy

```ulissy
// ─── Continuous path monitoring (like breadcrumb collection in TRIP) ───

import gns.npa             // Network Path Attestation module
import gns.npa.hamiltonian // Network Hamiltonian computation

// At each infrastructure node:
every incoming.request when npa.enabled {

    let chain = request.tripNpaHeader?.trajectory ?? PathTrajectory.empty

    // Append our breadcrumb
    let myBreadcrumb = networkBreadcrumb(
        nodeIdentity: me.identity,
        nodeType:     .service,
        processingTime: measureProcessingTime(),
        contextDigest: currentNodeHealth(),
        previous:     chain.last
    ).signed(me)

    chain.append(myBreadcrumb)

    // Evaluate
    let verdict = validatePathChain(chain)
    let policies = evaluateAllPolicies(chain)

    match verdict.level {
        case .nominal:
            forward(request, withUpdatedChain: chain)

        case .elevated:
            log.warn("Elevated path anomaly: \(verdict.anomalyScore)")
            forward(request, withUpdatedChain: chain)

        case .suspicious:
            log.alert("Suspicious path detected", chain: chain)
            // Hold and re-verify via Active Verification (draft-03 §12.2)
            let verification = await verifier.activeVerify(
                identity: chain.originIdentity,
                nonce: crypto.randomNonce(16)
            )
            if verification.valid {
                forward(request, withUpdatedChain: chain)
            } else {
                reject(request, reason: .pathAnomalyUnverified)
            }

        case .critical:
            log.critical("CRITICAL path anomaly — rejecting", chain: chain)
            reject(request, reason: .criticalPathAnomaly)
            alert(soc, incident: .criticalPathAnomaly, chain: chain)
    }
}
```

---

## 7. Concrete Example: API Request Through Microservice Architecture

### 7.1 The Journey

```
Client (mobile app)
  │
  ├─[b₁]─→ Cloudflare CDN         (node_type: cdn)
  │
  ├─[b₂]─→ AWS ALB                (node_type: loadbalancer)
  │
  ├─[b₃]─→ Envoy Sidecar          (node_type: sidecar)
  │
  ├─[b₄]─→ Payment Service        (node_type: service)
  │
  ├─[b₅]─→ PostgreSQL             (node_type: database)
  │
  └─[response carries full chain b₁...b₅ back to client]
```

### 7.2 What Each Hop Produces

```
b₁ = {
    hop_index:    1,
    node_pk:      0xCDN_PK...,
    node_type:    cdn (9),
    timestamp_us: 1708300000000000,
    processing:   450,                    // 450μs at CDN edge
    prev_hash:    SHA-256(request_id),    // Genesis
    context:      SHA-256("cpu:12|mem:34|conn:8821|err:0.01|ver:a3f2"),
    signature:    Ed25519(CDN_SK, fields[0..6])
}

b₂ = {
    hop_index:    2,
    node_pk:      0xALB_PK...,
    node_type:    loadbalancer (2),
    timestamp_us: 1708300000002100,       // 2.1ms later
    processing:   180,
    prev_hash:    SHA-256(CBOR(b₁)),      // Chained to CDN breadcrumb
    context:      SHA-256("cpu:45|mem:67|conn:12043|err:0.003|ver:b1c4"),
    signature:    Ed25519(ALB_SK, fields[0..6])
}

... and so on through b₃, b₄, b₅.
```

### 7.3 What the Client Verifies

Upon receiving the response, the client (or a security gateway) runs:

```
For each breadcrumb bᵢ in the chain:
  ✓ Ed25519 signature valid against node_pk
  ✓ hop_index is contiguous (1, 2, 3, 4, 5)
  ✓ timestamps are monotonically increasing
  ✓ prev_hash matches hash of previous breadcrumb
  ✓ node_pk is in the trusted node registry
  ✓ path sequence matches policy: cdn → lb → sidecar → service → db ✓

Network Hamiltonian:
  H_sequence = 0.12  (expected path pattern)
  H_timing   = 0.08  (normal latencies)
  H_topology = 0.00  (all edges known)
  H_context  = 0.05  (all nodes healthy)
  H_net      = 0.30 × 0.12 + 0.30 × 0.08 + 0.25 × 0.00 + 0.15 × 0.05
             = 0.036 + 0.024 + 0.000 + 0.0075
             = 0.0675 → NOMINAL ✓
```

### 7.4 Attack Detection Example: MITM Injection

An attacker intercepts traffic between the ALB and the Envoy sidecar,
injecting a rogue node:

```
b₁ → b₂ → b_ROGUE → b₃ → b₄ → b₅
```

Detection:

- **V5 fails**: b_ROGUE.node_pk ∉ TrustedNodes → IMMEDIATE REJECT
- **H_topology**: Unknown node → +5.0 energy → CRITICAL
- **H_sequence**: lb → unknown → sidecar never seen in training data
  → high -log(probability) → CRITICAL
- **H_timing**: Extra hop adds latency outside baseline σ → ELEVATED

Even if the attacker somehow registers a fake node identity:

- **H_sequence**: The transition pattern `lb → fake_type → sidecar`
  has near-zero probability in the Markov matrix
- **H_timing**: The timing fingerprint of the injected hop differs
  from any learned baseline for that position in the chain

---

## 8. Strategic Positioning

### 8.1 Relationship to draft-ayerbe-trip-protocol-03

This is a **companion specification**, not a modification:

| Aspect | draft-03 | TrIP-NPA |
|--------|----------|----------|
| Scope | Human identity via geospatial trajectory | Network path integrity via infrastructure trajectory |
| Modifies draft-03? | N/A | **No** — reuses primitives, extends to new domain |
| RATS mapping | Attester = mobile device | Attester = infrastructure node |
| Criticality Engine | PSD alpha + Levy flights + 6-component Hamiltonian | Network Hamiltonian (4 components) |
| Transport binding | Deferred (draft-03 §15.3) | Defined (HTTP header, gRPC metadata, IPv6 ext) |
| Breadcrumb type | Geospatial (H3 cell) | Infrastructure (node identity + hop) |

### 8.2 Why This Matters for the IETF Roadmap

1. **draft-03 is the current revision.** Usama's co-authorship and the physicist's corrections are incorporated. TrIP-NPA builds on top without modification.
2. **TrIP-NPA becomes a natural companion draft** (draft-ayerbe-trip-npa-00) that demonstrates
   the protocol's generality — the same math applies to both human movement
   and network path attestation.
3. **NASR and TPR become allies, not competitors.** TrIP-NPA fills their
   behavioral gap; their topology work fills TrIP-NPA's infrastructure gap.
4. **Real implementation path:** This can be deployed TODAY as HTTP middleware
   (Express.js, Envoy filter) without waiting for IPv6 extension header
   assignment.
5. **draft-03 strengthens TrIP-NPA foundations.** The new Lévy-PSD Bridge
   (draft-03 Section 6.3), Convergence Analysis (Section 6.4), and
   Statistical Classifier Limitations (Section 13.7) provide rigorous
   underpinning that the Network Hamiltonian inherits directly.

### 8.3 The Paradigm Shift

```
Traditional:   Perimeter → "Did it come through the front door?"
Zero Trust:    Identity  → "Who is asking?"
TrIP-NPA:     Trajectory → "Where has it been, and does the pattern make sense?"
```

TrIP-NPA adds the missing dimension: **behavioral memory at the protocol level.**

---

## 9. Next Steps

1. **Immediate:** Write proof-of-concept Express.js middleware that appends
   TrIP-NPA breadcrumbs to HTTP requests flowing through a service mesh.

2. **Short-term:** Propose TrIP-NPA as a companion Internet-Draft alongside
   draft-ayerbe-trip-protocol-03, referencing NASR and TPR as related work.

3. **Medium-term:** Engage with NASR BoF participants at IETF 119+ to
   position TrIP-NPA as the behavioral complement to their static attestation.

4. **Long-term:** ULissy compiler generates Envoy WASM filters from
   `.ul` policy files, enabling declarative network path policy enforcement
   compiled to production-grade infrastructure code.

---

## References

- [draft-ayerbe-trip-protocol-03] C. Ayerbe Posada, "TRIP: Trajectory-based
  Recognition of Identity Proof", Internet-Draft draft-ayerbe-trip-protocol-03,
  February 2026. https://datatracker.ietf.org/doc/draft-ayerbe-trip-protocol/
- [RFC 9334] H. Birkholz et al., "Remote ATtestation procedureS (RATS)
  Architecture", RFC 9334, January 2023.
- [NASR] "Network Attestation for Secure Routing", IETF BoF Request,
  https://datatracker.ietf.org/doc/bofreq-liu-nasr/
- [TPR] E. Voit, "Trusted Path Routing using Remote Attestation",
  draft-voit-rats-trusted-path-routing-01.
- [ULissy] C. Ayerbe Posada, "ULissy Language Whitepaper v0.1",
  ULISSY s.r.l., 2025.
- [RFC 8032] S. Josefsson, I. Liusvaara, "Edwards-Curve Digital Signature
  Algorithm (EdDSA)", RFC 8032.
- [RFC 8949] C. Bormann, P. Hoffman, "Concise Binary Object Representation
  (CBOR)", RFC 8949.
- [H3] Uber Technologies, "H3: Uber's Hexagonal Hierarchical Spatial Index",
  https://h3geo.org/

---

*ULISSY s.r.l. — Identity = Public Key · Trajectory = Proof · HUMANS PREVAIL*
