# High-level Rust and Python API validation

Validated locally on 2026-11-14 on Ubuntu 22.04.5 x86_64, Rust 1.85.1 and
CPython 3.10, using maturin 1.8.3 and patched PyO3 0.30.1. These are local
checkout results, not hosted CI, published packages or external modem evidence.
The original handoff archive is unchanged. No commit, push, publication or
system-wide installation was performed.

## Implemented contract

The [requirements mapping](requirements.md) and [design](design.md) describe the
canonical `docsis_ofdm_fec::api`, optional `pnm-input` capture loader and separate
`bindings/python` adapter. Existing simulation, signal processing and FEC code
is reused. The high-level API does not copy Qt orchestration or introduce another
engine. Rust owns scientific validation, residual-power operations and outcomes.

Tests cover binary and JSON profile/RxMER/channel imports, content-based kind
validation, bounded input/storage, separate original/intermediate provenance,
profile import/create/edit, exact grids and protected roles, OCD origins and
explicit bandwidth, CP/RP/depth defaults, independent capture collections,
controller-scoped IDs and atomic rejection. All four RxMER selections and
explicit/unity/nearest-time channel selection are implemented. Tests cover
missing contributors, duplicate epochs, unequal interpolation intervals, exact
original endpoints, nearest-time ambiguity and no extrapolation.

Loading and `prepare()` produce no simulation report or FEC outcomes. Only an
explicit run dispatches the existing engine. Direct-engine comparisons use the
same resolved settings/seeds and actual counters. Cancellation/runtime tests
preserve started results, unavailable denominators and not-started requests.
Complete execution status does not imply FEC success.

## Executed checks

| Check | Result and measured duration |
| --- | --- |
| `scripts/test-local.sh all`, before starting Python implementation | 395/395 workspace tests, 4/4 preparation-example tests, 136/136 no-default-feature tests; no skips; doctests passed. Total including compilation: **15m 48s**. |
| `scripts/cargo-local.sh test -p pnm-input --locked`, after rename and API refinements | **14/14 passed**, **71.20s** test time; doctests passed. |
| Direct engine parity after expanding unity/measured-response coverage | **1/1 passed**, **73.67s**; both response policies matched the direct engine. |
| Workspace Clippy, all targets, locked, warnings denied | Passed; **32.69s** including compilation. |
| Separate Python-workspace Clippy, all targets, locked/offline, warnings denied, Qt-free namespace | Passed; **0.57s** final warm check (an earlier compiled check took 9.51s). |
| Root and Python-workspace rustfmt checks; `git diff --check` | Passed. |
| Fixture-backed Rust `high_level` example | Complete with actual FEC/profile results; also built and run in the Qt-free namespace. |
| Rust `policy_parity` example | Generated fresh actual reference results for `Each`, `Individual`, `AverageAll`, `InterpolateAt`. |
| Installed native Python unittest suite against that Rust reference | **7/7 passed**, **12.283s** test time, **12.506s** command wall time. |
| Release/version tooling tests | **5/5 passed**, **0.361s**; current package/lock versions agree at 0.2.9. |
| Core, Qt and Python locked dependency audit | Zero advisories and zero informational warnings in all three graphs; no suppressions. |
| Rustdoc for high-level core and capture loader | Generated successfully in **7.27s**; six existing intra-doc-link warnings outside the new API modules. |
| Public-API skill and discovery links | Skill validation passed; both discovery symlinks resolve to the canonical skill. |

The full repository runner preceded later API-only refinements. Those refinements
and the crate rename were checked with the targeted API suite and Clippy; the
unchanged simulation/DSP/FEC engine was not subjected to a second full runner.
The final direct-engine test additionally exercises both a unity response and
an explicitly selected measured response, comparing actual status, FEC counters,
BER, rate and matching scopes.

Python checks include binary/JSON inputs, manual OCD/profile hypotheses, structured
errors, immutable values, copied options, foreign IDs, runtime continuation,
non-overwriting export, cross-thread cancellation, Ctrl-C partial results, Busy
protection, interpreter-lock release and ownership after controller deletion.
Counter/status parity is exact. Floating comparisons use absolute tolerances
of 1e-12 for ratios/power sums and 1e-6 for MER in dB and payload bit rate.
Elapsed host time and controller-scoped identities are excluded from parity.

## Independence verification

A bubblewrap mount namespace exposed system `/usr`, `/lib` and `/lib64`, the
MERLIN sources and pinned Cargo/Rust toolchain caches. A host audit found all
76 matching Qt library paths under `/lib/merlin/lib` and its `/usr/lib/merlin/lib`
alias; both installed-app directories were hidden with empty mounts. The
checkout's Qt app folder was also hidden, and external home/SDK directories
were absent. Qt environment
variables were cleared, network access was disabled and Qt build tools were
unavailable. The Rust CLI and input examples built offline; the CLI printed
`ofdm-fec 0.2.9` and the fixture-backed Rust example completed. The separate
Python adapter also passed Clippy in this Qt-free namespace.

