# Unchecked-item review

Reviewed on 2026-11-08 against the current checkout. This is an evidence snapshot;
the [official checklist](checklist.md) remains the source of completion status.
This snapshot predates RUN-01/02/03/05 duration-stepper completion; see the
[current checklist](checklist.md#configurable-duration-reference-run) for updates.
All30 originally unchecked parent items were reviewed at that baseline. None was found fully
complete under its existing requirement. Completed components inside seven
partial items are now explicit checked substeps, rather than hidden behind an
unchecked umbrella entry. Requirements and stable IDs are preserved.

This audit changes documentation only. It does not implement missing stages or
promote a short reference experiment into a duration-based channel runner.

## Architecture and scientific stages

| Item | Evidence already present | Why the parent stays open |
| --- | --- | --- |
| ARCH-05 | `simulation/test_points/{model,observer}.rs` provides typed events, stage/range selection and finite budgets; observer tests serialize selected events in test-owned sinks. | No maintained export adapter is exposed through the CLI or another application adapter. Test serialization is supporting evidence, not the missing export feature. |
| ARCH-06 | Pilot, PLC, waveform and payload observation tests compare retained values and stage history; payload tests include disabled, enabled, failed and exhausted callbacks. | Source/noise RNG stages have no common probe producer/invariance coverage. Generating seeded input before observing a downstream stage does not verify RNG observation. |
| ARCH-07 | `signal_processing/modulation/interleaving/mod.rs` exposes composed label encoding and inverse LLR permutation for16/256-QAM. | Parity, twist and demultiplexing remain combined in `build_permutation`; intermediate forward/reverse APIs are missing. |
| FEC-03 | `fec-codec/src/parameters.rs`, `concatenated.rs` and codec tests establish payload BCH14232/14400 and short16200 rate8/9 LDPC. NCP/PLC use separate control codecs. | Additional downstream payload modes and their shortening rules are not implemented. Separate control codecs do not complete this payload requirement. |
| QAM-02 | `signal_processing/modulation/qam.rs` maps label-ordered references; `soft_bits.rs` produces soft LLRs. | A standalone nearest-symbol hard slicer with explicit tie policy and distance/error results is absent. |
| QAM-03 | `interleaving/parameters.rs` supports short8/9 uniform16/256-QAM; mapping tables extend through16384-QAM. | Other orders still lack complete verified coding-bit rules, padding/shortening and placement. |
| QAM-04 | `profile/` supports analytical profile handling; uniform256-QAM transport is implemented. | Mixed per-carrier data transport and independent high-order interoperability vectors are missing. Analytical profiles do not supply this coding chain. |
| NCP-03 | `transmission/ncp/` supports real field FEC/CRC and receiver pointer discovery; payload admission checks live received configuration. | The fixed session rejects unsupported activation/profile changes instead of implementing dynamic switching. |
| LAYOUT-03 | `transmission/layout/physical.rs` validates caller-supplied additional continuous pilots. | Generation of those positions from the full PHY placement rules is absent; fixture positions remain explicit/synthetic. |
| WAVE-RX-05 | `signal_processing/receiver/{timing,equalization,extraction}.rs` supports known timing/response and actual received-bin extraction. | Known-reference compensation and noiseless recovery do not establish nonideal synchronization or channel estimation. |

The existing FEC-01/FEC-02, BITS-01, QAM-01, NCP-01/NCP-02 and waveform
completion marks already describe the implemented subset correctly. They were
not left unchecked. See [modulation scope](../reference/ofdm-support.md) for the
separation between available maps and a complete coding/placement chain.

## Duration-based channel runner

| Item | Evidence already present | Why the parent stays open |
| --- | --- | --- |
| RUN-01 | `simulation/payload/{transmitter,receiver,reference,link}.rs` and `simulation/waveform/` separate TX/RX state and bounded reference composition. | The duration-based runner/lifecycle owner under the planned `simulation/channel/` is not implemented. |
| RUN-02 | Continuous stateful stages, bounded sample blocks and `transmission/timing.rs::one_second_timing` exist. | The helper remains fixed to one second; there is no user-duration channel loop or checked64-bit duration plan. |
| RUN-03 | Payload collector/interleavers/PLC owners preserve history; PRBS tests verify chunk/codeword/period continuity; transport tests cross PHY resets and drain. | These components have not been composed and verified over duration-runner observation boundaries and longer intervals. |
| RUN-04 | The reference verifier preserves source/channel identities, warm-up/drain counts, recovered word comparisons and pending bits. | It rejects incomplete recovery; it does not provide the full duration-runner accounting for complete, incomplete and excluded payloads. |
| RUN-05 | `payload_waveform.rs` exercises both spacings with real NCP and waveform recovery. Four PRBS payloads per spacing yield56928 compared bits, zero errors and actual decoder outcomes/iterations. | No duration-based results/lifecycle exists in which to retain equivalent success/failure/incomplete accounting. |
| RUN-06 | Reference workloads and probe output are bounded. The older16-QAM coding runner supports cancellation/runtime budgets and completed-trial retention. | The waveform channel verifier does not implement that cancellation/partial-result lifecycle. The older coding runner is a different execution path. |
| RUN-07 | `run_payload_waveform_with_source` is a public bounded reference API; the CLI exposes the older `simulate` and `evaluate` commands. | No duration-channel API/CLI command with shared lifecycle results exists yet. |
| RUN-08 | The PRBS waveform example and source/transport design guides include repeatable settings, Mermaid and math. | They describe the short verifier, not a duration-based CLI path with a one-second default. |

The verified short example is a foundation for these items, not evidence that
its advertised512-symbol/16-payload workload supports arbitrary duration.
Completed constituent steps are checked under RUN-01, RUN-03, RUN-05 and RUN-08.

## Impairments, adapters and later scope

| Item | Evidence already present | Why the parent stays open |
| --- | --- | --- |
| MODEL-04 | `signal_processing/channel/reference.rs` supplies unity sample transfer; cell-domain AWGN exists in `signal_processing/noise/`. | Nonideal waveform channel models and their reference-plane validation are missing. |
| MODEL-05 | Scenario types preserve supplied channel observations; the waveform receiver accepts a known reference response. | A measurement-driven waveform channel/receiver-estimation path with verified residual-noise ownership is absent. |
| MODEL-06 | Analytical evaluation and measured per-carrier noise exist in the older coding experiment. | They are not the applied-profile, temporally associated full waveform/FEC channel run required here. |
| MODEL-07 | Temporal association preserves capture timing/grids. | No independently validated temporal/burst impairment generator is implemented. |
| QA-04 | The existing gate already discovers workspace tests and builds/smokes the current CLI. | Additional channel commands/export adapters do not yet exist to add their specific smoke tests and contracts. |
| QA-05 | Locally transcribed arithmetic/reference vectors and internal integration tests exist. | Independent externally produced full-channel interoperability vectors are not available. |
| QA-06 | Linux pinned-toolchain validation exists. | There is no Windows/macOS runtime validation or CI workflow in this checkout. |
| QA-07 | Version and guarded Git/release helpers exist. | There are no native installers or configured automatic publication workflow. |
| API-01 | Public Rust operations are designed for future adapters. | No optional Python binding crate or Python runtime tests exist. |
| UI-01 | Scientific modules remain independent of presentation. | No Qt presentation component exists; this is deliberately deferred. |
| LATER-01 | Downstream OFDM is the active scope. | OFDMA design/implementation is deliberately deferred. |
| LATER-02 | The current architecture models one fixed downstream channel. | Channel bonding, multi-channel simulation and FDX-specific operation require separate future design/validation. |

## Validation of this review

Evidence paths and original item coverage were checked. All19 focused Rust
tests passed, rechecking selection/budgets, payload observer invariance, stateful payload
transport, actual seeded/PRBS waveform recovery, PRBS vectors/continuity and bit
interleaving. These tests validate the cited foundations; they do not complete
the missing adapter, duration runner or broader PHY support. Documentation links,
code fences, original parent IDs/statuses and whitespace are checked separately.
Linux is the verified host; no new platform/runtime claim follows from this audit.
