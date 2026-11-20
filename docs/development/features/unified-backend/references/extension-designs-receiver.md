> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/receiver.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Wider receiver design

RX-EXT-01 closes design only for REQ-RX-01/04/06 and REQ-SYS-04. Current
configured timing, frame-pilot LS and explicit reference-variance policies remain
the only executable policies. The following algorithms are engineering receiver
choices; PHY I20 §7.4.9/10 and §7.4.15/16 establish transmitted geometry/pilots,
not a mandated implementation of a receiver estimator.

## Contracts and ownership

Extend `crates/phy/src/configuration/receiver.rs` with explicitly tagged policy
versions. Create `crates/phy/src/receiver/acquisition.rs` for bounded search,
`signal_processing/tracking.rs` for pilot-driven tracking,
`signal_processing/confidence.rs` for noise/error-variance estimation, and
`receiver/file_framing.rs` for an explicitly versioned finite payload policy.
Extend Rust facade/report models first and native conversion/type declarations
second; create independent cases in `crates/phy/tests/receiver_extensions.rs`
and `bindings/python/tests/test_receiver_extensions.py`. These paths are planned.

Input acquisition context carries allowed sample search interval, configured
numerology/CP/RP/resource map, oscillator search bounds, minimum evidence and
finite work budget. It MUST NOT carry injected delay/CFO/H/C, transmitted cells,
payload bits or expected CRC. Output carries observed timing/CFO candidates,
normalized scores, ambiguity/no-lock status and exact sample support. Oracle
truth is visible only to independent test observers.

## Resolved engineering algorithms

For scheduled data, first screen timing with normalized received CP-to-tail
correlation over configured candidate starts, excluding taper/unsupported pairs.
Then rank bounded candidates by known received pilot consistency after coarse
CFO correction. CP peaks alone do not establish a unique burst start; ties or
insufficient pilot support return ambiguous/no-lock. Ranging uses its separate
configured preamble matched filter from the ranging design, not this detector.

Estimate coarse CFO from received repeated-sample correlation phase with its
explicit unambiguous range; enumerate only configured alias hypotheses. Refine
with known pilots and a bounded coarse-to-fine frequency grid. Maintain emitted
sample coordinates and received-only phase state. Pilot common-phase tracking
uses circular phase residuals and a bounded first-order loop; frequency-selective
updates interpolate only within supported contiguous pilot brackets. Missing
pilots freeze estimates and grow age/uncertainty until an explicit maximum age
returns loss-of-lock. Reacquisition starts a new segment, never seamless BER.

Confidence calibration uses held-out or leave-one-out pilot residuals with
declared degrees-of-freedom correction, subtracting no injected noise truth.
For each supported interpolation region estimate complex error variance, require
at least8 independent residual observations and clamp only to an explicit positive
floor whose use is reported. Forward that variance through equalization gain to
the MaxLog demapper. If evidence fails, return uncalibrated/unavailable; retain
the existing user-supplied reference policy as a separately named choice.

Finite-file payload support is a new simulator transport, not DOCSIS MAC/CCF:
explicit length, source hash, ordered byte offsets, end padding and final partial
byte policy. Use length-prefixed manifest metadata supplied independently to
TX and legitimate RX framing, not PRBS seed or hidden original bytes. Report
recovered byte extents, gaps and hash comparison outside the receiver. No CRC/
MAC certification claim is inferred from that transport.

## Bounds and acceptance before implementation

Search at most65536 starts and256 frequency hypotheses with an admitted product
at most1048576 scored candidates; downsample screening or reject larger requests.
Keep at most8 candidates and existing aggregate16777216 sample/work budgets.
Confidence windows hold at most4096 residuals per supported region. File source
is capped at16MiB, streaming with bounded chunks; no unbounded input retention.
Cancellation checkpoints occur between candidate blocks and completed symbols.

Independent generated tones/impulses and externally calculated preamble/pilot
vectors SHALL test both numerologies, nulls/exclusions, CP edge windows, signed
CFO, alias ambiguity, false locks, no signal, low evidence, missing chunks and
drift beyond bounds. Statistical Monte Carlo evidence SHALL show empirical
coverage/variance and declared confidence calibration tolerances, not just BER
improvement. File tests SHALL independently verify final lengths/padding, failed
words, gaps, cancellation and wrong-but-valid decoded data. No extension becomes
executable until those source, stage, closed-loop and Rust/native gates pass.