A second namespace omitted access to the shared Rust toolchain/cache directories
and used only the local Python environment plus system paths. Checks confirmed
`cargo`, `rustc`, `qmake` and `qtpaths` were unavailable. The wheel was installed
with `pip --no-index --no-deps`, imported and executed successfully. `ldd` showed
only the platform loader, libc, libm and libgcc_s; no Qt dependency. The final checks explicitly verified the bundled QtCore library was absent
through both system aliases. The actual Python example completed with 82 observation-window codewords, zero corrected,
zero uncorrectable and BER 0 for that fixture. All seven Python tests then passed
in this same isolation against the fresh Rust policy reference.

This proves local dependency isolation and offline wheel installation. It does
not establish pristine-OS provisioning, portable manylinux compatibility or
runtime behavior on another operating system.

## Local delivery artifact

Ignored local artifact:
`local/python-wheels/merlin_ofdm-0.2.9-cp310-cp310-linux_x86_64.whl`

- Size: **1,537,105 bytes**.
- SHA-256: `63e76a13129ed193053d2a44a6d8e1227d203cefe433bb507fc3a2db6494cd69`.
- Adjacent `.whl.sha256` matches the artifact.
- ZIP integrity check passed. The wheel contains 88 locked dependency notice
  records, project LICENSE/NOTICE and embedded BCH/QAM license/notice assets.
- Patched PyO3 0.30.1 notices are included. The earlier experimental 0.27.2
  build is not a delivery artifact; its advisories were resolved by upgrading.

The helper `tools/python/build_wheel.py` regenerates notices before building.
The wheel is native CPython 3.10 Linux x86_64; no other interpreter ABI or
manylinux guarantee is implied. Source and local wheel remain unpublished.

## Remaining verification and explicit scope

Windows, macOS, other architectures and Python versions remain unverified.
Qt presentation was not changed; no new Qt/native-installer lifecycle checks
were required or claimed. Rustdoc's six pre-existing warnings remain outside
this milestone. External modem interoperability is not established by the
closed-loop fixtures. GNU Octave, continuous timeline playback, full CMTS MAC
services and simultaneous channels remain outside the agreed milestone.

Logs and isolation command manifests are retained under ignored
`docs/development/local/api-handoff/`; they are not distributed evidence in a
fresh clone. Reproduce the workflows using the tracked
[Rust guide](../../../user/api/README.md),
[Python guide](../../../user/python/README.md),
[Rust tests](../../../../crates/pnm-input/tests/high_level_api.rs) and
[Python tests](../../../../bindings/python/tests/test_parity.py).

## API-EXAMPLES-01 follow-up

Runnable source is centralized under `examples/api/rust` and
`examples/api/python`, indexed by [examples/api/README.md](../../../../examples/api/README.md).
Cargo checked both registered Rust example targets after relocation. Both
Rust and installed native Python workflow examples completed from their new
locations; Python again reported 82 codewords, zero uncorrectable and BER 0.
Rustfmt, updated relative links and `git diff --check` passed. Source behavior
and fixture bytes were preserved, so the full engine suite was not repeated.

## API-INTEGRATION-02 delivery-tooling follow-up

The separate `tools/python/update_integration_doc.py` generator now owns the
installation guide. Git save/branch push invoke refresh; bump releases refresh
and include the guide in their validated candidate/commit. Existing tags and
dry runs check without writes. Exact resume compares both metadata and guide
against committed template inputs. These paths preserve prior Git guards.

Final local tooling checks: 55 Git workflow tests passed in 27.198s, four
integration-document tests passed in 0.032s, and five release/version tests
passed in 0.242s. Tests used disposable checkouts and local-only bare origins;
no actual project commits, tags or pushes were executed. ShellCheck, Bash
syntax, Python compilation, generated-document checks, 665 changed/new Markdown
relative links, both updated skills and discovery links, and diff whitespace
checks passed. The simulation engine did not change; its suite was not rerun.

