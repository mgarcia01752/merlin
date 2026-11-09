# Receiver and measurement implementation

## Completed supported group

MODEL-05/06/07 and Receiver Channel Estimation (CH-EST-RX-01..06) now implement
the scoped receiver/measurement path. Channel Estimation (CH-EST) always includes
its direction in task IDs; CH-EST-TX is reserved for transmitter-side work.
This does not close full blind acquisition, other FEC modes, dynamic profile
activation, external interoperability or future UI/bindings.

| Owner | Implemented behavior |
| --- | --- |
| `signal_processing/receiver/channel_estimation/` | Independent pilot references/LS, finite age-limited history, phase-slope interpolation and bounded CP timing search. |
| `simulation/waveform/receiver.rs` | Explicit supplied-H/pilot-derived modes; no H input to pilot RX; optional independent CP acquisition factory. |
| `simulation/channel/protected_constellation/` | Four finite acquisition-safe paired streams, current-fit labels, prior predictions and partial status. |
| `simulation/channel/link.rs` | Initial bounded raw PLC block corrected from completed pilot estimates; calibrated residual transport without a second noise source. |
| `simulation/channel/timeline.rs` / `measurement.rs` | Explicit same-grid epochs, original-time preservation, imported schedule resolution and atomic validation. |
| `transmission/layout/bit_loading.rs` | PHY physical-to-source profile mapping Ci=P^-1(Bi). |
| `modulation/interleaving/mixed.rs` | Non-shortened mixed24-column parity/twist with bypassed demultiplexer, using pinned Tables27/28. |
| Packing / assembly / payload decoder | Variable-width coded-bit cells, zero-loaded clocks, final-cell padding and actual NCP/LDPC/BCH recovery. |
| PLC descriptors | Applied ProfileA transported in real DPD packets; RX derives its own profile rather than reading TX settings. |
| CLI / plots | Shared additive typed JSON fields and bounded test points; presentation-only plotting of actual receiver pairs. |

Source authority is local CM-SP-PHYv3.1-I20-230419 section7.5.15 for pilots;
section7.5.4.3.2 Tables27/28 for mixed twists; section7.5.5.2.2 for variable cell
mapping/padding; section7.5.5.5.1 for inverse-frequency profile mapping.
Lookups used the local versioned text; no new rendered-PDF verification is claimed.
The interpolation/history/timing-search policies are engineering algorithms,
not a uniquely prescribed CableLabs receiver.

## User-facing documentation

- [Receiver subsystem index](../../../user/receiver-estimation/README.md), with
  separate pilot, interpolation/equalization, timing and analyzer guides.
- [Applied-profile transport](../../../user/payload-transport/applied-profiles.md).
- [Temporal measurement model](../../../user/impairments/measurement-timeline.md).
- [CLI request extensions](../../../user/channel-runner/cli.md).
- [Public API](../../../reference/public-api.md).

## Validation evidence

The full local pre-push gate covers warnings-denied all-target Clippy, rustfmt,
workspace tests, analysis-only tests, helper tests, release CLI builds and actual
reference/measured/AWGN CLI smokes. New focused suites are:

| Suite | Independent or integration evidence |
| --- | --- |
| `channel_estimation_rx` | Gain/linear-phase vectors at both spacings and all pilot phases; prior-vs-current perturbation; atomic rejected calls; actual acquired waveform/PRBS recovery. |
| `channel_estimation_timing` | Independent prefix/tail vectors; real windowed waveform with unknown sample offset; ambiguous/silent/invalid search rejection. |
| `protected_constellation` | Four stream limits/order/phase; noisy FEC invariance with capture enabled; failed PLC observations; cancellation/invalid bounds. |
| `mixed_profile_transport` | Independent full24-column matrix/parity reconstruction and bijection; uniform16 plus mixed16/256/16K/zero loading at both spacings; acquired RX and physical-profile checks. |
| `channel_timeline` | Identical-epoch continuous RNG/FEC equality; invalid scheduling/identity/seed rejection; independently expected Gaussian power and lag checks. |
| CLI `receiver_measurements` | Real process JSON equals shared Rust results; bounded ready estimates/streams and no-clobber destinations. |

The measured CE-only pilot-derived run used the existing full192MHz/25kHz
capture grid,8ms observation and seed41. It compared9,620,832 payload bits with
zero bit errors and zero mean payload LDPC iterations. Four post-startup streams
retained128 pairs each. The final estimate reported7464 observed bins and maximum
age127 symbols. Original capture epochs/provenance remain in the supplied scenario
and effective response. Matching supplied-H figures remain unchanged as a baseline.

Plots were generated from the actual Rust CLI result. The pilot plots are labeled
in-fit; they are not independent accuracy evidence. PLC16-QAM/preamble and payload
recovery use the interpolated response and actual FEC. No added noise or invented
field I/Q is implied by these CE-only images.

Generated requests, full results and logs remain ignored under
`docs/development/local/receiver-measurement-completion/` and
`docs/development/local/pilot-channel-estimation/`. Tracked examples reproduce
synthetic reception and imported measured CE-only reception.

## Deliberate boundaries

- Known bootstrap numerology/PLC phase; CLI known timing. CP search does not solve
  carrier offset, unknown PLC phase or arbitrary stream discovery (WAVE-RX-05).
- Frequency-diagonal and sufficiently slow channel; bounded pilot aging and
  approximate interpolation. The ready flag is not a quality certificate.
- Raw Gaussian noise-budget calibration through the applied inverse, not complete
  posterior LLR uncertainty or a field-derived burst-correlation process.
- Frozen ProfileA ID0; no dynamic profile activation or shortening. Uniform16/256
  and verified mixed modes are supported; other uniform coding modes reject.
- Piecewise RxMER temporal playback requires one continuous seeded RxMER source.
  Static `use_rxmer_noise=false` provides CE-only reception; noiseless imported
  timelines are not supported by this contract.
- External modem vectors, Windows/macOS runtime checks, installers, Qt and Python
  bindings remain tracked separately. No system packages were installed.

## Final review checks

The full gate passed on Linux with the pinned Rust1.85.1 toolchain. Final review
followups (captured zero-loaded roles, exact transition-boundary coverage and
finite response/noise-calibration guards) passed their affected test suites,
all-target warnings-denied Clippy and a rebuilt release CLI measured example.
The CLI probe test additionally verifies dimensionless estimated-gain units,
profile-interleaved binary bits and equality with the direct Rust result.
The tracked measured CE-only request reproduces9,620,832 compared bits with zero
errors, zero mean payload LDPC iterations and128 pairs in each protected stream.
Documentation links/fences/whitespace and all three coding-skill discovery paths
were validated. Platform coverage remains Linux; no macOS/Windows claim is made.
