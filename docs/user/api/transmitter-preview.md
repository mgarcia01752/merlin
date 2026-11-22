# Idle transmitter preview

Rust `merlin::downstream` and installed Python `merlin.downstream` expose immutable
idle TX requests, controls and results. Admission starts no worker and advances no source.
An explicit `generate` creates a private transmitter/source and runs the existing
bounded TX/PSD pipeline. It does not change a live simulation or accepted input.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `TransmitterPreviewRequest::new(&TransmissionConfiguration, &Dpd, &SourceInput)` | `TransmitterPreviewRequest(transmission, profile, source)` | Existing validated TX admission; freeze timing, original profile/provenance and passive source. |
| `TransmitterPreviewRequest::from_capture(&TransmissionConfiguration, &ProfileCapture, raw_id, &SourceInput)` | `TransmitterPreviewRequest.from_capture(transmission, capture, profile_id, source)` | Same admission for an original encoded source profile. Existing idle preview supports IDs 0..15, bootstrapped as Profile A; other recognized IDs reject. |
| `TransmitterPreviewControl::new()`; `cancel()`; `snapshot()` | `TransmitterPreviewControl()`; `.cancel()`; `.snapshot()` | One-shot cloneable cancellation/progress token; no thread starts and no pause/resume is promised. |
| `request.generate(&control)` | `request.generate(control=None)` | Synchronous explicit TX generation; Python releases the GIL. None uses a fresh control. |
| `result.info()` | `result.info()` | Owned original settings/layout/key/provenance, actual source/sample/EOF/warmup accounting. |
| `result.sample_view(start, count)` | `result.sample_view(start=0, count=0)` | At most 4096 contiguous actual I/Q samples and absolute sample origin; zero selects up to 4096 remaining samples. |
| `result.spectrum(start, count)` | `result.spectrum(start=0, count=0)` | Existing centered PSD extrema, at most 2048 buckets; zero selects remaining FFT bins. |
| `transmission.symbol_timing(Option<f64>)` | `transmission.symbol_timing(velocity_factor=None)` | Existing bounded window timing and optional ideal homogeneous-cable path budget. |
| `transmission.window()` | `transmission.window()` | Complete original sample-clock coefficients and FFT/prefix/taper/stride geometry. |

`SourceInput` retains exact finite bytes/fill or an explicit continuous PRBS15
seed. Every generation uses fresh private source state; repeated requests with
fresh controls are deterministic. A finite source stops at EOF without repeating.
Preview support and source-ID admission are separate from full-loop run support.
The existing public Simulation/ProfileId behavior remains unchanged.

`TransmitterPreviewControl` is Send+Sync and clones share one execution slot.
Its snapshot reports `idle`, `running`, `completed`, `cancelled` or `failed`,
completed/total TX symbols and a one-way cancellation request. A reused/executing
control rejects with Busy. A pre-start cancellation remains idle until dispatch,
then returns an error and reaches cancelled with zero completed symbols. Cancellation
is checked at existing symbol/analysis boundaries; no new result is accepted on
failure/cancellation. Previously returned results remain usable. Cancellation uses
the existing structured InvalidInput engine category and the explicit lifecycle
state; callers do not need to parse diagnostic strings.

Results are immutable owned snapshots, cheaply cloneable in Rust. Python queries
return independent dictionaries/lists. Metadata, samples and PSD remain valid after
inputs, requests and controls are released and after later generations. Query bounds
reject overflow, empty/outside spans and sample budgets; invalid queries do not
change the stored result. Generation and queries release the Python interpreter
lock; no worker invokes Python callbacks or takes an interpreter-dependent core lock.

Samples and spectrum identify `tx_post_overlap_before_channel`. Samples include
actual prefix/window/overlap output after existing idle interleaver priming. PSD
uses the existing periodic Hann Welch estimate, 50% overlap, at most 32 averages,
and normalized sample units squared/Hz. Buckets retain original centered FFT spans
and dB extrema; zero/unavailable power stays absent. `zero_frequency_hz` identifies
original physical bin zero, `spacing_hz` the bin increment. A centered baseband
axis uses the same bins from -Fs/2. Electrical receiver-input or measured H units
are not inferred by this preview.

Timing and complete window coefficients are authoritative backend outputs.
Cable velocity must be finite in (0,1], or None for unknown. The optional ideal
path budget assumes zero reserved receiver margin and a homogeneous cable; it does
not predict echo strength, plant reach or guaranteed decoding. Presentation code
may position/format coefficients without reconstructing the scientific window.

```python
from merlin.downstream import TransmitterPreviewRequest, TransmitterPreviewControl

# transmission, profile and source are previously admitted immutable inputs.
request = TransmitterPreviewRequest(transmission, profile, source)
control = TransmitterPreviewControl()
result = request.generate(control)  # dispatch outside a GUI event loop
print(result.info()['sample_count'], control.snapshot()['state'])
print(result.spectrum()['averages'])
```

The separate direct-Rust example `api_transmitter_preview` provides a reproducible
native-process reference used by installed-wheel acceptance. Retained full-loop
physical stages/messages belong to the controlled-run API; this idle operation
reports only the samples it actually generated.

Traceability: [architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-09),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-01-bounded-idle-transmitter-preview-unit)
and [task](../../development/checklist.md#task-uiu-api-01).
