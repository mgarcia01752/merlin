> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/rust-only-signal-processing/`
> Logs, captures and specification images are not published.

# Standalone DOFT and Rust-only signal processing

User direction: standalone DOCSIS OFDM/OFDMA FEC toolkit, OFDM first. Remove
product documentation centered on an external application. Keep algorithms100%
Rust; optional Qt presentation and Python bindings follow simulation functionality.

## Implemented

- Replaced CXX/native BCH with safe Rust encoding, GF(2^14) syndromes,
  Berlekamp-Massey locator solving, Chien search and final syndrome verification.
  Field/generator tables are immutable, initialized once. Decoder failure returns
  the unmodified received decisions and zero claimed corrections.
- Removed native source/build script and CXX dependencies. Retained MIT licensing
  and algorithm provenance in the codec crate and NOTICE. Both simulation crates
  forbid unsafe code; ldpc-toolbox supplies Rust LDPC algorithms.
- Preserved the existing public codec signatures, noiseless/AWGN metrics and
  independently transcribed generator/parity-equation tests. Added varied
  correction positions and dense-error failure cases for the Rust implementation.
- Downstream data-order registry covers16,64,128,256,512,1024,2048,4096,8192,16384.
  Normative device support, profile analysis and coding-chain implementation state
  are separate. Only the initial16-QAM coding experiment is implemented.
- Removed the incorrect DOCSIS3.1 maximum13-bit restriction. Analytical generation
  defaults to14bits; explicit device caps remain available. NCP/pilot/PLC/zero
  assignments do not become data orders.
- Public-api and docsis-signal-processing skills have canonical directories and
  relative links in both discovery directories. Future Python adapters require
  typed models/errors, explicit ownership, cancellation and Rust-only algorithms;
  no binding dependency or UI work was added.

## Evidence

PHY4.0 I07 section7.3.5.5, PDF134: FDD mapping delegates to PHY3.1 section7.5.5.
PHY3.1 I20 section7.5.5.1, PDF120:16QAM and64..4096 required for CM/CMTS;
8192/16384 optional. Original PDF120 rendered and inspected locally; image retained
as phy-i20-p120.png. This verifies these pinned editions, not every later issue.
DOCSIS4.0 FDX remains a separate, unimplemented applicability scope.

## Remaining scope

Validate each higher-order mapper, soft demapper, padding/scrambling, column twist,
demultiplexing and codeword placement using exact evidence and independent vectors.
Do not turn registry recognition into a claim of complete simulation support.
Complete the channel-estimate-conditioned scenario model without double-counting
impairments already reflected in equalized RxMER. OFDMA remains deferred.

No changes outside DOFT, no commits/pushes and no system installation performed.
