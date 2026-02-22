---
title: "TRIP: Trajectory-based Recognition of Identity Proof"
abbrev: "TRIP"
category: info

docname: draft-ayerbe-trip-protocol-latest
submissiontype: independent
consensus: false
area: "Security"
workgroup: RATS
v: 3

author:
  -
    fullname: Camilo Ayerbe Posada
    ins: C. Ayerbe Posada
    organization: ULISSY s.r.l.
    street: Via Gaetano Sacchi 16
    city: Roma
    region: RM
    code: "00153"
    country: Italy
    email: cayerbe@gmail.com
  -
    fullname: "Muhammad Usama Sardar"
    ins: M. Usama Sardar
    organization: TU Dresden
    city: Dresden
    country: Germany
    email: "muhammad_usama.sardar@tu-dresden.de"

normative:
  RFC8032:
  RFC8949:
  RFC9334:

informative:
  H3:
    title: "H3: Uber's Hexagonal Hierarchical Spatial Index"
    author:
      - org: Uber Technologies
    target: https://h3geo.org/
    date: 2023
  PARISI-NOBEL:
    title: "Nobel Prize in Physics 2021: Giorgio Parisi"
    author:
      - org: The Nobel Foundation
    target: https://www.nobelprize.org/prizes/physics/2021/parisi/facts/
    date: 2021
  CAVAGNA-STARLINGS:
    title: "Scale-free correlations in starling flocks"
    author:
      - name: A. Cavagna
      - name: A. Cimarelli
      - name: I. Giardina
      - name: G. Parisi
      - name: R. Santagati
      - name: F. Stefanini
      - name: M. Viale
    seriesinfo:
      DOI: 10.1073/pnas.1005766107
    target: https://doi.org/10.1073/pnas.1005766107
    date: 2010
  BALLERINI-TOPOLOGICAL:
    title: "Interaction ruling animal collective behavior depends on topological rather than metric distance"
    author:
      - name: M. Ballerini
      - name: N. Cabibbo
      - name: R. Candelier
      - name: A. Cavagna
      - name: E. Cisbani
      - name: I. Giardina
      - name: V. Lecomte
      - name: A. Orlandi
      - name: G. Parisi
      - name: A. Procaccini
      - name: M. Viale
      - name: V. Zdravkovic
    seriesinfo:
      DOI: 10.1073/pnas.0711437105
    target: https://doi.org/10.1073/pnas.0711437105
    date: 2008
  BARABASI-MOBILITY:
    title: "Understanding individual human mobility patterns"
    author:
      - name: M.C. Gonzalez
      - name: C.A. Hidalgo
      - name: A.-L. Barabasi
    seriesinfo:
      DOI: 10.1038/nature06958
    target: https://doi.org/10.1038/nature06958
    date: 2008
  SONG-LIMITS:
    title: "Limits of Predictability in Human Mobility"
    author:
      - name: C. Song
      - name: Z. Qu
      - name: N. Blumm
      - name: A.-L. Barabasi
    seriesinfo:
      DOI: 10.1126/science.1177170
    target: https://doi.org/10.1126/science.1177170
    date: 2010
  VADAI-GPS:
    title: "Spectral Analysis of Fluctuations in Humans' Daily Motion"
    author:
      - name: G. Vadai
      - name: A. Antal
      - name: Z. Gingl
    seriesinfo:
      DOI: 10.1142/S0219477519500287
    target: https://doi.org/10.1142/S0219477519500287
    date: 2019
  MACZAK-SPECTRAL:
    title: "General spectral characteristics of human activity"
    author:
      - name: B. Maczak
    seriesinfo:
      DOI: 10.1038/s41598-024-54165-4
    target: https://doi.org/10.1038/s41598-024-54165-4
    date: 2024
  I-D.fossati-seat-expat:

keyword:
  - trajectory
  - identity
  - proof-of-humanity
  - breadcrumb
  - attestation
  - RATS
  - criticality

--- abstract

This document specifies the Trajectory-based Recognition of
Identity Proof (TRIP) protocol, a decentralized mechanism for
establishing claims of physical-world presence through
cryptographically signed, spatially quantized location
attestations called "breadcrumbs." Breadcrumbs are chained into
an append-only log, bundled into verifiable epochs, and
distilled into a Trajectory Identity Token (TIT) that serves
as a persistent pseudonymous identifier.

The protocol employs a Criticality Engine grounded in
statistical physics to distinguish biological movement from
synthetic trajectories. Power Spectral Density (PSD) analysis
detects the 1/f signature of Self-Organized Criticality in
human mobility through the PSD scaling exponent alpha. A
six-component Hamiltonian energy function scores each
breadcrumb against the identity's learned behavioral profile
in real time.

This revision (-03) addresses three areas identified through
expert review by researchers in the statistical physics
community: it replaces informal terminology with standard
spectral analysis nomenclature; it provides the analytical
and numerical bridge between the Levy flight displacement
exponent and the PSD scaling exponent; and it introduces a
convergence analysis framework for quantifying the minimum
trajectory length required for reliable single-trajectory
classification. Additionally, this revision removes Passive
Verification mode entirely, requiring all Attestation Results
to be bound to Relying Party nonces via the Active
Verification Protocol.

TRIP is designed to be transport-agnostic and operates
independently of any particular naming system, blockchain,
or application layer.

--- middle

# Introduction

Conventional approaches to proving that an online actor
corresponds to a physical human being rely on biometric
capture, government-issued documents, or knowledge-based
challenges. Each technique introduces a centralized trust
anchor, creates honeypots of personally identifiable
information (PII), and is susceptible to replay or deepfake
attacks.

TRIP takes a fundamentally different approach: it treats
sustained physical movement through the real world as evidence
of embodied existence. A TRIP-enabled device periodically
records its position as a "breadcrumb" -- a compact,
privacy-preserving, cryptographically signed attestation that
the holder of a specific Ed25519 key pair was present in a
particular spatial cell at a particular time. An adversary who
controls only digital infrastructure cannot fabricate a
plausible trajectory because doing so requires controlling
radio-frequency environments (GPS, Wi-Fi, cellular, IMU) at
many geographic locations over extended periods.

This document specifies the data structures, algorithms,
and verification procedures that constitute the TRIP
protocol. It intentionally omits transport bindings,
naming-system integration, and blockchain anchoring, all
of which are expected to be addressed in companion
specifications.

## Requirements Language

{::boilerplate bcp14-tagged}

## Terminology {#terminology}

Terms defined in the RATS Architecture {{RFC9334}}
(Attester, Evidence, Verifier, Attestation Result, Relying
Party) are used throughout this document with their RFC 9334
meanings. Additional terms specific to TRIP:

Breadcrumb:
: A single, signed attestation of spatiotemporal presence.
  The atomic unit of TRIP Evidence.

Trajectory:
: An ordered, append-only chain of breadcrumbs produced by
  a single identity key pair.

Epoch:
: A bundle of breadcrumbs (default 100) sealed with a
  Merkle root, forming a verifiable checkpoint.

