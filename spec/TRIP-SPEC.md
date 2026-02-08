# TRIP: Trajectory Routing Identity Protocol

## Dynamic Identity for the Physical World

**PROTOCOL SPECIFICATION**
*Version 1.1.0 — Draft*

**Authors:** GNS Foundation
**Date:** January 2025
**Status:** Draft Specification

---

## Abstract

**TRIP identities are MOVING patterns that must be continuously renewed through physical-world activity.**

TRIP (Trajectory Routing Identity Protocol) introduces **dynamic identity** — identity defined not by static tokens but by continuous movement through physical space. Unlike IP addresses (static locations) or HIP keys (static tokens), TRIP identities are not generated once and used forever. They grow, evolve, and prove ongoing physical existence.

TRIP enables secure, authenticated communication between any physical-world entity — humans, autonomous machines, vehicles, drones, robots, and IoT devices — by separating identity from network location. The protocol uses public key identifiers from a new **Trajectory-Verified Identity** namespace, where the trajectory itself IS the identity.

The protocol is designed to be resistant to denial-of-service (DoS), man-in-the-middle (MitM), and Sybil attacks. Unlike static identity systems, TRIP provides proof of physical existence, not merely proof of key possession.

**The core insight:**

> IP asks WHERE you are.
> HIP asks WHAT key you have.  
> TRIP asks HOW you move.
>
> Only movement proves existence.

---

## Table of Contents

