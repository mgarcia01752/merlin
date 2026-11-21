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

## INT-US-TX-01: transmitter implementation unit

Checkpoint `07969fd` accepted the typed foundation. REQ-US-CFG-05 explicitly
places the typed reference loop before JSON/capture loaders; implementation units
may now validate direct typed stages, while the combined CONFIG and TX tasks stay
unchecked until their complete dependency acceptance is available.

Adapt source `crates/phy/src/transmitter/`, `bit_processing/randomizer.rs`,
`signal_processing/{qam,qam_tables,demapping,pre_eq,cyclic_extension,transmit_window}.rs`
and synthetic `measurements/` into matching `crates/ofdma/src/` owners. Demapping
is included only to independently validate mapping/soft signs; normal RX composition
remains a later unit. Pre-Eq replay is a typed TX input contract, separate from
capture decoding and measured association.

| Source symbol | Contract / evidence | Gap | Decision / destination |
| --- | --- | --- | --- |
| `Transform::{new,forward,inverse}` | Source 2K/4K centered unitary Iq; independent direct sums | Duplicate butterflies | ADAPT typed finite/error wrapper over `merlin_dsp::transform`; retain named-field Iq JSON, no unsafe reinterpretation |
| `qam_tables::coordinates` and source `build.rs` | Same raw large tables/control-label permutation; source-only 8/32-QAM literals | Local embedding | REUSE existing `crates/ofdm/resources/qam-table` bytes at build time; SEPARATE directional integer table generation and upstream normalization, no downstream engine dependency |
| `Transmitter::new/next_frame`, `encode_grant`, `BurstAssembler` | Transactional source, all eight stages, bounded frame emission | Local composition | ADAPT to `crates/ofdma/src/transmitter/`; no source truth in receiver |
| `PreEqBank`, `PreEqPlayback` | Corrective C with explicit normalization and corresponding-probe event bases | Local TX inputs | ADAPT to signal_processing/measurements; physical H remains a separate future owner |

Add source tests `randomization`, `qam_mapping`, `pre_eq`, `transforms` and
`transmit_window` without changing expected values. Add a frozen source-transform
comparison/benchmark because typed array conversion changes a hot path.
QAM resources remain single-source files owned by the existing provenance record;
both PHY crates are scientifically independent. Current source builds require the
repository resource tree; publish=false crates do not claim standalone crates.io
source packages. Package inventory checks must retain these build-time resources.

### Transmitter unit validation

[Candidate fingerprint](transmitter-candidate.json) pins base `07969fd` plus
SHA-256 `3adc71f7244df1cb9092552402ea3d2735b3bf4504f6b68939445770d5fe5fd1`.
`test -p docsis-ofdma --locked` passed 21 cases with the explicit benchmark ignored.
`clippy -p docsis-ofdma --all-targets --locked -- -D warnings` passed;
`fmt --all -- --check` passed. Commands use the existing Rust 1.85.1 environment.

Exact new independent cases:
`crates/ofdma/tests/tx_framing.rs::codeword_information_fill_and_grant_pad_have_distinct_values`,
`crates/ofdma/tests/tx_framing.rs::transmitter_rejection_preserves_source_and_emission_stops_at_declared_support`,
and `crates/ofdma/tests/transform_reuse.rs::shared_transform_preserves_source_bits_and_error_fields`.
The preserved source cases in randomization, QAM mapping, Pre-Eq, transforms and
transmit window passed with unchanged literal expectations. Source plan tests from
the previous checkpoint remain passing.

`test -p docsis-ofdma --release --locked --test transform_reuse -- --include-ignored --nocapture`
passed both the equivalence case and explicit paired benchmark. Fifteen alternating
100-call batches after warmup gave median inverse times 31.366/31.673 microseconds
(source/shared adapter) at 2K, ratio 1.010, and 88.730/64.914 at 4K, ratio 0.732.
This is one host observation. Typed conversion adds two bounded vectors per call;
no unsafe layout cast or allocation profiler was used. No universal speed claim.

Private outputs: `docs/development/local/unified-backend/tx-integration/`.
The combined task remains unchecked; full JSON/source-field/session and later
received-only loop acceptance are not inferred. The
[user guide](../../../user/upstream/transmitter.md) describes the current stages.

## INT-US-CH-01: channel implementation unit

TX checkpoint `72c9145` supplies the typed waveform. Adapt source
`crates/phy/src/channel/{mod,stream,residual}.rs` into matching OFDMA owners.
Inputs are contiguous finite indexed blocks with run/generation/grant identity;
outputs preserve the sample clock, checked causal drain and actual coverage.
Keep H separate from transmitter C and the restricted RxMER residual reference.

| Source symbol | Contract / evidence | Gap | Decision / destination |
| --- | --- | --- | --- |
| `ChannelConfig`, `Channel::{new,push,finish}` | Exact unity bypass including signed zero; causal FIR→AWGN→delay→phase/CFO; independent convolution/noise/chunk tests | Local composition | ADAPT `crates/ofdma/src/channel/stream.rs` unchanged scientific operation/draw order; SEPARATE from downstream filter/state |
| `RxmerChannel::{new,push}` | Symbol-conditioned residual draws in physical-bin order; immutable playback; whole-call retry | Local residual transport | ADAPT `crates/ofdma/src/channel/residual.rs`; SHARE transform only; composition restrictions remain high-level admission |
| `RxmerPlayback`, `PreEqPlayback` | Power-domain average/hold/interpolation, corresponding-probe Set/Adjust bases | Local schedule evidence | REUSE implemented typed measurement owners; preserve nulls, complete banks and future-event rejection |

Add original independent `channel_models`, `rxmer_channel` and
`measurement_playback` cases with unchanged expectations. Add only exact pinned
rand 0.9.5, rand_chacha 0.9.0 and rand_distr 0.5.1 dependencies already present
in MERLIN; no compiler upgrade or new algorithm dependency. RNG state and stream
history commit only after complete block validation. A failed push leaves all
accepted state intact. Sample-count and retained-frame bounds remain explicit.

### Channel unit validation

[Candidate fingerprint](channel-candidate.json) records base `72c9145`, SHA-256
`40191833b516a4a22b8726d98abb0f55cb5109479d31b41e2c3e152d2db28d81`.
`test -p docsis-ofdma --offline` passed 27 cases, one explicit benchmark ignored;
Clippy all targets with warnings denied and formatting passed. Exact pinned
existing dependencies were resolved locally; the lock diff adds only the three
OFDMA RNG edges and changes no package identity/version/checksum.

Source-independent acceptance cases in `crates/ofdma/tests/`:
`channel_models.rs::literal_convolution_delay_phase_and_exact_drain_order`,
`channel_models.rs::exact_unity_signed_zeros_and_pinned_noise_chunk_invariance`,
`channel_models.rs::rejected_blocks_preserve_history_rng_cursor_and_terminal_state`,
`rxmer_channel.rs::every_bin_draw_order_actual_useful_samples_and_chunk_retry_invariance`,
`measurement_playback.rs::literal_power_average_hold_interpolation_nulls_and_boundaries`,
`measurement_playback.rs::snapshots_replace_and_adjustments_use_explicit_older_probe_base`.
Expectations and original source hashes remain unchanged.

Private output: `docs/development/local/unified-backend/channel-integration/rust-tests.log`.
High-level full-schedule/observer acceptance remains pending and the task stays
unchecked. [User guide](../../../user/upstream/channel.md).
