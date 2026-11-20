> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/channel.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Wider channel and composition design

CH-EXT-02 is design acceptance for REQ-CH-03/04/05 and REQ-PNM-04. Current
restricted RxMER synthesis, causal FIR, integer delay and constant phase/CFO
remain executable. This record does not enable a new impairment combination.
PHY I20 §7.4.17 and §9.4.8 plus CM-OSSI I27 D.2.13 establish the distinct
corrective coefficient/probe observation planes; neither proves exact plant H.

## Owners and source decisions

Create `crates/phy/src/measurements/inferred_response.rs` for explicit inverse-C
model derivation, `channel/fractional_clock.rs` for sample-clock resampling,
`channel/phase_noise.rs` for a seeded stochastic process, and
`channel/composition.rs` for reference-plane admission. Extend shared facade,
native conversion/type declarations and schema only after independent model
acceptance. Create `crates/phy/tests/channel_extensions.rs` and native parity
cases. Reuse `PreEqCapture`, measured association, Rust transform/RNG and the
existing stream/drain owners; never create a second capture decoder or Python DSP.

## Inferred response and incomplete grids

An applied C bank alone permits an explicitly named inverse-C hypothesis, not
a measured H. Require a supplied reference gain, delay convention, regularization
lambda>0 and physical RF grid. For accepted finite known bins define
Hhat[k]=g*conj(C[k])/(|C[k]|²+lambda), with an explicitly supplied phase/delay
factor. Report lambda, suppressed/deep-null bins and model uncertainty. Unknown
bins remain unknown; excluded bins retain their status. Reject waveform execution
unless the complete required transform response is defined by source evidence
or an explicitly selected engineering fill policy recorded separately. No default
zero-fill or inverse-C noise estimation is allowed. Use Hermitian symmetry only
for an explicitly real passband model, never assume it for complex baseband.

Convert the complete admitted complex response by the existing unitary transform
with independently verified scaling. Apply explicit bulk delay to choose a causal
support, cap FIR length at4096, and report truncation energy and phase convention.
Reject when discarded energy exceeds the selected error budget; do not silently
wrap noncausal energy into a causal impulse. Compare the reconstructed response
to retained input on all admitted bins before executing its FIR model.

## Fractional clock and phase noise

Use a bounded Rust polyphase windowed-sinc resampler with versioned kernel,
explicit tap count, initial fractional phase and rational time-step (or bounded
ppm drift). Record group delay, support loss, startup/drain and interpolation
error. Default remains exact integer bypass. Index output on its own sample
clock, carry the input-time map explicitly, and reject overflow or insufficient
support rather than manufacturing zero observations. Accept at most256 phases,
64 taps/phase and absolute clock offset200ppm in the first future model; these
are engineering budgets, not CableLabs tolerances.

Pin the first kernel to63 symmetric taps and256 fractional phases. Use the
symmetric Hann window, cutoff0.45*min(1, output_rate/input_rate) cycles per input
sample, and a normalized shifted sinc in each phase (DC sum1). Interpolate
adjacent phase-table outputs linearly, retaining the corresponding input-sample
shift at phase wrap. These are explicit engineering choices; alternate windows,
cutoffs or kernels require a new model version. Future acceptance compares direct
high-precision sinc/window sums and tones in the declared passband, with exact
sample-time mapping and reported startup/end support, before enabling this model.

For a first stochastic phase-noise model use an explicit discrete Wiener phase
increment variance in radians² per emitted sample, seeded independently of AWGN.
Apply phase after linear response and additive noise according to a versioned
stage order. Preserve state through chunks/pause and advance only for emitted
samples; zero variance selects bitwise bypass. Bound phase accumulation with
equivalent modulo2pi reduction and validate all finite inputs. Do not label this
one model a vendor oscillator spectrum or standards compliance experiment.

## Broader RxMER/C/H composition

An equalized-probe error target describes total observed residual error, not an
independently addable RF-noise variance. Broader synthesis requires an explicit
reference plane and an independently characterized deterministic contribution
for the same C/H/receiver policy. Compute residual random power only when that
contribution is defensibly subtractable; negative remainder rejects as inconsistent,
with both values reported. No automatic subtraction of simulated payload MER
from a measured probe target is permitted. Initially use separate named additive
noise experiments for nonunity C/H; the existing unity-only target model remains
the supported direct RxMER adapter. Preflight all later C events and clock/phase
policies before accepting any combined experiment.

## Independent future acceptance

Literal complex inverses/phase ramps and analytic FIRs SHALL verify scaling,
gain/delay, regularization and deep-null/missing-bin behavior. Fractional-clock
tests use independently calculated band-limited tones, impulse delay and rate
counts, with bounds on amplitude/phase/alias error across both numerologies.
Phase-noise tests use frozen independent draw sequences and statistical increment
variance/autocorrelation confidence intervals. Chunk/pause/cancel results SHALL
retain clocks, RNG state and exact output extent. Composition tests SHALL reject
unsupported reference mismatches, negative remainders and later incompatible
events before source consumption. Calibration and external capture claims require
their own evidence; internal waveform agreement is only model acceptance.
