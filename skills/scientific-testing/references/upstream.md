# Upstream OFDMA validation procedure

[Skill](../SKILL.md) · [Scientific contracts](../../docsis-signal-processing/references/upstream.md)

Use these checks when modifying upstream science under `crates/ofdma`, the shared
`crates/dsp`/`crates/fec-codec` leaves and `crates/pnm-input/src/upstream`. Existing
integrated cases and evidence are linked from the [upstream design](../../../docs/development/features/ofdma/design.md).
Record exact expected-value origins; this procedure alone accepts no new support.

## Independent codec evidence

The current standalone owner is `crates/fec-codec/src/upstream_ldpc/`, with
`tests.rs` and `tests/{source_audit,source_fixtures,oracle}.rs`. Run its cases
inside MERLIN; [integration evidence](../../../docs/development/features/unified-backend/fec-validation.md)
accepts codecs only. Upstream waveform/receiver acceptance remains separate.

For every declared upstream family, preserve independent source-table checks for
matrix dimensions, entries, shift orientation and row/edge fingerprints. Keep
frozen parity expectations where available. Verify the exact pinned source before
moving tests; do not regenerate expectations from the production table being tested.
Matching a source hash proves preservation, not that the transcription is correct.

Test systematic encoding/syndrome, noiseless exact information recovery, controlled
soft errors, finite LLR sign/scale, actual iteration counts, shortening boundaries
and invalid lengths/bits/nonfinite inputs. Ordinary data tests do not inherit
downstream BCH or ranging puncturing. Check convergence, parity, shortened-zero
consistency and original-information errors independently. Include a different
valid codeword that converges but fails comparison with the original source.

A synthetic BPSK Gaussian oracle can use positive-for-zero LLR = 2y/σ² with
explicit symbol energy, noise variance and deterministic seed. Count predecode
sign errors separately from decoded payload errors and actual iterations. Label
this as a codec experiment, not an OFDMA RxMER measurement. If the oracle shares
production matrix data, retain independent matrix-source checks: the Gaussian
experiment cannot detect an incorrect shared cell by itself.

## Waveform and receiver evidence

Retain impulse/DC/tone/direct-sum transform expectations independently of inverse
round trips. Verify both declared numerologies, exact normalization/bin ordering,
prefix/window boundaries, pilot/resource exclusions and continuity across arbitrary
input chunks. Validate direction-specific QAM labels, bit ordering, randomizer
states, joint interleaving, grant/minislot admission and pre-equalization semantics
from their pinned sources. Do not infer coverage from recognized enum values.

For integrated acceptance compare original source → TX → channel → RX → recovered
bits. Normal receiver tests consume received samples/configuration without hidden
TX payload/channel/scheduler truth. Verify received-pilot estimation against an
independent observer, ambiguous/invalid acquisition rejection and equalization
reference planes. Exercise noiseless recovery first, then bounded deterministic
impairments with documented tolerances. Preserve failed decisions in reports.

## Measurements, lifecycle and adapters

Capture tests must distinguish format decoding from measurement association and
simulation. Use the format owner's version/length/byte-order/scaling/sentinel
checks, provenance-preserving fixtures and explicit UCD/profile/Pre-Eq update
context. Missing metadata or incompatible grids must not become guessed defaults.
Test measured RxMER playback and applied Pre-Eq separately from simulator truth.

Exercise cancellation and pause/resume at defined safe boundaries with partial
results and actual work limits. Bound retained detail independently of scalar
totals. Instrumentation/observer failures or exhausted budgets must not consume
extra random draws or alter recovered results. Compare scientific determinism
separately from elapsed time and timing-budget completion.

Preserve existing downstream regression acceptance. Pair public Rust and native
Python configuration, error, ownership and lifecycle cases through the same engine;
follow the [paired API procedure](../../public-api/references/paired-api.md).
Run the existing MERLIN validation commands for the changed ownership scope;
do not import source-repository runners, cache cleanup or automatic commit commands.

Record actual command, candidate, host/toolchain, elapsed time, failures, omissions
and evidence category. Internal closed loops, independently transcribed normative
arithmetic, shared source-table checks and external vectors are different evidence.
An external dataset is useful when available, not a new minimum prerequisite.
Source-backend test reports remain historical source evidence until reproduced
for the integrated MERLIN candidate.
