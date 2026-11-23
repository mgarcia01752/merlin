# Downstream full-loop requests

`merlin::downstream::Simulation::prepare_full_loop` and installed Python
`merlin.downstream.Simulation.prepare_full_loop` prepare independent immutable
single-profile requests. Existing Simulation/Merlin evaluation sessions and their
options remain unchanged. Preparation validates accepted inputs through existing
Rust scientific owners and advances no source or simulation.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `Simulation::prepare_full_loop(FullLoopInputs, FullLoopOptions)` | `Simulation.prepare_full_loop(transmission, profile, source, measurements, channel, response=None, options=None)` | Own accepted typed inputs and freeze a validated request; Python collects the same fields into the Rust bundle. |
| `FullLoopOptions::default()`; `validate()`; `from_json(bytes)` | `FullLoopOptions()`; `FullLoopOptions.from_json(data)` | Same bounded policy; unknown/oversized JSON rejects. Profile-specific selection checks occur during preparation. |
| Options fields / serde | `.settings`; `.to_json()` | Typed Rust fields and owned native dictionaries/JSON, with identical field names and units. |
| `request.settings()`; `request.options()` | `.settings`; `.options` | Frozen exact scientific settings and accepted options; native copies cannot mutate the request. |
| `request.with_runtime_limit(max_runtime_ms)` | Matching method | Independent idle request with an admitted active host limit no greater than its prepared parent. |
| `FullLoopControl::new()`; `pause()`; `resume()`; `cancel()`; `snapshot()` | `FullLoopControl()`; matching methods | Direction-specific one-shot controls and actual safe-boundary pause acknowledgement. |
| `control.last_progress_age_ms()` | Matching method | Optional actual backend-notification wall age; continues through paused/terminal states, independently of simulated or active host time. |
| `request.run(&control)` | `request.run(control=None)` | Explicit bounded existing channel job; None creates a fresh control. |
| `result.report()` | `result.report()` | Actual typed Rust complete/partial report and owned native dictionary. |
| `result.inputs()`; `inputs.summary()` | `result.input_summary()` | Rust retains typed original inputs; native returns bounded original identity/provenance metadata. |
| `result.recovered_bytes()` | `result.recovered_bytes()` | Borrowed Rust decoded original-file bytes / owned native bytes, or None for PRBS. |
| `transmission.spectrum_average_capacity(max_samples)` | Matching method | Idle complete-window capacity for a sample budget and accepted FFT, capped by the existing spectrum work policy. |
| `result.calibrated_spectrum_sequence(query)` | `result.calibrated_spectrum_sequence(stage, snapshot, max_averages, start, count)` | Normalized ordered-frame sequence metadata and calibrated/relative power from one estimate. |

`FullLoopInputs` contains the previously admitted transmission, Dpd, SourceInput,
MeasurementSequence, ChannelConfiguration and optional original ChannelEstimate.
None explicitly selects synthetic unity H; receiver correction remains a separate
option. Measurement sequence timing must exactly match the request numerology;
original frames, order, mode, tolerance and clock are revalidated for the selected
profile. Original profile/H epoch association uses the sequence's explicit tolerance.
Existing spatial/coverage, supported role/loading, H/filter and receiver restrictions
remain authoritative. Inputs are immutable and original bytes/hashes stay retained.

Default full-loop policy matches the supported downstream workbench: 3 ms source
observation, 50 decoder iterations, 300000 ms active host budget, 64 retained word
details, noise seed 41, supplied-channel correction and existing finite QAM/protected
capture selections. The source keeps its own PRBS/file/fill policy. Options bound
observation to 1..1000 ms, iterations to 1..100, runtime to 1..300000 ms and retained
words to 0..10000. Native JSON options are limited to 64 KiB. Existing MCE/QAM policy
types remain with their Rust owners. This is a separate public request contract;
legacy evaluation defaults and duration domains are preserved.