Archive ignore checks passed for ZIPs, compressed tar files and Python wheels.
No archive files were tracked; local inputs were not deleted by this change.
The required maintenance and archive policies are recorded in the
[acceptance requirements](requirements.md#mandatory-downstream-installation-document-maintenance).

## Profile catalog discovery (API-INPUT-02)

Rust-first profile discovery is implemented and verified in the current checkout:
`Dpd::profile_ids_from_bytes` for decoded JSON and `CaptureLoader::profile_ids`
for binary/JSON. Python file/byte discovery delegates to this path. The real
channel-193 example discovers 0, 3 and 4 and runs all three without hardcoded IDs.

Checks: 16 Rust API tests, 136 analysis-only tests, 7 Python API tests, all four
Rust/Python policy parity modes, workspace/adapter Clippy and formatting passed.
A native Linux CPython 3.10 wheel was rebuilt with notices and installed only
in the local test environment. See the [design validation](design.md#api-input-02-discover-captured-profile-identities-in-rust)
for timings and example counters. Windows/macOS/other interpreters are unverified;
no wheel was published and no system installation was performed.

## Loaded DPD catalog member (API-INPUT-03)

Static discovery remains available. Rust Dpd::profile_ids() and Python
dpd.profile_ids expose the complete validated catalog retained during import,
independently of the selected profile. Reading needs no file or decoding.
17 Rust API tests and 8 Python API tests passed, including access after source
removal, created/edited metadata and ownership. Four policy parity modes still
match the preserved Rust baseline. Workspace/adapter Clippy and formatting
passed; the native Ubuntu/CPython 3.10 wheel was rebuilt locally. See
[design validation](design.md#api-input-03-retained-profile-id-member-on-loaded-dpd)
for timings. Other platforms remain unverified; no artifacts were published.

## API-EXAMPLE-BER-01: scientific BER presentation

Both Python fixture examples ran against the local native wheel. `high_level.py`
completed in 2.52s and `multi_rxmer.py` completed all discovered profiles in 6.41s.
Available zero BER prints `0.000e+00`; `.3e` also renders nonzero rates such as
`1.234e-06`. Missing BER remains `unavailable`. This is example presentation only;
Rust and Python result types/calculations are unchanged.

## API-RECEIVER-01: receiver settings and UI parity

The Rust API now exposes the existing ReceiverMode before its typed Python
adapter. The supplied-channel default is unchanged. Pilot and MCE modes reuse
existing engine implementations; high-level MCE uses engine defaults. The
collection example selects PilotEstimated and explicitly matches the curated
workbench's seeds, limits and timing. A fixture drift test checks these settings.

A controlled A/B run of the same current example changed only RECEIVER_MODE
from supplied-channel correction to pilot-estimated correction. Native release
Qt was rebuilt and rerun against the unchanged multi-rxmer-2 WB; its Profile
Performance counters match the new Python result for every discovered profile:

| Profile | Total CW (both) | Previous Python corrected | Matched Python / Qt corrected | Delta | UFEC (both) |
| --- | --- | --- | --- | --- | --- |
| 0 | 83 | 0 | 0 | 0 | 0 |
| 3 | 114 | 5 | 4 | -1 | 0 |
| 4 | 124 | 19 | 30 | +11 | 0 |
| All profiles | 321 | 24 | 34 | +10 | 0 |

All profiles complete, with post-FEC BER zero and UI undetected errors zero.
Clean codewords therefore change from 297 to 287. Previously Python corrected
using the exact known unity response, whereas Qt estimated H-hat from noisy
received pilots. The existing link chooses WaveformReceiver::new with supplied
H or WaveformReceiver::from_pilots respectively. Different equalized samples
change decoder inputs and corrected counts. Pilot correction is not guaranteed
to increase each individual profile's count: Profile 3 decreases by one here.
The explicit CP, roll-off, depth, seeds and iteration constants already matched
the old defaults; exposing them removes ambiguity but did not cause this delta.
The physical unity H, RxMER residual-power average and original captures remain
identical. No scientific algorithm or reporting counter was replaced.

A subsequent sequential run after validation/build contention ended measured
4.174s supplied versus 4.553s pilot for total experiment time (+0.379s, about
9.1%). This single pair is not a benchmark. The earlier UI/Python wall-time gap
cannot be attributed entirely to this policy: UI timing, observations and host
load differ. FEC agreement is verified independently of elapsed time.

Validation on Ubuntu x86_64 / CPython 3.10:

- Rust input/API tests: 20 API plus 4 export tests pass; 60.94s including compile,
  with API tests taking 45.98s. Includes direct-engine pilot/MCE comparisons,
  all receiver/static-policy forwarding combinations and independent report ownership.
- Analysis-only core: all 136 tests pass with no default features; Qt/Python
  dependencies remain absent. Workspace/all-target and Python-adapter Clippy,
  formatting and no-default-features check pass. All 438 relative links in
  changed documentation resolve, and the generated integration guide is current.
- Native wheel rebuilt with notices and installed into the project-local test
  environment. Eight existing API tests pass in 1.332s, three receiver tests in
  27.504s, and one export test in 0.228s. Receiver tests include the actual
  all-profile fixture run and compare observed UI counters.
- Fresh Rust baselines and native Python comparisons pass for Each, Individual,
  AverageAll and InterpolateAt under all three receiver policies (12 combinations).
  Python parity tests take 4.011s supplied, 7.946s pilot and 7.542s MCE.
- Optimized Qt build passes (4m04s); native UI run confirms each profile's FEC
  and complete status. The owned review process was closed without saving the WB.

Local-only logs, baseline JSON and native UI captures reside under
`docs/development/local/api-receiver-parity/`. Concurrent validation influenced
some wall times; they are execution evidence, not performance benchmarks.
The full default-feature simulation suite was not rerun for this API forwarding
change. Windows and other Python/platform targets remain unverified; nothing was
published or installed system-wide. General WB JSON import and custom high-level
MCE tuning remain outside this change.

## API-RECEIVER-02: expanded MCE names

Rust now uses ReceiverMode::MerlinChannelEstimated and Python uses
ReceiverMode.MERLIN_CHANNEL_ESTIMATED (native variant MerlinChannelEstimated).
The old typed spellings are replaced; downstream source callers must update.
Serde still writes merlin_estimated so existing Qt/CLI/workbench data and reports
retain their wire contract, and also accepts merlin_channel_estimated as input.
The algorithms, receiver defaults and scientific results are unchanged.

All 10 MCE tests pass (19.79s), including both input spellings and canonical
serialization. The API forwarding test passes for all receiver/selection modes
(0.31s). Workspace/all-target Clippy, adapter Clippy and formatting pass.
The CPython 3.10 Linux wheel was rebuilt (1m37s) and installed only in the local
test environment. Three Python receiver tests pass (5.292s); comparison with the
preserved Rust baselines passes under all three receiver modes and all four
RxMER policies (4.025s supplied, 4.339s pilot, 4.523s MCE). This includes actual
all-profile FEC agreement with the recorded native UI reference.

The Qt optimized release executable rebuild passes (3m29s); QML controls and
serialized values are unchanged. The generated integration guide is current and
relative documentation links resolve. No full simulation suite rerun or new
platform certification is claimed for this naming-only change. Local evidence is under
`docs/development/local/api-receiver-parity/rename-*`.

## API-EXAMPLES-06: default collection MCE run

multi_rxmer.py now selects ReceiverMode.MERLIN_CHANNEL_ESTIMATED. The native
Linux CPython 3.10 example loads all 132 captures before running and completes
Profiles 0, 3 and 4 in 4.786s total experiment time. Total/corrected/uncorrectable
counts are respectively 83/0/0, 114/4/0 and 124/30/0, with zero post-FEC BER.
These happen to match this fixture's pilot-mode counts; the modes still differ
in decoder reliability estimation and need not agree on other inputs.

The saved WB is unchanged and still selects PilotEstimated. Shared-settings
tests now explicitly allow this receiver difference, and the native UI regression
comparison explicitly selects pilot mode before comparing its recorded counts.
All three Python receiver tests pass, including the retained pilot UI comparison.
No Rust/binding changes or rebuild are needed; MCE uses the already verified
shared API and engine. Full simulation and platform suites are not rerun for
this example-only change. Evidence resides under
`docs/development/local/api-receiver-parity/mce-example-*`.

## API-EXAMPLES-07: saved-UI companion

The new multi_rxmer_ui.py executes the shared collection walkthrough with an
explicit PilotEstimated mode. It matches the bundled saved WB without changing
the MCE entry point or duplicating scientific orchestration. All three profiles
complete with total/corrected/uncorrectable counts 83/0/0, 114/4/0 and 124/30/0;
post-FEC BER is zero. Native example experiment time is 6.432s. All three Python
receiver tests pass in 5.724s, including the actual companion's comparison with
native UI anchors and the shared-settings drift check. The MCE default entry
point is also rerun after introducing the receiver argument.

No wheel/core/Qt rebuild is needed for this Python example-only change. The
scripts match the bundled WB, not arbitrary later UI edits; no general WB JSON
import or new platform support is claimed. Local logs reside under
`docs/development/local/api-receiver-parity/ui-companion-*`.

## API-WB-01: native saved-WB loading

Implemented Rust-first in the Apache pnm-input adapter with core ProcessingOptions
for existing MCE/filter/loss/power settings. File/byte imports verify bounded
schema 1/2 originals and saved selections before returning idle controllers.
The optional Python Workbench delegates to the same loader and existing native
controller execution. A WB-path example supplies no scientific settings in Python.
The saved PRBS15/static single/average subset and unsupported source/timeline
modes are described in the [user contract](../../../user/python/workbench.md).

Rust validation: 20 existing API tests and 4 export tests pass (48.08s and 0.12s);
six WB tests pass in 104.68s, including actual all-profile FEC against native UI
anchors. An additional resource-bound test passes in 0.16s, including sparse-file
size rejection before allocation. Tests cover schema/profile precedence, selected
capture hashes, idle loading, original-byte ownership, custom MCE/filter/loss/power,
structured failures and shared cancellation. The single and multi-rxmer bundled
WBs with measured H also import and preflight successfully. All 136 analysis-only
tests pass; workspace/all-target Clippy, native adapter Clippy and formatting pass.
The full default-feature physics suite was not rerun for this adapter/API change.

A compiling optimized Rust WB example generated fresh baseline reports from the
unchanged multi-rxmer-2 WB. The native Linux CPython 3.10 wheel was rebuilt with
notices and installed only into the local test environment. Four Python WB tests
pass in 26.329s, checking file/bytes, source removal, saved settings, immutable
option snapshots, structured errors, independent result ownership, Rust report
parity and another Python thread cancelling native work. Cancellation during
preflight honestly returns NotStarted; after engine startup it returns Cancelled.
Eight existing Python API tests pass in 1.476s and three receiver tests in 15.159s.

The WB-path Python example completes Profiles 0/3/4 with total/corrected/UFEC
83/0/0, 114/4/0 and 124/30/0, all zero post-FEC BER. It uses saved PilotEstimated,
not the separate MCE walkthrough's default. Its measured experiment time is
9.712s during concurrent validation/builds, not a performance benchmark.
Detailed Rust/Python comparisons match status, FEC, all-completed FEC, profile
performance, options and source provenance exactly. No counters are estimated.

Evidence is local-only under docs/development/local/api-receiver-parity/wb-*.
Other interpreter/platform wheels are unverified. No publication, commit, push
or system-wide installation was performed. File payload source import, timeline
playback and incomplete editor-only WB files remain explicitly unsupported.

Final checks also pass: all 12 receiver/static-selection Python regression
combinations (5.486s supplied, 4.665s pilot, 12.995s MCE), the Python export
regression (0.042s), 454 relative documentation links, generated integration-guide
freshness and diff whitespace. Optimized Qt rebuild passes in 4m22s; its saved
wire contract and QML are unchanged. The WB processing forwarding test also
verifies an actual saved echo-filter configuration and checked-capture subset.
All seven WB tests have passed, with the six-test run plus targeted bound and
updated processing cases recorded separately. Earlier failed compilation/lint
attempts and the over-specific cancellation assertion were fixed and rerun;
only the successful final checks above establish completion.

The only new dependency is exact base64 0.22.1 for embedded WB decoding. Cargo
lockfiles were synchronized; Qt's targeted local-package update also refreshed
existing dependency edges without adding/upgrading other package versions.
The native wheel includes the project's existing legal notice collection.

## API-WB-02: supplied-channel bundled defaults

All three runnable WB documents (single, multi-rxmer, multi-rxmer-2) now save
receiver_mode supplied_channel. A structural comparison with HEAD confirms
receiver_mode is the only changed setting in each WB: all embedded capture
bytes, hashes and other settings are unchanged. The saved-UI Python companion
selects CmChannelEstimated; workbench.py imports it automatically. The separate
MCE walkthrough retains its deliberate MCE selection. Earlier pilot-mode
measurements in this report describe the previous saved configuration.

Native release UI, Rust WB example and the native Python WB API now agree:

| Profile | Total CW | Previous pilot corrected | Supplied corrected | Delta | UFEC |
| --- | --- | --- | --- | --- | --- |
| 0 | 83 | 0 | 0 | 0 | 0 |
| 3 | 114 | 4 | 5 | +1 | 0 |
| 4 | 124 | 30 | 19 | -11 | 0 |

Every profile completes with zero post-FEC BER and zero UI undetected errors.
Only receiver correction changed: supplied unity replaces the noisy pilot
estimate for channel 193. Channel-34 bundles use their measured supplied H.
All seven Rust WB tests pass in 63.98s.
Four WB Python tests pass in 9.755s with fresh Rust report parity; three receiver
tests pass in 5.034s, and six packaging tests pass in 0.710s. Packaging tests now
guard CmChannelEstimated for every runnable bundled WB. Workspace/all-target Clippy,
formatting, generated integration-guide freshness and documentation links pass.
The actual owned native UI process was reviewed and closed without saving edits.
No wheel/Qt rebuild or full physics-suite rerun is needed for changed JSON,
example choices and fixture references. Platform packaging/runtime outside the
local Linux checks remains unverified. Evidence is local-only under
`docs/development/local/api-receiver-parity/supplied-wb-*`.

## API-EXAMPLES-08: descriptive Python filenames

Renamed all four Python scripts and updated imports, current guide links,
receiver test paths and the generated downstream integration guide. Historical
validation entries keep their original names; the design records the mapping.
All four scripts passed syntax checks and executed against the installed local
CPython 3.10 Linux native extension in 14.72s total. Three receiver regression
tests passed in 9.58s, including all-profile FEC parity with native UI anchors.
The CM channel-estimate and saved-WB scripts both retained totals 83/114/124
and corrected counts 0/5/19; MCE retained corrected counts 0/4/30. All runs
completed with zero uncorrectable codewords and zero BER. No scientific settings
or Rust code changed. Qt, full Rust checks and other platforms were not rerun.

## API-EXAMPLES-09: matching Rust filenames and Cargo targets

Renamed high_level.rs to run_single_capture.rs, policy_parity.rs to
run_rxmer_selection_parity.rs and workbench.rs to
run_multi_rxmer_cmce_workbench.rs. Cargo targets match their filenames.
Updated current commands and guide links; engine code/settings remain unchanged.
Rust formatting and pnm-input example Clippy passed. All three optimized
examples built and executed successfully in 13.96s total (2.47s, 5.41s and
6.08s respectively, including Cargo startup/build time). A fresh selection-policy
reference was generated for native Python comparison. The native parity test
passed all four selection modes in 3.70s. Full Rust/Qt suites and
other platforms were not rerun for these example-only changes.

## API-EXAMPLES-10: matching Rust collection examples

Added run_multi_rxmer_mce.rs and run_multi_rxmer_cm_chan_est.rs with matching
Cargo targets. Both reuse support/multi_rxmer.rs, a numbered developer
walkthrough with named settings matching Python. All 132 original RxMER captures
and profiles discovered through CaptureLoader are loaded before explicit run.
Rust retains the same AverageAll/unity policies and per-profile execution.

Formatting and pnm-input example Clippy passed. Both optimized Rust examples
and both corresponding Python scripts executed successfully in 20.60s total:
Rust MCE 6.13s, Python MCE 4.78s, Rust supplied 5.69s, Python supplied 3.99s
(including process/Cargo build startup). Independently parsed results matched
exactly by profile/status/FEC/BER. Profiles 0/3/4 had totals 83/114/124; MCE
corrected counts were 0/4/30, supplied counts 0/5/19. All completed with zero
uncorrectable words and BER. The scripts reported loading without simulation
before explicit run. Current example/user/architecture links were checked.

Single-capture and saved-WB counterparts already exist and are unchanged.
No core API/Qt or scientific algorithms changed; no full Rust/Qt suites or
other-platform runtime checks were repeated. Logs remain local-only under
docs/development/local/api-collection-examples/.

## API-EXAMPLES-11: custom designed RxMER

Added Rust/Python run_designed_rxmer examples using the unchanged custom binary
and the exact-grid-compatible single-bundle modulation profile. All four IDs
are discovered through the Rust decoder. Individual selection, unity physical
response, CmChannelEstimated correction, seeds, timing and bounds match. The recipe
is not applied again; binary loading does not automatically ingest its sidecar.
Original and decoded profile hashes confirm the sidecar's source-profile linkage.

Formatting, example Clippy and Python syntax checks passed. Independent native
Rust/Python execution passed in 46.59s total (Rust 23.95s including Cargo startup/
build, Python 22.64s). Actual source-observation FEC and displayed BER agree:

| Profile | Total | Corrected | Uncorrectable | BER |
| --- | --- | --- | --- | --- |
| 0 | 82 | 82 | 0 | 0 |
| 1 | 102 | 102 | 0 | 0 |
| 2 | 112 | 112 | 0 | 0 |
| 3 | 122 | 17 | 105 | 1.282e-02 |

All runs completed; completion does not imply successful FEC. SHA256 checks
confirmed binary and sidecar bytes unchanged. Import-only inspection using the
local installed-extension interpreter produced no output/run. A first inspection
using system Python failed to import merlin (not installed there); the project
local interpreter succeeded. No new core API/decoder algorithms, full-suite or
Qt checks were needed; other platforms remain unverified. Logs are local-only
under docs/development/local/api-designed-example/.

## API-EXAMPLES-12: shared designed-capture location

Moved both user-supplied files into examples/measurements/designed-rxmer/
without changing bytes. Added a collection README, linked the measurements
index and updated Rust/Python paths and current guide links. The sidecar's
unchanged sibling basename resolves correctly and its binary SHA256 matches.
Both relocated examples passed in 42.99s total (Rust 22.40s including Cargo
startup/build, Python 20.59s). All profile statuses/FEC/BER match the original
location's logs exactly. Formatting, targeted example Clippy, generated-guide
freshness and 451 relative documentation links passed. An initial formatting
check caught the longer Rust constant line; cargo fmt corrected it and the final
check passed. No engine changes or full-suite/Qt/platform checks were needed.

## API-DOC-01: generated high-level references

Implemented tools/python/build_api_reference.py and a pinned, isolated Sphinx
8.1.3 development environment. The helper builds strict rustdoc for the core
high-level API and pnm-input, then uses actual native Python signatures and
docstrings to generate searchable per-class references. A metadata inventory
records the native artifact/version and all 58 class exports with 191 declared
public members. Coverage validation checks every member anchor. Factory/result
types are identified as lacking a public constructor; no scientific object is
constructed during discovery. Mock/non-native modules, version drift, unknown
export kinds, redirected/unowned output and missing members fail explicitly.
Generated sources/HTML stay under ignored local/api-reference with a shared
landing page; the helper reuses the normal Cargo target cache.

Removed hidden interpreter arguments from Python-facing source docstrings and
documented input quantity units/ranges, result units, option defaults and ownership.
Rebuilt the CPython 3.10 Linux wheel with existing notice collection and installed
it only in project-local documentation/test environments. No scientific methods
or algorithms changed. Escaped literal bracket expressions with code formatting
in the six reported rustdoc warning sites; no warning suppression was introduced.

Validation on Ubuntu x86_64 / CPython 3.10:

| Check | Result |
| --- | --- |
| Full reference build with CARGO_NET_OFFLINE=true | Passed, final rebuild 5.09s; Rust and Sphinx warnings are errors |
| Native export/member HTML coverage | 58 classes, 191 members; passed |
| Documentation tooling regression tests | 6 passed, 0.012s |
| Native Python API tests | 8 passed, 2.176s |
| Native saved-workbench tests | 4 passed, 6.159s |
| Core and binding formatting; binding all-target Clippy | Passed |
| Generated HTML local links/resources | 1,208 resolved |
| Changed Markdown relative links | 650 resolved |
| Isolated documentation dependency check | Passed |
| Native help/signature smoke check | Public docs omit interpreter arguments; units/signatures present |
| Headless Chrome rendering of generated Merlin page | Reviewed; class signature, search, parameters, results and errors readable |
| Integration-guide freshness and diff whitespace | Passed |

The first discovery attempt correctly rejected the Maturin package wrapper;
support was then added for its verified native merlin.merlin extension as well
as direct extension installs. Initial Sphinx underline warnings and two omitted
result-quantity macro descriptions were corrected before final strict builds.
An inherited PYTHONPATH exposed unrelated package metadata to the initial isolated
pip resolver; subsequent setup/check/build commands clear it. No external project was
modified or used for documentation. No credentials, private inputs or compiled
artifacts are tracked. Existing full simulation/Qt suites were not rerun for
source-documentation and generator changes. Other platforms, publication and
hosted CI integration are not verified or enabled. Local evidence is under
docs/development/local/api-reference/.

## API-WHEEL-01: release-only native wheel validation

Shared CI now enables a CPython 3.10 wheel matrix only for release tags, on
Ubuntu 22.04 x64 and Windows 2022 x64/MSVC. Native build commands preserve Linux
compatibility tags without passing them to Windows; locked notice collection
and legal payloads are retained. The test harness verifies wheel identity/ABI,
platform, notices and native import, then executes the actual single-capture
example from outside the checkout in a fresh venv with isolated Python. It
requires Complete/82 total/0 corrected/0 uncorrectable/BER 0.000e+00. Successful
wheels alone are staged with SHA256SUMS; release collection requires both
platform artifacts and retains existing CodeQL/RustSec/application gates.

Linux validation rebuilt the wheel and installed it into a disposable clean
venv. The native extension imported from that venv and the real fixture matched
all expected FEC/BER fields. Build plus installed example passed in 6.58s; the
installed-wheel phase took 4.54s. All 13 Python tooling tests passed in 0.067s
and all 23 packaging tests passed in 0.487s, including wrong ABI/platform,
missing notices, cancelled/incorrect results, missing wheel platform and
modified checksum rejection. Workflow YAML parsing verified the release-only
condition, two native targets and publication dependency/artifact requirements.

Added a separate docs/user/python/releases.md guide with patterns, scope, gates
and checksum/installation instructions; Python installation, generated downstream
integration docs and release notes link it. No push, release or system install
was performed. The Windows job is configured but has not run here; hosted
Windows runtime/native artifact generation, both hosted release jobs and actual
asset publication remain unverified. Full Rust/Qt suites were not repeated for
Python packaging/workflow changes. Local logs/reports/wheel artifacts remain
ignored under docs/development/local/python-wheel-release/.

## API-WHEEL-02: Windows linker-path failure and patch

[v0.3.0 release run](https://github.com/mgarcia01752/merlin/actions/runs/43696778449)
passed its Ubuntu installed-wheel check but failed Windows compilation. The
compiler log identifies `C:\Program Files\Git\usr\bin\link.exe`, which rejects
MSVC flags. The Git Bash shell prepended that utility ahead of the initialized
MSVC environment. The patch selects PowerShell for the Windows wheel row and
Bash for Ubuntu, with a YAML-folded shared smoke command valid in both shells.
All 13 Python tooling tests pass (0.034 s); local YAML parsing verifies both
shell choices, the expression and the folded argument vector. This is a workflow
fix only. Native Windows success remains pending the new patch run; no tests or
publication requirements are removed.


## API-OCD-01 and API-PROFILES-01: local Ubuntu validation

Bandwidth resolution now distinguishes the ambiguous 186 MHz boundary from
unique 187/192 MHz spans; oversize/overflow inputs do not infer a width. Binary
and JSON imports resolve identically, and explicit settings preserve their
origin. The narrowed fixture still requires a caller override.

The controller retains four fixture profiles (0, 1, 2, 3), resolves settings
without running, and exposes instance IDs/lookup/selection. All-profile batches
preserve current selection and retain profile IDs for cancelled entries. Tests
cover unknown/duplicate IDs, independent old results and the 256-experiment cap.
The native loader checks aggregate source bytes before materializing profiles.

Local checks: 22 Rust API integration tests (51.74 s), bandwidth boundary unit
test, targeted collection-bound check (3.69 s), Rust example compilation and
Clippy, Python API tests (10 cases, 9.99 s), receiver tests (3 cases, 4.49 s),
all four selection-mode Rust/Python parity (3.37 s), and generated
Python references (58 exported classes, warnings fatal). Both all-profile
examples complete with actual per-profile FEC results; the Python single-capture
example remains 82 total / 0 corrected / 0 uncorrectable / BER 0.000e+00.
Direct Rust/Python all-profile FEC comparisons pass. Validation uses the rebuilt
CPython 3.10 Linux wheel; this change has not been tested on Windows or newer
Ubuntu runners, and no release was published.

## API-PROTOCOL-01 — Default interpretation validation

- Default capture loader matches explicit DOCSIS 3.1 OCD and profile inventory:
  one Rust fixture test passed (0.26 s); all Rust examples compile.
- Rebuilt and installed the CPython 3.10 Linux wheel into project-local test
  environments; 10 Python API tests passed (10.01 s), including omitted and
  explicit protocol loading.
- Core/input and Python binding Clippy, formatting and Rust documentation with
  warnings denied passed. Python API reference rebuilt (58 classes, 2.57 s).
- Both changed Python examples completed: single capture reports 82 codewords;
  all-profile example completes profiles 0, 1, 2 and 3.
- No full repository suite or Windows runtime validation was performed.

## API-SIMULATION-01 — Session and timed example validation

- Rust API integration: 24 tests passed in 44.22 s. A subsequent targeted reload
  test passed in 0.47 s, including incompatible-grid rejection with retained IDs.
- Python API tests: 11 passed in 9.05 s; the full binding suite then passed
  20 tests in 26.45 s against the rebuilt local CPython 3.10 Linux wheel.
  Rust/Python parity passes all four RxMER selection modes (3.54 s separate
  Python comparison). Existing Merlin and Workbench workflows remain covered.
- Final shared example phase: four example unit tests passed; single capture
  14.19 s, all profiles 67.37 s, API total 81.56 s including incremental
  compilation. Full example phase elapsed 1 min 39 s with exit 0.
- Isolated installed-wheel single-capture smoke: passed; example-only 0.69 s,
  complete wheel validation 4.40 s. No system installation was performed.
- Tooling: 14 Python documentation/wheel tests and 12 runner tests passed,
  including failure timing and canonical/compatibility documentation coverage.
- Rust/Python Clippy, formatting, Rust examples compilation, generated Rust and
  Sphinx references passed. Sphinx documents 59 exported class names including
  the legacy alias; generated HTML remains local and untracked.
- Analysis-only feature-boundary compilation passed.
- CI YAML, shell syntax, Python example syntax and integration-guide freshness
  passed. Full repository tests and hosted Windows/Ubuntu matrix execution were
  not run; configured release timing is not hosted execution evidence.

## API-CMCE-EXAMPLE-01 — Explicit channel-estimate walkthroughs

Receiver naming follow-up (API-CMCE-NAME-01): serialization compatibility passed;
seven Rust workbench tests passed in 53.08 s and direct-engine parity in 44.57 s.
Three Python receiver tests passed in 4.57 s and four workbench tests in 5.58 s.
The rebuilt Linux CPython 3.10 wheel passed both examples in an isolated install
(4.48 s total). Rust examples compile; core and binding Clippy passed. Generated
Rust/Python reference documentation passed. Other platforms remain unverified.

Both new examples completed with observation-window TotalCW 82, CCW 0, UFEC 0
and post-FEC BER 0. Rust JSON and Python output agree. The isolated CPython 3.10
Linux wheel check passed both examples, with the new Python case taking 0.55 s
and total wheel validation 4.39 s. The timed Rust example passed in 11.74 s,
including incremental compilation. Fourteen Python tooling tests and the targeted
failure-timing test passed. Example Clippy, Rust formatting and Python syntax
checks passed. No engine change, full repository suite or hosted Windows test
was performed. Shared release checks are configured to retain per-example times.

## API-CPA-01 — Shared gain and CPA controls

- Two Rust API tests passed in 0.07 s: atomic valid/invalid setters, effective
  target gain, target override, unloaded sessions and legacy/new JSON keys.
- Six existing power/composition tests passed (composition 2.98 s), including
  independent scalar-response checks and waveform payload recovery.
- Rebuilt CPython 3.10 Linux wheel; four receiver/binding tests passed in 4.23 s.
- Core and binding Clippy passed with warnings denied. Workspace/all-target
  compilation passed in 9.38 s. Rust formatting and generated Rust/Python
  reference documentation passed (59 exported classes).
- Native XCB Qt 6.8.3 component test passed for label/accessibility, apply/cancel
  and layout behavior. Updated dark editor and diagram captures use fixture
  backend settings and do not establish full application simulation parity.
- Three playback tests passed in 29.14 s and eight waveform-impairment tests
  passed in 25.98 s, including independent-noise MER loss and gain recovery.
- Full Qt check passed: Clippy, 82 Rust tests (53.90 s), native debug build
  and 77 QML checks (13.58 s). The desktop executable is rebuilt.
- Full core suite and Windows/macOS checks were not run for this change.

## API-INSTALL-01: Python-only installer

Four offline installer tests passed (0.005 s): stable tag validation, unrelated
folder protection, dry-run mutation guards and wheel checksum acceptance/rejection.
Four integration-guide generator tests passed (0.114 s). Shell syntax, source
prerequisite check, guide freshness and diff whitespace checks passed. No system
packages were installed. Full source compilation, public release downloads and
native Windows installation remain unverified for this installer.

## API-INSTALL-02: Newer Python and CI/CD

Six installer tests pass, covering newer Python acceptance, Ubuntu releases
without an allowlist, cp310/cp312/cp314 asset selection and integrity checks.
Four integration-guide tests pass. Shell syntax, YAML parsing and local
prerequisite checks pass. The source installer built and installed the native
CPython 3.10 wheel into local/installer-validation; optimized Rust compilation
took 24.87 s and the version-only import reported MERLIN 0.3.1. No simulation
or system installation was performed. Shared Master/Release jobs now cover
Ubuntu 22.04/3.10, 24.04/3.12, 26.04/3.14 and Windows/3.14; hosted execution and
public release downloads remain unverified. The read-only source preflight
locates the installed Rust toolchain without triggering rustup downloads.
