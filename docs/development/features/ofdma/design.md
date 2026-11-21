# OFDMA implementation record

The [architecture](../../../architecture.md#unified-upstream-configuration-and-source),
[requirements](../../../requirements.md) and [official checklist](../../checklist.md#int-us-config-01)
remain authoritative. This record refines the approved design; it adds no parallel plan.

User guide: [upstream configuration and source](../../../user/upstream/README.md).

## INT-US-CONFIG-01: typed foundation

Register independent `docsis-ofdma` (`docsis_ofdma`) in `crates/ofdma`.
Adapt configuration, UCD controls, grant selection, resource planning, checked
sample/bit types and continuous source/verifier from pinned revision
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`. Keep their independent literal
expectations. Retain Apache-2.0 copyrights and the source pin in every adapted file.

| Source path and symbol | Contract and evidence | Gap | Decision / destination |
| --- | --- | --- | --- |
| `crates/phy/src/configuration/` and `UsConfiguration::new` | Immutable geometry/inventory and atomic validation; source configuration tests | Local registration | ADAPT to `crates/ofdma/src/configuration/`; no downstream dependency |
| `crates/phy/src/resources/` and `ResourcePlan` | One pilot/capacity/address authority; independent pattern/permutation tests | Local registration | ADAPT to `crates/ofdma/src/resources/`; shared TX/RX plan |
| `crates/phy/src/grant/fec_selection.rs::select` | Three-family shortening/fill/pad ledger; literal thresholds and integer conservation | Bind to accepted codec | ADAPT imports to `fec_codec::upstream_ldpc` in `crates/ofdma/src/grant/` |
| `crates/phy/src/source/prbs15.rs::Prbs15::generate` | Atomic identity/range admission, continuous source | Duplicate scalar recurrence | SHARE `merlin_dsp::prbs15::step`; keep wrapper and independent verifier in `crates/ofdma/src/source/` |

Add the listed module trees, `src/{lib,error}.rs`, `src/types/`, manifest and
`tests/{configuration,grant_plan,prbs,upstream_pilots,upstream_interleaving}.rs`.
Edit root workspace registration and version inventory. Reuse existing DSP/FEC
without changing their algorithms. No adapter or downstream implementation edits.
Typed input returns immutable accepted configurations, explicit supported versus
represented state, finite grants, exact physical cells and source ranges.
Invalid inputs leave caller-owned accepted values and source position unchanged.
No thread, capture loader, source consumption or run begins on configuration.

This independently testable unit precedes waveform consumers. Versioned JSON,
complete source-field coverage and the final session remain part of the same
unchecked task until their own local acceptance. Evidence will identify actual
commands and candidate fingerprints; historical source passes are insufficient.

Per-file origin and hashes: [source adaptation inventory](source-files.json).

## Validation

Executed inside MERLIN with existing Rust 1.85.1 and the compiler/cache selection
recorded in [DSP validation](../signal-processing/validation.md#reproducible-build-environment).
[Code/build fingerprint](configuration-candidate.json) pins base `3a2f746` plus
SHA-256 `aec4d4f00e995a0229d25f52a254b169550902977b7f3a412595300cd66884f2`.

| Executed command | Observed result |
| --- | --- |
| `scripts/cargo-local.sh test -p docsis-ofdma -p merlin-dsp -p fec-codec --locked` | 92 passed, two explicit performance benchmarks ignored; includes 12 upstream foundation cases and unchanged shared kernels/codecs |
| `scripts/cargo-local.sh clippy -p docsis-ofdma --all-targets --locked -- -D warnings` | Pass |
| `scripts/cargo-local.sh fmt --all -- --check` | Pass |
| `python3 -B -m unittest discover -s tools/release -p 'test_*.py'` | Five version/release cases pass |

Focused upstream cases preserve the pinned source's literal/independent arithmetic
assertions. `configuration.rs::original_numerology_and_all_prefix_rolloff_values`
and `configuration.rs::frame_bandwidth_thresholds_are_encompassed_bandwidth`
cover geometry; `configuration.rs::explicit_iuc_inventory_and_rejected_candidate_preserve_snapshot`
and `configuration.rs::overflow_and_nonfinite_boundaries_reject_without_wrapping`
cover admission. `grant_plan.rs::source_threshold_and_two_shortened_word_literals`
and `grant_plan.rs::complete_bounded_capacity_sweep_conserves_every_role` cover all
three codec families, fill/pad and 700..49999 integer capacity conservation.
`prbs.rs::literal_seed_phase_period_and_chunk_continuity` and
`prbs.rs::rejected_calls_and_coverage_gaps_never_advance_state` cover source/verifier.
`upstream_pilots.rs` and `upstream_interleaving.rs` each retain two source-independent
role/address cases. File paths are relative to `crates/ofdma/tests/`.

REQ-US-CFG-01..06 and REQ-US-PRBS-01..04 retain full acceptance criteria.
This unit is validated as a typed foundation, with the combined task unchecked.
Versioned document admission, complete field coverage, final session and actual
TX/receiver comparisons remain pending. No waveform support, Python namespace,
external interoperability, Qt/hosted/platform acceptance is inferred.
Private command output is retained in
`docs/development/local/unified-backend/config-integration/rust-tests.log`.
