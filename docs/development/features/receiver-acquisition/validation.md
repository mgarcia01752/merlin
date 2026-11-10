# WAVE-RX-05 validation

Host: Ubuntu22.04 x64; pinned Rust/Cargo1.85.1. No remote CI, commits, pushes,
releases, installers or system changes were performed.

## Focused checks

- `receiver_search.rs`: five tests pass. Twelve width/spacing/channel cases
  cover24/90/192MHz,25/50kHz, integer offsets+2/-3, fractional3700Hz, unknown
  timing/phase, complex gain and actual reflected paths plus seeded perturbations.
- Six coded-block cases recover14232 information bits each with zero errors,
  after BCH/LDPC, bit processing, QAM and actual waveform samples. These use
  CFO-2carriers-1900Hz and reflected paths. Both decoder success and recovered
  source truth are checked. These are isolated blocks, not externally acquired
  full live PLC/NCP sessions.
- Split raw input blocks produce exactly equal bins/equalized samples and symbol
  IDs, including continued pilot-estimator state and total CFO phase.
- Silence, insufficient input, duplicate or physically indistinguishable
  hypotheses, unsupported offsets and exceeded search policies reject.
- `nonideal_acquisition.rs`: the two original fractional-only tests still pass.
- `qam_reference_tables.rs`: all32720 data labels match pinned source rows,
  hashes and independently transcribed normative energy factors; selected soft
  inverse signs match. Production uses the same source tables; this is not an
  independent coding-chain interoperability result.

## Shared checks

Workspace Clippy with warnings denied, formatting, no-default-feature tests,
changed-document paths, whitespace and the updated scientific-testing skill
through all three discovery paths passed. The complete workspace regression
suite also passed, including actual CLI import/channel runs. Final-source focused
tests, Clippy and analysis-only checks passed after consolidating named limits.

## Minimum scope clarification

WAVE-RX-05 closes the documented bounded minimum model. On2026-11-09 the user
clarified that MERLIN requires its own closed-loop verification, without another
implementation. Existing source-table and internal end-to-end tests therefore
close QAM-04 and QA-05 within that scope; see [acceptance evidence](../closed-loop-validation/validation.md).
No external interoperability comparison is claimed or required.