1. [The Identity Paradigm Shift](#1-the-identity-paradigm-shift)
2. [Introduction](#2-introduction)
3. [Protocol Overview](#3-protocol-overview)
4. [Trajectory-Verified Identity Namespace](#4-trajectory-verified-identity-namespace)
5. [Comparison: Static vs Dynamic Identity](#5-comparison-static-vs-dynamic-identity)
6. [Protocol Messages](#6-protocol-messages)
7. [Base Exchange (Handshake)](#7-base-exchange-handshake)
8. [Trust Levels](#8-trust-levels)
9. [Mobility Support](#9-mobility-support)
10. [Secure Channel](#10-secure-channel)
11. [Discovery and Rendezvous](#11-discovery-and-rendezvous)
12. [Payment Integration](#12-payment-integration)
13. [Security Considerations](#13-security-considerations)
14. [ULissy Language Bindings](#14-ulissy-language-bindings)
15. [Implementation Guidelines](#15-implementation-guidelines)
16. [References](#16-references)

---

## 1. The Identity Paradigm Shift

### 1.1 Three Generations of Network Identity

```
┌─────────────────────────────────────────────────────────────────┐
│              THE EVOLUTION OF NETWORK IDENTITY                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   GENERATION 1: IP (1981)                                       │
│   ═══════════════════════                                       │
│   Identity = WHERE you are                                      │
│   Static point in network topology                              │
│   "You are your network location"                               │
│                                                                 │
│   GENERATION 2: HIP (2004-2015)                                 │
│   ═════════════════════════════                                 │
│   Identity = WHAT key you have                                  │
│   Static cryptographic token                                    │
│   "You are your public key"                                     │
│                                                                 │
│   GENERATION 3: TRIP (2025)                                     │
│   ═════════════════════════                                     │
│   Identity = HOW you move                                       │
│   Dynamic trajectory pattern                                    │
│   "You are your journey through space-time"                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 The Problem with Static Identity

**Important Distinction: Identity vs Session**

Modern protocols like HIP correctly use ephemeral key exchange for session security — each connection gets fresh encryption keys via Diffie-Hellman. This is good cryptographic practice that TRIP also follows.

The "static" problem is not about sessions — it's about **what constitutes identity itself**:

```
┌─────────────────────────────────────────────────────────────────┐
│                     IDENTITY vs SESSION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   HIP:                                                          │
│     Identity = Public key (static, generated once)              │
│     Session  = Ephemeral DH keys (dynamic, per-connection)      │
│                                                                 │
│   TRIP:                                                         │
│     Identity = Public key + Trajectory (dynamic, ongoing)       │
│     Session  = Ephemeral X25519 keys (dynamic, per-connection)  │
│                                                                 │
│   Both have dynamic SESSIONS. The difference is:                │
│   HIP identity is a static KEY.                                 │
│   TRIP identity is a dynamic TRAJECTORY.                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**The static identity problem (what TRIP solves):**

**IP Addresses** identify by location:
- NAT breaks identity (millions share one IP)
- Mobility breaks identity (new IP per network)
- Not really identity at all — just a locator

**HIP Identity Keys** are cryptographically sound but static:
- Generated once, valid forever
- No proof of physical existence required
- No proof of continued operation
- Zero cost to create unlimited identities (Sybil vulnerable)
- A key from a decommissioned machine remains "valid"

The identity KEY in HIP is correct cryptography. The problem is that **possession of a key proves nothing about physical existence**. You can generate a million HIP identities in seconds — each cryptographically valid, none proving anything exists in the real world.

TRIP adds the missing piece: **the trajectory that proves the key represents something that actually moves through physical space**.

### 1.3 The Dynamic Identity Solution

TRIP introduces a fundamentally different paradigm:

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   STATIC IDENTITY (IP, HIP):                                    │
│   ══════════════════════════                                    │
│                                                                 │
│   Generate key ──► Use forever ──► No ongoing cost              │
│                                                                 │
│   Identity is a NOUN — a thing you possess.                     │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   DYNAMIC IDENTITY (TRIP):                                      │
│   ════════════════════════                                      │
│                                                                 │
│   Generate key ──► Move ──► Collect breadcrumbs ──► Build trust │
│        ↑                                                │       │
│        └──────────── continuous renewal ────────────────┘       │
│                                                                 │
│   Identity is a VERB — something you continuously DO.           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**The trajectory IS the identity.** Not a verification of identity — the identity itself.

### 1.4 Why Movement Matters

Movement through physical space proves:

1. **Physical Existence:** You cannot move without existing
2. **Continued Operation:** Movement requires ongoing activity
3. **Resource Investment:** Physical presence has real cost
4. **Temporal Commitment:** Trajectories take time to build
5. **Uniqueness:** No two entities can have identical trajectories

Static keys prove only that someone once generated a keypair. Trajectories prove ongoing physical existence in the real world.

### 1.5 The Philosophy

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   "You are not what you HAVE (a key).                           │
│    You are what you DO (your trajectory through the world)."    │
│                                                                 │
│   Static identity: I have a passport.                           │
│   Dynamic identity: I have a life — a path through space-time.  │
│                                                                 │
│   A key proves possession.                                      │
│   A trajectory proves EXISTENCE.                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Introduction

### 2.1 The Problem

The internet conflates identity with location at multiple layers:

**Network Layer (IP):** Your IP address serves as both identifier and locator. When you move networks, your identity changes. This was designed for stationary mainframes, not mobile devices.

**Application Layer (Platforms):** Your @username, email, or phone number are locators disguised as identities. They tell services where to route messages, but they're controlled by platforms, not by you.

**Host Layer (HIP):** RFC 7401 introduced cryptographic host identity, but it remains static — a key generated once and used forever, with no proof of physical existence.

**Physical Layer (Missing):** There is no standard protocol for physical-world entities — humans, machines, vehicles, drones — to establish identities verified by their actual existence and movement in the physical world.

### 2.2 Prior Art: Host Identity Protocol (HIP)

RFC 7401 defines the Host Identity Protocol:

> "HIP allows consenting hosts to securely establish and maintain shared IP-layer state, allowing separation of the identifier and locator roles of IP addresses. HIP uses public key identifiers from a new Host Identity namespace for mutual peer authentication. The protocol is designed to be resistant to denial-of-service (DoS) and man-in-the-middle (MitM) attacks."

HIP is technically sound but has fundamental limitations:
1. Requires operating system kernel modifications
2. Identity is a static key — no proof of physical existence
3. Zero cost to create identities (Sybil vulnerable)
4. No human-readable naming layer
5. No mechanism for verifying physical-world presence

**HIP solved identifier/locator separation but not the static identity problem.**

### 2.3 TRIP: Dynamic Identity for the Physical World

TRIP extends and transforms HIP's architecture:

| Challenge | HIP Approach | TRIP Approach |
|-----------|--------------|---------------|
| **Identity nature** | Static key | Dynamic trajectory |
| **Proves** | Key possession | Physical existence |
| **Lifespan** | Forever | Continuously renewed |
| **Movement** | Survives movement | IS the identity |
| **Sybil resistance** | None | Physical-world anchored |
| **Deployment** | Kernel changes | Application layer |
| **Entity types** | Hosts (machines) | Any physical-world entity |
| **Human names** | None | @handle namespace |
| **Payments** | None | Native Stellar integration |

### 2.4 Who Can Have a TRIP Identity?

Any entity that moves through physical space:

```
┌─────────────────────────────────────────────────────────────────┐
│                  TRIP IDENTITY APPLICABILITY                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   👤 HUMANS                                                     │
│      Smartphones, wearables                                     │
│      "Verified person, not a bot"                               │
│                                                                 │
│   🚁 DRONES / UAVs                                              │
│      Flight controllers with GPS                                │
│      "Verified aircraft, authorized flight path"                │
│                                                                 │
│   🚗 AUTONOMOUS VEHICLES                                        │
│      Onboard navigation systems                                 │
│      "Verified vehicle, documented route"                       │
│                                                                 │
│   🤖 ROBOTS                                                     │
│      Embedded systems with location                             │
│      "Verified robot, operating in physical space"              │
│                                                                 │
│   📦 DELIVERY DEVICES                                           │
│      IoT with positioning                                       │
│      "Verified device, chain of custody"                        │
│                                                                 │
│   🚢 SHIPS / TRUCKS / AIRCRAFT                                  │
│      Fleet tracking systems                                     │
│      "Verified transport, documented journey"                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

              ╔═══════════════════════════════════╗
              ║  If it MOVES, it can have a       ║
              ║       TRIP identity.              ║
              ╚═══════════════════════════════════╝
```

---

## 3. Protocol Overview

### 3.1 Design Principles

**1. Dynamic Identity**
Identity is not a static token but a living pattern of movement through space-time. Identities must be continuously renewed through physical-world activity.

**2. Application Layer Operation**
TRIP operates entirely at the application layer, requiring no operating system modifications. Deployable on existing infrastructure today.

**3. Physical-World Anchoring**
Identity verification is anchored in physical-world movement patterns, not computational work or biometric data.

**4. Progressive Trust**
Trust is earned over time through consistent physical presence, not granted immediately upon key generation.

**5. Entity Agnostic**
The protocol applies equally to humans, machines, vehicles, drones, robots, and any entity that moves through physical space.

**6. Cryptographic Sovereignty**
Entities control their own keys. No central authority can revoke or modify identities.

**7. Payment-Native**
Financial transactions are first-class protocol operations.

### 3.2 Protocol Stack

```
┌─────────────────────────────────────────────────────────────────┐
│                    APPLICATION PROTOCOLS                        │
│      Messaging │ Payments │ Fleet Management │ IoT │ etc.       │
└─────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────┐
│                         TRIP LAYER                              │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    TRIP Session                          │  │
│  │     Encrypted Channel │ Authenticated │ Mobile           │  │
│  └──────────────────────────────────────────────────────────┘  │
│                              │                                  │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │
│  │ Trajectory  │ │    Base     │ │   Trust     │ │  Payment  │ │
│  │  Verified   │ │  Exchange   │ │   System    │ │  Channel  │ │
│  │  Identity   │ │ (Handshake) │ │(Progressive)│ │ (Stellar) │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘ │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────┐
│                     TRANSPORT LAYER                             │
│              WebSocket │ QUIC │ TCP │ UDP                       │
└─────────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────────┐
│                      NETWORK LAYER                              │
│                          IP                                     │
└─────────────────────────────────────────────────────────────────┘
```

### 3.3 Core Components

**Trajectory-Verified Identity (TVI):** An Ed25519 public key whose validity is established through physical-world movement. The key is the anchor; the trajectory is the proof of existence.

**Trajectory Identity Tag (TIT):** A 128-bit (16-byte) hash of the TVI, used for compact routing and protocol headers. (Also called HIT for HIP compatibility.)

**Handle:** A human-readable name (@username) bound to a TVI through Proof-of-Trajectory.

**Trajectory:** A cryptographically-signed chain of location breadcrumbs proving physical-world presence over time. This IS the identity — not a verification of it.

**Trust Level:** A score derived from trajectory history, determining protocol privileges.

**Facet:** A derived identity for contextual separation (work, personal, fleet, device) while maintaining cryptographic linkage to the root TVI.

---

## 4. Trajectory-Verified Identity Namespace

### 4.1 Identity Hierarchy

```
                    TRIP IDENTITY NAMESPACE
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
          ▼                 ▼                 ▼
   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
   │   Human     │   │   Machine   │   │   Hybrid    │
   │  Identity   │   │  Identity   │   │  Identity   │
   │             │   │             │   │             │
   │  @alice     │   │ @drone_42   │   │ @truck_7    │
   │  (person)   │   │ (UAV)       │   │ (vehicle+   │
   │             │   │             │   │  driver)    │
   └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
          │                 │                 │
          └─────────────────┼─────────────────┘
                            │
                   ┌────────┴────────┐
                   │   Ed25519 Key   │
                   │       +         │
                   │   TRAJECTORY    │
                   │  (the identity) │
                   └────────┬────────┘
                            │
            ┌───────────────┼───────────────┐
            │               │               │
            ▼               ▼               ▼
      ┌──────────┐   ┌───────────┐   ┌──────────┐
      │   TIT    │   │  Stellar  │   │ X25519   │
      │(16 bytes)│   │  Address  │   │   Key    │
      │ routing  │   │ payments  │   │ encrypt  │
      └──────────┘   └───────────┘   └──────────┘
```

### 4.2 Trajectory-Verified Identity (TVI)

The TVI is an Ed25519 public key anchored by trajectory:

```
TVI = Ed25519_PublicKey (32 bytes) + Trajectory_History
```

The key alone is necessary but not sufficient. The trajectory proves the key represents a real, active, physical-world entity.

Properties:
- **Self-certifying:** The holder proves ownership by signing challenges
- **Physically-anchored:** Validity requires ongoing trajectory
- **Globally unique:** Cryptographically guaranteed
- **Platform-independent:** Not bound to any service or company
- **Entity-agnostic:** Works for humans, machines, vehicles, any physical entity

### 4.3 Trajectory Identity Tag (TIT / HIT)

The TIT is a compact representation:

```
TIT = SHA-256(TVI)[0:16] (16 bytes, 128 bits)
```

Compatible with HIP's HIT format for interoperability.

Properties:
- **Compact:** Half the size of the full public key
- **One-way:** Cannot derive TVI from TIT
- **Collision-resistant:** SHA-256 truncation maintains security
- **HIP-compatible:** Same derivation method as RFC 7401

### 4.4 Handle

A Handle is a human-readable name bound to a TVI:

```
Handle = "@" + [a-z0-9_]{1,20}
```

Examples:
- Humans: `@alice`, `@bob_smith`
- Machines: `@drone_42`, `@robot_arm_7`
- Vehicles: `@truck_fleet_12`, `@delivery_van_3`
- Systems: `@warehouse_bot`, `@traffic_light_42`

Properties:
- **Earned:** Requires Proof-of-Trajectory (minimum 100 breadcrumbs)
- **Globally unique:** First valid claim wins
- **Entity-agnostic:** Same namespace for humans and machines

### 4.5 Trajectory (The Identity Itself)

The trajectory is not merely proof — it IS the identity:

```
Trajectory = {
    entity: TVI,                    // The public key anchor
    breadcrumbs: [Breadcrumb],      // Movement history
    epochs: [Epoch],                // Published proof bundles
    trust_level: TrustLevel,        // Earned trust
}

Breadcrumb = {
    owner: TVI,
    index: u64,
    timestamp: Timestamp,
    cell: H3Cell,                   // Location (quantized ~5km²)
    context: Hash,                  // Sensor context digest
    previous: Hash,                 // Chain link
    signature: Signature
}

Epoch = {
    owner: TVI,
    breadcrumbs: [Breadcrumb],      // ≥ 100 breadcrumbs
    merkle_root: Hash,
    start_time: Timestamp,
    end_time: Timestamp,
    signature: Signature
}
```

**Key insight:** A TVI without trajectory is just a key. A TVI with trajectory is an IDENTITY.

---

## 5. Comparison: Static vs Dynamic Identity

### 5.1 The Three Generations

| Aspect | IP (Gen 1) | HIP (Gen 2) | TRIP (Gen 3) |
|--------|------------|-------------|--------------|
| **Year** | 1981 | 2004-2015 | 2025 |
| **Identity is** | Location | Static key | Dynamic trajectory |
| **Session security** | None | DH ephemeral ✓ | X25519 ephemeral ✓ |
| **Identity proves** | Network position | Key possession | Physical existence |
| **Identity lifespan** | Until IP changes | Forever | Continuously renewed |
| **Movement** | Breaks identity | Identity survives | **IS the identity** |
| **Sybil cost** | ~$0 | ~$0 | >$100 per identity |
| **Entity types** | Network interfaces | Network hosts | Any physical entity |

**Note:** HIP's session security is excellent — ephemeral DH keys per connection. The limitation is that the IDENTITY KEY is static and proves nothing about physical existence.

### 5.2 What Each Protocol Asks

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   IP asks:   "WHERE are you connected from?"                    │
│              → Proves: Network topology position                │
│              → Weakness: Position ≠ Identity                    │
│                                                                 │
│   HIP asks:  "WHAT key do you possess?"                         │
│              → Proves: Control of a keypair                     │
│              → Weakness: Keys are free to generate              │
│                                                                 │
│   TRIP asks: "HOW do you move through the physical world?"      │
│              → Proves: Physical existence and activity          │
│              → Strength: Movement cannot be faked at scale      │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 5.3 Sybil Resistance Comparison

**IP Addresses:**
```
Cost to create 1000 fake identities: ~$0
Method: Spoof source addresses, use proxies
Time: Seconds
```

**HIP Identities:**
```
Cost to create 1000 fake identities: ~$0
Method: Generate 1000 keypairs
Time: < 1 second
```

**TRIP Identities:**
```
Cost to create 1000 fake identities: >$100,000
Method: Need 1000 physical devices, 1000 operators, weeks of movement
Time: Weeks to months
```

### 5.4 Why TRIP's Approach Works

Static identities (IP, HIP) fail because:
- Digital tokens are free to copy
- No ongoing cost to maintain
- No proof of physical existence
- Bot farms can generate unlimited identities

Dynamic identities (TRIP) succeed because:
- Physical movement has real cost
- Ongoing activity required
- Proves physical existence
- Cannot parallelize without physical hardware

---

## 6. Protocol Messages

### 6.1 Message Format

All TRIP messages share a common header:

```
┌────────────────────────────────────────────────────────────────┐
│                      TRIP Message Header                       │
├────────────────────────────────────────────────────────────────┤
│  version     :  u8          │  Protocol version (1)            │
│  msg_type    :  u8          │  Message type code                │
│  flags       :  u16         │  Message flags                    │
│  sender_tit  :  [u8; 16]    │  Sender's TIT                     │
│  recip_tit   :  [u8; 16]    │  Recipient's TIT                  │
│  msg_id      :  u64         │  Unique message identifier        │
│  timestamp   :  u64         │  Unix timestamp (milliseconds)    │
│  verify_mask :  u8          │  Verification flags (see 13.2.4)  │
│  reserved    :  [u8; 3]     │  Reserved for future use          │
│  length      :  u32         │  Payload length                   │
├────────────────────────────────────────────────────────────────┤
│                         Payload                                │
│                    (varies by msg_type)                        │
├────────────────────────────────────────────────────────────────┤
│  signature   :  [u8; 64]    │  Ed25519 signature                │
└────────────────────────────────────────────────────────────────┘

Header size: 60 bytes (was 56, +4 for verify_mask and reserved)
```

**Verification Mask Bits:**
```
Bit 0: GPS_ONLY           - Self-attested location only
Bit 1: TEMPORAL_VERIFIED  - Passed speed-of-light checks
Bit 2: PEER_VOUCHED       - Has 1+ peer attestations
Bit 3: PEER_VOUCHED_MULTI - Has 5+ peer attestations
Bit 4: ANCHOR_VOUCHED     - Has 1+ anchor attestations
Bit 5: ANCHOR_VOUCHED_MULTI - Has 3+ anchor attestations
Bit 6: HARDWARE_ATTESTED  - Uses secure enclave
Bit 7: ENTROPY_VERIFIED   - Passed entropy analysis
```

### 6.2 Message Types

| Code | Name | Category | Description |
|------|------|----------|-------------|
| 0x01 | I1 | Handshake | Initiate connection |
| 0x02 | R1 | Handshake | Challenge response |
| 0x03 | I2 | Handshake | Proof response |
| 0x04 | R2 | Handshake | Session confirmation |
| 0x10 | DATA | Session | Encrypted application data |
| 0x11 | ACK | Session | Acknowledgment |
| 0x12 | PING | Session | Keep-alive request |
| 0x13 | PONG | Session | Keep-alive response |
| 0x14 | CLOSE | Session | Close session |
| 0x20 | UPDATE | Mobility | Endpoint change |
| 0x21 | UPDATE_ACK | Mobility | Endpoint change confirmed |
| 0x30 | FIND | Discovery | Find peer |
| 0x31 | FOUND | Discovery | Peer location |
| 0x32 | REGISTER | Discovery | Register with relay |
| 0x33 | REGISTER_ACK | Discovery | Registration confirmed |
| 0x40 | EPOCH_ANNOUNCE | Trust | New epoch available |
| 0x41 | EPOCH_REQUEST | Trust | Request epoch data |
| 0x42 | EPOCH_DATA | Trust | Epoch contents |
| 0x43 | VOUCH | Trust | Vouch for identity |
| 0x44 | VOUCH_REVOKE | Trust | Revoke vouch |
| 0x45 | PROXIMITY_REQ | Trust | Request proximity proof |
| 0x46 | PROXIMITY_PROOF | Trust | Proximity attestation |
| 0x47 | ANCHOR_REQ | Trust | Request anchor attestation |
| 0x48 | ANCHOR_ATTEST | Trust | Anchor attestation |
| 0x50 | PAY_REQUEST | Payment | Request payment |
| 0x51 | PAY_APPROVE | Payment | Approve payment |
| 0x52 | PAY_COMPLETE | Payment | Payment confirmed |
| 0x53 | PAY_REJECT | Payment | Payment rejected |
| 0xF0 | ERROR | Control | Error response |
| 0xF1 | NOTIFY | Control | Notification |

---

## 7. Base Exchange (Handshake)

### 7.1 Overview

The TRIP Base Exchange establishes a secure session between two trajectory-verified identities. Unlike HIP's computational puzzles, TRIP uses trajectory trust levels for DoS protection.

```
┌─────────────┐                              ┌─────────────┐
│  Initiator  │                              │  Responder  │
│  (Entity A) │                              │  (Entity B) │
└──────┬──────┘                              └──────┬──────┘
       │                                            │
       │  I1: Initiate                              │
       │  [TIT, TVI, requested_trust, nonce]        │
       │───────────────────────────────────────────►│
       │                                            │
       │                     ┌──────────────────────┤
       │                     │ Check trajectory     │
       │                     │ trust of initiator   │
       │                     └──────────────────────┤
       │                                            │
       │  R1: Challenge                             │
       │  [TVI, required_proof, DH_ephemeral]       │
       │◄───────────────────────────────────────────│
       │                                            │
       │  I2: Response                              │
       │  [trajectory_proof, DH_ephemeral, sig]     │
       │───────────────────────────────────────────►│
       │                                            │
       │                     ┌──────────────────────┤
       │                     │ Verify trajectory    │
       │                     │ Compute shared key   │
       │                     └──────────────────────┤
       │                                            │
       │  R2: Confirm                               │
       │  [session_id, granted_trust, lifetime]     │
       │◄───────────────────────────────────────────│
       │                                            │
       │  ═══════════════════════════════════════   │
       │       ENCRYPTED SESSION ESTABLISHED        │
       │  ═══════════════════════════════════════   │
```

### 7.2 Trust-Based DoS Protection

**HIP uses computational puzzles:** Initiator must burn CPU cycles.
- Problem: Punishes legitimate users, especially mobile devices
- Problem: Attackers with botnets can parallelize

**TRIP uses trajectory trust:** Initiator must have established trajectory.
- Advantage: Legitimate users with trajectory connect instantly
- Advantage: Attackers must invest weeks of physical activity per identity
- Advantage: No battery drain for puzzle solving

```
TRIP DoS Protection:

Initiator's trust ≥ Required? ──► YES ──► Connect immediately
                     │
                     NO
                     │
                     ▼
              Provide trajectory proof
                     │
                     ▼
              Proof valid? ──► YES ──► Grant trust, connect
                     │
                     NO
                     │
                     ▼
              Reject connection
```

---

## 8. Trust Levels

### 8.1 Level Definitions

| Level | Name | Requirement | Meaning |
|-------|------|-------------|---------|
| 0 | Unverified | Valid keypair | "Just a key, no trajectory" |
| 1 | Verified | 1+ epoch | "Active in physical world" |
| 2 | Established | 10+ epochs | "Consistently present" |
| 3 | Trusted | 100+ epochs | "Long-term physical presence" |
| 4 | Vouched | Active vouch from L3+ | "Trusted entity vouches" |

### 8.2 Trust Meaning by Entity Type

| Level | For Humans | For Machines | For Vehicles |
|-------|------------|--------------|--------------|
| 0 | Anonymous | Unregistered device | Unknown vehicle |
| 1 | Verified person | Active device | Operational vehicle |
| 2 | Established human | Established device | Fleet vehicle |
| 3 | Trusted person | Trusted device (fleet op) | Trusted fleet |
| 4 | Vouched by trusted | Manufacturer vouched | Fleet-operator vouched |

### 8.3 Privilege Matrix

| Capability | L0 | L1 | L2 | L3 | L4 |
|------------|----|----|----|----|----| 
| Receive messages | ✓ | ✓ | ✓ | ✓ | ✓ |
| Receive payments | ✓ | ✓ | ✓ | ✓ | ✓ |
| Send messages | ✗ | ✓† | ✓ | ✓ | ✓ |
| Send payments | ✗ | ✓ | ✓ | ✓ | ✓ |
| Claim handle | ✗ | ✗ | ✓ | ✓ | ✓ |
| Vouch for others | ✗ | ✗ | ✗ | ✓ | ✗ |
| Run relay node | ✗ | ✗ | ✗ | ✓ | ✗ |

† = Rate limited

### 8.4 Why Trust Decays (Unlike Static Systems)

In HIP, a key is valid forever — even if the host is offline for years.

In TRIP, trust is tied to ongoing trajectory:
- No new breadcrumbs → No new epochs → Trust doesn't grow
- Extended inactivity → Trust may be questioned
- Identity reflects **current** physical activity, not past

This is a feature: An abandoned device doesn't retain trust.

---

## 9. Mobility Support

### 9.1 Movement IS the Identity

In TRIP, mobility isn't just supported — it's fundamental:

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   IP:   Movement BREAKS identity (new IP = new identity)        │
│                                                                 │
│   HIP:  Movement SURVIVES (identity persists across IPs)        │
│                                                                 │
│   TRIP: Movement IS IDENTITY (trajectory defines who you are)   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

Every movement:
- Adds to trajectory
- Strengthens identity
- Builds trust
- Proves continued existence

### 9.2 UPDATE Protocol

When network endpoint changes:

```
Entity                          Relay / Peers
   │                                  │
   │  [Moves to new network]          │
   │  [New breadcrumb collected!]     │
   │                                  │
   │  UPDATE (new locators, signed)   │
   │─────────────────────────────────►│
   │                                  │
   │  UPDATE_ACK                      │
   │◄─────────────────────────────────│
   │                                  │
   │  [Session continues]             │
   │  [Trajectory grows stronger]     │
```

---

## 10. Secure Channel

### 10.1 Encryption

After handshake, all DATA messages are encrypted:

- **Algorithm:** ChaCha20-Poly1305 (AEAD)
- **Key Exchange:** X25519 ephemeral
- **Key Derivation:** HKDF-SHA256
- **Forward Secrecy:** New ephemeral keys per session

### 10.2 Key Derivation

```
shared_secret = X25519(my_ephemeral_private, their_ephemeral_public)

session_keys = HKDF(
    ikm = shared_secret,
    salt = initiator_nonce || responder_nonce,
    info = "TRIP-session-keys-v1",
    length = 96
)

encrypt_key_i2r = session_keys[0:32]
encrypt_key_r2i = session_keys[32:64]
mac_key         = session_keys[64:96]
```

---

## 11. Discovery and Rendezvous

### 11.1 Relay Network

TRIP uses relays for:
- NAT traversal
- Peer discovery
- Message routing
- Mobility support

### 11.2 Entity Discovery

```
Entity A                        Relay                         Entity B
   │                              │                              │
   │  FIND (entity_b_tit)         │                              │
   │─────────────────────────────►│                              │
   │                              │                              │
   │  FOUND (locators, online)    │                              │
   │◄─────────────────────────────│                              │
   │                              │                              │
   │  [Direct connection or relay routing]                       │
   │◄════════════════════════════════════════════════════════════►
```

---

## 12. Payment Integration

### 12.1 Overview

TRIP natively integrates payments via Stellar. The same Ed25519 key serves as both identity and wallet.

### 12.2 Key Equivalence

```
┌─────────────────────────────────────────────────────────────────┐
│                    KEY EQUIVALENCE                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Ed25519 Key (32 bytes)                                        │
│        │                                                        │
│        ├──────► TRIP Identity (sign, authenticate)              │
│        │                                                        │
│        ├──────► Stellar Wallet (send, receive payments)         │
│        │                                                        │
│        └──────► Trajectory Anchor (prove physical existence)    │
│                                                                 │
│   One key. Multiple roles. Unified by trajectory.               │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 12.3 Use Cases

**Human-to-Human:** Alice pays Bob for coffee
**Human-to-Machine:** Customer pays delivery drone
**Machine-to-Machine:** Drone pays charging station
**Fleet Operations:** Vehicle pays toll automatically

---

## 13. Security Considerations

### 13.1 Threat Model

TRIP is designed to resist:

| Threat | Mitigation |
|--------|------------|
| **Eavesdropping** | ChaCha20-Poly1305 encryption |
| **Man-in-the-Middle** | Mutual authentication via signatures |
| **Replay Attacks** | Nonces, sequence numbers, timestamps |
| **DoS (Connection Flood)** | Trajectory trust levels |
| **Sybil (Fake Identities)** | Proof-of-Trajectory (physical cost) |
| **Identity Theft** | Private key in secure enclave |
| **Location Tracking** | H3 quantization, differential resolution |
| **Trajectory Simulation** | Three Pillars of Verification |

### 13.2 Trajectory Integrity (Critical)

The "Security Considerations" section is the most scrutinized part of any protocol. In TRIP, the "proof of work" isn't a hash — it's coordinated movement. Therefore, the primary threat is **Trajectory Simulation (Spoofing)**.

#### 13.2.1 The Virtualization Threat ("Ghost Drone" Attack)

The primary attack vector against TRIP is the use of software-defined GPS emulators to "feed" a TVI-holder fake breadcrumbs. If an attacker can generate 1,000 trajectories from a single server, the Sybil resistance of the protocol collapses.

```
ATTACK SCENARIO:

    Attacker's Server
    ┌─────────────────────────────────────────┐
    │                                         │
    │   GPS Emulator Software                 │
    │   ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐      │
    │   │Fake │ │Fake │ │Fake │ │Fake │ ...  │
    │   │TVI_1│ │TVI_2│ │TVI_3│ │TVI_4│      │
    │   └─────┘ └─────┘ └─────┘ └─────┘      │
    │                                         │
    │   All generating "trajectories" without │
    │   any physical movement                 │
    │                                         │
    └─────────────────────────────────────────┘
    
    If this works → Sybil resistance = 0
    TRIP must prevent this.
```

#### 13.2.2 The Three Pillars of Verification

To ensure a trajectory represents a physical entity, TRIP implementations SHOULD employ a **defense in depth** strategy:

```
┌─────────────────────────────────────────────────────────────────┐
│              THREE PILLARS OF TRAJECTORY VERIFICATION           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   PILLAR A: Temporal Consistency                                │
│   ══════════════════════════════                                │
│   "You can't move faster than physics allows"                   │
│                                                                 │
│   PILLAR B: Peer Attestation                                    │
│   ══════════════════════════                                    │
│   "Other entities saw you there"                                │
│                                                                 │
│   PILLAR C: Anchor-Point Verification                           │
│   ═══════════════════════════════════                           │
│   "Fixed infrastructure confirms your presence"                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**PILLAR A: Temporal Consistency (Speed of Light Constraint)**

A trajectory is invalid if the distance between `Breadcrumb[n]` and `Breadcrumb[n+1]` requires a velocity exceeding the physical limits of the entity type.

```
VELOCITY LIMITS BY ENTITY TYPE:

Entity Type     Max Speed       Rationale
───────────────────────────────────────────────────
Human           50 km/h         Running/cycling
Ground Vehicle  300 km/h        High-speed rail
Maritime        100 km/h        Fast ferry
Drone/UAV       200 km/h        Racing drone
Aircraft        1,200 km/h      Commercial jet
───────────────────────────────────────────────────

VALIDATION:

distance = h3_distance(bc[n].cell, bc[n+1].cell)
time = bc[n+1].timestamp - bc[n].timestamp
velocity = distance / time

if velocity > max_speed[entity_type]:
    REJECT as physically impossible
```

Example: A "Human" entity moving at 800 km/h between two 10-minute breadcrumbs is flagged as anomalous.

**PILLAR B: Peer Attestation ("I Saw You" Proof)**

When two TRIP entities are within physical range (via Bluetooth, DSRC, Wi-Fi Direct, or NFC), they MAY exchange **Proximity Proofs**:

```
PROXIMITY PROOF EXCHANGE:

Entity A                                    Entity B
   │                                           │
   │  [Physical proximity detected]            │
   │  [Bluetooth/WiFi-Direct/NFC range]        │
   │                                           │
   │  PROXIMITY_REQUEST                        │
   │  [my_tit, timestamp, nonce]               │
   │──────────────────────────────────────────►│
   │                                           │
   │  PROXIMITY_PROOF                          │
   │  [your_tit, my_tit, timestamp,            │
   │   my_location, signature_B]               │
   │◄──────────────────────────────────────────│
   │                                           │
   │  PROXIMITY_PROOF                          │
   │  [my_tit, your_tit, timestamp,            │
   │   my_location, signature_A]               │
   │──────────────────────────────────────────►│
   │                                           │
   
Both entities now have signed proof that:
"Entity X attests that Entity Y was nearby at time T"
```

```
ProximityProof = {
    witness: TIT,           // Who is attesting
    subject: TIT,           // Who was seen
    timestamp: u64,
    location: H3Cell,       // Where (witness's location)
    proximity_type: u8,     // BLE=1, WiFi=2, NFC=3, DSRC=4
    signal_strength: i8,    // RSSI (optional)
    signature: [u8; 64]     // Witness's signature
}
```

**Trust Multiplier:** A trajectory backed by multiple unique peer witnesses earns trust significantly faster than an isolated trajectory:

| Peer Attestations | Trust Multiplier |
|-------------------|------------------|
| 0 (self-only) | 1x |
| 1-5 unique peers | 2x |
| 6-20 unique peers | 5x |
| 21+ unique peers | 10x |

**Why this works:** An attacker would need to physically colocate multiple devices OR compromise multiple independent entities. This dramatically increases attack cost.

**PILLAR C: Anchor-Point Verification (Network Attestation)**

Relays and **Fixed Facets** (smart traffic lights, retail gateways, charging stations, toll booths) act as **Trajectory Anchors**:

```
ANCHOR-POINT ATTESTATION:

Mobile Entity                              Fixed Anchor
(@drone_7)                                 (@charging_station_42)
   │                                           │
   │  [Enters radio range of anchor]           │
   │                                           │
   │  ANCHOR_REQUEST                           │
   │  [my_tit, my_breadcrumb]                  │
   │──────────────────────────────────────────►│
   │                                           │
   │                    ┌──────────────────────┤
   │                    │ Verify: Is entity    │
   │                    │ actually in my       │
   │                    │ radio range?         │
   │                    └──────────────────────┤
   │                                           │
   │  ANCHOR_ATTESTATION                       │
   │  [entity_tit, anchor_tit,                 │
   │   anchor_location (fixed, known),         │
   │   timestamp, signature_anchor]            │
   │◄──────────────────────────────────────────│
   │                                           │
```

```
AnchorAttestation = {
    entity: TIT,            // Who was present
    anchor: TIT,            // The fixed anchor
    anchor_location: H3Cell,// Known fixed location
    anchor_type: u8,        // Relay=1, Retail=2, Traffic=3, etc.
    timestamp: u64,
    signature: [u8; 64]     // Anchor's signature
}
```

**Why this works:** Fixed anchors have known, verified locations. Their attestation proves the entity was physically present within radio range of a specific coordinate. This is like getting a passport stamp — verifiable, timestamped proof of presence.

#### 13.2.3 Entropy of Movement

Real physical movement contains "sensor noise" and micro-deviations. Simulated trajectories often follow perfect geometric lines or repeating loops.

```
REAL vs SIMULATED TRAJECTORY:

REAL (High Entropy):
    ┌─────────────────────────────────────┐
    │     *   *                           │
    │   *       *  *                      │
    │  *          *   *    *              │
    │               *   * *   *           │
    │                       *    *        │
    │                          *   *      │
    └─────────────────────────────────────┘
    Natural variation, accelerations, pauses

SIMULATED (Low Entropy):
    ┌─────────────────────────────────────┐
    │ *                                   │
    │   *                                 │
    │     *                               │
    │       *                             │
    │         *                           │
    │           *                         │
    └─────────────────────────────────────┘
    Perfect line, constant velocity, no noise
```

TRIP verification nodes MAY use **Entropy Analysis** to distinguish scripted paths from real journeys:

- Velocity variance (real movement accelerates/decelerates)
- Heading changes (real paths have natural curves)
- Temporal patterns (real entities pause, stop, reverse)
- Spatial noise (GPS has natural jitter)

#### 13.2.4 Verification Mask

To support layered verification, TRIP messages include a `verification_mask` indicating which pillars have been satisfied:

```
Verification Mask (8 bits):
┌───┬───┬───┬───┬───┬───┬───┬───┐
│ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │
└───┴───┴───┴───┴───┴───┴───┴───┘
  │   │   │   │   │   │   │   │
  │   │   │   │   │   │   │   └── GPS_ONLY (self-attested)
  │   │   │   │   │   │   └────── TEMPORAL_VERIFIED
  │   │   │   │   │   └────────── PEER_VOUCHED (1+ peer)
  │   │   │   │   └────────────── PEER_VOUCHED_MULTI (5+ peers)
  │   │   │   └────────────────── ANCHOR_VOUCHED
  │   │   └────────────────────── ANCHOR_VOUCHED_MULTI (3+ anchors)
  │   └────────────────────────── HARDWARE_ATTESTED (secure enclave)
  └────────────────────────────── ENTROPY_VERIFIED
```

Recipients can make trust decisions based on verification quality:

```
TRUST DECISION MATRIX:

Verification Level              Trust Granted
────────────────────────────────────────────────────
GPS_ONLY                        Minimal (sus)
TEMPORAL_VERIFIED               Basic
+ PEER_VOUCHED                  Standard
+ ANCHOR_VOUCHED                High
+ PEER_VOUCHED_MULTI            Very High
+ HARDWARE_ATTESTED             Maximum
```

### 13.3 Privacy vs Auditability

A major concern with "Identity as Trajectory" is potential for permanent surveillance. TRIP addresses this via **Differential Resolution**:

```
┌─────────────────────────────────────────────────────────────────┐
│                    DIFFERENTIAL RESOLUTION                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   PUBLIC LAYER (Epoch Proofs):                                  │
│   ════════════════════════════                                  │
│   H3 Resolution 5 (~250 km²)                                    │
│   Proves: "Entity was in Northern Italy"                        │
│   Does NOT reveal: Specific street or building                  │
│                                                                 │
│   STANDARD LAYER (Trust Verification):                          │
│   ═════════════════════════════════════                         │
│   H3 Resolution 7 (~5 km²)                                      │
│   Proves: "Entity was in this district"                         │
│   Shared with: Trust Level 1+ peers                             │
│                                                                 │
│   PRIVATE LAYER (Operational):                                  │
│   ════════════════════════════                                  │
│   H3 Resolution 10+ (~0.01 km²)                                 │
│   Full precision for operational needs                          │
│   Shared with: Trust Level 3+ or same Facet only                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

| Layer | H3 Resolution | Area | Use Case |
|-------|---------------|------|----------|
| Public | 5 | ~250 km² | Epoch proofs (anti-Sybil) |
| Standard | 7 | ~5 km² | Peer verification |
| Private | 10+ | ~0.01 km² | Fleet management, delivery |

### 13.4 Trust Decay Algorithm

TRIP identities are MOVING patterns — they fade without activity. This ensures abandoned or stolen devices eventually lose trust.

```
TRUST DECAY MODEL:

trust_score = base_trust × activity_multiplier × time_decay

Where:
  base_trust = f(epoch_count, peer_attestations, anchor_attestations)
  
  activity_multiplier:
    - Active (breadcrumbs in last 24h): 1.0
    - Recent (breadcrumbs in last 7d): 0.9
    - Idle (breadcrumbs in last 30d): 0.7
    - Dormant (no breadcrumbs 30-90d): 0.4
    - Inactive (no breadcrumbs 90d+): 0.1
    
  time_decay = 0.99^(days_since_last_breadcrumb)
```

**Decay Timeline:**

| Inactivity Period | Trust Multiplier | Effect |
|-------------------|------------------|--------|
| 0-24 hours | 1.0 | Full trust |
| 1-7 days | 0.9 | Slight reduction |
| 7-30 days | 0.7 | Noticeable reduction |
| 30-90 days | 0.4 | Significant reduction |
| 90+ days | 0.1 | Near-zero trust |
| 180+ days | Review | Manual reactivation may be required |

**Why this matters:**
- Stolen device → Thief can't maintain trajectory → Trust decays
- Abandoned device → No movement → Trust decays
- Compromised key → Attacker must physically move device → Expensive

### 13.5 Sybil Resistance Analysis

**Without Three Pillars (GPS only):**
```
Attack: Run GPS emulator on server
Cost: ~$0
Result: Unlimited fake identities
Status: VULNERABLE
```

**With Three Pillars:**
```
Attack: Must have physical devices that:
  1. Move at plausible speeds (Temporal)
  2. Encounter real TRIP entities (Peer Attestation)
  3. Pass fixed infrastructure (Anchor Verification)
  4. Show natural movement entropy

Cost per identity:
  - Physical device: $100+
  - Operator/robot: $$$
  - Time: Weeks
  - Must physically move in real world

Cost for 1000 Sybils: >$100,000 + massive coordination
Status: RESISTANT
```

### 13.6 Hardware Attestation (Future)

For highest security applications, TRIP MAY leverage hardware attestation:

- **Secure Enclave:** Private key never leaves hardware
- **Trusted Execution Environment (TEE):** Breadcrumb signing in isolated environment
- **Hardware Security Module (HSM):** For relay/anchor nodes

This prevents key extraction even if device is compromised.

---

## 14. ULissy Language Bindings

### 14.1 Overview

ULissy — "A language for moving machines" — provides native TRIP support.

### 14.2 Identity Declaration

```ulissy
// Human identity
identity me = Keychain.primary
me.handle           // @alice
me.trajectory       // Movement history
me.trustLevel       // Current trust (0-4)

// Machine identity
identity drone = Keychain.device("drone_7")
drone.handle        // @drone_7
drone.trajectory    // Flight path history
drone.trustLevel    // Device trust level

// All identities have the same properties:
// - publicKey (TVI)
// - hit (TIT)
// - stellarAddress
// - trajectory
// - trustLevel
```

### 14.3 Cross-Entity Communication

```ulissy
// Human verifies drone delivery
when drone.nearby(me, within: 10.meters) {
    // Both have trajectories proving physical presence
    let proof = sign(packageId, me)
    send via drone { 
        type: "delivery_confirmed",
        signature: proof
    }
}

// Drone-to-drone coordination
identity leader = lookupHandle(@drone_leader)
when leader.trustLevel >= 3 {
    follow(leader.trajectory)
}
```

### 14.4 Fleet Management

```ulissy
// Fleet operator vouches for new vehicle
identity fleet_operator = Keychain.primary  // Trust level 3
identity new_truck = Keychain.device("truck_new")

vouch(for: new_truck) {
    expires: 365.days,
    reason: "Fleet vehicle #47"
}
// new_truck now has trust level 4 (vouched)
```

---

## 15. Implementation Guidelines

### 15.1 Required Components

A compliant TRIP implementation MUST support:

1. **Identity:** TVI generation, TIT derivation, signatures
2. **Trajectory:** Breadcrumb collection, epoch publishing
3. **Handshake:** I1, R1, I2, R2 with trajectory proof
4. **Encryption:** ChaCha20-Poly1305 session encryption
5. **Trust:** Trust level computation from trajectory

### 15.2 Entity Type Considerations

| Entity | Breadcrumb Source | Typical Interval | Notes |
|--------|-------------------|------------------|-------|
| Human (phone) | GPS/WiFi/Cell | 10-60 minutes | Battery-conscious |
| Drone | Flight controller | 1-10 minutes | High precision |
| Vehicle | Onboard GPS | 5-30 minutes | Route-based |
| Robot | Embedded sensors | Varies | Task-dependent |
| IoT device | Attached GPS | Hours | Low power |

### 15.3 Reference Implementation

See `reference/` directory for Rust implementation.

---

## 16. References

### Standards

- RFC 7401: Host Identity Protocol Version 2 (HIP)
- RFC 8032: Edwards-Curve Digital Signature Algorithm (EdDSA)
- RFC 7748: Elliptic Curves for Security (X25519)
- RFC 8439: ChaCha20 and Poly1305 for IETF Protocols
- RFC 5869: HMAC-based Key Derivation Function (HKDF)

### Related Work

- GNS Protocol Specification
- ULissy Language Whitepaper
- H3: Uber's Hexagonal Hierarchical Spatial Index

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| **TVI** | Trajectory-Verified Identity — Ed25519 public key anchored by trajectory |
| **TIT** | Trajectory Identity Tag — 128-bit hash of TVI (HIP-compatible) |
| **Breadcrumb** | Single signed location proof |
| **Epoch** | Collection of 100+ breadcrumbs, published as proof |
| **Trajectory** | Complete movement history — the identity itself |
| **Trust Level** | 0-4 score based on trajectory history |
| **Handle** | Human-readable name (@username) |
| **Facet** | Derived contextual identity |

---

## Appendix B: The Core Insight

```
╔═════════════════════════════════════════════════════════════════╗
║                                                                 ║
║   "IP asks WHERE you are.                                       ║
║    HIP asks WHAT key you have.                                  ║
║    TRIP asks HOW you move.                                      ║
║                                                                 ║
║    Only movement proves existence."                             ║
║                                                                 ║
╠═════════════════════════════════════════════════════════════════╣
║                                                                 ║
║   "You are not what you HAVE (a key).                           ║
║    You are what you DO (your trajectory through the world).     ║
║                                                                 ║
║    A key proves possession.                                     ║
║    A trajectory proves EXISTENCE."                              ║
║                                                                 ║
╠═════════════════════════════════════════════════════════════════╣
║                                                                 ║
║   "Static identity: I have a passport.                          ║
║    Dynamic identity: I have a life — a path through space-time."║
║                                                                 ║
╠═════════════════════════════════════════════════════════════════╣
║                                                                 ║
║              If it MOVES, it can have a TRIP identity.          ║
║                                                                 ║
║              The trajectory is the proof.                       ║
║              The journey is the identity.                       ║
║                                                                 ║
╚═════════════════════════════════════════════════════════════════╝
```

---

**Document Version:** 1.1.0-draft
**Last Updated:** January 2025
**Authors:** GNS Foundation
**License:** CC BY 4.0
