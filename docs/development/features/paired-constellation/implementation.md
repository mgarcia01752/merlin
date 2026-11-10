# Paired constellation implementation

CONST-01..05 implement the accepted [plan](plan.md) using the existing waveform
and received-payload path. The first supported mode remains uniform256-QAM.
No independent pilot estimator, new noise model, hard slicer or GUI was added.

## Ownership and reuse

`simulation/channel/constellation/selection.rs` validates the optional count,
profile and inclusive physical-frequency selection. `model.rs` owns the shared
Rust result and its raw/corrected I/Q pair records. `collector.rs` owns diagnostic
transport, receive-time ordering and finite retention. Existing source imports,
H/residual synthesis and receiver equalization are unchanged.

`ChannelRunSettings.constellation` is optional. `ChannelRun` constructs an empty
capture before acquisition, then starts its coordinate stream from the actual
ready receiver configuration. It pairs `ReceivedSymbol.bins` and `equalized` at
identical physical coordinates. A separate existing generic receive interleaver
carries these records through the same frequency/time inverse as payload cells.
This stream never becomes input to payload decoding.

`PayloadSymbolResult.data_soft` supplies accepted source-order payload ranks,
excluding received NCP/filler and pilot placeholders. LDPC/BCH failure flags do
not remove these samples. A terminal source-control diagnostic is not trusted to
classify new samples. Full decoder statistics are accounted before a diagnostic
collector failure could return, and all prior confirmed points remain retrievable.

## Coordinate and memory proof

For source reduced rank p and time depth M, the TX branch delay is p mod M.
A point from source symbol s was transmitted at physical channel symbol
s + (p mod M). At receiver step n the inverse releases source s=n-(M-1).
When source s has been classified, no future source can produce an eligible
physical coordinate at or before s. The collector therefore releases the sorted
confirmed entries through watermark s and retains only a bounded future tail.

The pending bound is NI*M points. The diagnostic inverse interleaver also uses
its existing finite NI/depth history; neither buffer grows with run duration.
Every stored point is a complete pre/post pair. After the target is reached,
pending storage and diagnostic interleaver history are dropped. Final take flushes
only previously classified pending pairs on complete, partial and failed jobs.
Ordering is deterministic receive-symbol/bin order among confirmed observations;
a partial failed run does not claim that unclassified earlier samples were data.

The core result uses `Unavailable`, `Partial` or `Complete` capture status,
separate from `ChannelJobStatus`. It defaults to8192 pairs and allows1..65536.
Zero or unsupported counts/profiles, incomplete/outside/reversed frequency bounds
and empty data selections reject before simulation. Serializer byte limits remain
independent; the optional result and settings fields are omitted when disabled.

## API, CLI and demonstration

`ChannelRun::constellation` borrows committed observations. `take_constellation`
returns the bounded partial/final result and stops only diagnostic retention.
`run_channel` attaches the finalized result on every job outcome. The existing
channel JSON request accepts `constellation`; the existing CLI serializes the same
Rust result without calculating or renaming simulation fields.

Complete requests are `examples/channel-constellation.json` (noiseless) and
`examples/measurements/single-channel/channel-constellation.json` (measured H,
RxMER and profile0,700–727MHz). The measured3ms/seed41 run exported8192 pairs,
compared3614928 payload bits with zero errors and produced the linked
[user-guide image](../../../user/impairments/paired-constellation.md).
The matching11dB stress scenario exported8192 pairs and retained all483 bit
errors; its entire simulation summary matched the saved no-collector experiment.

The optional Matplotlib renderer under `tools/plots/constellation/` presents
exported samples with shared axes. Its `--view before`, `--view after` and
`--view paired` modes can also render either stage individually. The standalone
pre-CE image uses the same8192 raw measured-demo FFT points without correction. Its packages are isolated under ignored
`local/doc-plots/`; no system packages or Rust runtime dependencies were added.
The generated image was visually inspected. Full result JSON and validation logs
are under ignored `docs/development/local/paired-constellation/`.

## Validation

Six core integration tests cover exact8192-pair capture at both spacings/maximum
depth, independent normalized256-QAM levels and complex-gain arithmetic, source/
physical carrier mapping, receive-time order, invalid requests, single-carrier
partial and zero-pre-acquisition output, cancellation, probe-export failure,
failed-payload retention, midstream diagnostic removal, block splitting and full
simulation-summary invariance. Two actual-CLI tests cover serialized Rust-result
equality, default count, invalid/unknown fields and non-overwriting output.

A first short-run test chose a carrier with no eligible payload in that source
window; it correctly returned unavailable. The test now selects an actually
observed carrier before verifying partial retention. CLI-versus-API comparison
passes both values through the same JSON float parsing boundary to avoid treating
parse-rounding differences as science changes. No source samples were rounded.

The final full `scripts/check-before-push.sh` gate passed:32 Python Git/release
helper tests, ShellCheck/Bash syntax, rustfmt, all-target warnings-denied Clippy,
workspace Rust tests (including six new core and two CLI integration tests),
fixture tests, analysis-only tests, release build and coding/channel/import/AWGN
smokes. The existing measured release smoke now asserts8192 actual paired points.
No stages or checks were bypassed. Linux evidence does
not establish Windows/macOS runtime support or modem interoperability. MODEL-05
and WAVE-RX-05 remain open for independently estimated/nonideal reception.

## CE-only demonstration follow-up

Reran the actual release channel CLI with the same measured response/layout and
selection, explicitly using `NoiseModel::Noiseless`. No RxMER frame was selected
or residual noise injected. The run exported8192 paired cells, zero payload bit
errors and zero mean LDPC iterations. Response coefficients exactly match the
previous measured-noise run. Rendered and visually checked the paired plot plus
separate raw/corrected views. The renderer labels noise-disabled configurations;
paired axes are shared, while standalone views autoscale their display axes.
No simulator algorithm changed for this demonstration.