Trajectory Identity Token (TIT):
: A pseudonymous identifier derived from an Ed25519 public
  key paired with trajectory metadata.

Criticality Engine:
: The analytical subsystem that evaluates trajectory
  statistics for signs of biological Self-Organized Criticality
  (SOC). In RATS terms, the Criticality Engine is a component
  of the Verifier.

PSD Scaling Exponent (alpha):
: The slope of the Power Spectral Density of the
  displacement time series in log-log space. This quantity is
  referred to in the spectral analysis literature as the
  "spectral exponent" or "scaling exponent." Human mobility
  produces alpha values in the range \[0.30, 0.80\],
  corresponding to 1/f pink noise -- the spectral signature
  of systems operating at criticality, as demonstrated by
  Parisi's work on scale-free correlations in biological
  systems {{PARISI-NOBEL}}.

Hamiltonian (H):
: A weighted energy function that quantifies how much a
  new breadcrumb deviates from the identity's learned
  behavioral profile.

Anchor Cell:
: An H3 cell where an identity has historically spent
  significant time (e.g., home, workplace).

Flock:
: The set of co-located TRIP entities whose aggregate
  movement provides a reference signal for alignment
  verification.

Proof-of-Humanity (PoH) Certificate:
: A compact Attestation Result containing only statistical
  exponents derived from the trajectory, with no raw location
  data.

# Breadcrumb Data Structure {#breadcrumb}

A breadcrumb is encoded as a CBOR map {{RFC8949}}
with the following fields:

