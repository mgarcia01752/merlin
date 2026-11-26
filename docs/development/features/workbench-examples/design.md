# Workbench example bundles and installed Open defaults

## UI-EXAMPLES-03: Directional bundles and runnable upstream projects

The user requires delivery on `master`. Local merge `bc37be7c` preserves both
parent histories and brings the accepted unified Qt application to master. Its
existing public `merlin::common::Project` / native `merlin.common.Project` admit
direction-tagged schema-3 files. Do not introduce another loader or public API.

Move existing downstream bundles under `examples/workbench/downstream/` without
changing saved documents, original captures, hashes or licenses. Add
`examples/workbench/upstream/` with four independently saved, self-contained
projects: ideal 25/50 kHz and synthetic measured-RxMER 25/50 kHz. Reuse the
existing admitted configuration and measured-plan fixtures exactly; no new
scientific modes, algorithms or public API are needed. Loading remains idle.

The root manifest remains schema 1, adds an explicit direction per bundle and
uses paths relative to the common examples root. Preserve downstream bundle IDs
and the deferred multiple-response scenario. Native Open still starts at that
root. Update source examples, tests, guides, installer inventories and path
references. Keep historical acceptance states and numerical evidence intact.

| Owner / symbol | Existing contract and gap | Disposition |
| --- | --- | --- |
| `examples/workbench/*` | Four runnable downstream bundles and one deferred folder; no upstream saved projects. | MOVE under `downstream/`; ADD four `upstream/` bundles with source documents and provenance. |
| `examples/cli/upstream/us-{25,50}khz.json` | Supported generic synthetic configurations with explicit modes/budgets. | REUSE unchanged as embedded configuration originals. |
| `examples/measurements/upstream/measured/capture-driven*.json` | Complete synthetic measured cohorts with explicit identities/reference planes. | REUSE unchanged; retain original capture bytes and synthetic limitations. |
| `crates/pnm-input/src/project` and facade re-export | Existing idle original-preserving schema-3 admission and independent sessions. | REUSE; add fixture acceptance and separate-process example reference. |
| `crates/pnm-input/tests/directional_workbench_examples.rs`, native `test_directional_workbench_examples.py` and `tools/testing/native_api.py` | New file-based admission, original identity, four complete upstream runs and separate-process native parity. | ADD/ADAPT; retain all prior tests. |
| Native `merlin.common.Project` | Same shared Rust validation and execution. | REUSE installed wheel; add direct saved-file parity cases. |
| `apps/qt/src/upstream/workbench/project_tests.rs` | Production asynchronous file/session owner. | ADD one test opening idle and completing all four actual saved examples. |
| `apps/qt/src/downstream/workbench/project_file.rs` | Curated test assumes every manifest row is downstream. | ADAPT test selection by direction; no simulation behavior changes. |
| `tools/packaging/{test_packaging,desktop_smoke}.py` | Inventories and provenance check four downstream bundles. | ADAPT to both directions; retain integrity/license/admission gates. |

