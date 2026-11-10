# Workbench example bundles and installed Open defaults

## UI-EXAMPLES-01: Curated runnable bundles

Organize `examples/workbench/single/` with one modulation-profile PNM, one RxMER
PNM, one DS channel-estimate PNM and self-contained `workbench.merlin.json`.
`multi-rxmer/` contains the same DPD and channel estimate with several compatible
RxMER inputs and its own WB. Reuse existing sanitized bytes and synthetic stress
variants; preserve original capture epochs and label synthetic inputs honestly.
Default the multi-RxMER WB to average linear residual power, not invented capture
time evolution. Keep CLI/scenario examples in their existing locations.

Extract exact baseline originals from the existing saved WB, verifying them
against tracked upstream provenance. Move that WB into the single bundle and
regenerate its display basenames to match the adjacent capture files. Include
per-bundle LICENSE/NOTICE and provenance hashes, plus an inventory manifest.
Production Rust project validation and full request preparation must accept both
bundles, including actual DPD role/grid compatibility. No PNM binary decoding or
DSP is implemented in development Python tooling.

## UI-OPEN-01: Relocatable examples directory

All native Open pickers initially show the curated examples root, independently
of current working directory. Rust resolves it relative to the executable:
portable `files/examples/workbench`, Linux `../share/merlin/examples/workbench`,
or macOS `../Resources/examples/workbench`. Development builds locate the marked
source examples root through executable ancestors. Save/Save As retain native
destination behavior. The isolated GTK chooser receives the resolved directory
only for Open; no Qt file-browser fallback or system installation is introduced.

## INSTALL-EXAMPLES-01: Application installer contract (pending)

Future native desktop installers must ship the curated runnable folders, WB
files, manifest, notices and provenance into the matching resource layout. Install
tests must verify hashes, license inclusion, offline WB restore, OFDM compatibility
and default Open directory from an unrelated working directory. Existing CLI
staging already copies `examples/`; this does not establish desktop installer or
installed-OS runtime validation. Do not build/install/publish OS packages here.

## CH-EST-PLAYBACK-01: Multiple channel-estimate inputs (deferred)

Reserve `multi-rxmer-dschan/` as a documented future scenario. Do not ship a fake
runnable WB or silently choose one response. Define response time alignment,
complex-coefficient interpolation, update cadence, receiver reference versus
estimated response, lifecycle, schema and tests in a later task. Current response
selection accepts one estimate (or flat/unity). Mark the scenario planned and
exclude it from installer runnable inventory until implemented.

## Validation

Use scoped Qt adapter tests for production WB/request validation, inventory
integrity and Open-directory resolution. Run read-only capture hygiene and hash
checks, then native Open/restore review and targeted UI checks. Preserve retained
build artifacts. No full non-Qt simulation suite is required for example layout
and native picker defaults. Update architecture, main/UI checklists and user docs.

## Implementation and evidence

UI-EXAMPLES-01 and UI-OPEN-01 are complete. Each runnable bundle has one DPD,
one supplied response and its original RxMER files beside the embedded WB.
`multi-rxmer` includes baseline plus four existing synthetic stress inputs.
Original SHA-256 values match tracked provenance, and display basenames match
adjacent files. The former root WB moved into `single`; CLI examples stay put.

Production Rust tests restore both WBs, verify every RxMER/response key equals
the DPD key, check the 7,480-position/25 kHz grid, prepare valid full requests,
match embedded originals to loose files and validate provenance hashes/notices.
Directory tests exercise portable, Linux share, macOS bundle, source and missing
resource layouts without consulting the working directory. The Qt check passed
formatting, warnings-denied Clippy, 16 Rust adapter tests and 26 QML checks.
The targeted package-resource staging test passes and checks example hashes and
license inclusion; its placeholder executable is not installed-app evidence.

Read-only hygiene found 10 binary captures, one sanitized modem and zero files
needing changes. The existing external sanitizer was used read-only; no reference
repository or capture contents were changed.

Actual Ubuntu 22.04 VM: copied the built executable and curated resources into an
ignored portable layout and launched from an unrelated directory, using the
development Qt SDK. Native Open selected the staged examples root, and the
multi-RxMER WB restored all five compatible inputs in average-power mode.
Restarted the regular development executable from the same unrelated directory:
File Open and RxMER Open both selected the source examples root. The relocated
single WB restored accepted readiness. Native logs contain no QML warnings.
Actual dark native screenshots are in the user guide. The current executable is
built/running; no system installation or non-Qt workspace test run occurred.
Local evidence is under `docs/development/local/workbench-examples/` and
`docs/development/local/visual-workspace/workbench-examples-*`.

INSTALL-EXAMPLES-01 and CH-EST-PLAYBACK-01 remain unchecked. Local staged path
validation does not establish installed-OS behavior, pristine offline setup,
multi-response playback or runtime support on the other target OS versions.
