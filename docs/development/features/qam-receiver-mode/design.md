# QAM-MODE-01: identify the equalizer that produced retained QAM

Trace UI receiver options through frozen ChannelRunSettings, WaveformLink and
WaveformReceiver to the constellation collector. MCE installs the pilot receiver
and received-only reliability estimator. Each received symbol is equalized by
its pilot-derived response; the collector keeps that actual equalized sample
alongside the raw FFT sample. Supplied channel coefficients remain impairment
inputs, not MCE receiver truth. PE shares pilot-derived correction with calibrated
reliability; CMCE uses the known supplied/model response.

Expose the recorded receiver mode in QAM metadata and show it above the plot and
in the equalized panel heading. Preserve mode with archived profile samples;
never label old samples from the current editable receiver choice or latest run.
The raw-only view remains explicitly before correction. No engine or scientific
behavior changes. Keep labels plain and full explanation in the shared-delay
hover tooltip. Unavailable runs must not claim a correction mode.

Validate the actual engine's retained raw/equalized samples across CMCE and MCE,
the Qt snapshot metadata, and QML labels for all three modes and unavailable runs.
Run the Qt check/build and update user documentation and a QAM screenshot.

## Code evidence

- `simulation/channel/link.rs`: only CMCE receives the model response; PE/MCE
  construct `WaveformReceiver::from_pilots` with no supplied response.
- `simulation/channel/runner.rs`: MCE additionally enables received-only reliability.
- `simulation/waveform/receiver.rs`: `receive` estimates from FFT bins and applies
  the estimated response before extracting signals.
- `simulation/channel/constellation/collector.rs`: retains `waveform.bins` and
  `waveform.equalized` at the same physical bin/time, checks restored sample
  identity, and stores `before_equalization` / `after_equalization`.
- Qt `receiver/selection.rs` returns those stored pairs; QML only plots them.

The MCE/CMCE engine regression passed (6.03 seconds): identical raw noisy
samples, distinct corrected samples, and pilot-derived correction provenance
for MCE. Scoped engine-test Clippy passed. QML workbench checks passed 49/49
(13.637 seconds), including all three mode labels and unavailable runs.

Full `scripts/qt-desktop.sh check` passed: formatting, Clippy, 83 adapter
tests (52.10 seconds), native debug build and 82 QML checks (20.145 seconds).
The adapter regression keeps an archived MCE mode while the latest result is
marked CMCE, proving metadata follows the selected samples. Windows runtime
validation and the full core suite were not run for this UI change.

Native Ubuntu/X11 visual review used a separate process and a temporary copy of
the adapted CMCE workbench switched to MCE. After received symbols were retained,
the debug review run was stopped; its partial result supplied the real paired
QAM screenshot. Both the recorded MCE label and pre-slicing panel heading are
visible. This visual check is not a completed-run FEC validation. The review
process was closed without changing the user's workbench or running windows.
