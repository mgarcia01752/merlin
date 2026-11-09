---
name: docsis-signal-processing
description: Implement or review DOCSIS OFDM Channel Sim signal-processing and FEC behavior using local pinned PHY evidence, Rust-only algorithms and explicit scientific support states. OFDMA work is deferred unless requested.
---

# DOCSIS OFDM Channel Sim scientific scope

- DOCSIS OFDM Channel Sim is standalone and DOCSIS-centric. Prioritize downstream OFDM functionality.
  OFDMA is a later scope. Optional Qt presentation and Python bindings are deferred.
- All DSP, channel models, constellation mapping/demapping, FEC and scientific
  orchestration must be Rust. This includes algorithm dependencies. Do not use
  C/C++ codecs, native FFT/BLAS DSP wrappers or Python scientific sidecars.
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
- Apply public-api and scientific-testing. All scientific JSON fields are Rust
  models/calculations shared by CLI and future adapters.