Controls are cloneable Send+Sync tokens independent of result storage and workers.
A fresh token admits one run; repeated/concurrent use rejects Busy. Snapshot states
are `idle`, `running`, `paused`, `completed`, `cancelled`, `runtime_exceeded` and
`failed`. `pause_requested` is separate from acknowledged `paused`: the original
worker enters that state only at a completed-step boundary. An initial pause waits
before the first step. Resume continues the same source/RNG/receiver state.
Cancel is one-way and also exits a paused wait. Active host runtime excludes actual
paused time; progress reflects the last completed scientific step.

Rust names the existing progress enum as `merlin::downstream::ChannelPhase`:
`control.snapshot().progress` contains an optional `ChannelProgress`, whose
`phase` field can be matched directly. Python keeps the same snapshot dictionary;
`snapshot["progress"]["phase"]` is a snake-case string when progress is present:
`acquisition`, `observation`, `completion`, `interleaver_drain`, `finished` or
`failed`. These processing phases are separate from the control lifecycle states.

`request.with_runtime_limit(max_runtime_ms)` returns an idle independent request
whose positive active host-runtime limit is no larger than the prepared budget.
The original request and all scientific settings, originals, observation policy
and simulated duration stay unchanged. Rust and Python use the same validation;
the operation supports dispatching each profile with a sweep's remaining budget.
It does not start a run or advance source/RNG state.

`control.last_progress_age_ms()` returns `None` before a backend notification and
otherwise its monotonic wall age in whole milliseconds. The backend records the
timestamp at the actual progress update; UI/Python polling never creates a new
notification. Age continues through pauses and after completion. This describes
notification age, separately from a frozen UI elapsed label or active host time.

After construction, cancellation/runtime/step failure returns the actual partial
report. Completed comparisons and retained FEC trials remain available; unfinished
words do not become fabricated comparisons. Pre-dispatch construction/admission
errors return structured ApiError. The request remains reusable with fresh controls,
and prior results remain valid after later runs and input/request disposal.
Rust `ApiError::engine_code()` and native `.engine_code` expose an optional
retained original diagnostic for registered help, with category/message unchanged.
An absent code stays absent even when display text resembles a known failure.

Python runs on a private bounded native worker while releasing the interpreter
lock. Another Python thread can poll/pause/resume/cancel without taking a scientific
result lock. Interpreter interrupts request cooperative cancellation and return the
partial report, consistent with existing session execution. No Python callback
executes inside the Rust job. Queries return owned dictionaries/lists/bytes; edits
cannot change the immutable Rust result. File output contains actual whole decoded
original bytes, excluding appended fill and coding padding, without filesystem I/O.

```python
from merlin.downstream import Simulation, FullLoopOptions, FullLoopControl

# All five required inputs are previously admitted immutable objects.
options = FullLoopOptions.from_json(b'{"duration_ms":1,"max_iterations":10}')
request = Simulation.prepare_full_loop(
    transmission, profile, source, measurements, channel, options=options)
control = FullLoopControl()
result = request.run(control)  # dispatch outside a GUI event loop
print(result.report()['status'], result.report()['summary']['comparison'])
```

[Opt-in retained stage observations](full-loop-observations.md) add typed bounded
copies of actual test-point data. Numerical/message queries and complete source-
catalog/sweep compatibility have scoped API acceptance; final Qt adoption remains
tracked under UIU-DS-01 and the unified UI checklist.
Large retained report arrays remain bounded by existing scientific policies;
Qt consumers use scoped public observations. The direct-Rust `api_full_loop`
example supplies independent process references for selected settings, actual
FEC/counts/words and retained source/pilot/physical/waveform windows.

