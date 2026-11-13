# MCE implementation and validation

Status: **complete for the bounded static-channel scope**, CH-EST-RX-07..12.
The [design](design.md) preserves the intended boundaries and deferred extensions.

## Delivered behavior and ownership

MCE means **MERLIN Channel Estimation**. It is Off by default, enabled in Receiver
Configuration, and saved with the existing receiver/run settings. Omitted settings
retain the supplied-H receiver and diagonal channel; legacy `pilot_estimated`
requests keep their calibrated-noise behavior. Apply is blocked during a run;
Cancel/X changes no accepted settings. Valid changes invalidate old results and
apply to the next run.

The main Receiver block receives only a small upper-right MCE chip. Its dimensions,
position and centered heading are unchanged. Accessible descriptions and the
parent tooltip explain the enabled mode. Runtime status adds Training/Tracking;
readiness describes the existing 128-symbol history, not certified accuracy.

| Owner | Delivered responsibility |
| --- | --- |
| `signal_processing/receiver/channel_estimation` | Existing independent pilot LS/interpolation plus new `MceSettings`, `ReliabilityEstimator` and bounded `ReceiverReliability` output. |
| `signal_processing/channel/filter` | `ChannelFilterSettings`, immutable `ChannelFilterPlan` getters and `StreamingChannelFilter`; integer echoes or explicit CM reconstruction. |
| `simulation/channel` | Compose frozen channel/RX choices, preserve noise/filter history, scale received-only reliability and keep diagnostic truth outside receiver decisions. |
| Shared request/job/session APIs | `merlin_estimated`, optional `mce`/`channel_filter` settings, final estimates/reliability/fit diagnostics and a separate channel-filter tail counter. |
| Qt adapter and QML | Receiver configuration/chip, separate Channel Model editor, bounded response comparison and known-error offline help. No QML signal-processing calculations. |

The MCE receiver is constructed from pilots rather than constructing a receiver
with supplied H and then replacing it. It also excludes exact channel-noise/LLR
calibration. Continuous-pilot temporal differences supply a scalar raw-noise proxy;
the estimated inverse gain converts it into decoder variance. This is a static
white-noise estimator with conservative sensitivity to channel movement, not a
per-carrier ingress/noise reconstruction or a complete uncertainty model.

## Bounded channel model

Sparse filters use at most 32 nonzero paths. Dense filters reuse 8192-point Rust
FFTs with overlap-add and at most 4096 input samples per chunk. Support is capped
at 2049 coefficients / 2048 delay samples. Streams retain history across chunks,
symbols, codewords and PLC boundaries, then flush L−1 samples once. Filter-tail
accounting is separate from TX window overlap and interleaver drain. Excluded-bin
nulls do not invalidate reference or legacy-pilot correction on usable carriers.

CM reconstruction uses explicit unity extension outside active simulation bins,
causal shift and hard truncation. It checks absolute complex RMS/maximum error on
active bins and reports discarded energy. Missing active observations still reject.
At initial implementation, synthetic paths rejected imported H and additional tilt.
The later composition extension described below permits diagonal tilt/cable loss. Scalar power conditioning remains explicit.
Response-epoch changes reject for continuous filters; static-H RxMER residual
playback can continue without resetting filter/noise state.

## Scientific evidence

Sources and their exact role are listed in the [MCE guide](../../../user/receiver-estimation/mce.md#sources-and-engineering-choices).
Pinned PHY text was checked for pilot purpose/periodicity, BPSK reference and
boost; no new rendered-PDF verification is claimed. LS division follows the
known-pilot signal model. Interpolation, temporal differencing, smoothing, floors
and filter reconstruction are MERLIN engineering choices, not a copied complete
receiver or a uniquely mandated DOCSIS algorithm.

| Validation | Result |
| --- | --- |
| Full `scripts/test-local.sh all` | 379 workspace, 4 preparation-example and 132 analysis-only tests pass; doctests pass. No quick/permutation skip option used. |
| Core warnings-denied Clippy and formatting | Pass. |
| Qt check | 66 Rust adapter tests, 57 QML tests, warnings-denied Clippy, formatting and current native build pass. |
| Independent temporal-noise math | Known raw pilot differences give independently calculated variance; malformed/sequence failures preserve previous state. |
| Continuous filter math | Analytic impulses/two-path delays and a dense 65-tap direct-convolution oracle match sparse/FFT streaming at both spacings and multiple chunk splits. Bounds, tails and rejected fits are covered. |
| Actual MCE waveform/FEC | Noisy echoes at both spacings, 8- and 256-sample delays recover bits; strong 2048-sample echoes produce actual degradation/failure. |
| Independence | Changing only simulator LLR calibration leaves MCE outcomes identical. Switching reference/MCE preserves actual matched noisy channel samples. |
| UI behavior | Default Off, draft cancellation, Apply, busy guard, averaging persistence, separate channel selection, chip visibility and unchanged heading geometry pass. |
| CLI example | `examples/cli/channel-mce.json` completes; estimated raw noise is approximately 9.56e-6 for injected 1e-5 sample noise, BER zero, 8 channel-tail samples. |
| Native Ubuntu CM-driven MCE | 23 observation symbols, 1,223,952 globally compared payload bits, zero bit errors. Actual dark/light configuration, chip and H/H-hat views inspected. |

The native count includes globally recovered payloads, including completion/drain;
it is not a statement that every compared word lies inside the observation window.
Window-specific TotalCW/CCW/UFEC retain their existing separate accounting.

The shipped single-channel CM capture **does not fit the default finite-filter
limits**: with 2048 delay samples and zero causal shift, RMS error is 0.122731 and
maximum error 1.195106. An explicit 128-sample shift lowers RMS to 0.017039 but
maximum error remains 0.735716, above the default 0.5 limit. Both are rejected;
tolerances were not loosened to obtain a success. The same supplied response
works with MCE through the existing diagonal model, as the native run confirms.
Do not claim arbitrary measured-capture recovery through the new FIR model.

Validation host: Ubuntu 22.04.5 x86_64, Cargo 1.85.1 and Qt 6.8.3. Native helpers,
workbench clones, result JSON and logs remain ignored under
`docs/development/local/independent-channel-estimation/`; no original captures or
shipped workbenches were modified. Other native platforms, installed packages,
external modem interoperability and physical cable measurements were not tested.

## Remaining extensions

Fractional delays, time-varying channel-response transitions, alternative CM
extension/fitting policies, spatial held-out pilot scores, frequency-selective
noise estimation, adaptive time-domain equalization and fully blind acquisition
remain separate work. Successful closed-loop tests do not establish those features.
See the [receiver guide](../../../user/receiver-estimation/mce.md) and
[filter guide](../../../user/impairments/waveform-filter.md) for current use/limits.

The later [channel composition change](../channel-workbench/design.md#ui-ch-02--wave-ch-compose-01-graphic-model-controls-and-composition) permits diagonal tilt/cable loss before synthetic echoes. Imported H and echo paths remain alternative response sources.
