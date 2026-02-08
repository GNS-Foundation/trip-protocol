# TRIP Protocol Message Formats

**Version:** 1.0.0-draft

This document specifies the binary format for all TRIP protocol messages.

---

## 1. Common Header

All TRIP messages begin with a 56-byte header:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|    Version    |   Msg Type    |             Flags             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Sender HIT                            +
|                          (16 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                       Recipient HIT                           +
|                          (16 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                        Message ID                             +
|                          (8 bytes)                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Timestamp                             +
|                          (8 bytes)                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Payload Length                         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
~                           Payload                             ~
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Signature                             +
|                          (64 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Field Descriptions

| Field | Size | Description |
|-------|------|-------------|
| Version | 1 byte | Protocol version (0x01 for v1.0) |
| Msg Type | 1 byte | Message type code |
| Flags | 2 bytes | Message flags (see below) |
| Sender HIT | 16 bytes | Sender's Human Identity Tag |
| Recipient HIT | 16 bytes | Recipient's Human Identity Tag |
| Message ID | 8 bytes | Unique message identifier |
| Timestamp | 8 bytes | Unix timestamp in milliseconds |
| Payload Length | 4 bytes | Length of payload in bytes |
| Payload | variable | Message-specific payload |
| Signature | 64 bytes | Ed25519 signature over header + payload |

### Flags

```
 0                   1
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|E|R|A|P| Reserved              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

E = Encrypted (payload is encrypted)
R = Relay (message is being relayed)
A = Acknowledgment requested
P = Priority message
```

---

## 2. Message Types

### 2.1 Type Registry

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
| 0x50 | PAY_REQUEST | Payment | Request payment |
| 0x51 | PAY_APPROVE | Payment | Approve payment |
| 0x52 | PAY_COMPLETE | Payment | Payment confirmed |
| 0x53 | PAY_REJECT | Payment | Payment rejected |
| 0xF0 | ERROR | Control | Error response |
| 0xF1 | NOTIFY | Control | Notification |

---

## 3. Handshake Messages

### 3.1 I1 (Initiate)

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                       Initiator HI                            +
|                          (32 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Req Trust     |   Reserved    |          Capabilities         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Capabilities                          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                           Nonce                               +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

| Field | Size | Description |
|-------|------|-------------|
| Initiator HI | 32 bytes | Initiator's full Ed25519 public key |
| Req Trust | 1 byte | Requested minimum trust level (0-4) |
| Reserved | 1 byte | Reserved for future use |
| Capabilities | 4 bytes | Capability flags bitmap |
| Nonce | 16 bytes | Random nonce for freshness |

### 3.2 R1 (Challenge)

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                       Responder HI                            +
|                          (32 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Grant Trust   | Req Proof     |          Capabilities         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Capabilities                          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                      DH Ephemeral Key                         +
|                          (32 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                          Challenge                            +
|                          (32 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                       Initiator Nonce                         +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                       Responder Nonce                         +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

| Field | Size | Description |
|-------|------|-------------|
| Responder HI | 32 bytes | Responder's full Ed25519 public key |
| Grant Trust | 1 byte | Trust level willing to grant |
| Req Proof | 1 byte | Required proof type (see below) |
| Capabilities | 4 bytes | Capability flags bitmap |
| DH Ephemeral Key | 32 bytes | X25519 ephemeral public key |
| Challenge | 32 bytes | Challenge for proof binding |
| Initiator Nonce | 16 bytes | Echo of I1 nonce |
| Responder Nonce | 16 bytes | New random nonce |

**Required Proof Types:**
- 0x00: None (trust already sufficient)
- 0x01: Epoch proof
- 0x02: Trajectory proof (recent breadcrumbs)
- 0x03: Vouch proof

### 3.3 I2 (Response)

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                      DH Ephemeral Key                         +
|                          (32 bytes)                           |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Proof Type    |   Reserved    |          Proof Length         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
~                          Proof Data                           ~
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                       Responder Nonce                         +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

| Field | Size | Description |
|-------|------|-------------|
| DH Ephemeral Key | 32 bytes | X25519 ephemeral public key |
| Proof Type | 1 byte | Type of proof provided |
| Reserved | 1 byte | Reserved |
| Proof Length | 2 bytes | Length of proof data |
| Proof Data | variable | Proof (epoch/trajectory/vouch) |
| Responder Nonce | 16 bytes | Echo of R1 nonce |

### 3.4 R2 (Confirm)

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Session ID                            +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Grant Trust   |   Reserved    |       Session Lifetime        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       Session Lifetime                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

| Field | Size | Description |
|-------|------|-------------|
| Session ID | 16 bytes | Unique session identifier |
| Grant Trust | 1 byte | Actual trust level granted |
| Reserved | 1 byte | Reserved |
| Session Lifetime | 4 bytes | Session lifetime in seconds |

---

## 4. Session Messages

### 4.1 DATA

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Session ID                            +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                          Sequence                             +
|                          (8 bytes)                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                           Nonce                               +
|                          (12 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
~                        Ciphertext                             ~
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                       Auth Tag                                +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

| Field | Size | Description |
|-------|------|-------------|
| Session ID | 16 bytes | Session identifier |
| Sequence | 8 bytes | Message sequence number |
| Nonce | 12 bytes | ChaCha20-Poly1305 nonce |
| Ciphertext | variable | Encrypted application data |
| Auth Tag | 16 bytes | Poly1305 authentication tag |

---

## 5. Mobility Messages

### 5.1 UPDATE

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Session ID                            +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                          Sequence                             +
|                          (8 bytes)                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Locator Type  | Loc Count     |           Reserved            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
~                          Locators                             ~
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### 5.2 Locator Format

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |   Priority    |           Lifetime            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|            Length             |                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+                               +
|                                                               |
~                         Address Data                          ~
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

**Locator Types:**
- 0x00: IPv4 (4 bytes address + 2 bytes port)
- 0x01: IPv6 (16 bytes address + 2 bytes port)
- 0x02: Relay (16 bytes HIT + variable host + 2 bytes port)
- 0x03: QUIC (address + 2 bytes port + 20 bytes connection ID)

---

## 6. Payment Messages

### 6.1 PAY_REQUEST

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Invoice ID                            +
|                          (16 bytes)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                           Amount                              +
|                          (8 bytes, stroops)                   |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Asset Code Length             |                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+                               +
|                         Asset Code                            |
~                        (max 12 bytes)                         ~
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                        Asset Issuer                           +
|                          (32 bytes, if not native)            |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Memo Length                   |                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+                               +
|                            Memo                               |
~                         (max 28 bytes)                        ~
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                           Expiry                              +
|                          (8 bytes)                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

---

## 7. Error Codes

| Code | Name | Description |
|------|------|-------------|
| 0x00 | SUCCESS | No error |
| 0x01 | INVALID_FORMAT | Invalid message format |
| 0x02 | INVALID_SIGNATURE | Signature verification failed |
| 0x03 | UNKNOWN_HIT | Unknown Human Identity Tag |
| 0x04 | INSUFFICIENT_TRUST | Trust level too low |
| 0x05 | PROOF_FAILED | Proof verification failed |
| 0x06 | SESSION_NOT_FOUND | Session does not exist |
| 0x07 | SESSION_EXPIRED | Session has expired |
| 0x08 | RATE_LIMITED | Rate limit exceeded |
| 0x09 | REPLAY_DETECTED | Replay attack detected |
| 0x0A | DECRYPTION_FAILED | Unable to decrypt |
| 0x0B | INVALID_STATE | Invalid state transition |
| 0x0C | RESOURCE_EXHAUSTED | Resources exhausted |
| 0x0D | NOT_REGISTERED | Not registered with relay |
| 0x0E | HANDLE_TAKEN | Handle already claimed |
| 0x0F | PAYMENT_FAILED | Payment operation failed |

---

## 8. Capability Flags

```
Bit 0:  RELAY       - Can route messages for others
Bit 1:  PAYMENT     - Supports payment channel
Bit 2:  VOUCH       - Can vouch for others
Bit 3:  FACETS      - Supports facet identities
Bit 4:  QUIC        - Supports QUIC transport
Bit 5:  IPV6        - Supports IPv6
Bit 6:  DHT         - Supports DHT discovery
Bit 7:  MOBILE      - Mobile-optimized
Bits 8-31: Reserved
```

---

## 9. Encoding

- All multi-byte integers are big-endian
- Strings are UTF-8 encoded with length prefix
- Public keys are raw 32-byte Ed25519 format
- Signatures are raw 64-byte Ed25519 format
- HITs are raw 16-byte format (not hex encoded)

---

## 10. Test Vector Reference

See [test-vectors/messages.json](../test-vectors/messages.json) for encoded test vectors.