| Key | CBOR Type | Description |
|-----|-----------|-------------|
| 0 | uint | Index (sequence number) |
| 1 | bstr (32) | Identity public key (Ed25519) |
| 2 | uint | Timestamp (Unix seconds) |
| 3 | uint | H3 cell index |
| 4 | uint | H3 resolution (7-10) |
| 5 | bstr (32) | Context digest (SHA-256) |
| 6 | bstr (32) / null | Previous block hash |
| 7 | map | Meta flags |
| 8 | bstr (64) | Ed25519 signature |
{: #breadcrumb-fields title="Breadcrumb CBOR Fields"}

## Spatial Quantization {#spatial-quantization}

The H3 geospatial indexing system {{H3}}
partitions the Earth's surface into hexagonal cells at
multiple resolutions. TRIP employs resolutions 7 through 10:

| Resolution | Avg. Area | Edge Length | Use Case |
|------------|-----------|-------------|----------|
| 7 | ~5.16 km^2 | ~1.22 km | Rural / low-density |
| 8 | ~0.74 km^2 | ~0.46 km | Suburban / general |
| 9 | ~0.11 km^2 | ~0.17 km | Urban / high-density |
| 10 | ~0.015 km^2 | ~0.07 km | Default / standard verification |
{: #h3-resolutions title="H3 Resolution Parameters"}

A conforming implementation MUST quantize raw GPS coordinates
to an H3 cell before any signing or storage operation. Raw
coordinates MUST NOT appear in breadcrumbs or in any protocol
message transmitted between TRIP entities.

H3 resolution is a configurable protocol parameter.
Implementations SHOULD default to resolution 10. Deployments
MAY select alternative resolutions based on jurisdictional
requirements, population density, and use-case sensitivity.
Lower resolutions (larger cells) provide stronger location
privacy at the cost of reduced spatial discrimination for
trust computation.

## Context Digest Computation {#context-digest}

The context digest binds ambient environmental signals to
the breadcrumb without revealing them. The digest is
computed as follows:

1. Construct a pipe-delimited string of tagged components
   in the following order:

   - "h3:" followed by the H3 cell hex string
   - "ts:" followed by the timestamp bucketed to 5-minute
     intervals (floor(Unix_minutes / 5) * 5)
   - "wifi:" followed by the first 16 hex characters of
     SHA-256(sorted comma-joined BSSIDs), if Wi-Fi scan data
     is available
   - "cell:" followed by the first 16 hex characters of
     SHA-256(sorted comma-joined tower IDs), if cellular data
     is available
   - "imu:" followed by the first 16 hex characters of
     SHA-256(IMU vector string), if inertial sensor data is
     available

2. Compute SHA-256 over the UTF-8 encoding of the resulting
   string.

Absent components MUST be omitted entirely, not represented
as empty strings.

## Signature Production {#signature-production}

The signable payload is the deterministic CBOR encoding (per
Section 4.2 of {{RFC8949}}) of a CBOR map
containing fields 0 through 7, with map keys sorted in
ascending integer order. The Ed25519 signature
{{RFC8032}} is computed over the raw bytes of
this CBOR encoding and stored at key 8.

~~~
signable_payload = CBOR-Deterministic(fields[0..7])
signature        = Ed25519-Sign(private_key, signable_payload)
~~~

Deterministic CBOR encoding ensures that any conforming
implementation produces identical byte sequences for the same
logical content, which is essential for reproducible signature
verification across heterogeneous platforms.

## Block Hash and Chaining {#block-hash}

The block hash is the SHA-256 digest of the complete
deterministic CBOR encoding of the breadcrumb (fields 0
through 8 inclusive, i.e., including the signature):

~~~
BreadcrumbHash(B) = SHA-256(CBOR-Deterministic(B[0..8]))
B[N+1].field[6]   = BreadcrumbHash(B[N])
B[0].field[6]     = null
~~~

Each breadcrumb at index > 0 MUST carry the block hash of
its immediate predecessor in field 6, forming an append-only
hash chain. The genesis breadcrumb (index 0) MUST set
field 6 to null (CBOR simple value 22).

# Chain Management {#chain-management}

## Location Deduplication {#deduplication}

Proof-of-Trajectory requires demonstrated movement. A
conforming implementation MUST reject a breadcrumb if the H3
cell is identical to the immediately preceding breadcrumb.
Implementations SHOULD also enforce a cap (default 10) on the
number of breadcrumbs recordable at any single H3 cell to
prevent stationary farming.

## Minimum Collection Interval {#min-interval}

Breadcrumbs SHOULD be collected at intervals of no less than
15 minutes. An implementation MAY allow shorter intervals
during explicit "exploration" sessions but MUST NOT accept
intervals shorter than 5 minutes.

## Chain Verification {#chain-verification}

A Verifier MUST check:

1. Index values form a contiguous sequence starting at 0.
2. Timestamps are monotonically non-decreasing.
3. Each previousHash matches the block hash of the prior
   breadcrumb.
4. Each Ed25519 signature verifies against the identity
   public key and the canonical signed data.

# Epochs {#epochs}

An epoch seals a batch of breadcrumbs (default 100) under a
Merkle root. The epoch record is a CBOR map containing:

| Key | Type | Description |
|-----|------|-------------|
| 0 | uint | Epoch number |
| 1 | bstr (32) | Identity public key |
| 2 | uint | First breadcrumb index |
| 3 | uint | Last breadcrumb index |
| 4 | uint | Timestamp of first breadcrumb |
| 5 | uint | Timestamp of last breadcrumb |
| 6 | bstr (32) | Merkle root of breadcrumb hashes |
| 7 | uint | Count of unique H3 cells |
| 8 | bstr (64) | Ed25519 signature over fields 0-7 |
{: #epoch-fields title="Epoch CBOR Fields"}

The Merkle tree MUST use SHA-256 and a canonical left-right
ordering of breadcrumb block hashes. An epoch is sealed when
the breadcrumb count reaches the epoch size threshold.

# Trajectory Identity Token (TIT) {#tit}

A TIT is the externally presentable identity derived from a
TRIP trajectory. It consists of:

- The Ed25519 public key (32 bytes).
- The current epoch count.
- The total breadcrumb count.
- The count of unique H3 cells visited.
- A trust score (see {{trust-scoring}}).

A TIT SHOULD be encoded as a CBOR map for machine consumption
and MAY additionally be represented as a Base64url string for
URI embedding.

# The Criticality Engine {#criticality-engine}

The Criticality Engine is the core analytical component of the
TRIP Verifier. It evaluates whether a trajectory exhibits the
statistical signature of biological Self-Organized Criticality
(SOC) -- the phenomenon where living systems operate at the
boundary between order and chaos, producing scale-free
correlations that are mathematically distinct from synthetic or
automated movement.

The theoretical foundation rests on three pillars:

First, Parisi's demonstration {{PARISI-NOBEL}}
that flocking organisms such as starling murmurations exhibit
scale-free correlations {{CAVAGNA-STARLINGS}}
where perturbations propagate across the entire group regardless
of size. Crucially, Ballerini et al. showed that these
interactions are topological (based on nearest k neighbors)
rather than metric (based on distance)
{{BALLERINI-TOPOLOGICAL}}. TRIP exploits this
through Power Spectral Density analysis ({{psd-analysis}}): human
movement produces characteristic 1/f pink noise that synthetic
trajectories cannot replicate.

Second, Barabasi et al.'s discovery
{{BARABASI-MOBILITY}} that human displacement
follows truncated Levy flights with approximately 93%
predictability {{SONG-LIMITS}}. TRIP learns each
identity's mobility profile -- displacement distribution,
anchor transition patterns, and circadian rhythms -- and
detects deviations from these learned baselines ({{mobility}}).

Third, a six-component Hamiltonian energy function ({{hamiltonian}})
that combines spatial, temporal, kinetic, flock-alignment,
contextual, and structural analysis into a single anomaly score
for each incoming breadcrumb. The Hamiltonian provides real-time
detection while the PSD and mobility statistics provide
aggregate trajectory assessment.

## Power Spectral Density Analysis {#psd-analysis}

The primary diagnostic is the Power Spectral Density (PSD) of
the displacement time series. Given a trajectory of N
breadcrumbs with displacements d(i) between consecutive
breadcrumbs, the PSD is computed via the Discrete Fourier
Transform:

~~~
S(f) = |DFT(d)|^2

where d = [d(0), d(1), ..., d(N-1)]
and d(i) = haversine_distance(cell(i), cell(i-1))
~~~

The PSD is then fitted to a power-law model:

~~~
S(f) ~ 1 / f^alpha
~~~

The exponent alpha is the PSD scaling exponent -- referred to
in the spectral analysis literature as the "spectral exponent"
or "scaling exponent." In the context of human mobility, this
quantity captures the degree of long-range temporal correlation
in movement patterns. The theoretical significance of this
exponent derives from Parisi's work demonstrating that
biological systems operating at criticality produce
characteristic scale-free correlations
{{PARISI-NOBEL}}. The PSD scaling exponent
is the critical diagnostic:

| Alpha Range | Noise Type | Classification |
|-------------|-----------|----------------|
| 0.00 - 0.15 | White noise | Synthetic / automated script |
| 0.15 - 0.30 | Near-white | Suspicious (possible sophisticated bot) |
| 0.30 - 0.80 | Pink noise (1/f) | Biological / human |
| 0.80 - 1.20 | Near-brown | Suspicious (possible replay with drift) |
| 1.20+ | Brown noise | Drift anomaly / sensor failure |
{: #alpha-ranges title="PSD Scaling Exponent Classification"}

A conforming implementation MUST compute the PSD scaling
exponent over a sliding window of the most recent 64
breadcrumbs (minimum) to 256 breadcrumbs (recommended).
The alpha value MUST fall within \[0.30, 0.80\] for the
trajectory to be classified as biological.

The key insight is that automated movement generators lack
the long-range temporal correlations ("memory") inherent in
a system operating at criticality. A random walk produces
white noise (alpha near 0). A deterministic replay produces
brown noise (alpha near 2). Only a biological system
operating at the critical point produces pink noise in the
characteristic \[0.30, 0.80\] range.

NOTE: The alpha range \[0.30, 0.80\] is a protocol-specified
classification boundary constructed from combined literature.
The boundaries are informed by empirical studies demonstrating
1/f-like spectral properties in human GPS trajectories
{{VADAI-GPS}} and general spectral
characteristics of human physical activity
{{MACZAK-SPECTRAL}}. Deployments MAY adjust
these boundaries based on population-specific calibration
data, provided that the biological range remains centered
near alpha = 0.55 and excludes the white noise (alpha <
0.15) and brown noise (alpha > 1.20) regions.

## Criticality Confidence Score {#criticality-confidence}

The Criticality Confidence is a value in \[0, 1\] computed
from the PSD scaling exponent and the goodness-of-fit
(R-squared) of the power-law regression:

~~~
alpha_score = 1.0 - |alpha - 0.55| / 0.25

criticality_confidence = alpha_score * R_squared

where:
  0.55 is the center of the biological range
  0.25 is the half-width of the biological range
  R_squared is the coefficient of determination of the
    log-log linear regression
~~~

A criticality_confidence below 0.5 SHOULD trigger elevated
monitoring. A value below 0.3 SHOULD flag the trajectory for
manual review or additional verification challenges.

## Levy-PSD Bridge {#levy-psd-bridge}

This section establishes the mathematical relationship
between the truncated Levy flight displacement exponent beta
({{levy-flights}}) and the PSD scaling exponent alpha
({{psd-analysis}}). Previous revisions of this specification
asserted both as independent properties of human mobility.
This section demonstrates that they are related through the
spectral properties of heavy-tailed random processes.

### Analytical Relationship {#analytical-relationship}

For a stationary stochastic process with displacement
increments drawn from a symmetric stable distribution with
stability index mu (where mu = beta - 1 for the Levy
flight exponent beta used in {{levy-flights}}), the Power
Spectral Density of the cumulative displacement series
scales as:

~~~
S(f) ~ f^{-alpha}

where alpha = 2 - mu = 2 - (beta - 1) = 3 - beta

For pure (non-truncated) Levy flights.
~~~

However, human displacement follows TRUNCATED Levy flights
with an exponential cutoff at distance kappa
({{levy-flights}}). The truncation modifies the spectral
relationship: at low frequencies (long time scales), the
exponential cutoff causes the process to resemble
Brownian motion (alpha approaching 2), while at high
frequencies (short time scales), the pure Levy scaling
dominates. For the intermediate frequency range relevant
to TRIP's sliding window (64-256 breadcrumbs collected
at 15-minute intervals, spanning approximately 16-64
hours of movement data), the effective PSD scaling
exponent is:

~~~
alpha_eff ~ (3 - beta) * g(N, kappa, delta_t)

where:
  beta    = Levy flight exponent (typically 1.50 - 1.90)
  g(...)  = correction factor for truncation and finite window
  N       = number of breadcrumbs in the analysis window
  kappa   = truncation distance (km)
  delta_t = mean inter-breadcrumb interval

For typical human values:
  beta  ~ 1.75  =>  3 - beta = 1.25
  g(...)  ~ 0.4 - 0.6  (empirically observed)
  alpha_eff ~ 0.50 - 0.75

This falls squarely within the biological range [0.30, 0.80].
~~~

### Empirical Evidence {#empirical-evidence}

The analytical relationship above is supported by empirical
studies:

- Vadai et al. {{VADAI-GPS}} analyzed GPS
  trajectory data and demonstrated 1/f-like spectral
  characteristics in human daily motion, with PSD scaling
  exponents consistent with the predicted range.

- Maczak et al. {{MACZAK-SPECTRAL}} studied
  42 human subjects and found spectral exponents close to
  1 in general physical activity data, confirming the
  presence of long-range temporal correlations consistent
  with Self-Organized Criticality.

- The original Levy flight analysis by Gonzalez, Hidalgo,
  and Barabasi {{BARABASI-MOBILITY}} reported
  beta values of approximately 1.75 +/- 0.15 across a
  population of 100,000 mobile phone users, which through
  the bridge equation predicts alpha\_eff in \[0.40, 0.70\]
  -- consistent with the biological classification range.

### Numerical Validation {#numerical-validation}

Implementers SHOULD validate the Levy-PSD bridge for their
specific deployment by conducting Monte Carlo simulations:

1. Generate 10,000 synthetic trajectories using truncated
   Levy flights with beta drawn uniformly from \[1.50, 1.90\]
   and kappa drawn from a log-normal distribution matching
   the target population.
2. Quantize each trajectory to H3 resolution 10 and apply
   the deduplication rules of {{deduplication}}.
3. Compute the PSD scaling exponent alpha for each
   synthetic trajectory.
4. Verify that the (beta, alpha) pairs fall within the
   expected relationship with the correction factor g in the
   range \[0.3, 0.7\].

Additionally, generate control trajectories from:

- Pure random walks (expected: alpha near 0)
- Deterministic replays of recorded trajectories
  (expected: alpha near 2)
- Correlated random walks with Gaussian increments
  (expected: alpha outside \[0.30, 0.80\])

The Monte Carlo validation confirms that the \[0.30, 0.80\]
classification boundary correctly separates biological
from synthetic trajectories with quantifiable error rates
(see {{convergence-analysis}}).

## Convergence Analysis {#convergence-analysis}

The PSD scaling exponent, Levy flight parameters, and flock
alignment metrics are fundamentally ensemble properties
derived from statistical physics. Applying them to a single
trajectory raises the question: how many breadcrumbs are
required for these ensemble properties to converge on an
individual trajectory with a given confidence level?

This section provides guidance on convergence behavior.
Definitive false positive and false negative rates require
empirical validation against real-world datasets (e.g.,
GeoLife, MDC), which is planned for a companion
publication. The framework below describes the expected
convergence properties and the protocol's mitigation
strategies.

### Convergence Regimes {#convergence-regimes}

The reliability of TRIP's statistical classifiers depends
on trajectory length. Three regimes are identified:

| Breadcrumbs | Regime | PSD Reliability | Levy Fit Reliability |
|-------------|--------|-----------------|---------------------|
| 0 - 63 | Bootstrap | Not computed (insufficient data for DFT) | Not reliable |
| 64 - 199 | Provisional | Computed but with wide confidence intervals; alpha estimate variance ~ 0.15 | Beta estimated but kappa poorly constrained |
| 200+ | Stable | Alpha estimate variance < 0.05; R-squared meaningful | Both beta and kappa well-constrained |
{: #convergence-table title="Convergence Regimes"}

The trust scoring formula ({{trust-scoring}}) incorporates
profile maturity through the factor
min(breadcrumb\_count / 200, 1.0), which scales Hamiltonian
weights during the bootstrap and provisional regimes.

### Composition of Independent Tests {#composition-defense}

TRIP does not rely on any single statistical test. The
six-component Hamiltonian ({{hamiltonian}}) combines independent
classifiers: spatial Levy fit, temporal Markov properties,
kinetic transition analysis, flock alignment, IMU
cross-correlation, and chain structural integrity. Even if
each individual test has a significant error rate on a
short trajectory, the composition of independent tests
reduces the combined error probability.

For k independent tests each with false positive rate p\_i,
the probability that a synthetic trajectory passes ALL
tests simultaneously is:

~~~
P(false_positive_all) = product(p_i, i=1..k)

For k = 6 tests each with p_i = 0.1 (conservative):
P(false_positive_all) = 0.1^6 = 10^{-6}
~~~

In practice the tests are not perfectly independent
(spatial and kinetic components share displacement data),
so the actual combined false positive rate will be higher
than the product bound. Empirical measurement is required.

### Error Cost Asymmetry {#cost-asymmetry}

TRIP's classification errors have asymmetric costs:

- **False negative** (human classified as bot):
  Low cost. The identity accumulates more breadcrumbs and is
  reclassified correctly as the trajectory lengthens.
  No permanent damage occurs.

- **False positive** (bot classified as human):
  Higher cost, but requires simultaneous spoofing across
  all six Hamiltonian components -- spatial displacement
  statistics, temporal circadian patterns, Markov transition
  probabilities, flock alignment, IMU cross-correlation,
  and chain timing regularity. This represents a
  significantly harder adversarial problem than defeating
  any single test.

### Minimum Breadcrumbs for Classification {#minimum-breadcrumbs}

Based on the convergence analysis above, the minimum
trajectory lengths for classification decisions are:

- **64 breadcrumbs**: Minimum for PSD
  computation. Sufficient for preliminary screening
  (reject obvious bots) but not for positive human
  classification.

- **100 breadcrumbs**: Minimum for handle
  claiming ({{trust-scoring}}). The Levy fit becomes usable
  and the Markov transition matrix begins to stabilize.

- **200 breadcrumbs**: RECOMMENDED for
  reliable positive human classification. At this length,
  the PSD alpha estimate has variance below 0.05 and the
  Levy parameters are well-constrained.

- **256+ breadcrumbs**: Sufficient for
  high-confidence classification suitable for
  high-stakes Relying Party decisions.

Determining precise false positive and false negative
rates at each breadcrumb count requires empirical
validation. Implementers SHOULD conduct the Monte Carlo
simulations described in {{numerical-validation}} and test against
publicly available human mobility datasets to establish
ROC curves and confidence intervals for their specific
deployment parameters.

# Mobility Statistics {#mobility}

This section defines the mobility model that enforces known
constraints of human movement, as established by Barabasi et al.
{{BARABASI-MOBILITY}}.

## Truncated Levy Flights {#levy-flights}

Human displacement between consecutive recorded locations
follows a truncated power-law distribution:

~~~
P(delta_r) ~ delta_r^(-beta) * exp(-delta_r / kappa)

where:
  delta_r = displacement distance (km)
  beta    = power-law exponent (typically 1.50 - 1.90)
  kappa   = exponential cutoff distance (km)
~~~

The exponent beta captures the heavy-tailed nature of human
movement: most displacements are short (home to office) but
occasional long jumps (travel) follow a predictable
distribution. The cutoff kappa is learned per identity and
represents the characteristic maximum range.

A conforming implementation MUST maintain a running estimate
of beta and kappa for each identity by fitting the
displacement histogram using maximum likelihood estimation
over the most recent epoch (100 breadcrumbs).

A new displacement that falls outside the 99.9th percentile
of the fitted distribution MUST increment the spatial
anomaly counter.

The relationship between beta and the PSD scaling exponent
alpha is established in {{levy-psd-bridge}}. Implementations
SHOULD verify internal consistency between the fitted beta
value and the observed alpha value; a discrepancy exceeding
the expected range of the correction factor g ({{analytical-relationship}})
MAY indicate data quality issues or adversarial manipulation
of one metric.

## Trajectory Predictability {#predictability}

Research has demonstrated that approximately 93% of human
movement is predictable based on historical patterns
{{SONG-LIMITS}}. TRIP exploits this by
maintaining a Markov Transition Matrix over anchor cells:

~~~
T[a_i][a_j] = count(transitions from a_i to a_j)
               / count(all departures from a_i)

where a_i, a_j are anchor cells.
~~~

An anchor cell is defined as any H3 cell where the identity
has recorded 5 or more breadcrumbs. The transition matrix is
rebuilt at each epoch boundary.

The predictability score Pi for an identity is the fraction
of observed transitions that match the highest-probability
successor in the Markov matrix. Human identities converge
toward Pi values in the range \[0.80, 0.95\] after
approximately 200 breadcrumbs. Deviations below 0.60 are
anomalous.

## Circadian and Weekly Profiles {#circadian}

The implementation SHOULD maintain two histogram profiles:

- A circadian profile C\[hour\] recording the probability of
  activity in each hour of the day (24 bins).
- A weekly profile W\[day\] recording the probability of
  activity on each day of the week (7 bins).

These profiles provide the temporal baseline for the
Hamiltonian temporal energy component ({{h-temporal}}).

# The Six-Component Hamiltonian {#hamiltonian}

To assess each incoming breadcrumb, the Criticality Engine
computes a weighted energy score H that quantifies how much
the breadcrumb deviates from the identity's learned behavioral
profile. High energy indicates anomalous behavior; low energy
indicates normalcy.

~~~
H = w_1 * H_spatial
  + w_2 * H_temporal
  + w_3 * H_kinetic
  + w_4 * H_flock
  + w_5 * H_contextual
  + w_6 * H_structure
~~~

| Component | Weight | Diagnostic Target |
|-----------|--------|-------------------|
| H_spatial | 0.25 | Displacement anomalies (teleportation) |
| H_temporal | 0.20 | Circadian rhythm violations |
| H_kinetic | 0.20 | Anchor transition improbability |
| H_flock | 0.15 | Misalignment with local human flow |
| H_contextual | 0.10 | Sensor cross-correlation failure |
| H_structure | 0.10 | Chain integrity and timing regularity |
{: #hamiltonian-weights title="Hamiltonian Component Weights"}

Weights are modulated by the profile maturity m, defined as
min(breadcrumb\_count / 200, 1.0). During the bootstrap phase
(m < 1.0), all weights are scaled by m, widening the
acceptance threshold for new identities.

## H_spatial: Displacement Anomaly {#h-spatial}

Given the identity's fitted truncated Levy distribution
P(delta\_r), the spatial energy for a displacement delta\_r
is the negative log-likelihood (surprise):

~~~
H_spatial = -log(P(delta_r))

where P(delta_r) = C * delta_r^(-beta) * exp(-delta_r / kappa)
and C is the normalization constant.
~~~

Typical displacements yield H\_spatial near the identity's
historical baseline. A displacement that exceeds the
identity's learned kappa cutoff by more than a factor of 3
produces an H\_spatial value in the CRITICAL range.

## H_temporal: Rhythm Anomaly {#h-temporal}

~~~
H_temporal = -log(C[current_hour]) - log(W[current_day])
~~~

Activity at 3:00 AM for an identity with a 9-to-5 circadian
profile yields high H\_temporal. Activity at 8:00 AM on a
Tuesday for the same identity yields low H\_temporal.

## H_kinetic: Transition Anomaly {#h-kinetic}

~~~
from_anchor = nearest anchor to previous breadcrumb
to_anchor   = nearest anchor to current breadcrumb
H_kinetic   = -log(max(T[from_anchor][to_anchor], epsilon))

where epsilon = 0.001 (floor to prevent log(0))
~~~

A home-to-office transition at 8:00 AM yields low H\_kinetic.
An office-to-unknown-city transition yields high H\_kinetic.

## H_flock: Topological Alignment {#h-flock}

Inspired by Parisi's finding that starlings track their k
nearest topological neighbors (k approximately 6-7) rather
than all birds within a metric radius
{{BALLERINI-TOPOLOGICAL}}, the flock energy
measures alignment between the identity's velocity vector
and the aggregate velocity of co-located TRIP entities.

~~~
v_self  = displacement vector of current identity
v_flock = mean displacement vector of k nearest
          co-located identities (k = 7)

alignment = dot(v_self, v_flock)
            / (|v_self| * |v_flock|)

H_flock = 1.0 - max(alignment, 0)
~~~

When flock data is unavailable (sparse network or privacy
constraints), the implementation SHOULD fall back to comparing
the current velocity against the identity's own historical
velocity distribution at the same location and time-of-day.

H\_flock defeats GPS replay attacks: an adversary replaying a
previously recorded trajectory will find that the ambient
flock has changed since the recording, producing a
misalignment signal.

## H_contextual: Sensor Cross-Correlation {#h-contextual}

~~~
H_contextual = divergence(
  observed_imu_magnitude,
  expected_imu_magnitude_for(gps_displacement)
)
~~~

Implementations that lack IMU access MUST set H\_contextual = 0
and SHOULD increase the weights of other components
proportionally.

## H_structure: Chain Structural Integrity {#h-structure}

- Inter-breadcrumb timing regularity: excessively uniform
  intervals suggest automation.
- Hash chain continuity: any break in the chain produces
  maximum H\_structure.
- Phase-space smoothness: the velocity-acceleration phase
  portrait of a human trajectory traces smooth loops, while
  bots produce either chaotic blobs or tight limit cycles.

## Alert Classification {#alert-classification}

The total Hamiltonian H maps to an alert level. The baseline
H\_baseline is the rolling median of the identity's own recent
energy values, making the threshold self-calibrating per
identity:

| H Range | Level | Action |
|---------|-------|--------|
| \[0, H\_baseline * 1.5) | NOMINAL | Normal operation |
| \[H\_baseline * 1.5, 3.0) | ELEVATED | Increase sampling frequency, log |
| \[3.0, 5.0) | SUSPICIOUS | Flag for review, require reconfirmation |
| \[5.0, infinity) | CRITICAL | Freeze trust score, trigger challenge |
{: #alert-levels title="Hamiltonian Alert Levels"}

# Proof-of-Humanity Certificate {#poh-certificate}

A PoH Certificate is a compact, privacy-preserving Attestation
Result (in the RATS sense) asserting that an identity has
demonstrated biological movement characteristics. It contains
ONLY statistical exponents derived from the trajectory -- no
raw location data, no GPS coordinates, no cell identifiers.

The certificate is encoded as a CBOR map:

| Key | Type | Description |
|-----|------|-------------|
| 0 | bstr (32) | Identity public key |
| 1 | uint | Issuance timestamp |
| 2 | uint | Epoch count at issuance |
| 3 | float | PSD scaling exponent alpha |
| 4 | float | Levy beta exponent |
| 5 | float | Levy kappa cutoff (km) |
| 6 | float | Predictability score Pi |
| 7 | float | Criticality confidence |
| 8 | float | Trust score T |
| 9 | uint | Unique cell count |
| 10 | uint | Total breadcrumb count |
| 11 | uint | Validity duration (seconds) |
| 12 | bstr (16) | Relying Party nonce (REQUIRED) |
| 13 | bstr (32) | Chain head hash at issuance (REQUIRED) |
| 14 | bstr (64) | Verifier Ed25519 signature |
{: #poh-fields title="PoH Certificate CBOR Fields"}

Fields 12 and 13 are REQUIRED in all PoH Certificates. Every
certificate MUST be issued in response to an Active
Verification request ({{replay-protection}}). There is no passive issuance
mode.

A Relying Party receiving a PoH Certificate can verify:

1. The Verifier signature (field 14) is valid against a trusted
   Verifier public key.
2. The PSD scaling exponent alpha (field 3) falls within
   \[0.30, 0.80\].
3. The criticality confidence (field 7) exceeds the Relying
   Party's policy threshold.
4. The trust score (field 8) meets application
   requirements.
5. The certificate has not expired (field 1 + field 11 >
   current time).
6. The nonce (field 12) matches the Relying Party's original
   challenge.
7. The chain head hash (field 13) provides freshness
   binding.

The certificate reveals NOTHING about where the identity has
been -- only that it has moved through the world in a manner
statistically consistent with a biological organism.

# Trust Scoring {#trust-scoring}

~~~
T = 0.40 * min(breadcrumb_count / 200, 1.0)
  + 0.30 * min(unique_cells / 50, 1.0)
  + 0.20 * min(days_since_first / 365, 1.0)
  + 0.10 * chain_integrity

chain_integrity = 1.0 if chain verification passes, else 0.0
T is expressed as a percentage in [0, 100].
~~~

The threshold for claiming a handle (binding a human-readable
name to a TIT) requires breadcrumb\_count >= 100 and T >= 20.

A trajectory that fails the criticality test (alpha outside
\[0.30, 0.80\]) MUST have its trust score capped at 50,
regardless of other factors.

# RATS Architecture Mapping {#rats-mapping}

TRIP implements the Remote ATtestation procedureS (RATS)
architecture defined in {{RFC9334}}. This section
provides the normative mapping between TRIP components and
RATS roles.

## Role Mapping {#role-mapping}

| RATS Role | TRIP Component | Description |
|-----------|---------------|-------------|
| Attester | TRIP-enabled mobile device | Collects breadcrumbs, signs them with the identity Ed25519 private key, chains them into the append-only trajectory log, and transmits H3-quantized Evidence to the Verifier. |
| Evidence | Breadcrumbs and epoch records | H3-quantized spatiotemporal claims including cell identifiers, timestamps, context digests, chain hashes, and Ed25519 signatures. Evidence is transmitted from Attester to Verifier. |
| Verifier | Criticality Engine | Receives Evidence, performs chain verification, computes PSD scaling exponents, fits Levy flight parameters, evaluates the six-component Hamiltonian, and produces Attestation Results. |
| Attestation Result | PoH Certificate and trust score | Contains only statistical exponents (alpha, beta, kappa) and aggregate scores. No raw Evidence (cell IDs, timestamps, chain hashes) is included in the Attestation Result. |
| Relying Party | Any service consuming PoH Certificates | Evaluates the Attestation Result against its own policy. Does not receive or process raw Evidence. |
{: #rats-roles title="TRIP-to-RATS Role Mapping"}

## Evidence Flow {#evidence-flow}

H3-quantized Evidence is transmitted from the Attester to
the Verifier. This is an explicit design choice: the Verifier
requires access to the full breadcrumb chain to compute PSD
scaling exponents, fit Levy flight parameters, and evaluate
the Hamiltonian.

Privacy preservation derives from the H3 quantization
transform applied by the Attester before any data leaves the
device, NOT from data locality. Raw GPS coordinates MUST NOT
be transmitted. The quantization transform is lossy and
irreversible.

The Verifier MUST NOT forward raw Evidence to Relying
Parties. Only the Attestation Result (PoH Certificate) is
disclosed to Relying Parties.

## Verifier Trust Model {#verifier-trust}

The Relying Party MUST trust the Verifier that produced the
Attestation Result. The TRIP protocol supports multiple
independent Verifiers. An Attester MAY submit Evidence to
more than one Verifier. A Relying Party MAY accept
Attestation Results from any Verifier it trusts.

Each Verifier MUST have its own Ed25519 key pair. The
Verifier signs PoH Certificates with its private key
(field 14). Relying Parties verify this signature against
the Verifier's published public key.

# Replay Protection and Active Verification {#replay-protection}

TRIP provides replay protection at two distinct layers:
protection of the Evidence chain against tampering, and
protection of Attestation Results against replay to Relying
Parties. All Attestation Results MUST be issued via the Active
Verification Protocol described in this section.

## Chain-Level Replay Protection {#chain-replay}

The monotonically increasing index and the chaining via the
previous block hash field provide replay protection within a
single trajectory. A replayed breadcrumb will fail the chain
integrity check. Cross-trajectory replay will fail Ed25519
signature verification.

## Active Verification Protocol {#active-verification}

The Active Verification Protocol provides cryptographic
freshness guarantees by binding the Attestation Result to a
Relying Party-supplied nonce, the current chain head, and the
current time. This is the ONLY verification mode supported
by TRIP. There is no passive verification mode.

The protocol proceeds as follows:

1. The Relying Party generates an unpredictable nonce
   (RECOMMENDED: 16 bytes from a cryptographically secure
   random number generator) and sends a Verification Request
   to the Verifier:

   ~~~
   VerificationRequest = {
     0 => bstr .size 32,   ; identity public key
     1 => bstr .size 16,   ; nonce
     2 => uint,            ; request timestamp
     3 => uint,            ; requested freshness window (seconds)
   }
   ~~~

2. The Verifier delivers a Liveness Challenge to the
   Attester via a real-time channel (e.g., WebSocket push,
   push notification):

   ~~~
   LivenessChallenge = {
     0 => bstr .size 16,   ; nonce (from Relying Party)
     1 => bstr .size 32,   ; verifier identity (public key)
     2 => uint,            ; challenge timestamp
     3 => uint,            ; response deadline (seconds)
   }
   ~~~

3. The Attester constructs and signs a Liveness Response
   binding the nonce to the current chain state:

   ~~~
   LivenessResponse = {
     0 => bstr .size 16,   ; nonce echo
     1 => bstr .size 32,   ; chain_head_hash
     2 => uint,            ; response timestamp
     3 => uint,            ; current breadcrumb index
     4 => bstr .size 64,   ; Ed25519 signature over fields 0-3
   }
   ~~~

4. The Verifier validates the Liveness Response by
   checking:

   - The Ed25519 signature (field 4) is valid against the
     identity's public key over fields 0-3.
   - The nonce echo (field 0) matches the original
     Verification Request.
   - The chain\_head\_hash (field 1) is consistent with the
     Verifier's stored trajectory state.
   - The response timestamp (field 2) is within the
     deadline.
   - The breadcrumb index (field 3) matches or exceeds
     the Verifier's last known index.

5. Upon successful validation, the Verifier produces a fresh
   PoH Certificate with field 12 set to the nonce and field 13
   set to the chain\_head\_hash, signs it, and returns it to the
   Relying Party.

6. The Relying Party verifies the PoH Certificate per
   {{poh-certificate}}, confirming that field 12 matches its original
   nonce.

If the Attester does not respond within the deadline, the
Verifier MUST return an error. The Verifier MUST NOT issue a
PoH Certificate without a valid Liveness Response.

## Active Verification CDDL {#active-cddl}

~~~
; Active Verification Protocol CDDL Schema

verification-request = {
  0 => bstr .size 32,        ; identity_key
  1 => bstr .size 16,        ; nonce
  2 => uint,                 ; request_timestamp
  3 => uint,                 ; freshness_window_seconds
}

liveness-challenge = {
  0 => bstr .size 16,        ; nonce
  1 => bstr .size 32,        ; verifier_key
  2 => uint,                 ; challenge_timestamp
  3 => uint,                 ; response_deadline_seconds
}

liveness-response = {
  0 => bstr .size 16,        ; nonce_echo
  1 => bstr .size 32,        ; chain_head_hash
  2 => uint,                 ; response_timestamp
  3 => uint,                 ; current_breadcrumb_index
  4 => bstr .size 64,        ; ed25519_signature
}
~~~

# Protocol Design

We propose to use post-handshake attestation protocol {{I-D.fossati-seat-expat}}.
TODO: Further details to be discussed.

# Security Considerations {#security}

## GPS Replay Attacks {#gps-replay}

An adversary records a legitimate trajectory and replays the
GPS coordinates on a different device. TRIP detects this
through multiple channels:

- H\_flock: the ambient flock has changed since the
  recording.
- H\_contextual: unless the adversary also replays Wi-Fi
  BSSIDs, cellular tower IDs, and IMU data, the context digest
  will not match.
- H\_structure: the timing regularity of a replay is
  typically either too perfect or shifted in a detectable
  pattern.

## Synthetic Walk Generators {#synthetic-walk}

- PSD scaling exponent test: random walk generators produce
  white noise (alpha approximately 0). Brownian motion
  generators produce alpha approximately 2. Neither falls in
  the biological \[0.30, 0.80\] range.
- Levy flight fitting: synthetic displacements rarely match
  the truncated power-law distribution with biologically
  plausible beta and kappa values.
- Predictability test: synthetic trajectories either show
  near-zero predictability (random) or near-perfect
  predictability (scripted), both outside the human \[0.80,
  0.95\] range.

## Emulator Injection {#emulator}

An adversary runs the TRIP client on an Android/iOS emulator
with spoofed GPS. Detection relies on H\_contextual (emulators
lack real IMU data) and context digest (emulators lack real
Wi-Fi and cellular data).

## Device Strapping (Robot Dog Attack) {#robot-dog}

An adversary straps a phone to a mobile robot or drone. This
is the most sophisticated attack because it produces real GPS,
Wi-Fi, cellular, and IMU data from actual physical movement.
Mitigation relies on PSD analysis (robotic movement lacks 1/f
noise), phase-space smoothness (H\_structure), and circadian
profiles. This attack remains an active area of research.

## Verifier Compromise {#verifier-compromise}

A compromised Verifier could issue fraudulent PoH
Certificates. Mitigations: Relying Parties SHOULD accept
certificates from multiple independent Verifiers; key
rotation and revocation procedures SHOULD be established;
the Active Verification Protocol ensures even a compromised
Verifier cannot produce a valid certificate without the
Attester's cooperation.

## Denial of Service {#dos}

Verifiers SHOULD rate-limit requests per identity and per
Relying Party. The Active Verification Protocol's real-time
requirement provides an inherent rate limit on valid
completions.

## Statistical Classifier Limitations {#statistical-classifier-limits}

The Criticality Engine applies ensemble statistical properties
(PSD scaling exponent, Levy flight parameters, flock
alignment) to individual trajectories. As discussed in
{{convergence-analysis}}, the reliability of these classifiers depends on
trajectory length. Implementers MUST be aware that:

- Classification confidence is low during the bootstrap
  regime (fewer than 64 breadcrumbs) and moderate during the
  provisional regime (64-199 breadcrumbs).
- The alpha range \[0.30, 0.80\] is a protocol-specified
  boundary informed by, but not directly taken from, a single
  peer-reviewed source. Deployments SHOULD calibrate this range
  against population-specific data.
- The composition of six independent Hamiltonian components
  provides defense in depth, but the actual combined error rate
  depends on the degree of independence between components,
  which requires empirical measurement.
- Definitive ROC curves and confidence intervals require
  validation against real-world human mobility datasets. This
  validation is planned for a companion publication and is
  outside the scope of this protocol specification.

# Privacy Considerations {#privacy}

## Quantization-Based Privacy {#quantization-privacy}

TRIP's privacy model is based on lossy spatial quantization,
not on data locality. H3-quantized Evidence is transmitted
from the Attester to the Verifier. Raw GPS coordinates MUST
NOT be transmitted. At the default resolution 10, each cell
covers approximately 15,000 m^2, providing meaningful
ambiguity in populated areas.

## Verifier Data Handling {#verifier-data}

The Verifier MUST NOT forward raw Evidence to Relying Parties.
The Verifier MUST disclose its data retention policy. The
Verifier SHOULD retain only statistical aggregates and MAY
discard individual breadcrumbs after incorporation. The
Verifier MUST support data deletion where required by law.

## Relying Party Data Minimization {#rp-minimization}

The PoH Certificate reveals statistical exponents and
aggregate counts. A Relying Party does NOT learn which
cities or specific locations the identity has visited, the
identity's home or workplace, the identity's daily schedule,
or any raw trajectory data.

## Trajectory Correlation and Sybil Resistance {#sybil}

A single physical entity operating multiple TRIP identities
simultaneously constitutes a Sybil attack. TRIP raises the
cost: each identity requires a separate physical device,
weeks of sustained movement, and independent trajectory
accumulation. The H\_flock component provides detection of
co-located trajectories with identical displacement vectors.

## Population Density Considerations {#population-density}

In sparsely populated areas, even cell-level granularity may
narrow identification. Implementations SHOULD use lower
resolution in rural areas and MAY allow users to override to
a lower resolution at any time.

# Deployment Considerations {#deployment}

## Multiple Verifier Deployments {#multi-verifier}

Any entity that implements the verification procedures defined
in this specification MAY operate as a TRIP Verifier. An
Attester MAY submit Evidence to more than one Verifier.

## Verifier Interoperability {#verifier-interop}

All conforming Verifiers MUST implement chain integrity
verification, PSD scaling exponent classification, and the
PoH Certificate format. Two Verifiers processing the same
Evidence SHOULD produce consistent alpha, beta, and kappa
values within numerical precision bounds.

## Transport Binding {#transport-binding}

This specification does not mandate a specific transport.
Implementations MAY use HTTPS, WebSocket, CoAP, or any
transport providing confidentiality and integrity protection.
The Active Verification Protocol requires a real-time channel.

## Naming System Integration {#naming}

The binding of human-readable names to TRIP identities is
outside the scope of this specification and is expected to
be addressed in a companion document.

## Accessibility and Low-Mobility Users {#accessibility}

TRIP does not require geographic travel. It requires sustained
physical existence over time. A person who remains in a
single H3 cell generates a valid trajectory; the trust
scoring formula assigns 20% weight to temporal continuity
and 40% to breadcrumb count, both accumulating regardless
of spatial diversity. The context digest provides
environmental diversity even without movement. The
Hamiltonian is self-calibrating per identity. For stationary
users, implementations SHOULD supplement spatial PSD with
temporal PSD (analyzing inter-breadcrumb timing patterns).
Deployments MUST NOT impose minimum spatial diversity
requirements that would exclude users with mobility
limitations.

# IANA Considerations {#iana}

This document has no IANA actions at this time. Future
revisions may request CBOR tag assignments, a media type
registration for application/trip+cbor, and an entry in a
TRIP Verification Mode registry.

--- back

# Acknowledgements {#acknowledgements}

The TRIP protocol builds upon foundational work in cryptographic
identity systems, geospatial indexing, statistical physics, and
network science. The author thanks the contributors to the H3
geospatial system, the Ed25519 specification authors, and the
broader IETF community for establishing the standards that TRIP
builds upon. The Criticality Engine framework is inspired by the
work of Giorgio Parisi on scale-free correlations in biological
systems and Albert-Laszlo Barabasi on the fundamental limits of
human mobility.

The authors thank Jun Zhang for raising critical questions about
accessibility and the applicability of mobility models to users
with limited physical mobility, leading to the Accessibility and
Low-Mobility Users section.

The authors thank an anonymous reviewer from the statistical
physics community for identifying three critical issues addressed
in this revision: the need for standard spectral analysis
terminology, the missing analytical bridge between Levy flight
parameters and PSD scaling exponents, and the fundamental
question of applying ensemble properties to single trajectories.
These contributions led to Sections 6.3 and 6.4 and the new
Section 13.7 on statistical classifier limitations.

# History
{:numbered="false"}

Version -01 introduced a Criticality Engine grounded in
Giorgio Parisi's Nobel Prize-winning work on scale-free
correlations {{PARISI-NOBEL}} and
Albert-Laszlo Barabasi's research on the fundamental limits
of human mobility {{BARABASI-MOBILITY}}.

Version -02 formalized the mapping to the RATS Architecture
{{RFC9334}}, introduced the Active Verification
Protocol with cryptographic freshness guarantees, and corrected
the privacy model.

This revision (-03) addresses three substantive issues
identified through expert review by researchers working in
the statistical physics of complex systems:

First, it replaces the informal term "Parisi Factor" with
the standard spectral analysis term "PSD scaling exponent
alpha," properly attributing the theoretical foundation to
Parisi's work without conflating tribute with established
nomenclature.

Second, it provides the missing analytical and numerical
bridge between the Levy flight displacement exponent beta
({{levy-flights}}) and the PSD scaling exponent alpha
({{psd-analysis}}). Previous revisions asserted both as
independent properties of human mobility without
demonstrating their mathematical relationship.

Third, it introduces a convergence analysis framework
({{convergence-analysis}}) that addresses the fundamental question of
applying ensemble statistical properties to single
trajectories, including guidance on minimum trajectory
length and error rate estimation.

Additionally, this revision removes Passive Verification
mode entirely. All Attestation Results MUST now be bound
to Relying Party nonces via the Active Verification
Protocol ({{replay-protection}}), eliminating the replay vulnerability
identified in -02 review.


## Changes from -02 {#changes}
{:numbered="false"}

This section summarizes the substantive changes from
draft-ayerbe-trip-protocol-02:

- Replaced the term "Parisi Factor" with the standard
  spectral analysis term "PSD scaling exponent alpha"
  throughout the document. The theoretical contribution of
  Parisi's work is acknowledged in the motivation and
  terminology, not in the variable naming.

- Added {{levy-psd-bridge}} (Levy-PSD Bridge) providing the
  analytical relationship between the Levy flight displacement
  exponent beta and the PSD scaling exponent alpha, with
  supporting references to empirical studies
  {{MACZAK-SPECTRAL}} {{VADAI-GPS}}.

- Added {{convergence-analysis}} (Convergence Analysis) addressing the
  application of ensemble statistical properties to single
  trajectories, including minimum trajectory length guidance
  and error rate estimation framework.

- Removed Passive Verification mode entirely
  ({{replay-protection}}). All Attestation Results MUST now be produced
  via the Active Verification Protocol with Relying
  Party-supplied nonces. The PoH Certificate fields for
  nonce (field 12) and chain head hash (field 13) are now
  REQUIRED, not optional.

- Updated the PoH Certificate ({{poh-certificate}}) to reflect
  mandatory Active Verification fields.

- Added references to recent empirical studies on spectral
  properties of human GPS trajectories.
