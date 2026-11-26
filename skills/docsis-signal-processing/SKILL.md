---
name: docsis-signal-processing
description: Implement or review MERLIN downstream OFDM and explicitly scoped upstream OFDMA signal-processing/FEC work using pinned PHY evidence, safe Rust algorithms and explicit support states.
---

# DOCSIS OFDM Channel Sim simulation scope

- DOCSIS OFDM Channel Sim is standalone and DOCSIS-centric. Prioritize downstream OFDM functionality.
  Preserve the implemented downstream engine and its Qt/native Python adapters.
  Upstream runtime integration remains separate from migrated skill guidance;
  follow the user's explicit scope and reviewed architecture before source migration.
- All DSP, channel models, constellation mapping/demapping, FEC and simulation
  orchestration must be Rust. This includes algorithm dependencies. Do not use
  C/C++ codecs, native FFT/BLAS DSP wrappers or Python simulation sidecars.
- Use the versioned local PHY skills. Keep editions, sections/pages, delegation
  and CM/CMTS/FDD/FDX applicability explicit. Do not claim latest-revision or
  rendered-PDF verification without checking the relevant source.
- Target downstream data orders through16384-QAM/14bits per symbol. PHY4.0 I07
  section7.3.5.5 delegates FDD mapping to PHY3.1 I20 section7.5.5.1. The baseline
  already includes optional8192/16384-QAM. Do not incorrectly cap DOCSIS3.1 at8K.
- Keep normative device support, analytical profile support, mapper support and
  actual coding-chain support distinct. Consult docs/reference/ofdm-support.md
  and types::ofdm_modulation. Pending modes must return unsupported, never select
  a substitute constellation, interleaver or padding rule.
- Ideal/reference trials and measurement-driven trials share the same engine.
  The latter conditions on supplied RxMER, channel estimates and a profile;
  establish reference points before combining impairments to avoid double counting.
- Keep measured, estimated and simulated results separate. MER screening is not
  decoder reliability. No impulse-noise or temporal-correlation model follows
  from MER alone. Preserve individual frames, gaps and worst conditions.
- Verify non-square mappings and high-order scrambling/padding against local
  tables/figures. Tests need independent evidence, not only reversible transforms.
- Apply public-api and scientific-testing. All simulation JSON fields are Rust
  models/calculations shared by CLI and future adapters.

For field RxMER/profile screening, FEC margin interpretation and empirical
validation cases, use [docsis-pnm-rxmer-fec](../docsis-pnm-rxmer-fec/SKILL.md).
Its PNM guidance supplements the versioned PHY evidence; it does not replace it.

## Upstream OFDMA work

Use the [canonical unified owner map](../../docs/architecture.md#unified-backend-integration)
for local paths: existing downstream `crates/ofdm`, existing common `crates/fec-codec`,
upstream `crates/ofdma` and existing `crates/dsp` (`merlin-dsp`). The latter
owns centered unitary 2K/4K/8K transforms and the one-step PRBS15 recurrence;
directional geometry, errors, source identity/state and verification remain in
the PHY wrappers. See [DSP acceptance](../../docs/development/features/signal-processing/validation.md).
Do not infer upstream stage acceptance from shared-kernel tests. The stage sections
identify the directional leaf modules/tests. Source paths in retained historical
contracts are provenance, not MERLIN dependencies; use the canonical owner map
and current integration evidence for callable paths and acceptance. The retained
INT-DOC-02 record documents the original migration reconciliation.

For explicitly scoped upstream planning, implementation or review, read the
[upstream scientific contracts](references/upstream.md) and the
[upstream test procedure](../scientific-testing/references/upstream.md).
They retain distinct codec definitions, processing order, pilot-estimated C×H,
Pre-Eq semantics and receiver separation from simulator truth. These are guidance
contracts, not a claim that upstream stages have landed in MERLIN.
