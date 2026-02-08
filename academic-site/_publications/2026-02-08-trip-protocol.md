---
title: "TRIP: Trajectory-based Recognition of Identity Proof"
collection: publications
permalink: /publication/2026-02-trip-protocol
excerpt: 'IETF Internet-Draft specifying a decentralized protocol for establishing claims of physical-world presence through cryptographically signed, spatially quantized location attestations. Introduces a Criticality Engine grounded in statistical physics for distinguishing biological movement from synthetic trajectories.'
date: 2026-02-08
venue: 'IETF Internet-Draft (Individual Submission)'
paperurl: 'https://datatracker.ietf.org/doc/draft-ayerbe-trip-protocol/'
citation: 'Ayerbe Posada, C. (2026). &quot;TRIP: Trajectory-based Recognition of Identity Proof.&quot; <i>Internet-Draft draft-ayerbe-trip-protocol-01</i>, IETF.'
---

## Abstract

This document specifies the Trajectory-based Recognition of Identity Proof (TRIP) protocol, a decentralized mechanism for establishing claims of physical-world presence through cryptographically signed, spatially quantized location attestations called "breadcrumbs." Breadcrumbs are chained into an append-only log, bundled into verifiable epochs, and distilled into a Trajectory Identity Token (TIT) that serves as a persistent pseudonymous identifier.

Revision -01 introduces a formal trust-scoring framework grounded in statistical physics. A Criticality Engine evaluates the Power Spectral Density (PSD) of movement trajectories for the 1/f signature characteristic of biological Self-Organized Criticality (SOC). A mobility model based on truncated Lévy flights and Markov anchor transition matrices enforces known constraints of human movement. A six-component Hamiltonian energy function detects anomalies in real time.

## Links

- **IETF Datatracker:** [draft-ayerbe-trip-protocol](https://datatracker.ietf.org/doc/draft-ayerbe-trip-protocol/)
- **Full Text (HTML):** [draft-ayerbe-trip-protocol-01](https://www.ietf.org/archive/id/draft-ayerbe-trip-protocol-01.html)
- **Full Text (TXT):** [draft-ayerbe-trip-protocol-01.txt](https://www.ietf.org/archive/id/draft-ayerbe-trip-protocol-01.txt)
- **Diff from -00:** [iddiff](https://author-tools.ietf.org/iddiff?url2=draft-ayerbe-trip-protocol-01)
- **IPR Disclosure:** [#7153](https://datatracker.ietf.org/ipr/7153/)
- **GitHub Repository:** [GNS-Foundation/trip-protocol](https://github.com/GNS-Foundation/trip-protocol)

## RATS Alignment

TRIP maps to the RATS Architecture (RFC 9334):

| RATS Role | TRIP Component |
|-----------|---------------|
| Attester | TRIP-enabled device producing breadcrumbs |
| Evidence | Individual breadcrumbs and epoch records |
| Verifier | Criticality Engine |
| Attestation Results | Proof-of-Humanity Certificate |
| Relying Party | Any service accepting PoH Certificates |

## Key References

- Parisi, G. (2021). Nobel Prize in Physics — scale-free correlations in complex systems
- Cavagna, A. et al. (2010). "Scale-free correlations in starling flocks." *PNAS*
- Barabási, A.-L. et al. (2008). "Understanding individual human mobility patterns." *Nature*
- Song, C. et al. (2010). "Limits of Predictability in Human Mobility." *Science*
