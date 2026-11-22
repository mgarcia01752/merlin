# Unified Qt validation and acceptance

This record distinguishes inspected planning material, historical backend
evidence and per-task runtime acceptance. The [official checklist](../../checklist.md#unified-qt-workbench-delivery)
owns current completion. Initial documentation checks below establish no runtime
acceptance. Subsequent named task records report only their executed evidence.

## UIU-PLAN-01: accepted planning baseline

The user's [continuous execution instruction](execution-authorization.md) accepts
the plan at `63844f8` and authorizes implementation/local task commits. The
planning audit was rerun before task acceptance: all 178 original Qt hashes,
27 unique requirement IDs, 21 tasks, 31 acyclic edges, requirement/architecture/
checklist return links, all 19 master sections and original prompt bytes pass.
Historical checkbox states and explicit anchors remain preserved.

Current-mode feature documentation, backend integration traceability, version
metadata, Python guide consistency and diff whitespace checks pass. No scientific
or Qt runtime acceptance is claimed by this planning task. Execution continues
with UIU-BASE-01's fresh Qt baseline.

## UIU-BASE-01: fresh Qt baseline accepted

Candidate `e033152f815fb896deeab20b02a6c3d69978cc8f` passes the fresh Qt baseline.
The [machine-readable record](baseline-evidence.json) includes exact case names,
command outcomes, source-inventory/log/executable hashes and native output.
`scripts/qt-desktop.sh check` passes format, warnings-denied Clippy, all 87 Rust
tests, native executable build and 89 offscreen/software QML checks in 296.682 s.
The real downstream closed-loop, profile sweep, calibrated/estimated receiver,
Designer and project-restoration cases pass within the Rust adapter suite.

The fresh executable accepts all four curated workbench files. Two XCB desktop
smokes (empty and single-channel project) exit successfully with Fusion style
and no stderr/import/type errors. Current DISPLAY/XAUTHORITY were discovered;
preferences were isolated without modifying a user's open workbench. All 178
application hashes still match the original inventory. No caches were cleaned.
This is Linux checkout evidence; Windows, installed updates and a manually
driven native run are not claimed. UIU-BOUND-01 is the next eligible task.

## UIU-BOUND-01: public API boundary accepted

The [boundary guard](../../../../tools/validation/qt_api_boundary.py) and its
[independent tests](../../../../tools/validation/test_qt_api_boundary.py) pass.
Sixteen boundary cases cover package aliases/path identity, dev/build/target
dependencies, qualified/grouped/extern imports, comment/raw-string handling,
unchanged-import/new-alias-call rejection, stale exceptions, source escapes,
unknown package identities and final zero-debt mode. The native picker's `pnm`
boolean is correctly classified as infrastructure rather than a crate reference.

All 73 existing/new validation-tool tests and seven development-tool tests pass.
An initial tool run found the new JSON record lacked a navigation link; adding
the design/architecture links resolves it and the full tool suite passes.
Actual locked Cargo metadata/source audit passes with three legacy scientific
dependencies, 45 frozen files and 145 root-reference sites. The
[legacy record](legacy-qt-boundary.json) ties these to UIU-DS-01 removal; unknown
direct packages fail closed. Qt check and Master preflight call the guard.
Shell syntax, ShellCheck and diff whitespace checks pass. The
[exact boundary evidence](boundary-evidence.json) identifies tested source bytes.

Application source and scientific behavior are unchanged, so the fresh baseline
executable remains current; no core/Qt runtime rebuild was needed for this tool
unit. Hosted CI execution is not claimed. Migration-gate acceptance does not
satisfy final Qt acceptance while any scientific exception remains.

## Planning checks

Candidate: `feature/unified-qt-workbench` based on
`23f63c9558ef659830338f6a61080705ce3d55f1`, with this uncommitted documentation
addition. The inventory pins original application bytes independently of these
planning documents. Planning checks must verify all 178 tracked Qt paths/hashes,
27 unique new requirement IDs, 21 unique tasks, nonempty exact requirement sets,
requirement→task and task→requirement links, architecture ownership, known acyclic
dependencies, full master-section routing and preserved historical task states.
Run general current-mode feature documentation and integration traceability
checks, version/guide consistency and diff whitespace. Actual planning results
are recorded below. Do not use historical documentation-migration mode
or modify completed backend inventories to include this new UI phase.

Planning results, 2026-11-21:

- Complete inventory/ad hoc source audit: 178/178 application hashes match both
  current bytes and accepted HEAD; all destinations are unique and all recorded
  consumer/test paths exist. No application source was changed.
- New traceability audit: 27 unique requirement anchors, 21 pending tasks and
  31 known acyclic dependency edges; exact nonempty task requirement sets,
  requirement/task return links, architecture return links, all 19 master
  sections and original prompt bytes verified. Original explicit anchors and
  every historical checkbox line/state remain present.
- `python3 -B tools/validation/feature_documentation.py`: passes in current mode;
  its registered historical inventory covers 520 baseline IDs and 430 downstream
  task mappings. The additional 27 UIU IDs were checked separately above; the
  existing tool's count is not an assertion that it registered this new phase.
- `python3 -B tools/validation/integration_traceability.py`: passes existing
  backend task/requirement/architecture and recorded implementation chains.
- `python3 -B tools/release/versions.py`: version metadata agrees at 0.4.0.
- `python3 -B tools/python/update_integration_doc.py --check`: guide current.
- `git diff --check`: passes. Changes are documentation only and uncommitted.

These checks accept neither the UI planning decision on the user's behalf nor
new runtime functionality. No Qt/scientific/Python runtime suite was rerun for
documentation-only work; fresh UI baseline execution remains UIU-BASE-01.

## Historical baseline

The accepted backend checkpoint retains
[regression evidence](../unified-backend/regression-evidence.json),
[documentation evidence](../unified-backend/documentation-evidence.json) and
[acceptance evidence](../unified-backend/acceptance-evidence.json).
These are prior evidence, not reruns on new UI work. The accepted Qt source
organization [UI-ORG-01/02 record](../qt-source-organization/design.md) also remains
historical; its file/test counts describe that milestone, not today's inventory.
Do not reopen completed backend/FEC planning or overwrite accepted manifests.

## Baseline

UIU-PLAN-01 requires review/agreement on the actual planning files, source
inventory and task graph. UIU-BASE-01 then records a fresh pre-migration candidate,
existing Qt Rust/QtTest checks, native executable build and startup, saved examples
and bounded downstream run. Capture exact commands/toolchains, source hashes,
results and any pre-existing failures before code moves. Preserve build caches.

## API

UIU-BOUND-01 verifies actual dependency identities and direct source references,
with negative renamed/path/fully-qualified cases. Migration exceptions must be
exact existing sites with sunset owners. Every API extension task finalizes
signatures/fields/budgets/errors/parity before code, runs independent scientific
vectors and meaningful API compatibility tests, installs the matching wheel in
an isolated environment and verifies namespace/operation/project parity there.
Use relevant existing core/analysis/no-default-features checks; broaden validation
when shared owners/dependency contracts change. Commit the validated API unit
before a Qt consumer. Never change scientific expected values to mask drift.

Grid tests cover literal physical coordinates, accepted patterns/numerologies,
roles/counts/order, bounds/overflow/missing states and idle no-mutation behavior.
Run/observation tests cover actual stages, configuration/run identities, paired
data, PSD normalization/units/reference planes, decoder partial outcomes and
old imports/defaults/results. Native inputs preserve original bytes/hashes,
timestamps, grids/nulls/exclusions and admitted measurement combinations.

## Organization

UIU-ORG-01/UIU-DS-01 validate each focused file move: production QML registered
exactly once, QObject files together, maintained imports/includes/resources,
unchanged entry URL/module/target identity, license/NOTICE preservation, existing
Qt Rust and QtTest cases and a freshly built native executable. Compare actual
downstream configuration/readiness/results with the baseline, not just screenshots.
Qt-only moves use targeted Qt checks; no unnecessary full workspace rebuild.

## Shell

UIU-SESSION-01/UIU-SHELL-01 test two populated sessions, dirty/path/selection
isolation, active run/paused-slot ownership, second Start rejection, tab switching,
scoped controls, late/stale/failed/cancelled callbacks, destruction and bounded
retention. Verify File/Settings/About/updates, removed internal canvas title,
persistent lower-right controls, focus/keyboard/accessibility, themes and dialogs.
Tests should enable later dispatch-policy changes without assuming shared science
state. Future actual concurrent scheduling needs its own aggregate-limit tests.

## Upstream

UIU-US-01 requires the user's diagram/final layout input before visual
implementation. Verify CM→channel→CMTS samples versus control arrows and all
H/C/C×H/RxMER labels. UIU-US-02/03 exercise actual admitted synthetic and measured
runs/input combinations, progress/cancel/partial outcomes and isolated DS state.
Unsupported combinations are rejected by the public API with consistent UI errors.
UIU-US-04 compares displayed cells against authoritative snapshots and checks
axis direction, grant/IUC/pattern/role readouts, pan/zoom/selection, batched object
count and bounded viewport responsiveness on the target Qt build. A compiled
placeholder or mock-only scene cannot establish upstream acceptance.

## Instruments

UIU-SHARED-01/02 require two real direction consumers per extracted scientific
renderer, with direction tests and actual API data. Verify QAM pairing/stage/
physical identity, spectrum units/window/normalization, response reference planes,
FEC family/actual iterations/word status/LLR bounds, PRBS recurrence/seed/order,
BER denominator, partial results, original measurement provenance and explicit
unavailable states. Keep DS PLC/NCP overlays and BCH semantics out of upstream
data displays. Pure pixel transforms are reviewed separately from numerical science.

## Persistence

UIU-API-05/UIU-FILE-01 validate legacy schema 1/2 examples and proposed explicit
direction schema 3 round trips in UI, public Rust and installed Python. Check
malformed/oversized/wrong-direction/hash/version errors, atomic admission, target
tab dirty guards, picker/write cancellation, no unintended input/export overwrite
and app-close handling for both dirty tabs. UIU-RECOVERY-01 validates a bounded
private bundle of both API-valid projects and restore state, including errors.
Updater integration tests never install/elevate from a source checkout. Native
polkit/UAC/installed lifecycle remains pending until actually tested in an
appropriately authorized environment; preserve existing pending QA tasks.

## Acceptance

UIU-QA-01 requires all dependencies accepted with actual MERLIN-workspace
evidence, both direction runs, final zero scientific migration exceptions,
installed-wheel and affected CLI/core regressions, scientific presentation,
persistence/recovery and freshly built Qt executable. Resolve maintained links,
reconcile final source inventory/owner/skill/guide correspondence and licenses,
and report exact source candidates, tests, outcomes, native platform limits,
remaining scope deferrals and focused commit chronology. New runtime evidence
belongs here or in linked per-unit records; do not rewrite historical acceptance.

## UIU-API-01 input admission unit

This independently validated unit implements optional native-input facade wiring
and passive typed transmission/source admission. UIU-API-01 remains unchecked:
measurement/channel/Designer editor operations and bounded idle TX preview still
need public API coverage before Qt adoption. No Qt source changed in this unit.

[Exact source/test/wheel record](input-admission-evidence.json) links the accepted
candidate to [architecture](../../../architecture.md#unified-qt-public-api-boundary),
[task](../../checklist.md#task-uiu-api-01), the
[design](design.md#uiu-api-01-typed-transmission-and-source-admission) and
[method parity/limitations](../../../user/api/input-admission.md).

- Four facade-only native consumers and six existing namespace tests pass.
- Five API admission cases cover independent integer timing anchors, incompatible
  profiles, malformed/oversized settings, byte identity, empty EOF, seed/target
  bounds and retained ownership without running a source.
- All 68 native-input adapter tests pass, retaining previous project/Designer/FEC,
  binary and decoded input, export, provenance and measured association checks.
- A fresh CPython 3.10 Linux wheel passes all 52 installed native API tests,
  including the four new admission cases and existing direction import identity,
  actual direct-Rust process parity, saved workbench, Designer and cancellation
  checks. Python has no direct scientific/native-decoder dependency.
- Common-only, downstream-only and upstream-only facade builds, formatting and
  facade/native binding Clippy with warnings denied pass. Lock changes move only
  the local adapter edge; dependency pins/versions and default features remain.

This is local Linux acceptance, not a published-wheel, external interoperability,
Qt migration, Windows or installed-desktop lifecycle claim. Historical backend
acceptance and existing downstream behavior stay in their original records.

## UIU-API-01 original measurement preview unit

`RxMer::preview` and `ChannelEstimate::preview` now expose the existing bounded
original-observation viewer through the high-level facade. The
[exact candidate/evidence](measurement-preview-evidence.json),
[typed design](design.md#uiu-api-01-original-measurement-preview-unit) and
[Rust/Python parity](../../../user/api/input-admission.md#original-measurement-previews)
retain the parent [task](../../checklist.md#task-uiu-api-01) and
[architecture](../../../architecture.md#unified-qt-public-api-boundary) links.

Three new Rust cases and the existing lower-owner case pass: literal frequency,
signed H/phase/dB/gap anchors, immutable original ownership/provenance, complete
1024-group limits and viewport overflow/rejection. Facade and native-binding
Clippy with warnings denied and both format checks pass. A fresh Linux CPython
3.10 wheel passes all 55 installed native cases, including three new preview
cases; actual Rust reference processes and existing WB/Designer/FEC parity pass.

The first packaging attempt used maturin's default manylinux tag and was rejected
by the existing local-wheel verifier. The expected Linux tag resolves it. Native
regression then exposed a test-only timing race: its 1 ms experiment could finish
before its 50 ms cancellation request. The cancellation test now uses a 1000 ms
observation with a 1000 ms safety deadline and preserves every outcome assertion.
The targeted case and full 55-case native rerun pass; successful fresh Rust
reference outputs remain the comparison evidence. Product behavior is unchanged.

UIU-API-01 remains unchecked for collection combination/playback, Designer,
channel composition and idle TX previews. No Qt source, scientific kernel,
completed historical task or accepted downstream API behavior changed here.

## UIU-API-01 measured sequence unit

The immutable `MeasurementSequence` and `TransmissionConfiguration::check_response`
reuse existing physical measurement/response admission and preserve original order,
identity, epochs, bytes and explicit tolerance. [Exact candidate/evidence](measurement-sequence-evidence.json),
[typed owner/design](design.md#uiu-api-01-measured-sequence-admission-unit),
[Rust/Python parity and bounds](../../../user/api/input-admission.md#measured-sequence-admission-and-preview)
and [architecture](../../../architecture.md#unified-qt-public-api-boundary)
trace back to the still-open [parent task](../../checklist.md#task-uiu-api-01).

Five new API/numerical cases and all three existing playback cases pass. Independent
anchors cover three-source linear power average and every contributor, interpolation
half-step, exact synthetic cadence, boundary/last hold, unavailable conversion,
original ownership, duplicates/counts/epochs/coverage/mode/overflow rejection and
explicit unity/captured-H tolerance. Both facade/native-binding all-target Clippy
checks with warnings denied, format checks and the analysis-only test suite
(139 passing cases) pass. The final rebuilt Linux CPython
3.10 wheel passes all 59 installed native cases, including four sequence cases,
parallel immutable queries, old imports, actual Rust references and WB/Designer/FEC
regressions. A final review tightened overlong-list rejection before traversal;
only the final candidate/wheel hashes below establish this unit's acceptance.

No original timestamps are combined into a fictional average epoch. Equivalent
RxMER previews and captured RxMER previews remain different typed results/units.
No Qt scientific code, saved-project format, historical task state or existing
Simulation behavior changed. Designer, channel composition and idle TX previews
remain open before UIU-API-01 completion; legacy temporal project support stays
under UIU-API-05.
