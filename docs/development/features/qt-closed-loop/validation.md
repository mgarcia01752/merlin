# Qt closed-loop validation

Validated locally on Ubuntu 22.04.5 x64 with the pinned Qt 6.8.3 SDK.
Ubuntu 24.04/26.04, other desktops, installed packages and hosted CI are unverified.
No commit, push or system installation was performed.

## Scoped automated checks

- Qt `scripts/qt-desktop.sh check`: formatting, warnings-denied Clippy, 13 Rust
  adapter tests, native executable build and 24 QML tests passed.
- Core `channel_job`: five tests passed, including exact simulation equality
  between baseline and paused/resumed runs, and cancellation while paused.
- Core `channel_playback`: two tests passed. A real captured Profile 0 run with
  identical interpolated endpoints preserves the single-frame run's exact
  summary; out-of-window transitions are rejected. Original epochs survive
  preparation, and phase filtering preserves observation retention budget.
- Targeted timeline/playback validation passed during integration. Core library
  and changed integration tests pass warnings-denied Clippy.
- Project tests reopen exact binary payload, binary DPD and RxMER originals,
  reject checksum corruption and exercise atomic replacement and nonregular
  destination rejection.
- The adapter integration test runs a real 1 ms source-to-compare job, verifies
  nonzero compared bits, zero errors, RX samples and matched QAM cells, then
  verifies accepted-input changes clear the retained result.

The full workspace DSP suite was not rerun for each UI iteration. Cached Cargo
artifacts were retained; the latest Qt executable is built for handoff.

## Actual native VM review

- Loaded DPD Profile 0 and RxMER demos, accepted PRBS15 and flat H=1+0i,
  configured Receiver and enabled independent comparison: all nine blocks green.
- Main Start, Pause and Resume operated on the same Rust session. The 3 ms
  observation completed with **3,614,928 compared bits and zero bit errors**.
- A fresh launch reopened the saved project and completed a 1 ms run with
  1,223,952 compared bits and zero errors. Save then persisted the edited
  duration at the same Save As destination.
- Receiver rendered actual retained payload QAM points and RX input PSD.
  The raw/corrected paired panels use the same cells and axes. RX FFT-bin power
  and actual FEC totals were also inspected; the final native log has no QML
  warnings after the RX timing-panel guard fix.
- The workbench, receiver navigation and new receiver configuration were reviewed
  at 960×640. Configuration text and controls were checked in both themes;
  documentation screenshots remain dark.
- Native Save As wrote self-contained JSON, and subsequent Save used the accepted
  destination. The document contains source/profile/measurement sections,
  receiver options and comparison acceptance.
- Native Open restored all green blocks and cleared previous run observations
  without starting a new simulation.
- Opening a project with a deliberately corrupted original SHA-256 reported an
  error and preserved the valid current workbench.

Actual dark screenshots are linked from the
[user workflow](../../../user/closed-loop-workbench.md) and
[receiver guide](../../../user/receiver-workbench.md). Local logs and disposable
project files are retained under ignored `docs/development/local/qt-closed-loop/`.

## Explicit remaining scope

The live UI supports Profile A ID 0; it does not silently run another imported
profile as ID 0. NCP matched I/Q collection, generalized carrier/time selectors,
channel-response/echo and slicer detail rendering remain pending. Decoder/NCP/PLC
summary counters are actual core results. This validates MERLIN's supported
internal closed loop, not external interoperability or arbitrary RF acquisition.