Requirement: [REQ-UI-EXAMPLES-03](../../../requirements.md#req-ui-examples-03).
Task: [UI-EXAMPLES-03](../../checklist.md#task-ui-examples-03).
Architecture: [curated examples](../../../architecture.md#curated-workbench-examples-and-open-location).

Acceptance: preserve downstream binary/WB hashes across relocation; resolve
active paths and internal links; load all manifested saved projects through the
public Rust owner; execute all upstream projects with actual decoder/source
comparison; verify native Python against separately executed Rust outcomes and
settings, round-trip ownership and original provenance; validate Qt saved-file
admission and actual upstream Open/readiness; run affected packaging/resource,
formatting, Clippy and traceability checks. Reuse caches and the installed native
wheel built for the integrated master candidate. The prerequisite merge makes
this a Level 3 milestone: run the full applicable combined regressions once,
then reuse the matching artifact for the saved-file cases. Record
executed evidence and deferred gates before checking the task.

### UI-EXAMPLES-03 executed validation

Accepted on master after the integrated Level 3 gates in the
[execution receipt](directional-examples-evidence.json). All 930 workspace cases,
140 analysis-only cases, four preparation cases, 14 API examples and the doctest
pass. Optional facade native-input contracts pass all 29 cases. Complete Qt
acceptance passes 648 Rust cases and 441 QML records; the separately invoked
actual GUI artifact gate passes, followed by three installed-native project
comparisons. The optimized 0.5.1 executable admits all eight workbenches.

All 221 installed-wheel tests pass, including four new upstream file runs. Fresh
compiler/native exports and 417 exact executed Rust case mappings validate the
637 Rust / 608 native operation rows. The decoder workspace passes 102 cases;
packaging passes 26 cases. Real native Open/Save/dirty/cancel/rejection operations,
scientific CLI smokes and the extracted portable CLI archive pass. Its eight
bundles retain all manifested bytes, hashes and legal files. All 186 relocated
non-Markdown downstream files match the original baseline; 43 example links
resolve. Registry identities/checksums remain unchanged across all three locks.

The first installed run failed nine cases through one stale fixture root. The
complete corrected run reuses the exact wheel and successful Rust references;
no failed run is reported as passed. Other compilation/harness corrections and
existing unsuppressed warnings are retained in the receipt. Three existing host
benchmark skips remain explicit; the ordinary ignored GUI artifact case was
executed separately against fresh desktop exports. Hosted/Windows/installed
lifecycles and publication are separate gates. The source checkout's latest
production executable is built. No release/tag/version change is made.

Branch cleanup preserves unique commits in a verified Git bundle and retains
all dirty auxiliary worktrees, with source patches and untracked-file archives
under ignored `docs/development/local/directional-workbench-examples/branch-backup/`.

## UI-EXAMPLES-01: Curated runnable bundles

Organize `examples/workbench/downstream/single/` with one modulation-profile PNM, one RxMER
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

## UI-EXAMPLES-02: Channel 193 measured RxMER collection

Add `multi-rxmer-2` from the pinned public PyPNM demo: all 132 channel 193
RxMER binaries and its companion modulation profile. Preserve original epochs,
Git blob identities, source/current SHA-256 and upstream notices. The profile
header retains an unsanitized MAC while all RxMER headers use the demo alias;
normalize all bundled copies to `00:ca:00:00:00:01`. Only the evidenced six-byte
MAC field changes; reverse reconstruction must reproduce each source hash.
Verify grid/device/channel association and selected profile through the existing
Rust importer and full request preparation. No matching DS channel estimate is
available; persist `response: null` (flat/unity), never substitute channel 34.

Raise the shared capture-count limit from 64 to 256 across core playback,
project restore, staging and native selection; retain 64 MiB original-input and
4,000,000 carrier-value limits. Validate the new count boundary and this actual
collection. Default to average linear residual power: original captures include
a long gap, and fixed-symbol playback is an explicit simulation clock rather
than the irregular acquisition cadence. Include installer inventory and a guide.


UI-EXAMPLES-02 is complete. The collection restores all 132 distinct RxMER
captures with Profile A/ID 0 and a common 7,600-position/25 kHz grid spanning
835.000–1024.975 MHz. Original PNM header epochs span 1764820676–1764824335;
the profile epoch is 1764824372 (37 seconds after the last RxMER). An explicit
7,200-second association tolerance covers the collection. Original timestamps
are slightly earlier than filename times. The gap is preserved in the source
history; average-power mode does not invent a playback cadence.

Validation: all 23 Qt adapter tests pass, including full request preparation
for all three bundles, every grid/identity, embedded-versus-loose bytes and
reconstruction of all 133 upstream SHA-256 values after undoing only the MAC
alias. All three focused core playback tests pass, covering 256 accepted versus
257 rejected captures and the unchanged four-million-value limit. All six
packaging resource tests and 29 QML checks pass; Qt and focused core Clippy
and formatting pass.
The Qt executable is rebuilt. Native interaction automation did not establish
Open/restore on this VM; no screenshot from that attempt is used as evidence.
No whole-workspace test run, installer or publication was performed. Logs and
unchanged upstream originals remain under ignored
`docs/development/local/multi-rxmer-2/`.