Traceability: [architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-09),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-02-downstream-controlled-request-unit)
and [task](../../development/checklist.md#task-uiu-api-02).


## Spectrum sample budgets

Use the accepted transmission configuration to plan how many complete Welch
windows fit a retained-sample budget. Rust owns the half-overlap cadence and
caps capacity at the existing 32-window analysis policy. A budget smaller than
one FFT raises the shared input error. Larger budgets preserve that work cap;
actual retained sample availability is checked by the result's spectrum query.

```rust
use merlin::downstream::{TransmissionConfiguration, TransmissionSettings};

// Existing 32768-sample retained-window policy, independent of FFT length.
const SAMPLE_BUDGET: usize = 32_768;
let transmission = TransmissionConfiguration::new(TransmissionSettings::default())?;
let averages = transmission.spectrum_average_capacity(SAMPLE_BUDGET)?;
assert_eq!(averages, 7); // Default 25 kHz spacing; 50 kHz admits 15 windows.
```

```python
import merlin.downstream as m

SAMPLE_BUDGET = 32_768  # Same retained-sample policy, expressed in sample units.
transmission = m.TransmissionConfiguration()
averages = transmission.spectrum_average_capacity(SAMPLE_BUDGET)
assert averages == 7
```

Pass the returned count as `max_averages` in the existing spectrum query using
that configuration. This capacity query leaves inputs idle and unchanged. At
25 kHz with a 1024-sample CP, four actual TX blocks contain 36864 samples: the
32768-sample prefix selects seven complete windows while the full span admits
eight. Original contexts, reference planes and simulation outcomes remain owned
by the retained result. See the [separate Rust reference](../../../crates/ofdm/examples/api_spectrum_average_capacity.rs).

## Calibrated ordered-frame diagnostics

`FullLoopResult::calibrated_spectrum_sequence` accepts the existing
FullLoopSpectrumQuery for IFFT, prefix, window or RX-useful frame stages. Its
typed FullLoopCalibratedSequenceSpectrum contains `sequence`, the unchanged
normalized FullLoopSequenceSpectrum, and `power`, the calibrated ChannelSpectrum
from the same estimate. Native Python returns the matching owned dictionary
with `sequence` and `power` keys and the existing sequence-query defaults.

The sequence retains original segment/sample origins, stop reason, physical
stride and explicit contiguity. Concatenated frame-stage samples may have
physical gaps or overlaps; electrical calibration preserves that diagnostic
sampling basis. Use the continuous waveform spectrum operation for actual
continuous-output observations. Absent electrical calibration keeps relative
units. The channel-power reading covers the same full occupied band during zoom.

The operation starts no simulation and preserves inputs, retained samples and
decoder outcomes. Existing sequence/grid/work/window failures remain structured.
See the bounded [separate Rust reference](../../../crates/ofdm/examples/api_calibrated_sequence.rs)
and [paired validation](../../development/features/unified-qt-workbench/validation.md#uiu-ds-01-calibrated-ordered-frame-api-accepted).


## Source-verified whole-run recovery

`FullLoopResult::fec_recovery()` returns an owned FullLoopFecRecovery with five
word counts: generated, recovered, rejected, mismatched and unrecovered. Native
Python exposes `result.fec_recovery()` as an owned dictionary with the same keys.
The operation reads the actual complete, cancelled or failed result without
running the simulation again or depending on retained detailed word records.

Recovered means a completed decoder-accepted word has no detected independent
source mismatch. If no bits were compared, this is not positive proof of source
agreement; inspect the report's compared bits and BER separately.
Rejected means LDPC or BCH rejected the word; it counts once
even if both stages failed. Mismatched means the decoder accepted a word but the
source comparison disagreed. Unfinished generated words remain unrecovered.
These whole-run recovery counts do not replace observation-window FEC outcomes,
BER denominators or the separate clean/corrected decoder classifications.

```rust
// `result` is an existing complete or partial FullLoopResult.
let recovery = result.fec_recovery();
println!("Recovered {} of {} generated words", recovery.recovered, recovery.generated);
```

```python
# `result` is an existing complete or partial merlin.downstream.FullLoopResult.
recovery = result.fec_recovery()
print("Recovered", recovery['recovered'], "of", recovery['generated'], "generated words")
```
