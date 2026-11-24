# Unified Qt validation and acceptance

This record distinguishes inspected planning material, historical backend
evidence and per-task runtime acceptance. The [official checklist](../../checklist.md#unified-qt-workbench-delivery)
owns current completion. Initial documentation checks below establish no runtime
acceptance. Subsequent named task records report only their executed evidence.

Current scheduling follows the [three-level risk-based testing strategy](testing-strategy.md):
focused commit checks, complete relevant official-task acceptance and full
applicable milestone/final regressions. Mandatory expansion covers all affected
consumers of shared science, API compatibility, CXX-Qt registration, project
serialization and cross-direction lifecycle changes. Existing acceptance sections
and historical records below remain intact; record executed/not-required/deferred
checks explicitly for each subsequent unit.

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

UIU-US-01's [user-approved layout](upstream-startup-layout.md) was supplied on
2026-11-22; its visual-input gate is satisfied. After shell dependencies pass,
verify actual rendering, clickable blocks/drill-ins, connector roles, keyboard/
focus behavior and resizing against that layout. Approval alone does not pass
these checks. Verify CM→channel→CMTS samples versus control arrows and all
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

## UIU-API-01 Designer admission unit

`inputs::RxMerDesign` wraps the existing native saved-recipe evaluator with typed
immutable baseline/position/replacement operations. Exact `Dpd::key` and existing
measurement source-kind metadata remain accessible through Rust/native Python.
[Exact source/test/wheel evidence](designer-admission-evidence.json),
[design and owners](design.md#uiu-api-01-immutable-rxmer-designer-admission-unit) and
[Rust/Python method parity](../../../user/api/input-admission.md#immutable-rxmer-designer-inputs)
link to the [architecture](../../../architecture.md#unified-qt-public-api-boundary)
and still-open [parent task](../../checklist.md#task-uiu-api-01).

Nine native adapter cases (three existing unit, four new admission, two saved-WB),
11 facade cases and six existing independent numerical Designer cases pass.
The new cases preserve original envelope bytes/masks/epochs, typed profile identity,
source-kind labels, literal tilt endpoints and original-baseline replacement.
Version/position/nesting/random/recipe-byte/edit/value limits and failed edits
reject atomically. Existing real saved-WB FEC/reference parity remains passing.
Warnings-denied all-target Clippy passes for facade, native input and bindings;
format/version/Python-guide checks pass. The final rebuilt Linux CPython 3.10 wheel
passes all 63 native cases, including four new Designer cases and bounded pre-copy
original-input rejection. Only its recorded final source/wheel hashes establish
this unit's acceptance; Qt source and scientific kernels are unchanged.

No source document, capture or completed checklist history was replaced. New typed
configuration aliases remain re-exports; scientific evaluation stays with its
existing Rust owner. Profile scientific summaries, channel composition and idle
TX previews still keep UIU-API-01 unchecked before Qt consumer migration.

## UIU-API-01 profile inspection unit

Read-only `ProfileCapture` exposes original complete source catalogs/schemes and
bounded authoritative metric views. Existing Dpd summary/view reuse its numerical
owners; legacy ProfileId remains 0..15. [Exact candidate/test/wheel record](profile-inspection-evidence.json),
[owner/design](design.md#uiu-api-01-profile-capture-inspection-unit),
[Rust/Python contract](../../../user/api/profile-inspection.md) and
[architecture](../../../architecture.md#unified-qt-public-api-boundary) trace to
[UIU-API-01](../../checklist.md#task-uiu-api-01).

Five new core cases plus five existing parser/contract cases pass. Independent
vectors preserve ID 200 and unassigned code 255, source order/header/frequencies,
PLC data/preamble separation, raw zero/absent loadings, role counts, uncoded
capacity and ideal Shannon extrema. Legacy/opt-in imports yield identical
scientific profiles/errors; old callers omit inspection arrays. Twelve facade
cases verify binary/JSON scientific equivalence and retained original bytes.
All 72 native-input cases pass, including existing WB/Designer/real-FEC history.
Warnings-denied facade/native-input/binding Clippy, format/version/guide checks
pass. A fresh Linux CPython 3.10 wheel passes all 67 native cases, including four
profile-inspection cases and actual direct-Rust reference/WB/Designer regressions.

Source recognition preserves the full encoded byte catalog without broadening
legacy simulation admission or claiming a new coding/receiver mode. UIU-API-02
and UIU-DS-01 retain the explicit source-ID full-loop compatibility gate.
No Qt source, normative assignment table, Shannon/rate kernel or completed
historical checkbox changed. Channel composition and idle TX previews remain
open before parent task acceptance and Qt consumer migration.

## UIU-API-01 channel preview unit

Immutable `ChannelConfiguration` validates existing loss/power/filter parameters
and obtains authoritative configured transfer from existing scientific owners.
[Exact candidate/test/wheel record](channel-preview-evidence.json),
[design/owners](design.md#uiu-api-01-configured-channel-composition-preview-unit),
[Rust/Python contract](../../../user/api/channel-preview.md) and
[architecture](../../../architecture.md#unified-qt-public-api-boundary) link to
[UIU-API-01](../../checklist.md#task-uiu-api-01).

All 24 targeted core/API/channel cases and 13 facade cases pass. Independent
anchors cover unity/excluded slots, -6 dB tilt, +6 dB amplifier and 2 dB CPA
(+4/-2 dB endpoint transfer), electrical scale/noise and actual two-tap echoes.
Original bytes/grids and raw source ID 200 survive planning; unknown settings,
parameter/transport limits, epoch/identity and imported-H/echo ambiguity reject
without mutation. All-target facade/native-input/binding Clippy with warnings
denied, format/version/guide checks pass. The fresh Linux CPython 3.10 wheel
passes all 71 installed native cases, including four new channel cases and actual
Rust-reference/WB/Designer/compatibility regressions.

The configured transfer is explicitly before receiver correction, separate from
original measured H. No source/noise RNG, waveform/filter stream or PHY run is
created. Existing legacy API admission delegates to shared private helpers with
unchanged validation semantics. No Qt source, scientific kernel, historical
checkbox or saved-project contract changed. Bounded idle TX preview remains open
before the parent task can be accepted.

## UIU-API-01 idle TX preview unit

[Exact candidate/test/wheel evidence](transmitter-preview-evidence.json),
[typed design/owners](design.md#uiu-api-01-bounded-idle-transmitter-preview-unit) and
[paired API guide](../../../user/api/transmitter-preview.md) document the bounded
idle request, one-shot Busy/cancel/progress control and immutable actual results.
Twelve targeted Rust cases and 14 facade cases pass. Existing-owner comparisons
preserve actual samples/PSD; independent timing/count/window anchors cover
135168 samples, first sample 130560, 32 averages, 37500 Hz Hann ENBW, 42.5 µs symbol,
1.875 µs guard and exact overlap sums. Finite EOF/hash/ownership, deterministic
regeneration, unsupported encoded IDs and overflow/window/cable-velocity rejection
pass. A separate Rust process supplies selected I/Q/PSD/timing/progress references.

The final Linux CPython 3.10 wheel passes all 76 native cases, including five new
idle-TX cases, real cross-thread cancellation/Busy/progress while the GIL is
released, owned query results and direct-process parity. An initial new assertion
used the wrong provenance dictionary key; it was corrected before the final pass.
The wheel implementation was unchanged. Draft Rust tests also corrected an explicit
profile selection and literal distance arithmetic; final candidate tests pass.
No Qt source or numerical algorithm changed. Original low-level atomic cancellation
imports and generation semantics remain available.

## UIU-API-01 public admission accepted

The [official task](../../checklist.md#task-uiu-api-01) now has passing local
acceptance for all scoped admission/editor/preview units above. The
[authoritative operation matrix](../high-level-api/compatibility-matrix.md) appends
99 Rust and 75 native rows without changing any of its original 364/373 rows.
Fresh compiler/native discovery checks 463 Rust rows (461 discoverable methods
plus two preserved hidden Serde helpers) and 448 native rows, including exact
signatures, owners, types and explicit language differences. Every referenced new
owner test has an [actual passing execution record](api-01-operation-test-results.json).
The compiler-inventory resolver uses explicit imported namespaces, canonical
compiler signatures and linked primitive-alias declarations; seven independent
inventory cases reject missing/ambiguous owners and signature/type drift.

The final complete `scripts/test-local.sh all` passes in 420 seconds: 635 workspace
cases, doctest checks, four fixture-example cases, 12 API example processes and
139 analysis-only cases. Workspace/facade/native-input/binding all-target Clippy
with warnings denied, format/version/guide and actual Qt boundary checks pass.
Fourteen testing-tool cases and the reconciled validation-tool suite pass.
Original architecture, requirement IDs, completed history and all old operation
rows/evidence remain intact. No Qt source changed and the frozen boundary debt
remains for UIU-DS-01. This accepts public admission locally; UIU-API-02 retains
controlled downstream jobs and observations, followed by the remaining official
Qt tasks and final native/platform gates.

## UIU-API-02 controlled request unit

The independent immutable request/options/control/result models extend public
Simulation admission without changing existing evaluation sessions.
[Exact source/test/wheel evidence](full-loop-request-evidence.json),
[owner/design](design.md#uiu-api-02-downstream-controlled-request-unit),
[paired operation guide](../../../user/api/full-loop.md) and
[exact executed cases](api-02-request-operation-test-results.json) trace to the
[architecture](../../../architecture.md#unified-qt-public-api-boundary) and
still-open [UIU-API-02](../../checklist.md#task-uiu-api-02).

Four new core cases and all six existing job cases pass. The actual old-owner
report/retained-word comparison, clean closed-loop zero BER/FEC failures, 23 complete
source symbols, exact 112-bit finite file recovery and original ownership pass.
Acknowledged safe-boundary pause, frozen progress, resume without reseeding,
concurrent/reused Busy, cancellation while paused and retained completed FEC trials
pass. Initial pause/pre-cancel produce zero completed trials. Invalid transport,
limits, timing and H association reject without changing accepted inputs.
Fifteen facade/native-input/namespace cases and warnings-denied all-target
facade/native-input/binding Clippy, format/version/guide and 14 testing-tool cases
pass. Existing kernel/scientific algorithms remain unchanged.

The fresh Linux CPython 3.10 installed wheel passes all 82 native cases, including
six new full-loop cases. A separate direct-Rust process supplies actual configured
science/count/FEC/word reference results. GIL-releasing cross-thread pause/resume/
cancel and an actual interpreter interrupt during acknowledged pause return real
partial reports. Each equivalent synthetic input retains its own original JSON
encoding/hash; source hashes are not normalized to force a cross-language match.
The matrix adds 18 Rust/17 native rows (481/465 total), preserves every prior row
and checks fresh compiler/native signatures/owners/exports plus successful cases.

No Qt source or completed historical task changed. The passive probe is disabled
in this unit; retained stage/message/spectrum/response/soft-decision queries and
complete source-catalog/sweep compatibility keep UIU-API-02 unchecked. Those gates
precede Qt migration; this scoped unit does not establish parent or platform acceptance.

## UIU-API-02 retained stage unit

[Exact source/test/wheel/Qt evidence](stage-retention-evidence.json),
[owner/design](design.md#uiu-api-02-retained-test-point-ownership-unit),
[paired contract](../../../user/api/full-loop-observations.md) and
[exact executed cases](api-02-retention-operation-test-results.json) trace to the
[architecture](../../../architecture.md#unified-qt-public-api-boundary) and
still-open [parent task](../../checklist.md#task-uiu-api-02).

Twenty-one targeted integration cases and one independent owner case pass.
Enabled/disabled capture preserves actual scientific comparisons and retained
words. Literal anchors cover 14232 original source bits/PRBS prefix, 8192 physical
FFT bins, 204.8 MHz sample clock, actual warmup-vs-observation tags, pilot power
four times the reference and original offsets. Every sampled continuous pilot
matches the same retained physical symbol's actual bin/IQ. The first test exposed
that excluded bins are known zeros, distinct from unavailable history; the test
was corrected to the existing frame contract and a separate owned-data vector
proves None/zero/offset preservation. A real closed-loop pilot-emitter gap was
fixed with a private borrowed notification over the already-computed frame vectors.
No pilot regeneration, DSP/FEC change or public assembly signature change occurs.

Byte/event/element exhaustion, duplicate/unsupported/invalid policies, absent
stages/events, overflow/outside windows and independent query ownership pass.
Fifteen facade cases, 139 analysis-only cases/doctest checks and warnings-denied
facade/native-input/binding Clippy pass. Because the shared core now emits selected
pilot events to legacy consumers, the complete existing Qt check was executed:
87 Rust and 89 QML checks pass, and the latest executable is built. All Qt source
files remain unchanged. This is compatibility evidence, not facade/tab adoption.

The fresh Linux CPython 3.10 wheel passes all 86 native cases, including four new
retention cases and separate direct-Rust source/pilot/physical/waveform references.
The exact matrix appends 14 Rust/10 native rows (495/475 total), retains every prior
row and checks fresh compiler/native signatures/types/owners plus successful cases.
Frozen result policy/provenance and typed-kind actual windows remain usable after
request disposal. Accounted-byte overflow drops copies explicitly; it never turns
missing observations into ideal data or alters completed scientific trials.
Numerical/message queries and complete source-catalog/sweep compatibility remain
open before API-02 completion and Qt migration.

## UIU-API-02 passive numerical query unit

[Exact source/test/wheel evidence](numerical-query-evidence.json),
[query design](design.md#uiu-api-02-passive-numerical-query-unit),
[paired contract](../../../user/api/full-loop-observations.md#passive-numerical-observations)
and [executed cases](api-02-instrument-operation-test-results.json) trace through
the [architecture](../../../architecture.md#unified-qt-public-api-boundary) to the
still-open [parent task](../../checklist.md#task-uiu-api-02).

Ten targeted Rust cases pass: six actual retention/numerical API cases and four
independent PSD/impulse/likelihood owner cases. Actual retained post-overlap
samples reproduce the existing normalized Welch result. Literal anchors include
204.8 MHz sampling, 8192 FFT bins, 25 kHz spacing, seven averages over four
contiguous 8704-sample events and 37.5 kHz Hann equivalent noise bandwidth (with
floating-point tolerance). Original-bin viewports and output buckets preserve
their exact extents. A deliberately truncated event leaves a real sample gap;
the query rejects rather than joining across it. Frequency stages, excessive
work, missing/outside/overflow selectors and invalid velocity reject.

Configured unity H preserves 7480 observed bins, excluded gaps, zero dB gain and
zero signed peak delay. A real pilot-estimated run supplies independent receiver
H-hat; absent or incomplete estimates never fall back to configured H. Actual
retained 16200-value payload LDPC-input statistics match the existing likelihood
owner, retain coding order/sign and distinguish truncated retained count from
original total. Repeated queries and edited native dictionaries do not mutate
owned science or retention accounting. Facade/native-input and binding Clippy
pass with warnings denied. Qt sources and original numerical producers are
unchanged, so the prior built executable and retained-stage Qt acceptance remain
applicable; this unit does not claim Qt adoption.

The fresh installed Linux CPython 3.10 wheel passes all 89 native cases, including
three new instrument cases and separate direct-Rust process references. The first
full attempt passed the new cases but exposed an intermittent old upstream pause
test: its 32-grant work could complete before Python requested its second pause.
An isolated replay passed. Test-only work is now bounded to 128 grants with a
fresh post-resume deadline; the fresh full run passes without engine changes.
The exact matrix appends three Rust/three native rows (498/478 total), preserves
all old rows/history and validates fresh compiled/installed signatures and actual
successful cases. Selected-cell/message and complete source-catalog/sweep parity
remain mandatory before API-02 completion and downstream Qt migration.

## UIU-API-02 retained PLC message unit

[Exact source/test/wheel evidence](plc-message-evidence.json),
[owner/design](design.md#uiu-api-02-retained-plc-message-unit),
[paired contract](../../../user/api/full-loop-observations.md#retained-tx-plc-messages)
and [executed cases](api-02-message-operation-test-results.json) trace through the
[architecture](../../../architecture.md#unified-qt-public-api-boundary) to the
still-open [parent task](../../checklist.md#task-uiu-api-02).

Fifteen targeted integration cases pass: seven actual retention/numerical/message
API cases and eight existing fixed PLC descriptor/CRC/reassembly/frame cases.
Eleven library cases also pass, including five focused message/descriptor-owner
cases. Independent fixed packet bytes establish channel/profile/change identity,
default 256-QAM assignment, exact TLV offsets and source address. Both numerologies
preserve received timing/geometry meanings and every original encoded field. Zero
loading, NCP QPSK and recognized 16384-QAM metadata remain distinct; reserved OCD
parameters reject through the existing receiver decoder rather than guessed values.

Literal sample boundaries prove that a packet remains scheduled one sample before
its carrying frame ends and becomes completely transmitted at that boundary.
Zero stride and arithmetic overflow reject. Gapped, truncated and corrupted later
frames stop with a structured issue and preserve earlier complete packets. Repeated
valid frames prove the 32-packet and 64-frame work limits. Real full-loop output
supplies CRC-validated OCD, data DPD and NCP DPD bytes, matching original source
offsets and actual transmitted-sample coverage. Missing stage coverage rejects;
truncated capture returns an explicit issue without changing completed science.

All 139 analysis-only cases/doctest checks and warnings-denied facade/native-input/
binding Clippy pass. The fresh installed Linux CPython 3.10 wheel passes all 92
native cases, including three new message cases and exact separate-process Rust
packet/view parity. Owned query dictionaries cannot alter retained originals.
The matrix appends one Rust/one native operation (499/479 total), preserves every
old row/history and checks fresh compiler/installed signatures and actual passing
cases. Qt sources and original scientific operations remain unchanged; prior Qt
compatibility/build evidence applies, with no claim of Qt adoption. Selected-cell
queries and complete source-catalog/sweep parity still keep API-02 incomplete.

## UIU-API-02 selected receiver cell unit

[Exact source/test/wheel and alias evidence](receiver-cell-evidence.json),
[owner/design](design.md#uiu-api-02-selected-receiver-cell-unit),
[paired contract](../../../user/api/full-loop-cells.md) and
[executed cases](api-02-cell-operation-test-results.json) trace through the
[architecture](../../../architecture.md#unified-qt-public-api-boundary) to the
still-open [parent task](../../checklist.md#task-uiu-api-02).

Seventeen targeted integration/scientific cases, sixteen library cases, sixteen
facade cases and all 139 analysis-only cases/doctest checks pass. Five focused
cell/decision owner cases establish an independent 4096-cell BPSK power vector:
all errors lie beyond the first 2048 displayed cells, yet full-selection raw MER
is 23.0102999566 dB; filtering to the erroneous half gives 20 dB. Corrected samples
are perfect-reference, and pagination never changes the measured population.
Missing matched references disable TX-reference metrics without discarding actual
nearest-point measurements. Empty/invalid filters, ranges/work limits and owned
output mutation preserve original availability/science.

Real full-loop tests cover payload, accepted NCP, PLC data/preamble and both pilot
roles. Every displayed pair retains original physical/source/channel identities
and I/Q/soft values. A known boosted BPSK vector establishes normalized/physical
coordinates and distances without fitting. Protected roles keep their empty LLRs;
physical decisions never replace decoder inputs. Core/facade/native-input and
binding Clippy pass with warnings denied. The fresh Linux CPython 3.10 wheel passes
all 96 cases, including four new cell cases and separate Rust-process results.

The compiler audit exposed incorrect documentation names for sixteen existing
public aliases and three helper method paths. The inventory now reads exact
compiler `pub use ... as ...` declarations; an independent parser case rejects an
ordinary expression as alias evidence. Eight inventory cases pass, and an actual
facade import/constructor case verifies the public paths. Original source exports,
signatures and scientific acceptance are unchanged. The matrix preserves all three
prior operation rows and sixteen type records verbatim in its explicit historical
correction record; other prior rows/history remain unchanged. It adds four Rust/
five native operations (503/484 total), with fresh compiled/installed signatures
and passing named cases. No Qt source changes or Qt-adoption claim occur. Remaining
decision-region/ideal-reference, detailed soft-display, physical-view audit and
complete source-catalog/sweep gates keep API-02 unchecked.

## UIU-API-02 slicer reference and soft-view unit

[Exact source/test/wheel evidence](slicer-view-evidence.json),
[owner/design](design.md#uiu-api-02-slicer-reference-and-soft-view-unit),
[paired contract](../../../user/api/slicer-views.md) and
[executed cases](api-02-slicer-operation-test-results.json) trace through the
[architecture](../../../architecture.md#unified-qt-public-api-boundary) to the
still-open [parent task](../../checklist.md#task-uiu-api-02).

Fifteen targeted integration/scientific cases and twenty-two library cases pass.
Six focused numerical-owner cases cover every accepted reference geometry,
independent BPSK half-plane/viewport and known-amplitude vectors, original label
paging, finite/unsupported/work errors, literal histogram endpoints/zero/indices
and bounded exact extrema. Every tested decision-region vertex is checked against
every original reference using independent squared distances. Closed boundaries
retain the original slicer's lowest-label tie rule; outer regions are explicitly
clipped to a finite viewport rather than claimed bounded.

An additional float-boundary audit found that offset-based histogram placement
could round a tiny negative value into the zero bin, or an immediately preceding
float into the next interval. The new diagnostic owner now uses direct comparisons
against explicit bin edges. Independent tiny-sign and adjacent-float vectors pass;
no original likelihood value, decoder operation or existing API result is changed.
The final targeted Rust run, current Clippy and rebuilt installed wheel use this
corrected candidate. The earlier transient wheel is not acceptance evidence.

Actual public queries preserve physical-cell versus coding-order soft planes,
original offsets/totals, absent/protected data, finite work and independent owned
results. All 139 analysis-only cases/doctest checks and warnings-denied facade/
native-input/binding Clippy pass. The final Linux CPython 3.10 wheel passes all 99
native cases, including three new cases and separate Rust-process reference,
region and actual soft-view results. The matrix appends four Rust/six native
operations (507/490 total), preserving all previous rows and alias-history records.
The static operations retain both native Simulation/Merlin alias paths and their
explicit delegation to the same public Rust Simulation operations.
Qt sources and original scientific producers are unchanged; no Qt adoption is
claimed. Physical-spectrum/role and complete catalog/sweep parity keep API-02 open.

## UIU-API-02 physical-bin and transmitted-role unit

The [physical-view manifest](physical-view-evidence.json) and
[executed operation record](api-02-physical-operation-test-results.json) preserve
candidate source hashes, exact named cases and installed-wheel/reference commands.
The [public contract](../../../user/api/physical-views.md) distinguishes configured
geometry from observed I/Q and physical Data locations from transported roles.

Four additive Rust/native operations provide configured original full-FFT roles,
actual retained TX/raw-RX bin I/Q and instantaneous power, and typed mapped/
interleaved transport cells with proven optional packing provenance. No original
producer, default retention policy, raw-stage limit or normalized contiguous PSD
operation changes. A separate metadata-only interleaver transports roles only for
the first aligned retained source symbol; later or insufficient history stays
unknown. No GPL Qt numerical code was copied.

Validation passed 17 API/layout cases, all 28 library cases (six new focused
owners), 139 analysis-only cases with doctest checks, workspace/native Clippy
with warnings denied, formatting and 77 validation-tool cases. The independent
numerical anchors cover both 4K/8K physical grids, fixed PLC/pilot coordinates,
3+4j power=25, zero versus unavailable power, finite overflow rejection and an
NI=8/depth=2 role vector with four true history gaps. The actual loop compares
original context, offsets, I/Q and typed cell values. Acquisition uses explicit
BPSK filler; no test assumes every retained event contains a gap.

The fresh local Linux CPython 3.10 wheel passes 102 installed native cases,
including three focused idle/actual/direct-Rust-process cases. Full original
source hashes agree; loader-specific provenance names/contracts are tested
separately. Numerical output is compared recursively with explicit floating-point
tolerance. Editing returned dictionaries cannot change retained results.

Compiler/installed inventories trace 511 Rust and 494 native operation rows;
all previous 507/490 row prefixes and historical alias corrections remain intact.
All 178 Qt files remain unchanged, and the existing 45-file/3-dependency migration
boundary remains frozen. This unit does not establish Qt adoption or UI acceptance.
Calibrated channel spectra/power, explicit ordered-frame PSD, legacy scalar
amplifier-boundary views, complete source-catalog/sweep compatibility and final
API-02 acceptance remain open. No hosted, installer, publication, external
interoperability or wider PHY support is claimed.

## UIU-API-02 ordered frame-stage spectrum unit

The [candidate manifest](sequence-spectrum-evidence.json) and
[executed operation record](api-02-sequence-spectrum-operation-test-results.json)
link exact source/wheel hashes and named cases. The
[sampling contract](../../../user/api/full-loop-observations.md#ordered-frame-stage-spectrum)
keeps logical frame-stage ordering separate from physical sample time.

`FullLoopResult::spectrum_sequence` admits actual IFFT, prefix, window and RX
useful frames. The waveform owner checks consecutive symbols and original stride,
retains original segment coordinates, stops at incompatible/incomplete history,
and reports true physical contiguity and analyzed coverage. Existing Welch math
and the unchanged PSD viewport projection supply the numerical values. The
original contiguous `spectrum` operation and default retention limits are preserved.

Validation passes 20 API/scientific cases and all 31 library cases, including
three independent gap/overlap/stride/offset/truncation/limit vectors. Actual-loop
queries match literal concatenation of retained original I/Q through the existing
Welch owner; the original query still uses a single frame at these boundaries.
A truncated prefix event supplies one complete FFT window, reports its missing
suffix and stops before joining the next event. Both numerologies retain their
independent tone/integral/bandwidth checks. Analysis-only tests pass 139 cases
with doctest checks. Workspace/native Clippy, formatting and 77 validation-tool
cases pass with warnings denied.

The exact local Linux CPython 3.10 wheel passes all 104 installed native cases,
including two focused default-metadata/actual/direct-Rust-process cases. The
accepted default remains 32 and is visible in the real importable method
signature. A test-only constant rename initially changed the invalid value 33
into an undefined identifier; after correcting that expression, the complete
installed suite and metadata inventory were rerun against the identical wheel
and successful separate Rust references. The failed attempt remains recorded;
no scientific source or wheel changed during that retry. A final naming-only review
separated unavailable-snapshot and mutation markers, absolute/relative tolerances
and transport byte limits, and clarified the borrowed-sample lifetime. A fresh
wheel and isolated full 104-case suite pass for that final source candidate;
unchanged successful independent Rust references were retained. The final core
library and workspace Clippy checks also pass.

The numeric audit derives FFT dimensions from SubcarrierSpacing and Welch work
from its existing owner. Application frame/role-span/identifier limits have
separate owners; retention count, Welch average count, view windows and reference/
source fixtures have distinct names even when values coincide. Independent
expected test anchors remain independent of implementation constants. The local
coding-style skill now explicitly requires this purpose/units/rationale audit.
No accepted numeric limit, source value or existing public behavior changes.

Compiler/native inventories trace 512 Rust and 495 native operations; the prior
511/494 row prefixes and historical alias corrections remain intact. All 178 Qt
files remain unchanged and the frozen legacy boundary passes. Calibrated channel
spectra/power, scalar amplifier-boundary views, complete source-catalog/sweep
compatibility and final API-02 acceptance remain open. This unit does not establish
Qt, hosted, installer, external interoperability or wider PHY acceptance.

## UIU-API-02 calibrated channel and amplifier unit

The [candidate manifest](channel-spectrum-evidence.json) and
[executed operation record](api-02-channel-spectrum-operation-test-results.json)
retain exact source/wheel hashes, commands and named cases. The
[channel spectrum contract](../../../user/api/channel-spectrum.md) describes
actual versus scalar-derived planes, electrical units and fixed-band integration.

The existing Welch/power/layout owners now back calibrated actual time-stage
spectra, fixed channel taps, amplifier-only physical-bin observations and idle TX
calibration. A focused Rust owner performs electrical projection and validated
scalar observations; Python only converts arguments and owned results. Original
normalized methods and actual retained sources remain unchanged. No Qt scientific
implementation was copied into the Apache backend.

All 25 integration/scientific cases and 34 library cases pass. Three independent
owner cases pin flat density at both FFT sizes, the inclusive 190-MHz envelope
with an internal exclusion, normalized power 0.927734375 and zero equivalent
dBmV per 6 MHz at an explicit zero input reference. A separate twenty-dB amplifier
maps 3-4j to 30-40j independently of six-dB CPA. Tests cover full-band zoom
invariance, relative/zero/missing data, negative/nonfinite density, scalar/calibration
overflow, original geometry, actual receiver output, idle gain exclusion and
unchanged retained science. The final owner run includes the added negative/
calibration-overflow vectors. Analysis-only validation passes 139 cases and
doctest checks. Workspace/native Clippy, formatting and all 77 validation-tool
cases pass; original Qt boundary debt stays frozen.

A freshly built local Linux CPython 3.10 wheel passes all 107 installed native
cases without skips, including three new actual/bounds/ownership/default-metadata
and separate Rust-process parity cases. All reference examples were regenerated
for this unit, including full-loop, idle TX, saved workbench and Designer inputs.
Rust/native inventories trace 516/499 operations, adding four operations each.
All prior 512/495 rows and historical alias corrections remain intact. All 178
Qt files remain unchanged; no new Qt runtime acceptance is claimed.

Complete source-catalog/sweep compatibility and final API-02 acceptance remain
open before Qt adoption. This focused unit establishes local API acceptance,
without hosted/installer/publication, measured RF compliance or external
interoperability claims.

## UIU-API-02 complete catalog and bounded sweep unit

The [candidate manifest](sweep-evidence.json) and
[executed operation record](api-02-sweep-operation-test-results.json) retain exact
source/wheel hashes, commands and named cases. The
[serial plan contract](../../../user/api/full-loop-sweep.md) separates complete
raw inventory, checked run IDs, source-observation duration and active host budgets.

ProfileCapture preserves every raw source ID and exact original provenance.
Its additive simulation_profile operation admits original selected loading through
the unchanged ProfileId domain; legacy loaders keep their original behavior.
FullLoopSweepPlan validates the complete selection before admitting a plan,
preserves edited current loading, sorts original captured selections, and shares
base/catalog ownership instead of retaining all measurement copies per profile.
It creates one independently controlled public request at a time, with unchanged
options and observation policy. The plan owns no worker or result archive.

Seven full-loop/control/admission cases and five original capture cases pass in
the complete Rust regression. Independent anchors retain a mixed [200,7,0,2]
source inventory while admitting original 7, exact original bytes, sorted [0,7],
14960 selected assignments, two milliseconds of requested source observation,
120000 milliseconds of summed active host budget and 128 retained-word details.
Tests reject unsupported all-captured identities, unsupported selected coding,
empty/duplicate/missing IDs, mismatched keys and resource limits atomically.
Actual complete/cancelled requests preserve previous results and independent
one-shot controls. Current preserves the exact edited request. An early negative
fixture changed an encoded assignment without its matching text/bit fields; it
was corrected to a consistent recognized QPSK assignment so the test reaches the
intended unsupported full-loop admission boundary.

The final local Linux CPython 3.10 wheel passes all 110 installed cases, including
three new catalog/selection/control/ownership and separate Rust-process cases.
All reference examples were regenerated for this unit. A final constructor-order
review moved two unchanged inspection methods after existing constructors;
byte-identical moved blocks are recorded, Clippy was rerun, and a fresh final
wheel and isolated 110-case suite pass using the unchanged successful Rust
references. No scientific expression changed in that ordering adjustment.

Compiler/native inventories trace 520/503 operations, adding four each. Prior
516/499 rows and historical alias corrections remain intact. Every new operation
has requirement/task/owner/native parity and executed-case links. All 178 Qt
files and the frozen legacy dependency boundary remain unchanged. No Qt
scientific implementation was copied into the Apache backend.

## UIU-API-02 public full-loop and observations accepted

The complete current Rust regression passes 687 workspace cases, doctest checks,
four fixture-generator cases, all 12 API example processes and 139 analysis-only
cases. Three existing explicitly ignored performance-only benchmarks are not
executed; their paths, names and reasons are recorded in the candidate manifest.
The bounded four-process runner finishes in 628 seconds. Final workspace
and native Clippy, formatting and all 77 validation-tool cases pass. The exact
final installed wheel passes all 110 native cases without skips, covering old
imports/operations, saved workbenches/Designer, upstream regressions and the new
downstream request/observation/query/catalog contracts. Exact inventories and
executed records preserve existing operation rows and historical alias evidence.

The focused API-02 units above cover controlled requests, opt-in original stage
retention, passive numerical queries, PLC messages, selected receiver cells,
reference/soft views, physical/transmitted roles, ordered-frame diagnostics,
calibrated channel/amplifier observations and complete catalog/sweep admission.
All applicable backend acceptance gates are satisfied. UIU-API-02 is complete;
UIU-DS-01 owns Qt adoption and UIU-SESSION-01 owns aggregate UI result budgets.
Existing downstream implementation and completed history remain intact. This is
local backend/API acceptance, not new Qt, hosted, installer, publication, measured
RF compliance or external interoperability evidence.

## UIU-API-03 idle upstream resource-grid accepted

The [candidate evidence](resource-grid-evidence.json) and
[executed cases](api-03-resource-grid-operation-test-results.json) establish the
bounded idle observation for REQ-UIU-02/14/24 and
[UIU-API-03](../../checklist.md#task-uiu-api-03). The
[exact design](design.md#uiu-api-03-idle-upstream-resource-grid-observation) and
[user guide](../../../user/upstream/resource-grid.md) describe the implemented
Rust/native Python contracts. Allocation comes from ResourcePlan; this unit adds
no pilot placement, interleaving, receiver or Qt scientific algorithm.

Five public grid cases and two existing independent pilot cases pass in the
complete upstream run: 88 passed, one preexisting performance benchmark ignored.
Sixteen facade cases pass. Literal edge pilots retain carriers 100/107/116/123
at symbols 0/2, and the loaded body minislot has two known and two complementary
cells with 44/92 ordinary cells at 50/25 kHz. Every admitted pattern, frame size
and numerology preserves original visits, loading, gains and bit offsets.
Selected zero-loaded, excluded and unallocated physical positions remain distinct.
Independent timing anchors preserve the second-frame prefix at tick 16960 and
symbol-two prefix/useful starts at 21248/21344. Full capacities 328/656 bits are
unchanged by zoom. A nonzero-noise actual closed loop verifies unchanged source,
noise, receiver outcomes and configuration identity after inspection.

The final Linux CPython 3.10 wheel passes all 114 installed cases without skips,
including four new grid cases. Real import/default metadata, invalid conversion,
shared bounds, unchanged accepted state, owned result lifetime, paused-session
Busy/cancel cleanup and separate Rust-process parity at both spacings pass.
Snapshots retain their configured allocation plane before CM Pre-Eq; they do not
claim actual waveform observations, physical H or received C×H. Queries cover
the configured base grant, without inventing later batch identities.

Workspace and native all-target Clippy, both formatting checks, 77 tooling cases,
documentation/traceability and the dependency-boundary audit pass. Compiler and
installed inventories append two Rust and two native operations, giving 522/505
rows. Every previous row and historical alias correction remains intact. All 178
Qt sources match their accepted baseline; three direct dependencies and 45 frozen
legacy source files remain migration debt. The most recent complete downstream
Rust regression remains the passing API-02 run above; it was not rerun for this
additive upstream projection.

UIU-API-03 is complete. UIU-US-04 separately owns Qt rendering, zoom/inspection,
batched geometry and object bounds. Evidence is local Rust/installed-wheel
acceptance, with no new hosted, native installer or interoperability claim.

## UIU-API-04 bounded recovery view unit

The [candidate evidence](recovery-evidence.json),
[executed cases](api-04-recovery-operation-test-results.json) and
[exact owner/signature design](design.md#uiu-api-04-upstream-instrument-gaps-and-report-views)
trace this focused unit to REQ-UIU-02/12/19/20/24 and
[UIU-API-04](../../checklist.md#task-uiu-api-04). Typed verification, whole-report
FEC summaries and bounded original completed-word pages reuse the existing
receiver, grant and independent PRBS verifier data. No scientific algorithm,
decoder behavior, original report field or existing default changes.

Seven focused cases pass inside the complete upstream package run: 95 passed,
one preexisting performance benchmark ignored. Sixteen facade cases pass.
Independent 50-kHz contiguous allocation anchors give capacities 2820/8516/22756
bits and source counts 1976/6776/19776 for 8/24/64 minislots. Retained information
lengths [840,420,720], [5040,840,420,476] and [14400,4620,756] preserve upstream
short/medium/long selection, shortening and borrowing. Both numerologies retain
exact original decisions, flags, code dimensions, source/coded offsets and paging
continuations. A second grant retains source start 1976; its last short word
starts at 3236. The first word copies exactly 2800 mother/information/payload bits.

Actual complete, cooperatively cancelled and destructive-channel runs establish
runtime outcomes. Generated, compared and unverified source coverage stays
separate, unavailable BER/maxima remain absent, and failed decoder decisions are
retained. Deliberately edited flag/metadata fixtures prove projection independence
and explicit rejection; they are not reported as actual decoder experiments.
The destructive-channel test initially assumed valid parity, failed, and was
corrected to retain the actual parity/nonconvergence failures before acceptance.

The exact Linux CPython 3.10 wheel passes all 119 installed cases without skips,
including five new recovery cases. Installed defaults, integer conversion and
shared position/copy budgets, independent owned dictionaries, original decisions,
actual cancellation/failure and separate Rust-process parity at both spacings pass.
Workspace/native all-target Clippy, both formatting checks and all 77 tooling
cases pass. Exact compiled/installed inventories append three Rust and three native
operations (525/508 total), preserving all earlier rows and alias history. All 178
Qt sources and the three-dependency/45-file boundary debt remain unchanged.

This accepts only the recovery-view unit. Received-pilot estimates, explicit
C/H/estimated C×H response/impulse views, actual soft retention and remaining
constellation/reference contracts keep UIU-API-04 unchecked. Qt adoption remains
separate. The [user guide](../../../user/upstream/recovery.md) describes these
limits; no new hosted, installed-desktop or external interoperability evidence is
claimed. The latest complete downstream Rust regression remains API-02 above.

## UIU-API-04 configured response and periodic impulse unit

The [candidate evidence](response-evidence.json),
[executed cases](api-04-response-operation-test-results.json) and
[exact model/owner design](design.md#uiu-api-04-configured-response-and-periodic-impulse-unit)
trace this focused unit to REQ-UIU-02/12/18/24 and
[UIU-API-04](../../checklist.md#task-uiu-api-04). Idle sessions and frozen reports
expose configured corrective C, deterministic H and their composite, with an
optional complete periodic inverse. Actual received-pilot estimates remain
separate. Existing C replay, channel settings and the shared unitary transform
remain the authorities; no waveform or RNG is executed by these queries.

Eight focused API/scientific cases pass in the complete upstream run: 103 passed,
one preexisting performance benchmark ignored. Sixteen facade cases pass. At
both spacings, the literal two-tap h=[1,0.5], delay two and quarter-turn phase give
H(DC)=1.5j, H(-fs/4)=0.5-j, H(fs/4)=-0.5-j and H(-fs/2)=0.5j. Periodic impulse
samples two/three are j/0.5j, with -6.020599913279624 dB for the second sample
relative to the full-period peak. Raw corrective 2j normalizes to j over every
nonexcluded carrier. Replay event 17 retains actual bank generation 11.
Excluded zeros, full-impulse zoom independence, missing/zero/budget/overflow
states and nonzero-CFO static-H rejection pass. Negative and 1e308-radian phases
are compared with the actual channel owner. Passive queries preserve noisy-run
decisions and frozen identities; never-started reports invent no run/generation.

All five new installed response cases passed on the first complete native run.
That run failed an existing second-pause acknowledgement deadline. The unchanged
case then passed alone and in 50 diagnostic repetitions. Its cause was not
reproduced; no timeout, assertion or historical test was changed. A full retry of
the exact Linux CPython 3.10 wheel passes all 124 installed cases without skips.
The retry reuses the preceding candidate's unchanged successful separate Rust
process outputs, with exact output/command hashes retained. The failed initial
run and isolated/repeated logs remain in the evidence; they are not counted as a
passing suite. Later lifecycle acceptance must continue running this regression.

Installed defaults, all configured planes/units/periodic samples, checked errors,
original replay metadata, independent owned dictionaries, Busy/cancel cleanup and
separate Rust-process parity at both spacings pass. Workspace/native all-target
Clippy, both formatting checks and all 77 tooling cases pass. Exact compiler/native
inventories append two Rust/two native operations (527/510 total), preserving every
earlier row and historical alias correction. All 178 Qt sources and current
three-dependency/45-file boundary debt remain unchanged. No receiver, codec,
scientific channel implementation, report schema or existing default changes.

This is a configured deterministic diagnostic with a complete N-sample periodic
inverse. It excludes noise/RxMER residuals and does not claim measured H, a
received C×H estimate, unaliased causal echo support or finite-burst/window/ISI
reconstruction. The [user guide](../../../user/upstream/response.md) states these
limits. Actual receiver-estimate/soft and remaining constellation/reference work
keep UIU-API-04 unchecked. Evidence remains local Rust/native Python acceptance;
no new Qt, hosted, installer or interoperability claim is made.

## UIU-API-04 actual received-pilot estimate unit

The [candidate evidence](receiver-estimate-evidence.json),
[executed cases](api-04-estimate-operation-test-results.json) and
[exact retention design](design.md#uiu-api-04-received-pilot-estimate-retention-unit)
trace REQ-UIU-02/12/18/24 to the existing API-04 parent. This unit retains actual
normal receiver estimates after completed frame processing. Disabled defaults,
existing constructors/options and default report JSON remain compatible.

Scoped Rust acceptance passes 23 upstream cases: six new API/scientific cases,
two new deterministic observation-boundary cases and fifteen existing receiver/
experiment cases. Seventeen facade cases pass, including one new actual public
namespace execution. Literal H=2+j and normalized C=j yield received-pilot
C×H=-1+2j at both spacings. Original unused carriers stay unavailable; actual
zero channel yields present complex zero. Complete missing frames retain all
unavailable estimates. Invalid transactions and cancellation inside the next
frame preserve the previous completed estimate without a timing race.

Enabled and exhausted observation preserve actual noisy decoded words, BER and
matched MER. Aggregate frame/scalar limits span grants and preserve original
run/generation/grant/frame identities. Mode-specific policy admission and typed,
document and JSON replacement reject atomically. Query windows preserve optional
physical coordinates; copied data outlives sessions and reports. The native Busy
guard protects policy access while an acknowledged pause holds the session.

The installed Linux CPython 3.10 wheel passes ten scoped cases without skips:
six new observation cases and four existing namespace/typing/compatibility cases.
One fresh separate Rust-process reference supplies every optional physical
coefficient and metadata at both spacings. Unchanged downstream reference
pipelines and the complete native/Rust suites were not rerun for this partial
unit; full regressions remain the final integration gate. The preceding recorded
pause-timing failure remains unresolved historical evidence, not a fixed defect.

Core/facade and native all-target Clippy pass. Exact compiler/installed inventories
append six Rust and six native operations (533/516 total), preserving every
earlier operation and alias correction. All 178 Qt sources remain unchanged.
Formatting, focused metadata-tool tests, documentation/traceability and strict
operation checks are recorded with the candidate evidence.

Retention stores complete estimates only; interrupted partial frames are absent.
Disabled inventory describes observation availability rather than all receiver
work. Missing frequency support is not converted into a full-band impulse.
Actual soft metrics, remaining response-display/reference contracts and Qt
adoption keep API-04 unchecked. The [user guide](../../../user/upstream/receiver-estimates.md)
states these boundaries. Evidence is local Rust/native acceptance, with no new
hosted, installer, hardware or external interoperability claim.

## UIU-API-04 actual receiver soft-observation unit

The [candidate evidence](soft-observation-evidence.json),
[executed cases](api-04-soft-operation-test-results.json) and
[exact owner/retention design](design.md#uiu-api-04-actual-receiver-soft-metric-retention-unit)
trace REQ-UIU-02/12/19/24 to API-04. The existing receiver now optionally retains
actual raw/derandomized cell metrics and the exact transmitted-position input
of completed LDPC calls. Independent immutable soft policies compose onto the
existing receiver observation policy, including soft-only observation. Existing
constructors, estimate limits and omitted-disabled JSON fields remain compatible.

Forty-one scoped upstream cases pass: six new soft API/scientific cases, two new
deterministic soft boundary cases and existing estimate/recovery/receiver/experiment/
QAM/randomizer regressions. Eighteen facade cases pass, including one new actual
public soft-observation execution. Unit BPSK at complex variance 0.5 gives
LLR=8×I before clipping. Literal impulse-seed mask positions independently verify
raw-versus-derandomized signs. Both spacings reconstruct every retained LDPC input
from actual cell metrics at its original coded offset. Long/medium/short family
dimensions, shortening, actual erasure failures and independent decoder flags pass.

The initial full-cell fixture incorrectly assumed the same allocation at both
spacings, then exceeded the 1024-cell retention limit on its wider 25-kHz grid.
Those failed logs remain recorded. The final explicit 64-carrier fixtures use
eight 50-kHz versus four 25-kHz minislots, with independent 2820/2932-bit capacities.
All limits remain unchanged. Missing-frame finish records actual erasure outcomes;
mid-frame cancellation preserves completed words without publishing incomplete
inputs. A 700-LLR budget omits the first larger word and retains original word
ordinal one/coded offset1120 at retained index zero, preserving original identity.

Enabled/exhausted observation preserves noisy decisions, BER and matched MER.
Aggregate limits span grants; cell pages and word copies use separate positive
budgets checked before allocation. Native dictionary ownership, empty-end,
never-started and unavailable states pass. The installed Linux CPython 3.10 wheel
passes 20 scoped cases without skips: five new soft cases plus existing receiver
estimate, recovery and namespace/typing cases. Three fresh separate Rust processes
provide actual cell/word/estimate/recovery references; full native/downstream
reference pipelines were not repeated for this partial unit.

Core/facade and native Clippy pass, with formatting and metadata/traceability checks
recorded in the evidence. Exact inventories append six Rust and six native methods
(539/522 total), preserving every prior operation and alias correction. All 178 Qt
sources remain unchanged. Full Rust/native/Qt regressions remain the final
integration gate; the preceding pause-timing failure remains recorded as unresolved.

Computed cell observations can include grant pad or metrics whose bits did not
reach a completed codec call after cancellation. Word observations contain actual
transmitted input, excluding restored shortened priors; posterior LLRs are not
claimed. The [user guide](../../../user/upstream/soft-observations.md) explains
these boundaries. Remaining response-display/constellation references and Qt
adoption keep API-04 unchecked. No hosted, installer or interoperability claim.

## UIU-API-04 QAM pairing and received-response display unit

The [candidate evidence](qam-display-evidence.json),
[executed cases](api-04-display-operation-test-results.json) and
[exact display/association design](design.md#uiu-api-04-constellation-pairing-and-received-response-display-unit)
trace REQ-UIU-02/12/16/18/24 to API-04. Actual compatible QAM now has canonical
source-labelled references and backend unit normalization. Original raw values,
identities and role/loading remain paired with the derived display. Actual role
gains are retained privately at the existing ResourcePlan observation boundary,
without changing original QAM output or default serialized reports.

Twenty-two scoped upstream cases pass, including six new scientific/API cases
and existing configured/received response and QAM-owner cases. Nineteen facade
cases pass, including one new direct public QAM/pairing execution. Literal upstream
16-QAM labels 0/7/10 are (3,3)/(1,-1)/(-1,3) divided by sqrt(10). Boosted pilots
retain zero payload bits and raw power three, while the recorded sqrt(3) gain
normalizes their actual observations to unit power with canonical BPSK references.
The largest supported upstream 4096-point constellation and independent copy
limits pass. Pre-Eq/raw-FFT incompatible overlays reject, preserving raw access.

Both numerologies retain every original coordinate through exact stage pairing.
The boosted fixtures have 112/240 matches and sixteen unpaired TX pilots. Capping
preserves whole selected/matched/unpaired counts and left retention order. Failed
and never-started histories remain empty/unpaired rather than fabricated. Queries
preserve actual noisy words, BER and matched MER. The original raw QAM and main
report JSON omit the private gain ledger.

Actual received-pilot -1+2j has amplitude6.989700043360188 dB and principal
phase2.0344439357957027 rad at both spacings. Missing and exact-zero estimates have
no logarithmic/phase readings. The shared robust transfer projection preserves
existing configured phase/impulse/overflow behavior and original error contexts;
no configured response replaces actual receiver data or fills missing support.

The installed Linux CPython 3.10 wheel passes nineteen scoped cases without skips:
four new display cases plus existing estimate/configured-response and namespace
cases. Three fresh separate Rust-process references cover full observations,
labels, original pairs and actual received display values. Core/facade and native
Clippy, formatting, metadata tools and strict operation traceability pass as
recorded. Exact inventories append two Rust/two native operations (541/524 total).
The previous received-query rows are archived before adding field-level evidence;
all original IDs/assertions and alias history remain. All 178 Qt sources are intact.

Identified API-04 instrument gaps now have scoped backend acceptance within their
declared support. Pair counts describe retained histories rather than source/FEC
coverage; raw stage planes stay distinct and missing spectral support does not
establish a full-band impulse. The [user guide](../../../user/upstream/qam-views.md)
states those limits. The parent remains unchecked until its full integration gate
passes. Qt adoption, hosted jobs, installers and external interoperability remain
separate; no such acceptance is claimed by this unit.

## UIU-API-04 upstream instrument backend accepted

Candidate `6a1e4d43d25e2adff8cf88eadf85c9a08144b660` passes the final
[backend gate](api-04-final-evidence.json) for
[UIU-API-04](../../checklist.md#task-uiu-api-04). The five preceding instrument
units preserve their scoped scientific and API evidence. Together they close the
identified gaps for QAM/PSD, configured C/H/composite and periodic impulse,
actual received-pilot estimates, actual cell/word soft inputs, FEC and independent
source verification. Unsupported planes remain explicit.

`scripts/test-local.sh all` passes 732 workspace tests, four fixture-generator
cases, twelve API example processes and 139 analysis-only cases in 468 seconds.
All doctest phases pass. Three existing explicitly ignored host-performance
observations remain excluded; no correctness cases were added to exclusions.

The exact current Linux CPython 3.10 wheel passes all 139 installed native cases
without skips in 71.904 seconds; reference/install/inventory preparation brings
the complete native gate to 82.406 seconds. Fourteen separate upstream/PNM Rust
reference programs are refreshed. Seven downstream reference files from the
previous accepted 124-case candidate are reused after an exact unchanged-owner
and lock comparison from `b16c36119d5ae79fc3547c9797c9c661029598fa` to this
candidate. Their current hashes, prior receipt, comparison command, wheel hash
and every executed native case are retained in the gate record. The full Rust
gate independently re-executes downstream runtime regressions and examples.

The operation matrix remains 541 Rust/524 native rows with its original IDs,
assertions and alias/revision history intact. All 178 Qt source hashes remain
unchanged. Prior warnings-denied Clippy, formatting and scoped acceptance stay
linked from each unit. The preceding intermittent pause timeout remains recorded;
this passing complete run establishes no root cause or fix.

API-04 is complete within its declared backend support. Qt instrument adoption,
native rendering, hosted jobs, installer lifecycle, hardware and external
interoperability remain separate gates. The next eligible task is API-05's
direction-aware public project admission.

## UIU-API-05 saved downstream full-loop unit

The [exact candidate evidence](saved-full-loop-evidence.json) and
[executed operation cases](api-05-workbench-operation-test-results.json) trace
REQ-UIU-02/22/24 to the additive legacy-project preparation unit. Seven saved-input
cases, eight existing/new full-loop cases and ten facade cases pass; nine installed
Linux CPython 3.10 cases pass without skips. The current candidate also reruns the
saved-input, independent-response and facade cases after shared constants moved
to their authoritative native document owner. Receipts distinguish these scoped
runs from the preceding complete full-loop run.

Original file bytes `81 00 5a` recover exactly with a finite 64-bit target and
independent fill seed 7, while dormant continuous-source seed zero is ignored.
An independently established 17-second H/profile difference admits with zero
RxMER/profile tolerance; 16 seconds rejects. Sweeps preserve both policies.
All selected and unselected originals are integrity checked. Actual saved PRBS/FEC
summaries, settings and provenance match a separate Rust-process reference;
Designer output matches the existing shared materialization. Playback retains
stored inventory order and physical-symbol intervals, with duration-capacity
rejection before execution.

The [single operation matrix](../high-level-api/compatibility-matrix.md) appends
four Rust and four native operations, preserving every previous row and assertion.
Existing Workbench loaders keep their static PRBS contract. There are no Qt source
changes. Direction-tagged schema 3, upstream project admission, aggregate input
budgets and the parent API-05 acceptance gate remain unfinished. Native rendering,
hosted jobs, installers and external interoperability are separate gates.

## UIU-API-05 direction project unit

The [project evidence](project-evidence.json) and
[executed operation cases](api-05-project-operation-test-results.json) trace
REQ-UIU-02/13/22/23/24 to the direction-tagged native project owner and optional
facade/common export. Six project-input cases, one aggregate cached-byte boundary
case and eleven facade cases pass. The related seven saved full-loop input cases
also pass. Fifteen scoped installed Linux CPython 3.10 project/saved-plan/namespace
cases pass without skips, with fresh separately executed Rust references.

Actual observed downstream PRBS/FEC outcomes and settings match across Rust/native
Python after schema-3 round trips. Both upstream numerologies execute using
original configuration or measured-plan documents, explicit option overrides and
validated receiver/soft observation policies. Original captured source bytes and
measured-plan lineage survive admission, serialization, project disposal and runs.
Neither load nor export executes science. Returned plans/sessions own their inputs.

Wrong-direction, malformed/unknown/duplicate fields, unsupported versions,
corrupt original hashes, invalid observation/run policies, nonregular files and
oversized sparse files reject without changing an accepted project. Collection
checks retain every original, reject duplicate/excessive directions and use checked
byte sums. A private unit instruments only the cached admitted input-byte counter
to exercise the inclusive 96 MiB boundary and overflow without huge allocations.
This establishes storage admission, not concurrent runtime or result budgets.

The operation matrix appends eighteen Rust entries (nine facade and nine native
adapter operations) and eight native common.Project entries, preserving all prior
545 Rust/528 native rows. Compiler discovery now follows the actual common facade,
with a regression case for extra re-exports beyond the neutral leaf. The unchanged
common constructor renders its return as RunControl rather than Self in facade
rustdoc; exact prior Rust/native rows are archived before reconciling that compiler
presentation, preserving every ID, signature behavior and acceptance assertion. Strict
contents parsing uses the existing pinned Serde raw-value feature so duplicate
fields cannot disappear into an intermediate dictionary.

The parent API-05 full backend gate remains pending until its exact receipts are
recorded. Qt file choosers, direction routing, dirty-state handling and private
restart recovery remain their later tasks. All 178 original Qt source hashes are
unchanged; no Qt rendering, hosted, installer or interoperability claim is made.

## UIU-API-05 project backend accepted

The [exact full-gate evidence](project-evidence.json) closes the public project
backend prerequisite. `scripts/test-local.sh all` passes 747 workspace cases,
139 analysis-only cases, fourteen API examples and four fixture-generator cases
in 667.946 seconds. Workspace and analysis doctest phases pass. Three existing
explicitly ignored host-performance observations remain excluded; no new
correctness exclusions were introduced.

The complete installed Linux CPython 3.10 suite passes all 150 cases without skips
in 83.723 seconds. The first complete native run passed 149 cases and failed the
new test fixture's Sequence type conversion. That fixture now inherits the declared
Python interface while retaining its iterator-rejection assertion. No native or
scientific code changed. The final run reuses the exact unchanged installed wheel
and all successful separate Rust reference outputs after recording their hashes;
the original failed receipt, commands and outputs remain intact.

All new Rust/native operations have scoped Clippy, formatting, strict matrix and
metadata acceptance. The matrix contains 563 Rust/536 native rows. The one common
constructor compiler presentation correction retains its exact historical rows;
all operation IDs, earlier test assertions and compatibility history remain.
The common inventory uses actual facade rustdoc, including optional input exports.

API-05 is complete as a headless backend task. UIU-FILE-01 and UIU-RECOVERY-01 own
Qt adoption and lifecycle acceptance. There are no Qt source changes. Concurrent
runtime, final numeric review, native Qt rendering, hosted jobs, installer lifecycle
and external interoperability remain their separate gates. UIU-ORG-01 is the next
eligible official checklist task.

## UIU-ORG-01 generic presentation accepted

The [exact organization evidence](organization-evidence.json) closes
REQ-UIU-04/25 under UIU-ORG-01. Fifteen generic QML files move to
`qml/shared/{components,theme,about}` with byte-identical contents, including their
notices, values, handlers and type names. Existing consumers/tests change only
relative imports. The pinned 178-file source inventory remains historical evidence.
Six live downstream requirement owner locators and canonical skill references now
point to the moved files/directories; original IDs and completed history remain.

`scripts/qt-desktop.sh check` passes the ordinary dependency guard, formatting,
warnings-denied Clippy, 87 Qt Rust tests, a development native build and 89 QML
cases without skips. All 82 production QML files remain registered exactly once.
`scripts/qt-desktop.sh build` passes the optimized native build. Fresh release
executable checks accept four bundled downstream workbenches and pass normal plus
loaded-project XCB/Fusion startup without stderr diagnostics. The current display,
SDK version, binary hash, exact commands and timings are retained in the evidence.

Module URI, Main entry URL, QObject registration placement, package/executable
identity, branding resources, notices and licenses are preserved. No scientific
source or public API changed; no non-Qt workspace rerun was needed. There are no
visual layout or style changes, so existing user-guide visuals remain accurate.

Coupled shell lifecycle views remain for UIU-SHELL-01. The three current direct
scientific dependencies and 45 frozen legacy source files remain the explicit
UIU-DS-01 migration boundary; this presentation acceptance does not accept those
as the final architecture. Scientific instrument sharing still needs its own
actual direction consumers. Final numeric/full UI gates, hosted jobs, Windows
installed lifecycle and external interoperability remain separate. UIU-DS-01 is
now the next eligible official checklist task.

## UIU-DS-01 downstream placement prerequisite accepted

The [placement evidence](downstream-placement-evidence.json) accepts the structural
prerequisite only. All 104 approved moves are implemented: 45 existing Rust owners
under `src/downstream/` and 59 QML views under their direction-specific destinations.
One module-wiring file declares existing Rust children. QObject entries remain
in `src/bridge/`, the module remains `Merlin.Workbench` and Main's resource URL is
unchanged. Its instantiated view is now the approved `DownstreamWorkbench` type.

All 55 existing changed Rust source bodies match their prior versions after
reversing only module/resource paths and normalizing Rust formatting. All 59 moved
QML bodies match after excluding import lines. Values, handlers, source vectors,
scientific operations and serialized contracts are unchanged. Compile-time
resources still resolve to the same original inputs. Thirty live owner/test
locators and maintained navigation/skill references now point to current paths.
The master prompt and archived upstream extension remain verbatim historical
sources; they are not rewritten as current locator tables.

The [original guard](legacy-qt-boundary-initial.json) is retained verbatim.
The current guard contains explicit reviewed relocation proofs, the same three
dependencies, 45 frozen files and 145 scientific access sites. Whole-file checks
remain strict; sixteen negative/regression cases pass. No operation expansion or
new exception is accepted by a file move.

The Qt check phases pass formatting, Clippy, all 87 existing Rust cases and a
development build. Their function identities are preserved under the new module
prefix. The first source-loaded QML pass exposed an implicit former-shell import;
an explicit import restores the same scope. The targeted QML rerun passes all 89
original case IDs without skips. The optimized native build and six fresh release
XCB/Fusion workbench/startup smokes pass without stderr. Failed aggregate-check
receipts remain preserved; the evidence distinguishes their successful Rust phases
from the later passing QML/build commands.

UIU-DS-01 remains unchecked. Its next units must replace scientific calls with
accepted facade operations, remove all direct scientific dependencies and legacy
guard entries, and preserve actual downstream behavior. This placement does not
accept the final API boundary, session isolation or shared scientific instruments.
No non-Qt source/public API changed, so no non-Qt workspace rerun was needed.
Hosted/Windows installed lifecycle and interoperability remain separate gates.

## UIU-DS-01 OFDM settings facade accepted

The [focused settings record](downstream-settings-evidence.json) covers the
existing QObject's migration to accepted `merlin::downstream` configuration and
timing operations. The five-field JSON, bundled registry, defaults, atomic Apply,
4096-byte limit and visible summary remain compatible. Independent literal
summaries cover both FFTs at a 512-sample prefix; invalid edits preserve state.
The old two settings/registry tests remain and a third summary case is added.

The display conversion `SAMPLES_PER_MEGASAMPLE` belongs to
`src/shared/presentation/constants.rs`. Scientific clock and timing calculations
come from the backend. The document bound reuses its existing public owner;
4096-byte and 4096-sample test anchors retain their different meanings.
No backend, Python, persisted setting, QML control or scientific algorithm changes.
The Qt lock adds only the local facade package and direct dependency, retaining
every existing dependency record/version. The settings adapter's exact legacy
entry is retired with its history. Three dependency exceptions, 44 frozen files
and 144 access sites remain; UIU-DS-01 stays unchecked.

Acceptance includes formatting/Clippy, all 88 Qt Rust cases, all 89 QML cases,
16 boundary regressions, the optimized build and six fresh native XCB/Fusion
project/startup checks without stderr. This validates the changed consumer;
existing backend/API/Python evidence remains applicable because its owners are
unchanged. Original failed preparatory guard/cache checks remain distinguished
from successful acceptance. Final boundary, numerical review, session isolation
and hosted/installed lifecycle acceptance remain separate tasks.

## UIU-DS-01 decision-region facade accepted

The [receiver-region evidence](downstream-region-evidence.json) records removal
of Qt's duplicate nearest-reference sorting and polygon clipping. The adapter
queries accepted `Simulation::decision_region` at the existing unit-energy plane
and maps owned coordinates into the same four display fields. Unsupported and
out-of-range selectors produce empty geometry without indexing a private table.
No public API, backend algorithm, saved setting or QML contract changes.

The original histogram and geometric-containment tests remain, covering square
16-QAM, cross 128-QAM and 4096-QAM with the same numerical assertions. Reference
coordinates now come from public `constellation_reference`. A third focused case
covers unsupported and overflowing order/label selectors. All three scoped Rust
cases, three QML soft-display cases and sixteen boundary regressions pass, with
Qt formatting/Clippy, optimized build and six native project/startup checks.
Every other active guard and the original snapshot remain exact. The retired
entry retains its history; 43 frozen files and 143 direct scientific access sites
remain alongside three dependency exceptions.

The confidence histogram implementation remains byte-for-byte unchanged and
still needs the coupled receiver-result migration. Retiring this direct-import
entry does not accept that numerical presentation or the parent task. Final
numeric, zero-exception, session and unified runtime gates remain pending. Existing
backend/Rust-Python scientific acceptance remains unchanged; no non-Qt rerun was
needed for this consumer-only replacement.

## UIU-DS-01 adapter contracts facade accepted

The [adapter-contract evidence](downstream-contracts-evidence.json) covers the
lifecycle bridge's public OCD admission and the receiver bridge's public stage
identities. Existing QObject names, selectors, displayed tap IDs, configure-time
invalidation, worker/provider calls and atomic Apply remain compatible. Receiver
source changes are otherwise byte-identical after reversing its public enum
import/name. This does not accept the still-legacy worker or instrument providers.

OCD size guards reuse `MAX_TRANSMISSION_DOCUMENT_BYTES`. Receiver-option Apply
and Restore reuse the independently documented inclusive application byte ceiling
`MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES` in `src/downstream/common/constants.rs`.
Both retain 4096 bytes; their different contracts are not tied together. Original
oversize diagnostics and rejected-state behavior remain unchanged. Public OCD
errors forward their existing human-readable message without a new category prefix.

Five closed-loop compatibility and three settings Rust cases pass, along with
49 workbench QML cases, sixteen boundary regressions, formatting/Clippy, optimized
build and six native project/startup smokes. Every other guard and dependency
remains exact, with retired entries retained as history. Three dependency
exceptions, 41 frozen files and 141 direct scientific sites remain; UIU-DS-01
stays unchecked. No backend, Python, persisted schema or QML files changed, so
existing paired scientific/API evidence remains applicable. Final numeric and
unified runtime acceptance remain separate.

## UIU-DS-01 verification contract facade accepted

The [recovered-data chart record](downstream-verification-evidence.json) covers
the public `FullLoopReport` contract and centralized
`MAX_VERIFICATION_WORD_BARS`. All three display-cap uses now share one downstream
presentation owner. The unchanged 64-bar policy remains independent of the
backend retention budget. Recovery/comparison totals, per-word ordering, zero
coordinates, incomplete comparisons and omitted counts retain their behavior.

Reversing only the nominal report import/name, named cap substitutions and the
fixture explanation yields the exact original source. The existing independent
12:3 ratio/zero/attention assertions pass without modification. The scoped Rust
case, three QML soft-display cases, sixteen boundary regressions, formatting,
Clippy, optimized build and six native project/startup checks pass. Retire only
this file's direct report-import exception and preserve its history; 40 frozen
files and 140 direct scientific sites remain alongside three dependency exceptions.

This accepts the focused presentation contract only. The original result provider,
shared-direction instruments, final numeric review and unified runtime gates remain
pending under their existing tasks. No backend, Python, saved schema or QML owner
changes, and no non-Qt scientific rerun is needed.

## UIU-DS-01 source-profile compatibility API accepted

The [source-profile compatibility evidence](source-profile-compatibility-evidence.json)
accepts the backend prerequisite before its Qt consumer. Rust
`TransmissionConfiguration::check_profile_capture` and the same native Python
method borrow an immutable source catalog and original byte-sized identity. They
delegate to the existing selected-profile lookup and editor grid validator; no
new geometry rules, simulation, source advance or run-ID admission is introduced.
The previous `check_profile` operation and all prior behavior remain unchanged.

Six source-catalog and five existing admission Rust cases pass, covering both
numerologies, original ID 255, absent IDs, spacing/occupied-band failures, unchanged
original bytes/settings and rejected run identity. Thirteen installed wheel
admission/catalog/namespace cases pass with native integer conversion boundaries,
typed error categories and owned provenance. An initial test compared an error
enum with a string; only the assertion's expected type changed. The exact same
installed wheel then passes; the failed receipt remains preserved. The initial
zero-case feature selection, incomplete rustdoc inventory and Rust fixture-style
failure are preparatory failures, not acceptance evidence.

Core/native Clippy, root/binding formatting, analysis-only compilation and 34 API
metadata regressions pass. Fresh complete-feature rustdoc and actual installed
native metadata validate the exact 564 Rust/537 native operation matrix against
recorded executed cases. Every previous operation row and class export remains
deep-equal; every previous installed root/namespace operation signature/kind is
preserved. Original full backend/native acceptance stays historical, with this
small additive query receiving scoped acceptance rather than a claimed full rerun.

The Qt tree, executable, dependency records and all 40 current guard entries remain
unchanged in this prerequisite. Commit this public extension before adapting its
profile editor consumer. UIU-DS-01 and final numeric/unified runtime gates stay open.

## UIU-DS-01 idle replay-capacity API accepted

The [idle-capacity evidence](idle-replay-capacity-evidence.json) accepts the next
backend prerequisite before its Qt consumer. Rust
`MeasurementSequenceOptions::capacity` and its native Python method expose the
existing complete-symbol clock calculation without a profile, captures or sequence.
The previous admitted-sequence method delegates to that same owner. Existing
duration admission, resource saturation, static modes and temporal results remain
compatible; zero temporal intervals return InvalidInput rather than dividing by zero.
Static modes ignore dormant intervals and original epoch tolerance remains separate.

Six Rust sequence cases and thirteen installed native sequence/admission/namespace
cases pass. Independent 1-ms anchors cover 23/44 complete symbols for 25/50 kHz
with a 512-sample prefix, strict final-symbol positions, zero/static/maximum intervals,
the existing one-hour API duration domain and 256-capture saturation. Options and
timing remain unchanged, and original admitted-sequence power/ownership cases pass.
No source or RNG advances; this reports possible positions, not admitted inputs.

Root/native Clippy, formatting, analysis-only compilation and 34 API metadata
regressions pass. Fresh complete-feature rustdoc and actual installed native metadata
validate all 565 Rust/538 native operation rows against recorded executions. Every
previous row, class export, constructor and signature remains exact. Full historical
backend/native acceptance is retained separately from this scoped additive query.

All Qt sources and forty current guard entries remain unchanged; the three
scientific dependency exceptions and 140 direct sites still require migration.
Commit the API prerequisite before adapting selection status. No resource policy,
operating mode, schema or dependency is added. UIU-DS-01 and final numeric/unified
runtime acceptance remain open.

## UIU-DS-01 minimum observation timing API accepted

The [minimum-window evidence](minimum-observation-timing-evidence.json) accepts
`TransmissionConfiguration::minimum_observation_milliseconds_after` and its
installed native Python method before Qt adoption. The existing timing owner
computes the inverse complete-symbol clock with checked integer arithmetic.
The shared DSP common constants own exact millisecond/nanosecond SI scales;
source-observation limits, host runtime and UI budgets remain separate policies.

Ten Rust timing/admission cases and fourteen installed native sequence/admission/
namespace cases pass. Independent rational anchors cover both sample clocks,
exact versus fractional millisecond rounding, an alternate prefix, zero/maximum
source indices and minimality. The largest index reports a theoretical threshold
above observation admission; no run, acquisition/drain/window-tail substitution,
source/RNG advance or input mutation occurs. Original test assertions remain.

Root/native Clippy, formatting, analysis-only compilation and 34 API metadata
regressions pass. Fresh complete-feature compiler and installed native inventories
validate 566 Rust/539 native operation rows against recorded executions. Every
previous row, class export, constructor and signature stays exact. The installed
wheel admission subset was rerun after adding exact-boundary test anchors; wheel
code is unchanged and both receipts remain. Historical full backend/native
acceptance is separate from this scoped API query.

All Qt sources, dependency records and forty frozen boundary entries stay exact.
The three scientific dependencies and 140 direct sites remain migration work.
Qt help adoption and final numeric/unified runtime acceptance stay open under
UIU-DS-01/UIU-QA-01. No new operating mode, schema or dependency is introduced.

## UIU-DS-01 relative channel power API accepted

The [relative-power evidence](relative-channel-power-evidence.json) accepts the
additional optional `ChannelSpectrum.normalized_power_db` observation in Rust
and the matching native Python dictionary key. The existing scientific owner
derives this full-band relative reading after unchanged linear integration.
Zero stays None/null, with linear zero preserved; no electrical calibration,
per-Hz reference, gain application or finite floor is inferred. Its named power
decibel factor is shared with the existing density conversion through common
constants; amplitude scaling and the logarithm base retain separate meanings.

Ten scoped Rust cases pass: three scientific-owner cases, five idle TX cases and
two actual channel-observation cases. Independent fixed flat density gives
0.927734375 normalized band power and -0.32576351350964183 dB, with both sample
clocks, internal exclusions, zero/null and zoom/calibration independence covered.
Twelve installed native channel-spectrum/TX/namespace cases pass without skips,
including fresh separate Rust full-loop and idle TX process references. Original
timing, waveform, cancellation and scientific test assertions remain in place.

Root/native Clippy, formatting, analysis-only compilation, 34 API metadata cases
and strict operation tracing pass. All 566 Rust/539 native operation rows and all
installed class/constructor/member signatures remain exact. The public result was
already non-exhaustive; no new operation or construction requirement is added.
Historical full backend/native gates remain separate from this scoped addition.

All Qt sources, locks and forty frozen legacy entries remain unchanged. Commit
this API prerequisite before idle preview adoption. Parent DS-01 and final
numeric/unified runtime acceptance remain open; no hosted/installed GUI or
external interoperability acceptance is claimed.

## UIU-DS-01 unnamed file source API accepted

The [unnamed-source evidence](unnamed-file-source-evidence.json) accepts the next
DS-01 compatibility prerequisite before Qt adoption. The existing Qt release
accepts an empty file-source display label and rejects empty scientific capture
provenance. The explicit public `SourceInput::unnamed_file` and native
`SourceInput.from_unnamed_bytes` preserve that distinction, retaining an empty
original label, exact bytes/hash and independent optional fill. Existing named
constructors keep their validation. Saved full-loop loading uses the same source
owner; profile/RxMER/H still use named capture/provenance admission.

Twenty scoped Rust cases and sixteen installed native admission/saved-full-loop/
namespace cases pass, including a fresh separate Rust saved-workbench reference.
They preserve existing source/preview/timing/real-FEC assertions and add exact
original-file recovery with an empty label, caller ownership, zero-byte input,
independent fill and byte/name/integrity boundaries. Literal source SHA-256 and
bit-count anchors remain independent of production constants. The old saved
name/base64 endpoints move unchanged to their common policy owner.

Root/native Clippy, formatting, analysis-only compilation, 34 API metadata cases
and strict compiler/installed operation tracing pass. All earlier rows, class
exports, constructors and signatures stay exact; one operation per language
brings the matrix to567 Rust/540 native rows. Full historical gates are retained
separately from this focused prerequisite.

Sixteen Qt WIP files remain preserved under the recorded local stash/manifest;
this API commit changes no Qt sources, dependencies, locks or frozen boundary
entries. Restore/verify that work, adopt the explicit file-label operation and
finish Qt/build/native checks afterward. Parent DS-01 and final numeric/unified
runtime acceptance remain open. No push, release or external interoperability
acceptance is claimed.

## UIU-DS-01 idle transmitter preview facade accepted

The [preview consumer evidence](downstream-preview-evidence.json) accepts the idle
TX adapter through the public facade. Original accepted source/profile sections
become owned SourceInput/ProfileCapture inputs; public preview requests, one-shot
control, progress/cancellation and results replace direct engine access. Public
symbol timing/window coefficients and calibrated spectrum observations supply the
original graph fields, axis labels, PLC shading and zoom-independent power.
Qt retains only drawing/label projection. Named captures remain strict, while the
separately accepted unnamed-file operation preserves original empty source labels.
Independent Open preparation uses its own original sections, without GUI globals.

Eleven targeted Rust cases pass on the final adapter: three original preview,
two timing/window, five project-load cases and one original-byte/file-input case.
Both FFTs and all44 valid original CP/RP pairs retain independent clock/taper/
overlap assertions. Every old preview assertion and case name remains; original
fixtures now enter through the public capture loader. Added cases preserve dormant
seed/fill semantics, arbitrary exact file bytes, explicit empty labels and the
existing zero-count/full-width viewport convention. All89 original QML cases,
16 boundary regressions, final Clippy/formatting and optimized Qt build pass.
Four curated native preflights and two native XCB/Fusion/software startup checks
pass with the fresh executable and rediscovered VM display credentials.

Two exact original guard entries retire; the remaining38 files, three dependency
exceptions and original immutable snapshot stay intact. No backend, Python,
lockfile, QML or QObject registration change occurs in this consumer unit.
Shared spectrum rendering currently serves idle TX; legacy channel/receiver
formatters still await convergence. This is scoped migration acceptance, not final
DS-01/session/upstream/installed-platform or complete numeric-audit acceptance.

The first Clippy run rejected an equivalent inclusive-end expression; the final
exclusive-end form passes all11 targeted Rust cases and final Clippy. Earlier
receipts remain. Backend/native prerequisites retain their own accepted evidence
without repeating full workspace tests. WIP restore hashes and doc reconciliation
are recorded. A confirmed empty RCC resource-watch path causes repeated adapter
builds; address that in a separate focused unit, preserving targets and the SDK.

## UIU-DS-01 resource build cache accepted

The [resource-cache evidence](resource-build-cache-evidence.json) accepts a
build-only correction to the pinned resource-list parser's trailing empty path.
The unchanged branding QRC is generated with the existing public Qt tools and
its initializer object is linked directly. Only empty list paths are filtered;
the 114 current watches all resolve to existing input files, including all five
unchanged branding assets. All 210 lock package records retain their exact
versions, checksums and dependencies except the Qt root's two reviewed build edges
to already-present exact pins. The initial immutable science boundary is unchanged.

Actual optimized Cargo builds prove two no-op cache hits, a cache hit after an
ignored target-artifact write, real QRC invalidation and a final cache hit.
The four cache hits take 0.13–0.21 seconds and preserve output/object/executable
hashes and modification times. Touching the QRC, without changing its bytes,
causes a 98.23 second rebuild. No target cache or SDK is cleaned or patched.
The final executable retains branding resource initializer symbols and passes
four curated native project checks and two XCB/Fusion/software startup checks,
all without stderr. Formatting, Qt all-target Clippy and 20 boundary regressions
pass. New boundary fixtures prove that the resource dependencies remain build-only,
owned build helpers receive scientific-call checks, and helpers cannot escape
the owned source trees. The earlier external-module guard failure is retained
in local receipts.

This unit changes no scientific implementation, public API, Python adapter or
QML presentation, so their prior acceptance is reused rather than rerun.
The new build helper is unignored and explicitly included in the commit; a fresh
extracted source-package build and Windows/installed lifecycle were not executed.
DS-01, its 38 remaining frozen files, three science dependencies and final numeric
audit remain open. The latest optimized executable includes this correction.

## UIU-DS-01 public progress phase accepted

The [progress-phase evidence](progress-phase-evidence.json) accepts public Rust
reachability of the exact existing ChannelPhase enum through
`merlin::downstream`. Its scientific owner, six variants and serialization are
unchanged. The facade compatibility test proves nominal legacy identity and a
typed ChannelProgress field projection; three actual Rust control cases retain
completion, initial pause, resume, cancellation and completed FEC comparisons.

A fresh environment outside the checkout installed the prior unnamed-file wheel
and passed six native full-loop and four namespace cases without skips, including
actual completed progress whose
phase remains `"finished"`. The direct-process comparison explicitly reuses the
recorded historical Rust full-loop reference; its settings and science are
unchanged by this re-export. The earlier fresh-source-wheel claim was incorrect:
the local runner selected the older archive. Original receipts are retained and
the evidence records this as compatibility reuse. The worker-budget prerequisite
now revalidates the phase/snapshot cases against its exact newly built and
checksum-matched installed artifact. All 38 API metadata tests, formatting, scoped
all-target Clippy and strict fresh Rust/installed-native inventory checks pass.
Only one Rust enum type export is added; all 567 Rust/540 native callable rows,
112 native class exports and prior execution records remain exact.

Qt wall-time consumption has separate scoped acceptance below. Parent DS-01
and final numerical review remain pending. This phase-export unit changed no Qt,
engine, Python runtime implementation, dependency or saved schema. Historical full-suite/platform acceptance is kept separate.

## UIU-DS-01 wall-time adapter facade accepted

The [wall-time evidence](downstream-wall-time-evidence.json) accepts the timing
adapter's use of the separately committed public ChannelPhase enum. Existing
elapsed/profile/progress-age values, freeze behavior and all six phase labels
are preserved. Standard Duration conversion replaces inline milliseconds-to-
seconds division. Shared presentation constants distinguish seconds/minute from
minutes/hour and derive seconds/hour; neither definition is a simulation budget.
The existing named profile-timing cap remains with its current result owner
pending that owner's migration, avoiding a duplicate policy definition.

Three scoped Rust cases pass, retaining all original assertions and adding
all-phase label coverage. One focused QML elapsed/pause/completion case passes
(three harness checks including initialization/cleanup). Qt all-target Clippy,
formatting, 20 boundary regressions, current guard, optimized build and six
native project/startup checks pass. One exact original frozen entry retires;
the other 37 files, three scientific dependencies and immutable initial record
are unchanged. No QML, backend, binding, saved setting or dependency changes.

This is scoped presentation acceptance. The full-loop worker and other science
consumers, session isolation, complete DS-01 and final line-by-line numeric audit
remain pending. Backend/native API suites retain the separately committed
prerequisite evidence; no full historical workspace gate is repeated here.

## UIU-DS-01 worker budget and progress age accepted

The [worker execution evidence](worker-budget-progress-evidence.json) accepts two
paired public operations before Qt consumption. Runtime narrowing uses existing
policy validation, rejects widening and preserves the original request/settings,
inputs, tolerance and observation policy. Actual limited execution returns the
existing runtime-exceeded report. Notification age comes from a private monotonic
timestamp under the scalar-progress mutex; reads do not refresh it or change
snapshot fields. None before progress, paused age and terminal age are tested.

Six scoped Rust and twelve exact-archive installed native cases pass, including
original completion/FEC, pause/resume/cancel, initial pause, original-file recovery,
interrupt and namespace checks. Independent runtime/timer fixtures are named or
explained, separate from production policy endpoints. Native integer conversion
and shared runtime boundaries are covered. Formatting, root/native all-target
Clippy, 38 metadata cases and strict fresh compiler/installed/executed inventory
checks pass. Two operations per language bring the matrix to 569 Rust/542 native
rows; every earlier row, constructor/signature and all 112 native class exports
stay exact. The normal-run direct Rust-process reference is explicitly reused;
new budget/age cases execute the current backend independently.

The first compile failed on an incorrect Instant import and was corrected. The
first native runner selected an older wheel, causing missing-method errors and
paused test workers; that process was terminated. The corrected run installs the
exact just-built archive in a new environment, verifies the installed native
module's checksum against the archive and checks operation presence before
bounded scientific tests. Failed receipts remain. The build initially reused a
prior generated-wheel output path; its new archive was moved to a unique owner.
The prior installed artifact and test receipts remain, while no restoration of
the overwritten generated archive is claimed.

That audit also corrects the earlier public-phase record: its ten native cases
used the prior unchanged runtime wheel, rather than the newly built phase archive.
Original records remain, with an explicit [provenance correction](progress-phase-evidence.json).
This current exact-archive run revalidates its phase and snapshot assertions;
there is no retrospective fresh-build claim. Qt/engine algorithms, dependencies,
saved schemas and frozen boundary records are unchanged. Parent DS-01, coherent
worker migration and final line-by-line numeric audit remain open.

## UIU-DS-01 structured engine diagnostic accepted

The [diagnostic evidence](engine-diagnostic-evidence.json) accepts a typed
downstream getter for the already-retained original backend error code. None
continues to mean no retained cause; neither the generic category nor display
text invents one. Native `.engine_code` copies the original code through the
appropriate public Rust getter for each direction, with None as the manual
exception default. Existing category/message/legacy-code/field/offset semantics
and Rust error serialization remain unchanged.

Ten scoped Rust cases and thirteen exact-archive installed native cases pass.
They include actual rejected zero-runtime validation, clone/borrow/serde
ownership, misleading text without a source, facade/category separation,
upstream MissingSetting metadata, source/admission regressions, original
namespace/control behavior and targeted full-loop/budget errors. The installed
module matches the newly built archive before tests start. Root/native Clippy,
formatting, analysis-only compilation, 38 metadata cases and strict fresh
compiler/installed/executed tracing pass. One Rust operation and two native
namespace field projections bring the matrix to 570/544; all prior rows/types,
948 prior native root/namespace operation instances and 112 class exports remain
exact after removing only the new field from comparison.

No scientific implementation, error category, dependency, saved schema, Qt or
frozen boundary changes. This does not claim full historical workspace or Qt
acceptance. Commit this prerequisite before help/worker consumption; parent
DS-01, coherent run migration and the final line-by-line numeric audit remain open.


## UIU-DS-01 controlled worker facade accepted

[Design and exact owners](design.md#uiu-ds-01-controlled-worker-facade-migration),
[structured evidence](downstream-worker-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-07](../../../requirements.md#req-uiu-07),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-17](../../../requirements.md#req-uiu-17),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this focused consumer.

The serial downstream worker now uses public requests, sweep preflight, controls
and actual bounded stage queries. Original source/profile bytes, accepted RxMER
frames and original H remain paired during edits and background Open. A read-only
public report projection supports existing instruments; no private engine call,
mutable report, reconstructed stage values or Qt numerical algorithm is added.
UI wall time remains separate from simulation time and uses actual notification
age. Cancellation, pause, active-runtime narrowing and stale-result rejection
remain explicit. Independent H tolerance fixes the old worker's reuse of RxMER's
policy, with equality/one-second rejection tested separately.

Passing scoped checks include 28 Rust cases: all five original worker scientific/
instrument cases, actual file recovery with a dormant zero continuous seed and
an empty display name, paired-original failures, pause/cancel partial-report
ownership, invalidated late results, H tolerance, timing/help and saved-file/
background Open regressions. Eight affected QML behavior cases pass, plus harness
setup/cleanup (ten total); two Qt focus-property warnings are recorded.
The impaired fixture still yields 20.010 dB TX-reference MER versus 33.233 dB
nearest-point MER, within the original independent tolerances. This is MERLIN-only
closed-loop evidence, not external interoperability.

Clippy for all Qt targets and twenty architectural-boundary regressions pass.
The exact reviewed boundary drops from 37/133 to 33/105 frozen files/reference
sites, with the same three direct scientific dependencies. Four fully migrated
file entries retire. The Designer's one affected real-FEC test changes to public
execution; its production and all other test bytes are exact. Its remaining
scientific references stay frozen. That test now uses the accepted public Qt
retention defaults rather than its former one-word/no-probe policy; its source,
runtime and scientific assertions remain intact.

Named directional constants cover independent dispatch, time-window, message
and help policies. API validators/defaults own scientific domains and retention;
stage lists own array dimensions. The final line-by-line cross-module numeric
audit remains pending. No backend/API/binding/dependency/lock or persisted schema
change occurs; prior Rust/native API evidence is preserved rather than relabeled
as a fresh installed-wheel run. The latest optimized Qt build passes all six native VM checks (four curated
workbench admissions and two XCB/Fusion startup/Open runs), with fresh display
credentials and no stderr. Final rustdoc-only banner additions are recorded
with tested/final source hashes; Clippy, formatting and the optimized build
were repeated after them. No executable token changed. The parent DS task and
later global-session/upstream/final acceptance remain incomplete.


## UIU-DS-01 transmitted PLC message viewer facade accepted

[Design/owners](design.md#uiu-ds-01-transmitted-plc-message-viewer-facade),
[structured evidence](plc-message-viewer-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this focused consumer.

The Qt viewer now calls public FullLoopResult.plc_messages() for actual
CRC-validated TX packets, source/profile identity, typed descriptor meanings,
exact offsets, pending/issue state and complete-versus-scheduled coverage. The
adapter formats labels and bytes; private frame/MAC/descriptor decoding,
modulation/CP/RP interpretation and the duplicated frame-end calculation are
removed. A shared byte formatter preserves row/address conventions. Named
presentation policies distinguish record limits, NCP wire identity, QPSK labels,
TLV header width and assignment byte annotations. Backend admission remains
separate; zero loading retains the legacy raw-only tree presentation.

Eleven focused Rust cases pass: five Qt tree/real-instrument cases, five existing
backend message/descriptor owner cases and one actual public message integration
case. The three original tree test names and every original byte/label/offset/
unsupported assertion remain. Their typed meanings are explicit format fixtures;
the owner tests separately validate scientific decoding. The retired duplicate
Qt scheduled-frame test's exact four last-sample/zero-stride/overflow assertions
are already covered by the existing backend independent-anchor test, executed in
this unit. No scientific source/test or vector changed. The actual public message
case also exercises absent/truncated data, passive FEC outcomes and original-byte
ownership. These are MERLIN-only checks, not external interoperability.

The affected message/QAM/tree navigation QML case passes with setup/cleanup
(three harness cases). All-target Qt Clippy passes. Twenty boundary regressions, the actual source/dependency guard, formatting,
documentation and traceability pass. The latest optimized Qt build passes all
six native VM checks (four curated admissions and two XCB/Fusion startup/Open
runs), using fresh display credentials and producing no stderr. Exact receipts
and executable/source hashes are retained in the structured record. Two whole source exceptions retire; every
remaining frozen entry is exact. Current debt is 31 files/103 direct source sites/
three scientific dependencies. No API/binding/dependency/lock/schema or QML layout
change occurs, and no full historical suite or new native Python wheel is claimed.
The parent downstream task and final line-by-line numeric audit remain incomplete.


## UIU-DS-01 pre-channel physical-bin scatter facade accepted

[Design/owners](design.md#uiu-ds-01-pre-channel-physical-bin-scatter-facade),
[exact evidence](physical-input-scatter-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) own this focused consumer.

The pre-channel physical-bin scatter viewer now consumes public actual retained
I/Q, original symbol/frequency coordinates and carrier roles. It no longer
constructs private CarrierLayout or calculates carrier frequency/roles. Public
SubcarrierIndex owns the unchanged explicit coordinate domain; the UI's -1 open
bounds, reversed-range rejection and error ordering remain. Excluded/missing
bins stay absent. Physical Data never becomes guessed payload/NCP provenance.
Qt retains only finite drawing selection and named display caps, axis center,
TX minimum extent and ten-percent headroom; scientific values remain unchanged.

Three focused Rust cases pass: the new independent selector-domain/ordering
case, existing actual Qt worker/instrument assertions (including exact I/Q,
selection and amplifier behavior), and the existing public physical-bin/
transmit-cell test covering original values, power, roles, context, missing/zero
history and invalid windows. No old assertion, scientific source or vector is
changed. The transmitter message/QAM navigation QML case passes with setup/
cleanup (three harness cases). Formatting, all-target Clippy, twenty boundary regressions, the actual
source/dependency guard and canonical document checks pass. The latest optimized
Qt build passes all six native VM checks with fresh display credentials and no
stderr. Exact source/executable hashes and receipts are retained in the structured
record. One whole legacy exception retires; every
other frozen entry remains exact. Current debt is 30 files/102 source sites/
three scientific dependencies.

No API/binding/dependency/lock/schema or QML layout change occurs. The native
Python operation history remains separate from this consumer's acceptance; no
new installed wheel or full historical suite is claimed. Physical-power graphs,
waveform-spectrum sample budgets, later session/upstream work, the parent DS
milestone and the final line-by-line numeric audit remain incomplete.


## UIU-DS-01 spectrum sample-budget API accepted

[Design/owners](design.md#uiu-ds-01-spectrum-sample-budget-prerequisite),
[exact evidence](spectrum-budget-evidence.json),
[executed operation cases](ds-01-spectrum-budget-operation-test-results.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this API prerequisite.

The additive TransmissionConfiguration.spectrum_average_capacity method delegates
to the existing waveform spectrum owner in Rust. It counts complete half-overlap
windows fitting an explicit sample budget and caps work at the existing policy.
Budgets below the accepted FFT reject; oversized positive budgets cannot widen
work or allocate/read samples. Configurations stay immutable and idle. Actual
retained availability remains separate. The estimator keeps its original length,
sample and frequency validation and the same numerical loop, reusing the owner's
window-count helper. Existing queries, fields/defaults/serialization and legacy
imports remain unchanged; Python performs no scientific calculation.

Thirteen focused Rust cases pass, including independent two-spacing boundaries,
saturation, original tone/integral/ENBW/silence/nonfinite vectors, facade identity
and a real long-CP retained-prefix case. With 25 kHz spacing and the supported
1024-sample CP, four actual blocks contain 36864 samples/eight windows; the
legacy 32768 prefix uses seven. Context, first-sample/reference identity and the
entire actual report and its FEC comparison remain unchanged. This is MERLIN-only numerical
and compatibility evidence, not external interoperability.

A fresh uniquely owned Linux CPython 3.10 wheel passes archive-to-installed-module
hash preflight before thirteen native admission/direct-Rust-process/namespace
cases, with no skips. All prior 570 Rust/544 native rows and type exports remain
exact; all 951 prior root/namespace native operation instances are exact after
removing only the new method. The candidate has 571 Rust/545 native rows and
80 root class exports and 112 directional/common class exports (79 downstream,
30 upstream and three common). Final strict inventory/operation, metadata and
format/Clippy receipts are recorded with the exact source/wheel hashes.

No dependency, saved schema, Qt source or frozen record changes. Completed API/
FEC/planning tasks remain closed. Qt spectrum consumers, the 30-file/102-site/
three-dependency migration debt, the parent DS task and final numeric audit remain
incomplete. The latest Qt executable is still the preceding accepted consumer;
this prerequisite adds no Qt adoption or native desktop/hosted/release claim.

## UIU-DS-01 retained transmitter views facade accepted

[Design/owners](design.md#uiu-ds-01-retained-transmitter-views-facade-unit),
[exact evidence](transmitter-views-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this scoped consumer.

Physical graphs consume actual public bin power, frequencies and bounded role
spans. QAM consumes backend transported roles and missing history. Qt retains
only display reduction, finite drawing coordinates, role-group quotas and the
existing stage pages/cards. Duplicate snapshot role/grid copies are removed.
Distinct page/raster/bit/label/power policies have named directional owners;
cell views reuse their shared amplitude extent and shared drawing/unit constants.

Twelve focused Rust cases pass: six Qt cases, five backend role/power cases and
the actual bin/cell API case. All old graph/rare-role and worker assertions remain.
The original 128-cell/depth-four interleaver comparison, one NCP, 96 missing
positions and later-symbol rejection now run at the backend role owner, beside
the independent eight-cell vector. The original 3+4i→25 oracle runs at the
existing bin-power owner; the Qt test formats supplied power with a real public
configured layout. Added checks compare actual rendered powers/roles against
public results and verify parallel roles at nonzero retained offsets.

Three whole-file exceptions retire, leaving 27 files/96 sites/three dependencies.
The views file receives a focused review for stage-only migration. Its two TX
spectrum functions remain byte-for-byte exact, identified by their retained hash;
no remaining spectrum debt is accepted. Original IFFT/prefix/window plots join
up to 16384 ordered-frame samples. The normalized public sequence query and
single-frame calibrated query have different sampling contracts. A separate
typed calibration prerequisite must preserve that distinction before migration.
The accepted sample-capacity helper is insufficient alone.

QML navigation, boundary regressions, Clippy, optimized/native checks and their
exact executable identity are recorded in the evidence. The UI appearance and
existing screenshots are unchanged. No new wheel, complete historical suite,
hosted/platform lifecycle or external interoperability is claimed. This parent,
remaining spectrum/receiver/input migrations and the final numeric audit stay open.

## UIU-QA-01 risk-based validation workflow refined

[Design/file review](design.md#uiu-qa-01-validation-scheduling-refinement) and
[testing strategy](testing-strategy.md) implement the user's three-level
scheduling instruction under [REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-26](../../../requirements.md#req-uiu-26) and the existing
[acceptance owner](../../../architecture.md#unified-qt-acceptance).
The policy is routed from AGENTS and the canonical development-workflow,
scientific-testing and git-workflow skills. Runners, CI/push/release gates,
tests/vectors, dependencies and build/cache profiles are unchanged.

| Validation scope | Actual result / reason |
| --- | --- |
| Level 1 executed | Current-mode feature_documentation.py and integration_traceability.py pass. quick_validate.py passes for all three modified skills. Python integration guide --check and git diff --check pass. |
| Preservation audit executed | Every official checkbox line/state and task anchor is exact; all 21 UIU tasks remain, with nine completed. Requirement anchors are unchanged; only REQ-UIU-26 gains a scheduling reference. The pending calibrated-sequence source/test/design work is preserved separately and is not included in this workflow commit. |
| Not required for this change | Rust/scientific/runtime tests, Clippy, Qt/QML/native launch, wheel rebuild/native parity, CLI/project and packaging regressions were not run: this unit changes only documentation and skill routing, with no executable contract, source, test, runner, dependency or build change. No runtime suite is reported passed. |
| Deferred acceptance gates | Complete relevant UIU-DS-01 feature/crate/Qt/compatibility/traceability acceptance remains Level 2; full applicable integration/final suites and UIU-QA-01 remain Level 3. Existing task-specific criteria, zero scientific-boundary gate and numeric audit remain required. |

This is workflow/documentation acceptance only. No official UIU status changes
and no prior implementation/planning milestone is reopened. Continue the existing
dependency order with the preserved calibrated ordered-frame API prerequisite.

## UIU-DS-01 calibrated ordered-frame API accepted

[Design/owners](design.md#uiu-ds-01-calibrated-ordered-frame-api-prerequisite),
[exact evidence](calibrated-sequence-evidence.json),
[executed cases](ds-01-calibrated-sequence-operation-test-results.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this prerequisite.

FullLoopResult.calibrated_spectrum_sequence returns unchanged normalized
sequence metadata and ChannelSpectrum from one full-resolution estimate. Rust
owns retained-frame ordering, physical gaps/overlaps, stop/error semantics and
electrical calibration. Python only converts arguments/owned output and detaches
Rust work from the interpreter. Existing normalized/continuous/channel methods,
queries, defaults, numerical algorithms and imports remain compatible.

Level 1 passes ten focused Rust and eleven installed native cases. Coverage
includes original independent PSD and sequence vectors, exact normalized output,
relative/electrical units and zoom-independent power, missing/incomplete frames,
invalid work/stage/viewport, immutable reports/retention, owned native dictionaries,
legacy/facade imports and separate current Rust-process parity. Existing tests
and assertions are retained. Formatting, scoped root/native Clippy, analysis-only
build and complete additive compiler/native API tracing pass.

One exact wheel is built, installed outside the checkout and verified against
its archive/module hash. Initial native channel parity encountered a historical
reference lacking the previously accepted normalized_power_db field. Current
separate Rust full-loop/transmitter references resolve that comparison; earlier
logs/reference files remain intact. The same installed wheel is reused without
changing assertions/tolerances or rebuilding. All 571/545 old operation rows and
prior native members remain exact; the candidate has 572 Rust/546 native rows,
one additional Rust report type and unchanged 80 root/112 namespace class exports.

The evidence explicitly records not-required and deferred checks. No Qt source,
registration, lifecycle, project schema, dependency or scientific algorithm is
changed; no full workspace/Qt/native/CLI/project/packaging suite is claimed.
Qt adoption remains the next focused consumer. This API unit retires no frozen
entry: 27 files/96 sites/three dependencies remain. Official UIU-DS-01 Level 2,
major/final Level 3 and the final numeric audit remain pending.

## UIU-DS-01 TX spectrum facade accepted

[Design/owners](design.md#uiu-ds-01-tx-spectrum-facade-adoption),
[exact evidence](tx-spectrum-facade-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this consumer.

Qt now queries public calibrated ordered-frame/continuous results, with separate
named 16384/32768-sample prefixes planned by the accepted public capacity method.
Public inventory supplies readiness and configured public layout supplies fixed
PLC bounds. Shared drawing-only formatting preserves axes, scaling, labels and
zoom-independent calibrated/relative full-band power. No private estimator or
CarrierLayout remains in views.rs. One whole exception retires, leaving 26
files/94 reference sites/three scientific dependencies; all other frozen entries
and prior history remain exact.

Level 1 passes two actual Qt Rust cases, two QML behavior cases (four including
harness initialization/cleanup), actual locked boundary audit, Qt formatting/
all-target Clippy, optimized build and documentation/traceability checks. The
new supported long-CP/three-frame run independently preserves three frame-prefix
averages and seven continuous-output averages, authoritative power, zoom/axis/
PLC and error behavior, plus immutable report/retention. Every previous worker/
instrument assertion remains. The latest optimized executable hash is recorded.

Backend/API/native sources and QML/registration/lifecycle/persistence contracts
are unchanged. Another wheel, root/scientific/native/complete Qt suite, unrelated
startup/Open and packaging checks were not required; the evidence records why
and makes no such pass claim. Existing screenshots still describe the unchanged
appearance. User guides clarify existing sampling meanings and accepted PSD
units. Official parent Level 2, milestone/final Level 3 and numeric audit remain
pending.


## UIU-DS-01 RxMER animation preview facade accepted

[Design/owners](design.md#uiu-ds-01-rxmer-animation-preview-facade),
[exact evidence](rxmer-animation-preview-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this consumer.

The viewer now uses existing public NamedCapture, RxMer.from_frame and
RxMer.preview owners. The original name/bytes remain separate from decoded
or Designer data; only the selected frame is adapted, never the collection.
No averaging, replay configuration or simulation is introduced. The complete
preview.rs exception retires; all other frozen entries/history remain exact,
leaving 25 files/93 reference sites/three scientific dependencies.

Level 1 passes the existing targeted real-project Rust animation regression
with added byte/frame/hash immutability checks, three QML behavior cases (five
including harness initialization/cleanup), Qt formatting/all-target Clippy,
the actual locked boundary audit, optimized build and documentation/traceability
checks. Every prior sparse-order, original-index, 132-input, viewport and error
assertion remains. The guide documents unchanged preview behavior; no visible
layout or screenshot changes are needed.

The backend, native bindings, public contract, registration, lifecycle and
project serialization are unchanged. A fresh wheel, non-Qt scientific/full
workspace, complete Qt/QML, native startup/Open, CLI and packaging suites did
not run for this focused unit; the evidence records the scope reasons. These
are not new passes or waived task requirements. Complete official UIU-DS-01
Level 2, milestone/final Level 3 and the final numeric audit remain pending.


## UIU-DS-01 source-verified FEC recovery API accepted

[Design/owners](design.md#uiu-ds-01-source-verified-fec-recovery-prerequisite),
[exact evidence](fec-recovery-evidence.json),
[executed cases](ds-01-fec-recovery-operation-test-results.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-19](../../../requirements.md#req-uiu-19),
[REQ-UIU-20](../../../requirements.md#req-uiu-20),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this prerequisite.

The public `FullLoopResult::fec_recovery` returns the constant-size typed
FullLoopFecRecovery; native Python returns the matching owned dictionary.
The existing downstream simulation counter owner derives recovered words without
counting decoder rejection or independently mismatched source data as success.
Generated and unfinished counts remain actual whole-run totals, independent of
observation windows or retained detailed word limits. Existing reports, source,
decoding and execution are unchanged.

Level 1 passes six Rust cases: two independent accounting cases, the existing
complete/cancel/empty/file public cases and the legacy/facade type contract.
Eight exact-archive installed native cases cover those actual outcomes, owned
results and separate current Rust-process parity plus all namespace contracts.
Scoped root/native Clippy, formatting, analysis-only compilation, strict actual
573/547 operation inventories and documentation/traceability checks pass. Every
previous 572/546 row, native class/member and execution record remains intact.
A private runner initially stopped before checks on an already-owned log name;
separate retry logs preserve that tooling failure without changing source/tests.

No Qt source, CXX registration, CLI/schema/lifecycle, upstream algorithm, shared
codec or package contract changed. Their runtime suites and unrelated full
workspace/Qt/package regressions were not required for this focused prerequisite;
all final gates remain pending. The FEC Qt file is still frozen, with 25 files/
93 sites/three dependencies unchanged, until separate consumer acceptance.


## UIU-DS-01 FEC monitor recovery facade accepted

[Design/owners](design.md#uiu-ds-01-fec-monitor-recovery-facade),
[exact evidence](fec-monitor-facade-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-19](../../../requirements.md#req-uiu-19),
[REQ-UIU-20](../../../requirements.md#req-uiu-20),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this consumer.

The monitor formats the accepted public FullLoopFecRecovery result rather than
subtracting private counters. Segment keys/order/labels/fractions, histogram
bins, observation-only categories, compared bits/BER, partial states and bounded
retained words remain compatible. The exact original five-word classification
oracle passed in prerequisite 7752a02a and now remains in its backend owner;
the two existing Qt drawing tests retain their complete assertions.

Level 1 passes four targeted Qt Rust tests: those chart tests, the actual worker
with authoritative per-segment/passivity assertions, and the original four-profile
archival/retention regression. Two QML behaviors (four including harness setup/
cleanup) pass overview/tab/profile/partial/empty and receiver navigation checks;
the latter emits two focus-property warnings with no failures. Qt all-target
Clippy, formatting, actual locked boundary audit, optimized build and document/
traceability checks pass. One whole-file exception retires, leaving 24 files/
91 sites/three scientific dependencies; every other frozen record stays exact.

Backend changes in this unit are documentation-only clarification: decoder
acceptance without a detected source mismatch is not positive proof when no bits
were compared. Existing counts/algorithms/public signatures and native bindings
are unchanged. Another wheel, root/full/scientific/native suites, unrelated
startup/Open/CLI/persistence/package checks were not required; the exact evidence
records why and claims no such passes. Original screenshots still match this
unchanged presentation. Parent Level 2, milestone/final Level 3 and numeric audit
remain pending.


## UIU-DS-01 profile plot inspection facade accepted

[Design/owners](design.md#uiu-ds-01-profile-plot-inspection-facade),
[exact evidence](profile-plot-facade-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this consumer.

Public CaptureLoader.profile_capture and ProfileCapture.inspect provide exact
original coordinate/role/loading/extrema, ideal Shannon and gross useful-rate
values. Qt retains existing graph normalization, output keys, role lanes,
selection, tick/axis choices and text; it no longer calculates scientific
profile metrics. Independent single/overlay axis extents, request policy and
single-profile grouping are named in the direction constants owner. Frequency
and bitrate SI conversions have separate shared definitions despite equal values.

Level 1 passes five Qt Rust tests: all four original profile cases and one new
raw-ID/request-boundary/passivity regression. Original binary/JSON equivalence,
source grids/loading/roles, missing markers, overlay/Shannon numeric assertions,
empty filters, errors and accepted-state preservation all remain. The added
case keeps raw IDs 17/255 distinct from run IDs, checks transport/list/metric
bounds and preserves original bytes/hash/document/assignments. Three affected
DPD QML behaviors pass (five with harness setup/cleanup). Qt format/all-target
Clippy, actual locked boundary, optimized build and docs/traceability pass.

The marker test's fixture previously mutated an internal profile separately
from original bytes. Only that preparation now creates the same first-carrier
exclusion as a valid decoded original, using existing encoded assignment 16.
Every original assertion, all production bytes and every other test byte in
profile_input/mod.rs remain exact. Its two remaining direct sites stay frozen
under a focused recorded review. All other entries/dependencies/history remain
exact; plot.rs retires completely, leaving 23 files/88 sites/three dependencies.

No backend/binding, schema, registration, lifecycle or package contract changed.
Another wheel, scientific/root/full Qt/native/CLI/persistence/package suites and
native startup were not required for this focused unit; evidence records the
reasons and makes no new full/installed pass claim. Existing dark screenshots
still describe the unchanged presentation. Parent Level 2, milestone/final
Level 3 and the final numeric audit stay pending.


## UIU-DS-01 source-file admission API accepted

[Design/owners](design.md#uiu-ds-01-source-file-admission-metadata-prerequisite),
[exact evidence](source-file-api-evidence.json),
[executed cases](ds-01-source-file-operation-test-results.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-09](../../../requirements.md#req-uiu-09),
[REQ-UIU-20](../../../requirements.md#req-uiu-20),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this prerequisite.

SourceInput.file_limits/file_info expose the existing FilePayload byte/bit bounds,
actual original/target/fill counts and independent append policy. With_fill
returns a revalidated immutable file selection sharing the exact original byte
buffer; None removes append and continuous inputs reject it. Empty finite files
remain distinct from PRBS; explicit zero-length append remains distinct from no
policy. These operations generate no source/RNG state or payload serialization.
Native Python delegates validation/counts and returns owned dictionaries/handles.

Level 1 passes six Rust cases: five focused source-admission cases plus the exact
legacy/facade namespace contract. Independent 16-MiB/134217728-bit ceilings,
8-/11-bit targets, original pointer sharing, name/hash/byte preservation, partial
bytes, empty/unnamed files and invalid changes are covered. Ten exact-archive
installed native cases cover source/namespace behavior, result ownership and a
passive separate Rust-process reference. No full simulation is needed for the
new source queries. Scoped root/native Clippy, formatting, analysis-only build,
strict actual 576/550 operation inventories and docs/traceability pass. Every
prior 573/547 row, native class/member and execution record remains intact.

The initial new Rust test failed to compile because its json macro import was
missing; the import was corrected without changing assertions. An earlier
build-only wheel contained two metadata operations; final source review added
necessary shared-buffer fill replacement before Qt adoption. A separate exclusive
final wheel/receipt supplies all native acceptance, including method-presence
and exact archive/installed module checks. Earlier logs/artifacts remain separate
and are not claimed as final native acceptance.

No Qt, CLI/schema/lifecycle, upstream/shared-codec or packaging contract changed.
Their runtime/full suites were not required for this focused additive source
API; reasons and pending gates are explicit in the evidence. Qt debt remains
23 files/88 sites/three dependencies until separate consumer acceptance. Parent
Level 2, milestone/final Level 3 and the line-by-line numeric audit stay pending.


## UIU-DS-01 source editor admission facade accepted

[Design/owners](design.md#uiu-ds-01-source-editor-admission-facade),
[exact evidence](source-editor-facade-evidence.json),
[architecture](../../../architecture.md#unified-qt-public-api-boundary),
[REQ-UIU-02](../../../requirements.md#req-uiu-02),
[REQ-UIU-03](../../../requirements.md#req-uiu-03),
[REQ-UIU-08](../../../requirements.md#req-uiu-08),
[REQ-UIU-20](../../../requirements.md#req-uiu-20),
[REQ-UIU-22](../../../requirements.md#req-uiu-22),
[REQ-UIU-24](../../../requirements.md#req-uiu-24),
[REQ-UIU-27](../../../requirements.md#req-uiu-27) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) trace this consumer.

Public source admission, checked seeds, file_info/limits and with_fill replace
private file/generator use and Qt count subtraction. Original names/bytes/hash,
empty/unnamed and zero/equal/partial fill behavior, independent/dormant seeds,
draft/accepted/readiness semantics and Options/export JSON stay compatible.
The native chooser uses the authoritative public byte cap. The worker compares
bounded public source metadata captured before ownership transfer; it does not
serialize private source configurations or reconstruct payloads. The session's
source snapshot holds SourceInput; every other session byte/owner stays exact.

Level 1 passes six Qt Rust cases: both original source cases, a new independent
saved-section/buffer-sharing regression, seed/capture mismatch admission,
actual unnamed-file recovery/dormant settings and source/project corruption/
round-trip. One affected source-options QML behavior passes (three with harness).
Because source export uses a different getter, affected saved-project coverage
also passes two existing Rust and two installed native named/unnamed/file/fill/
recovery cases. The exact accepted prerequisite wheel is verified and reused;
no new wheel is built. Qt format/all-target Clippy, actual boundary, optimized
build and document/traceability checks pass.

The initial Qt compile caught an unavailable FullLoopRequest.inputs getter.
Metadata is now read from the public admitted source before ownership transfer;
no private workaround or API addition is needed. Original failed logs and final
successful runs remain separate. Full CXX declarations/resources are unchanged;
no new native startup/chooser/platform execution is claimed. The source export
shape is independently pinned; no schema/default/embedded payload changes occur.

Two whole exceptions retire. A focused source-type/import-only session review
preserves every other byte and all nine remaining private sites. Other frozen
records/dependencies/history stay exact; 21 files/85 sites/three scientific
dependencies remain. Unaffected full workspace/Qt/Python/CLI/package suites and
unchanged validator suites were not run; evidence records why, separately from
pending parent Level 2, milestone/final Level 3 and numeric-audit gates.

## UIU-DS-01 raw source-catalog API accepted

[Exact contract/owners](design.md#uiu-ds-01-raw-source-catalog-admission-prerequisite),
[machine evidence](source-catalog-api-evidence.json) and
[executed API cases](ds-01-source-catalog-operation-test-results.json) establish
focused Level 1 prerequisite acceptance for REQ-UIU-02/03/08/09/20/24/27.
ProfileCapture.frame exposes only existing original source data; native Python
returns an independent dictionary. Measured sequence/response and Designer
source-catalog variants reuse existing planners, evaluators and bounds. Raw byte
IDs remain independent of the unchanged 0..15 simulation ProfileId domain.

Executed: 25 Rust cases (source catalog, sequence, Designer, facade, underlying
playback and existing Designer workbench), 21 exact-archive installed native cases
(complete affected source/sequence/Designer modules plus namespace compatibility),
three targeted Qt Rust consumers, scoped root/native Clippy, formatting,
analysis-only build, CLI compile and fresh complete API inventories. Original
assertions, source order, roles/nulls, bytes/hash/epochs and existing operation
signatures remain. Literal residual powers 1 and 0.1 give 0.55, distinct from a
dB average. Borrowed-frame pointer equality and owned native-copy mutation/lifetime
checks establish the declared ownership. Same-input legacy methods return the
same designs, results and errors. Original algorithms and numerical vectors are
unchanged.

The matrix grows 576/550 to 585 Rust/556 native operations, retaining every prior
row/type/execution record and all native members after removing only the six
added methods. One Rust source-data alias is added; the existing 80 root and
112 directional/common class exports remain. Fresh module bytes match the one
wheel archive before outside-checkout native testing. The earlier source-file
wheel is historical evidence, not used for the new methods.

Not required: unrelated workspace/upstream/FEC suites, QML/native desktop startup,
project persistence and packaging, because no algorithm, QML/CXX registration,
project format, lifecycle, upstream or package owner changes. Qt's existing
profile, measurement restoration and Designer envelope consumers are exercised;
the CLI compile confirms its existing public contracts. No new scientific code
is added to Qt. Parent Level 2, milestone/final Level 3, full numeric audit and
Qt input adoption remain incomplete. The boundary remains 21 frozen files,
85 sites and three scientific dependencies. No official checkbox is advanced.

## UIU-DS-01 profile input owner facade accepted

[Exact contract](design.md#uiu-ds-01-profile-input-owner-facade) and
[machine evidence](profile-owner-facade-evidence.json) establish focused Level 1
consumer acceptance for REQ-UIU-02/03/08/09/20/22/24/27, consuming prerequisite
05d30422. Profile admission, decoder dispatch, assignment interpretation and OCD
geometry checks now use the public facade. Immutable catalog and original-byte
storage are shared by draft/accepted snapshots; plotting borrows that retained
catalog. Existing independently owned session DTOs are copied only on publication.
Generic read-only JSON presentation retains original MAC case and UTF-8 transport
handling, without another scientific import or modification of original bytes.

Executed: eight targeted Qt Rust cases (all original profile/plot cases, a new
shared-storage/raw-ID/text/tree/saved-profile case and two existing project
regressions); three QML behaviors (five harness cases); Qt formatting, all-target
Clippy and optimized build. Persistence risk expansion passes two Rust and two
native saved-file/fill/profile-name compatibility cases, with the exact accepted
05d30422 wheel's installed module hash verified before reuse. Documentation,
traceability, source/dependency boundary and generated-guide checks apply.

Sorted UI source IDs and original capture order remain distinct. The new test
pins uppercase original MAC text, initial BOM/whitespace, exact hashes/bytes,
4096-QAM source rows, tree depths, catalog/original buffer pointer sharing,
independent published DTOs and saved raw-ID restoration. Existing supported input,
profile/readiness/plot/project assertions are preserved through public accessors.
The accepted public contract still requires named profile captures and permits
unnamed payload files. The former passive Qt decoder could stage an empty capture
name that public run/project admission rejects; the profile owner now enforces
that established capture-name contract and leaves the previous draft intact.
The obsolete spacing-error suffix claiming channel editing was pending is replaced
by the current public diagnostic; OCD editing is already implemented.

The initial adapter compile caught a typed MAC array being used as a display
string. The final adapter retains original validated JSON presentation text,
which also preserves case, rather than inventing a normalized display identity.
No assertion was removed to resolve this failure; the new preservation case passes.
Two complete legacy entries retire: 21 files/85 sites become 19/79; three direct
scientific dependencies remain. CXX declarations and project schema are unchanged.

Not required: a fresh wheel, backend/API inventory refresh, unrelated full
workspace/upstream/FEC suites, native desktop startup or packaging: backend and
bindings, exports, CXX/QML registration, lifecycle and package owners are unchanged.
Current Rust/native project cases execute against the unchanged accepted artifact;
no historical run is reported as a new pass. Parent Level 2, milestone/final Level 3,
direction-aware Save/Open and final numeric audit remain pending.

## UIU-DS-01 original measurement-frame API accepted

[Exact contract](design.md#uiu-ds-01-original-measurement-frame-prerequisite),
[machine evidence](measurement-frame-api-evidence.json) and
[executed API cases](ds-01-measurement-frame-operation-test-results.json) establish
Level 1 paired prerequisite acceptance for REQ-UIU-02/03/08/09/20/24/27.
ChannelEstimate.frame borrows the existing admitted key/epoch/decoded-source/I/Q
frame. Native Python returns an independent bounded dictionary. The channel and
RxMER frame aliases preserve exact nominal data types and the existing RxMerFrame
structural validator. No scientific engine, calculation, original observation,
constructor signature, resource limit or saved format changes.

Executed: 13 Rust cases (complete original-measurement and measured-sequence
modules, one binary lineage case and facade identity), eight exact-archive
installed native cases (complete original-measurement and namespace modules),
two targeted Qt measurement consumers, root/native formatting and scoped Clippy,
analysis-only build, CLI compile and fresh exact Rust/native inventories.
Independent [3,4] I/Q and null anchors, borrowed-pointer identity, independent
copied buffers/dictionaries and retained-result lifetimes establish the declared
scope. Retained binary provenance independently pins device/channel, 7,480 samples
and original epoch; decoded-frame hashes remain distinct from original-byte hashes.
Existing RxMER views, sequence behavior and every original assertion remain.

The matrix grows 585/556 to 587 Rust/557 native rows, preserving every earlier
row/type/execution record and all old native members after removing only the
one new channel method. Two Rust DTO exports are added; existing 80 root and
112 directional/common native classes remain exact. A fresh installed native
module is archive-hash matched before outside-checkout testing. Initial test
setup corrections (a nonexistent channel DTO validate helper and an outdated
binary fixture path) are retained separately; no original assertion or scientific
implementation was removed or weakened. Native conversion was added after all
Rust frame acceptance cases passed.

An initial compiler-inventory attempt found missing source links in a cached
facade Project re-export. Regenerating only facade rustdoc restored canonical
source links and exact export discovery; no cache clean, tool relaxation or
historical-row rewrite was used. The earlier failed invocation using an unsupported
cargo-rustdoc flag remains separate from the successful final command.

Not required: unrelated complete workspace/upstream/FEC suites, QML/native startup,
project/CLI runtime or packaging: no scientific behavior, UI/CXX/QML registration,
project format, lifecycle, CLI handler or package owner changes. The CLI compiles
and two affected existing Qt consumers execute. Complete parent Level 2 and
milestone/final Level 3 gates, measured-input Qt adoption and final numeric audit
remain open. The boundary is unchanged at 19 frozen files/79 sites/three scientific
dependencies; no official checkbox is advanced.

## UIU-DS-01 ordinary measurement admission facade accepted

[Exact contract](design.md#uiu-ds-01-ordinary-measurement-admission-facade) and
[machine evidence](measurement-owner-facade-evidence.json) establish focused Level 1
ordinary-input acceptance for REQ-UIU-02/03/08/09/20/22/24/27, consuming 1820cdff.
Ordinary binary/JSON RxMER and H admission now uses public CaptureLoader and
immutable RxMer/ChannelEstimate frame observations. Decoder dispatch, scientific
JSON import, original hashing and private scientific DTO imports leave this path.
Only independent compatibility copies receive the same legacy original-file
source-kind/hash/length projection; public input objects and decoded provenance
remain unchanged. The old Designer detection/dispatch fragment stays byte-exact
and its coarse format import remains frozen; actual Designer science and measured
planning remain separate migration owners.

Executed: all 13 Qt measured-input module tests, including all original model,
animation, PLC and six Designer cases with actual FEC; two existing Qt project
regressions; three affected QML behaviors (five harness cases); Qt formatting,
all-target Clippy and optimized build. Persistence risk expansion executes two
Rust and two native saved-input cases against the exact unchanged 1820cdff wheel,
with installed module/archive hash preflight. Actual documentation, traceability,
boundary and generated-guide checks apply.

The new independent capture regression pins binary original/decoded hashes,
source kind/byte length, original channel/epoch and exact I/Q equality with the
public observation. BOM-prefixed JSON pins the legacy decoded hash after BOM
removal, separate original hash, unchanged original bytes, explicit null and
42.75-dB anchor. Existing file fields, accepted/draft selection, source clocks,
Designer recipes, exported originals, grids and lifecycle remain. No project
schema, CXX declaration, backend/binding or public API operation changes.
The existing 64-MiB editor aggregate cap is named centrally and remains distinct
from the public sequence's 96-MiB budget. One bounded temporary byte copy bridges
the unchanged legacy editor storage; retaining public handles belongs to remaining
measured-model consolidation, without adding global caches.

A focused whole-file boundary review replaces only the ordinary import/decoder
sites and proves the preserved Designer fragment. The entry remains frozen with
one coarse site: 19 files/79 sites become 19/77, with all other entries/history and
three dependencies exact. No complete-file retirement or parent checkbox advance
is claimed for this partial owner.

Not required: a new wheel, API inventory refresh or unrelated complete scientific/
upstream/workspace/CLI/package suites, because backend/bindings/exports and package
owners are unchanged. No new native startup/installed-platform execution is claimed:
CXX/QML registration/resources and lifecycle are unchanged; affected QML and the
current optimized build pass. Parent Level 2, milestone/final Level 3, complete
Designer/model adoption, direction-aware persistence and final numeric audit stay
open. Existing tests/vectors and earlier evidence remain intact.

## UIU-DS-01 profile-performance observation API accepted

[Exact contract](design.md#uiu-ds-01-profile-performance-observation-prerequisite),
[machine evidence](profile-performance-api-evidence.json) and
[executed API cases](ds-01-profile-performance-operation-test-results.json) establish
Level 1 paired prerequisite acceptance for REQ-UIU-02/03/08/19/20/24/27.
FullLoopResult.profile_performance returns one point per original source carrier
with exact physical frequency/bin/role, existing typed CarrierMer powers/count/MER,
independent ideal Shannon capacity, observation-window FEC/BER and whole-run
unfinished payloads. Existing FecReport/CarrierMer projection helpers are shared
with classic PerformanceReport; original public schemas/fields stay unchanged.
No source, RNG, receiver or decoder advances, and no scientific algorithm changes.

Executed: eight Rust cases (two independent report projection cases, actual full-
grid and pre-cancelled observations, three classic direct-engine receiver-mode
comparisons and facade nominal compatibility); ten exact-archive installed native
cases (new observation/ownership/direct-Rust-project parity, existing full-loop,
classic completed/partial reports and namespace imports); one actual Qt worker
consumer; scoped root/native Clippy and formatting, analysis-only build, CLI compile
and complete fresh inventories. A small independent Rust public-adapter process
runs the same existing self-contained project as native Python; every reported
field/point matches. It reuses existing dependencies and does not change manifests
or locks. The new private projection tests explicitly discover two cases with the
high-level API feature; the initial zero-discovery command is retained as an
unexecuted coverage attempt, not a pass.

Independent anchors include unit reference plus 0.1 error giving 20 dB, zero-error
versus no-sample states, a three-outcome decoder/source partition and eight-bit
ideal Shannon capacity 24.06540180433955 dB. Actual 7,480-carrier source scope,
640-MHz first center/physical bin 356, complete power/sample mapping, optional BER,
window FEC and whole-run unfinished counts remain distinct. Native copies cannot
mutate a result and owned views survive result release. Diagnostic TX reference
comparisons never enter normal receiver processing; ideal capacity is not a
receiver operating requirement, measured RxMER or coded-throughput prediction.

The matrix grows 587/557 to 588 Rust/558 native rows and preserves every earlier
row/type/execution record and native member after removing only the new method.
Two typed Rust models are added; 80 root/112 directional-common native class
exports remain. Fresh native module bytes match the sole wheel archive before
outside-checkout testing. Canonical facade rustdoc is regenerated explicitly to
preserve source links; no validator relaxation or cache clean occurs.

Not required: unrelated complete workspace/upstream/codec suites, new QML/native
startup or optimized UI build, project/packaging regressions: no algorithm,
UI/CXX/QML source/registration, saved schema, lifecycle, dependency or package
owner changes. Shared report risk expands to all three classic Rust modes, current
native completed/partial legacy reports, one actual Qt consumer and CLI compile.
No unexecuted suite or installed-platform behavior is claimed. Parent Level 2,
milestone/final Level 3, Qt profile viewer adoption, bounded receiver-history
migration and final numeric audit remain pending. The source/dependency boundary
is unchanged at 19 files/77 sites/three dependencies. No official checkbox advances.

## UIU-DS-01 profile-performance viewer facade accepted

[Design](design.md#uiu-ds-01-profile-performance-viewer-adoption) and
[machine evidence](performance-viewer-facade-evidence.json) record Level 1 consumer
acceptance for REQ-UIU-02/03/08/19/20/24/27 after the paired API prerequisite.
The viewer reads FullLoopResult.profile_performance and public FecReport getters.
Existing curve/counter JSON, observed/ideal series order and identifiers, original
source frequencies/roles, PLC bands, optional metrics, profile selection and
partial statuses remain. Window FEC/BER and whole-run unfinished totals retain
their distinct scopes. Qt performs only presentation grouping/axis/label work.
Independent QAM history remains unchanged; this unit does not claim its migration.

Executed: three Qt Rust cases (actual completed worker with public-observation
scalar/zoom comparisons, independent four-profile sweep and original PLC clipping
vectors), two affected QML behavior cases (four including harness), Qt formatting,
all-target Clippy and optimized release build, plus boundary/document/traceability/
guide/diff checks. All existing worker assertions remain exact. Independent display
policies are named centrally; existing same-purpose frequency ticks/denominator
and SI units are reused. No domain limit or scientific algorithm changes.

The complete performance-viewer boundary entry retires: 19 files/77 sites becomes
18 files/75 sites, with all three dependencies and every other frozen entry exact.
No backend/binding/API, CXX/QML registration, saved schema or lifecycle changes;
the accepted exact-wheel API evidence remains applicable without rebuilding it.
Unrelated workspace/native/CLI suites, startup, project and packaging checks are
not required for this presentation-only consumer change. No unexecuted suite is
claimed passed. Remaining downstream consumers, parent Level 2, final Level 3
and the final numeric audit stay open; no official checkbox changes.

## UIU-DS-01 channel and receiver observations facade accepted

[Design](design.md#uiu-ds-01-channel-and-receiver-spectrum-adoption) and
[machine evidence](channel-observations-facade-evidence.json) record Level 1
acceptance for REQ-UIU-02/03/08/09/16/17/18/24/27. Existing public channel/calibrated
spectrum, amplifier_bins, physical_bins, response and physical-layout operations
replace private Qt numerical calls. Qt no longer calculates these PSD, electrical
integration, amplifier gain, logarithmic bin power, H or impulse observations.
The shared renderers format authoritative values, source coordinates and PLC
bounds. Raw FFT power remains separate from PSD; amplifier-only gain remains
separate from CPA/H/noise; configured H never substitutes for unavailable H-hat.

Executed: five Qt Rust cases (actual full-loop/tap/amp-QAM compatibility, actual
MCE/configured/estimated response, existing independent TX sampling-budget test,
original two-spacing/two-position PLC clipping vectors through public admission,
and physical-bin selector boundaries), five affected QML behaviors (seven with
harness), Qt formatting, all-target Clippy and optimized release build, plus
boundary/document/traceability/guide/diff audits. Actual raw FFT single-bin values,
coordinates/readouts and unavailable gaps match public observations; time-stage
full-band power remains unchanged on zoom. All original worker assertions remain;
PLC fixture preparation changes admission owner while preserving its independent
source geometry, pilots, PLC endpoints and every clipping assertion.

Four complete frozen files retire and the private response call leaves diagnostics:
18 files/75 sites becomes 14 files/66 sites. The exact remaining slicer/FEC tail is
preserved. The 54 measurement/Designer/channel-configuration/session and 12 receiver
QAM/soft-history sites, followed by three dependency removals, bound the remaining
UIU-DS-01 work. No new checklist task, API, binding, schema or lifecycle contract.
Existing paired API evidence remains applicable; no fresh wheel or unrelated
workspace/native/CLI/project/packaging suite is required. No installed-platform
or unexecuted-suite pass is claimed. Parent Level 2, downstream milestone Level 3,
later tasks and final numeric/shared-constant acceptance remain pending.

## UIU-DS-01 measured-input prerequisite APIs accepted

[Design](design.md#uiu-ds-01-measured-input-prerequisite-gaps),
[machine evidence](measured-input-api-evidence.json) and
[exact operation cases](ds-01-measured-input-api-operation-test-results.json)
record Level 1 acceptance for REQ-UIU-02/03/08/09/21/24/27. Two demonstrated Qt
consumer gaps receive paired public operations before Qt adoption: a synthetic
unity-response preview on the exact original source grid, and a report of existing
Designer scientific/resource limits. Synthetic unity remains separate from a
measured response and contains no invented capture provenance. The existing
four-million-value saved-Designer guard acquires a purpose-specific common name
without changing its value or admission behavior.

Executed: 29 Rust cases across complete affected source-catalog, captured-response,
native Designer/workbench, independent scientific Designer and facade-import
coverage; 23 exact installed-wheel native cases, including all four new cases,
old affected contracts and genuine namespace imports; one real Qt Designer/FEC
consumer; and three native-runner contract cases. Independent literal unity
projections, raw source ID 200, original coordinates, viewport/missing/error and
ownership behavior pass. Exactly four million Designer values admit; 4,000,256
reject through the original guard. Every unity-report point/field and complete
limits report equal the separate public Rust reference process. No new native
class is introduced. Existing source documents, domain values, public signatures,
historical matrix rows and tests remain intact.

Formatting, scoped core/native all-target Clippy, CLI compilation, analysis-only
build, fresh wheel, Rustdoc/native facade export inventories, strict executed-case
matrix, boundary, documentation, traceability, versions, generated guide and diff
checks pass. The native acceptance runner now supplies all five existing optional
downstream Rust reference processes, including these new cases; its strict
no-skips/completed-case rules remain. The full native gate itself is deferred,
not reported passed. Current operation inventory is 591 Rust / 560 native.

No Qt/CXX/QML source or registration, lifecycle, saved schema, manifest/lock or
scientific algorithm changes. The latest optimized UI executable remains applicable
to unchanged Qt sources; a new QML/startup/release build and unrelated full
workspace/upstream/codec/CLI execution/project/packaging regressions are not
required for this focused prerequisite. Qt measured-input/Designer/channel/session
adoption is still pending. The frozen boundary remains 14 files/66 sites/three
dependencies. No official checkbox changes; UIU-DS-01 Level 2, downstream milestone
Level 3 and final acceptance/numeric audit remain open.

## UIU-DS-01 measured input and channel facade accepted

[Design](design.md#uiu-ds-01-measured-input-and-configured-channel-adoption) and
[machine evidence](measured-owners-facade-evidence.json) record the migrated
measurement model, bridge implementations, configured-channel owner, passive input
storage and PLC presentation for REQ-UIU-02/03/08/09/16/17/18/21/22/24/27.
Existing public catalog/sequence, original response, synthetic-unity and channel
operations replace private measurement projections, residual-power conversion,
physical channel planning and gain calculations. No backend API or algorithm is
added. Original source catalogs accompany selected frame DTOs; passive saved-state
publication remains separate from scientific admission, including incomplete or
mismatched idle projects. Captured H and its independent association tolerance,
original bytes/hashes, raw IDs, selected order, legacy labels and PLC overlays remain.

Executed: all 103 Qt Rust cases, including the original actual worker/FEC,
multi-profile sweep, independent response-tolerance, stale cancellation and saved
project/corruption assertions. Two added cases exercise passive mismatched/empty
publication and raw ID 200 with independent literal unity values and all four old
measurement projection labels. A subsequent test-initializer-only Clippy correction
reruns its one affected case; production behavior is unchanged. Fifteen affected
installed-native sequence/channel/saved-full-loop cases reuse the exact accepted
wheel with a verified extension hash, without another wheel build. Seven affected
QML behaviors pass (nine cases including setup/cleanup): measurement/channel
readiness, layout, selected originals, capture animation, loss/power controls and
project draft restoration.

Formatting, warnings-denied Qt all-target Clippy, optimized UI build, boundary,
documentation, traceability, versions, generated-guide and diff checks pass.
The three touched CXX declaration blocks, QML resources, manifests/locks, saved
schema and independent scientific vectors remain exact. The combined-channel
chart now consumes the public bounded gain view; scientific values/gaps/units
remain authoritative, with backend viewport padding/readouts and the existing
receiver-curve JSON tick keys adapted for presentation. Additional/effective
endpoint chart data comes directly from the public configured-channel report.

Seven complete frozen files and 35 sites retire: 14 files/66 sites becomes
**seven files/31 sites**, with all three direct dependencies still present.
Designer production is byte-exact; only its existing public-playback consumer test
changes accessor/arguments while keeping the independent residual-power oracle.
The original measurement decoder/metadata file is byte-exact after removing two
added public-handle adapters; its one private Designer transport import remains
explicitly frozen. All other remaining entries and initial history stay intact.
The finite frontier is now Designer/measurement transport/export (19 sites) and
receiver QAM/soft history (12 sites), followed by dependency removal.

Two initial compilation failures and one test-style Clippy failure are retained
as failed attempts, followed by successful corrected runs; none counts as a pass.
No scientific/API/binding/schema/registration or package change requires unrelated
workspace, new wheel, CLI execution or packaging suites for this Qt adapter batch.
Full native parity, downstream milestone Level 3, parent UIU-DS-01 Level 2, final
numeric/shared-constant audit and integrated session/upstream work remain pending.
No official checklist checkbox or installed-platform acceptance changes.

## UIU-DS-01 Designer and export facade accepted

[Design](design.md#uiu-ds-01-designer-and-export-owner-adoption),
[machine evidence](designer-export-facade-evidence.json),
[public boundary](../../../architecture.md#unified-qt-public-api-boundary) and
[official parent](../../checklist.md#task-uiu-ds-01) trace REQ-UIU-02/03/08/09/21/22/24/27.
Immutable public RxMerDesign operations replace Qt recipe evaluation, flat-mask
construction, private source validation and measurement projection. Public native
export replaces private export imports. Original envelope/control transport,
labels, epochs, hashes, BOM normalization, PLC overlays, range/zoom interaction,
common-axis drawing and atomic cancellation/adoption remain compatible.
QML reads distinct scientific domains from the public limits report; absent
reports disable scientific actions. Existing independent scope, residual-power,
actual FEC and export-range assertions remain, with an additional repeated-edit
oracle: 48 dB → 42 → 45 → 48 from an unchanged flat baseline.

Executed: 32 distinct Qt Rust cases (16 complete measured-input cases, two
Designer adoption guards, eleven affected worker cases and three project cases),
29 shared Rust Designer/export/workbench contract cases, 15 QML behaviors (17 with
harness), and 14 exact-wheel native Designer/export/legacy saved-project cases.
The QML cases vary the supplied limits independently and check unavailable
reports, mouse/keyboard stepping, scopes, zoom, Save and Cancel. The final QML run
has no warnings; an earlier run exposed a transient undefined count binding,
corrected by reading the typed count directly. Separate fresh Rust/native process
comparison is not claimed for this Qt-only unit; native tests reuse the verified
unchanged installed wheel and their independent existing anchors.

Qt formatting/all-target Clippy, the rebuilt optimized executable, native
Designer/legacy project admission and embedded-resource startup/Open pass. Final
boundary, documentation/traceability, version/guide and whitespace checks are
recorded in the machine evidence.
Four complete frozen owners/19 sites retire: seven files/31 sites becomes
**three receiver files/12 sites**, with three scientific dependencies still
present. Earlier boundary entries, retirements/reviews and every official
checkbox remain unchanged. No public API, numerical algorithm, domain value,
saved schema, binding, manifest/lock or global registration changes. No wheel
rebuild or unrelated complete workspace/CLI/package regression is required.
Bounded receiver archives, matcher-invisible QAM/soft calculations, dependency
removal, UIU-DS-01 Level 2, downstream milestone Level 3 and final numeric/common-
constant acceptance remain open. Prepared agent branches establish no integrated
session/upstream acceptance.

## UIU-DS-01 bounded receiver archive API accepted

[Design](design.md#uiu-ds-01-bounded-receiver-archive-prerequisite),
[machine evidence](receiver-archive-api-evidence.json),
[exact executed cases](ds-01-receiver-archive-operation-test-results.json),
[public boundary](../../../architecture.md#unified-qt-public-api-boundary) and
[official parent](../../checklist.md#task-uiu-ds-01) trace REQ-UIU-02/07/09/16/17/18/19/20/24/27.
The concrete existing history consumer needs detached data: FullLoopResult clones
retain entire jobs, inputs and stages. The new immutable Rust/native archive owns
only bounded actual role cells and the first nonempty decoder event's finite
prefix, with original coordinates, coding context, offset and total. Existing live
QAM/MER/decision/soft implementations are shared; no numerical algorithm changes
or reconstructed receiver input. Query/history/decoder/byte policies remain
separate and named in the downstream common constants owner.

Nineteen distinct Rust cases pass:14 full-loop library cases, three affected
public scientific observation cases and two facade/legacy type-identity cases.
Sixteen cases pass against the exact final installed wheel: four archive, four
existing cell, three slicer/soft, four namespace/typing/control and one legacy
real-FEC result case. A freshly executed separate Rust process matches every
archive info/QAM/decision/cell-soft/decoder-soft field and existing live views.
Independent vectors preserve 10003→8192 with last index10001; all-selected MER
23.010299956639813/20 dB beyond2048 display rows; six distinct role/amplitude
contracts; strict soft histogram endpoints; decoder prefix/context; zero/missing
states; byte arithmetic rejection; and weak-reference release of the actual job,
stage store and original profile bytes while archive clones remain usable.

The installed-wheel namespace check caught missing new aliases in the legacy
merlin.merlin path. The compatibility-only repack verified an unchanged native
payload; the subsequent final named-limit candidate reruns all16 native cases.
Earlier compile/Clippy/fixture-path attempts and the premature local reference
preflight remain explicit failed/nonacceptance history. Source-only/default-
feature compilation is not used as high-level API acceptance.

Core/native Clippy, formatting, CLI and analysis-only compilation, Rust/facade
documentation and actual export inventories pass. Final strict operation and
metadata checks are retained in machine evidence. Nine Rust and ten native rows
append (600/570 total); all previous rows, classes/signatures, execution history,
requirements and official checkbox states remain intact. No Qt/CXX/QML source,
manifest/lock, saved schema, scientific vector or decoder changes. The Qt executable
remains current for its unchanged UI sources; no Qt adoption, new Qt runtime test,
complete workspace/native/CLI/package suite or hosted-platform pass is claimed.
Three frozen Qt files/12 sites/three direct scientific dependencies remain.
Receiver adoption, dependency removal, DS Level2/downstream Level3, prepared-agent
integration and final numeric/common-constant acceptance remain pending.

## UIU-DS-01 receiver public API and dependency exit

[Design](design.md#uiu-ds-01-receiver-consumer-and-dependency-exit),
[machine evidence](receiver-consumer-evidence.json),
[architecture](../../../architecture.md#unified-qt-source-ownership) and
[UIU-DS-01](../../checklist.md#task-uiu-ds-01) own the final downstream consumer
batch. REQ-UIU-02/03/07/08/09/16/17/18/19/20/24/27 retain their existing contracts.

Qt now queries live/detached public QAM, physical decisions and cell/decoder soft
views. It preserves all-selected metrics independently of bounded display pages,
exact requested-run isolation, received reference planes and missing decoder
coverage. Detached history preserves finite counts and releases full job/input/
stage owners. Archive failure preserves completed FEC/performance. Original
scientific oracles remain in their backend owners; Qt retains independent actual
worker/noise assertions and verifies selector/formatting/ownership compatibility.

The final boundary guard passes with **zero direct scientific dependencies and
zero frozen legacy files/sites**. Original frozen inventory, relocation/review and
retirement history remain preserved. The manifest/lock remove only direct Qt
scientific dependency entries; existing transitive source pins remain unchanged.

Complete Qt Rust acceptance passes **105 cases**, and all integrated QML checks
pass **91 cases**, with no failures or skips. Existing focus-state QML warnings
are recorded separately from test outcomes. The first locked invocation stopped
before test discovery because the manifest-only removal had not updated the Qt
package's lock dependency list; the corrected locked full run passes.

Complete Rust milestone execution passes 784 workspace cases, four fixture
generator cases, 14 API example processes, 139 analysis-only cases and doctest
phases. Three existing opt-in performance benchmarks are ignored, not passed.
Workspace and Qt warnings-denied Clippy, formatting, 20 boundary cases, 25 package
regression cases, optimized Qt build and six native XCB launch/Open probes pass.
Analysis/example-only builds retain unused-helper warnings for
`minimum_observation_milliseconds_after` and `PayloadRecovery::from_counters`;
no warning suppression or production-source change is made by this consumer unit.

Complete installed-native acceptance passes **179 cases**, without failures or
skips, against fresh separate Rust reference processes. The exact accepted
receiver-archive wheel is reused: production Rust/API/binding/package bytes are
unchanged; only backend test assertions change. Native imports and exported
operations remain compatible. No wheel rebuild is necessary for Qt adoption.

This accepts UIU-DS-01 Level 2 and the downstream Level 3 milestone after final
metadata reconciliation. The rebuilt optimized executable and six native probes
use the same recorded SHA. No schema, scientific algorithm, resource registration
or user-facing layout changes. New screenshots and native installer execution
are not required by this consumer change; package contract regressions pass.
Prepared session/shell/upstream work is not yet integrated or accepted. Continue
in dependency order with UIU-SESSION-01; unified runtime/final numeric acceptance
and Windows/hosted/installed lifecycle evidence remain separate.

## UIU-SESSION-01 explicit ownership foundation

[Design](design.md#uiu-session-01-explicit-ownership-foundation),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-foundation-evidence.json) and
[UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/06/07.
The prepared agent's six-file unit is reviewed and adopted after the completed
downstream gate, with one test-placement correction described below. No other worktree writes or
commits in the integration checkout.

All **16 pure application tests** pass in this checkout with the pinned compiler.
Standalone Clippy with all normal lints and warnings denied, formatting and the
final zero-scientific-dependency boundary pass. Tests cover checked identity
exhaustion, direction/generation/run rejection, paused/cancelling Busy ownership,
partial/failed/foreign terminal cleanup, destroyed owners, queued/old payload
reservations and dirty Open/Save guards. Fixed-size fixture counters/bytes are
independent application anchors; no scientific vectors or runtime limits change.

This is Level 1 primitive acceptance. No Cargo manifest, compiled Qt consumer,
CXX-Qt/QML registration, scientific/API/binding/project/schema source changes.
The current optimized downstream executable remains unchanged. Another full Qt,
workspace, native wheel/CLI/package run is not required for this unregistered
std-only unit; those milestone results remain their recorded candidate evidence.
No UI screenshot or user-control guide change is applicable.

The actual downstream input/latest-result stores and worker callbacks still need
migration. Real direction controls/results, reachable-allocation accounting,
terminal delivery including panic/spawn failure, native Qt integration and Level 2
session acceptance remain pending. UIU-SESSION-01, shell and later tasks stay
unchecked; source-only agent/component evidence cannot accept them.

The first final-boundary run rejected the prepared external Rust test-module
path. Move its exact body to `src/shell/tests.rs` and use ordinary `mod tests;`;
no guard exception or numerical/behavior assertion changes. Final owner checks
rerun on that adapted placement, preserving the rejected invocation as evidence.

## UIU-SESSION-01 explicit downstream context migration

[Design](design.md#uiu-session-01-explicit-downstream-context-migration),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-context-evidence.json) and
[UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/06/07.
All six former input/section/latest-result/QAM/performance/prepared-model data
singletons now belong to an explicit downstream context. Only construction uses
a fixed downstream bootstrap handle; data operations never consult selected tabs.
Prepared models use an independent candidate owner and rebind after target
transfer, with a destruction test guarding against retained-owner cycles.

Four new owner tests pass: distinct accepted sources/sections, real public-run
result/archive isolation, targeted prepared adoption/destruction and thread-safe
owned transfers. All **91 affected downstream/bridge Rust cases** pass, including
those four; 18 unchanged platform cases are filtered, not reported passed.
All **91 integrated QML checks** pass without failures or skips. Existing
scientific/worker assertions remain exact after removing only explicit context
bindings/arguments, formatting and optional commas. No numerical oracle or
independent reference input is weakened. Warnings-denied Qt Clippy, all-target
compile, formatting and the final zero-dependency boundary pass.

A read-only QA review finds no introduced ownership, recursive-lock, cross-owner
reload or result/history fallback defects. This is source review, not additional
runtime evidence. Early mechanical migration compile failures are preserved in
the private receipts and corrected before the passing runs; no rejected build is
reported as test execution. The final optimized build and six native XCB launch/Open probes pass on the
recorded executable SHA; exact source hashes are recorded in the machine evidence.

This focused Level 1 change expands to every affected downstream adapter and
QML consumer. Scientific/public API/binding/CLI/package sources, Cargo manifests,
CXX-Qt registration, schemas and numerical fixtures are unchanged. Their completed
downstream milestone evidence remains historical; another full workspace,
wheel/installed-Python, CLI or packaging run is not required for this context-only
unit. QML source/layout is unchanged, so new visual screenshots are not required.
The Qt executable is rebuilt for the migrated consumers before handoff.

UIU-SESSION-01 stays unchecked. Actual coordinator identity/generation/run
callbacks, terminal acknowledgement on every worker path, both real direction
controls, aggregate reachable-allocation accounting and complete Level 2 session
acceptance remain pending. Existing global original-path no-clobber protection
remains infrastructure; owner-scoped path removal is still required when the two
real project owners are wired. Shell, upstream UI, persistence and final gates
retain their existing dependency order and status.

## UIU-SESSION-01 terminal worker transport

[Design](design.md#uiu-session-01-terminal-worker-transport),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-terminal-evidence.json) and
[UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/06/07.
The shared std-only transport carries genuine run tokens and holds the sole
lease outside scientific-operation unwinding. Known terminal envelopes release
admission after disposal by a destroyed GUI owner; dropping active work still
keeps Busy. Failed OS thread admission returns the exact unstarted lease to the
host. Existing direction-specific channel policies supply backpressure.

All **22 application owner tests**, including six new real-thread/transport
cases, pass without failures or skips. Warnings-denied Clippy, formatting and the
unchanged final zero-scientific-dependency boundary pass. Tests exercise success
in either direction, typed errors/panic, GUI destruction before operation return,
queued charged payload disposal, deterministic OS-spawn failure and invalid
thread names. The first Clippy pass rejected two test err().expect() expressions;
expect_err preserves their assertions, and all final checks pass.

This source-only Level 1 component is not yet registered or used by either
scientific worker. The existing Qt executable remains current; another Qt/QML,
wheel, core, CLI or package rebuild is not required for an unregistered std-only
transport. These passes do not establish real direction integration, aggregate
memory bounds or UIU-SESSION-01 Level 2. Actual adapters must preserve immutable
accepted owners, validate every token before adoption and supply independently
audited reservations for all reachable retained allocations. Logical public
population counts alone are not a byte bound. The parent remains unchecked.

## UIU-SESSION-01 accepted edits during cancellation

[Design](design.md#uiu-session-01-accepted-edits-during-cancellation),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-edit-cancel-evidence.json) and
[UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/06/07.
Two explicit transitions preserve valid accepted edits while cancelling the old
scientific generation. Candidate budget ownership and both checked revisions are
preflighted before any cancellation or mutation. The old worker remains Busy
until its terminal acknowledgement; its stale results cannot replace the edited
configuration. Existing reject-while-Busy operations retain their behavior.

All **27 application owner/transport tests**, including five new transition cases,
pass without failures or skips. Warnings-denied Clippy, formatting and the final
zero-dependency boundary pass. Tests cover both active directions, paused edits,
repeated edits/cancel-once, idle-other-direction isolation, rejected foreign
accounting/closing/exhaustion, stale report rejection and terminal release.

This is source-only Level 1 preparation for actual adapters. No registered Qt,
scientific/public API/binding/project/schema code changes, so no Qt/QML, wheel,
workspace/CLI/package rebuild or complete regression is required. The latest
accepted downstream executable remains current. Actual coordinator/worker
consumers and complete Level 2 session acceptance remain pending.

The [retention accounting comparison](retention-contract-decision.md) records a
material clarification for user decision. It remains proposed and changes no
accepted requirement, byte guarantee or task state. Controller/terminal work can
continue independently; aggregate accounting acceptance cannot precede that
decision and the demonstrated public metadata prerequisites.

## UIU-SESSION-01 actual upstream controller preparation

[Design](design.md#uiu-session-01-actual-upstream-controller-preparation),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-upstream-controller-evidence.json) and
[UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/06/07.
The lead reviews and adopts the isolated d1a1aa7 preparation: three byte-exact
source files and only the new controller handoff section. The prior primary
handoff adaptation remains intact; 93dd675 shell tabs remain unintegrated.
Later Workbench execution reuses this owner under UIU-US-02/REQ-UIU-12.

The controller owns an actual public upstream Simulation/document and real
complete/partial reports. Admission is off-GUI; execution moves the scientific
session outside the short lock and an unwind guard restores it. Shared terminal
transport carries the genuine host-issued token/lease. Public controls preserve
actual pause/progress/cancel behavior. Charged Arc owners follow queued/old report
aliases and destroyed controllers. Local immutable-key checks never substitute
for canonical host revalidation before UI adoption.

All **37 combined primary-checkout cases** pass: 27 application ownership/transport
cases plus ten real public-API controller cases. The unchanged 50-kHz fixture
checks complete recovery/original provenance, actual acknowledged pause/resume,
paused partial cancellation, failed admission and genuine public-control Busy
errors, stale generation/session/run events, queued/old charged aliases,
destruction and restoration after application unwind. No failures or skips.
Warnings-denied Clippy (including all normal lints), formatting and the unchanged
final zero-scientific-dependency boundary pass. The primary harness applies no
lint allowances; agent-only unregistered-harness allowances remain recorded as
historical preparation context.

Fixed accounting ticks test reservation lifetime only; they are explicitly not
backend allocation estimates. Public metadata/preallocation bounds and the
[pending retention decision](retention-contract-decision.md) remain unresolved.
No aggregate acceptance is inferred from these tests.

This is source-only Level 1 acceptance. No scientific/API/binding/manifest/
registration/schema/QML/fixture bytes change. Reuse the compiled facade artifact;
its exact hash and feature fingerprint are recorded. Another executable/wheel,
Qt/QML, full workspace, CLI or packaging build/regression is not required here.
The optimized executable remains the current accepted downstream artifact.
Canonical two-direction host wiring, actual downstream shared-worker migration,
Qt runtime/Level 2 and all later checklist gates remain pending; no task checkbox
changes. No push, merge, release or installation occurs.


## UIU-SESSION-01 approved multidimensional retention admission

[Design](design.md#uiu-session-01-approved-multidimensional-retention-admission),
[approved decision](retention-contract-decision.md),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-retention-dimensions-evidence.json) and
[UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/06/07.
The user selected bounded payload bytes plus independent ordered-entry and
shared-root counts on 2026-11-23. This supersedes the pending-decision wording
in prior unit receipts; their executed tests and historical byte statistics
remain unchanged. Bytes explicitly exclude allocator/tree/refcount bookkeeping
and do not promise total heap or RSS.

The shared application primitive preflights all checked additions and three
inclusive ceilings atomically before changing any counter. Reservations survive
queued, replaced and aliased ownership, releasing all quantities after payload
destruction. Existing byte-only operations preserve their tests; their constructor
admits zero entries/roots rather than unlimited populations.

All **45 combined primary-checkout cases** pass, including eight new accounting
cases and the existing ten real public upstream controller cases. Independent
exact/limit-plus-one and overflow anchors cover every dimension; failed combined
admission leaves all counters unchanged. Real threads test shared atomic admission,
queued/final aliases and destructor/unwind ordering. A nonzero three-dimension
lifecycle case verifies paused cancellation, rejected replacement and owner
destruction without premature charge or run-lease release. Production/test Clippy with
all warnings denied, format and the zero-scientific-dependency boundary pass;
requirement/link/history checks are recorded in the evidence. No lint allowances.

This is Level 1 primitive acceptance only. Synthetic charges do not estimate
scientific allocations. Actual public root metadata, pre-production bounds and
two-direction host adoption remain mandatory before aggregate acceptance. No
registered Qt, scientific/API/binding/schema/QML/manifest source changes, so a
Qt/QML/executable rebuild, wheel, workspace/CLI/package regression is not required
for this unit. Existing compiled artifacts are reused and the latest accepted
Qt executable stays current. UIU-SESSION-01 remains unchecked pending complete
Level 2 runtime/compatibility acceptance; no historical checkbox changes.


## UIU-SESSION-01 public retained-root metadata

[Design](design.md#uiu-session-01-public-retained-root-metadata-prerequisite),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-resource-metadata-evidence.json),
[executed operation cases](session-resource-metadata-operation-test-results.json)
and [UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/07.
The lead selectively adopts 665d758 and only its appended handoff; all 32 source/
test files initially match exactly. One new unused private-test import is removed
following the first Clippy rejection; final Clippy has no suppressions. Original
scientific statements, defaults, serializers and historical archive statistics
are unchanged. Read-only independent audits find no current-root undercount.

Upstream Simulation and BatchReport now expose typed passive retained_footprint
operations in Rust and installed native Python, using one common immutable model.
Private cached playback, original/canonical/provenance storage, spare capacities,
actual decoder decisions, skipped trace role gains and shared receiver/soft stores
are included. Struct/variant audits detect future fields needing review. Common
walkers use checked arithmetic and conservatively repeat shared pointees. These
quantities exclude private allocator/tree/refcount bookkeeping, not total heap/RSS.

Executed: **9 common, 137 upstream and 25 facade Rust cases**, plus **20 installed
native cases** without failures/skips. Twenty Rust metadata cases and five native
metadata cases are new. The existing opt-in transform benchmark remains ignored,
not passed. Native checks include original downstream/root aliases and detached
archives, upstream field/recovery compatibility, immutable/lifetime/failed-config
behavior, paused Busy/cancellation and eight independently executed Rust/native
50/25-kHz complete/pre-cancelled cases with estimate/soft retention on/off.
Warnings-denied owner/native Clippy, independent no-default upstream compilation,
formatting, fresh compiler/installed inventories and exact source/artifact hashes
pass. Two Rust and five native rows are additive to all previous 600/570 operation
rows. Boundary, requirement/link/history and strict operation checks are recorded
in the evidence.

This is Level 1 current-root metadata acceptance. No registered Qt consumer,
presentation, schema or scientific algorithm changes, so full unrelated workspace,
Qt/QML, CLI and packaging regressions/rebuilds are not required for this unit.
The existing accepted Qt executable remains current. Downstream root metadata,
pre-production result/copy bounds, independent input/presentation charges and
actual canonical host/worker wiring remain mandatory before aggregate acceptance.
The isolated downstream preparation uses a new branch while preserving 665d758;
its source-only work is not accepted here. UIU-SESSION-01 remains unchecked and
all later Level2/Level3 acceptance gates remain intact.

## UIU-SESSION-01 downstream retained-root metadata

[Design](design.md#downstream-retained-root-preparation),
[architecture](../../../architecture.md#unified-qt-session-and-file-lifecycle),
[exact evidence](session-downstream-metadata-evidence.json),
[executed cases](session-downstream-metadata-operation-test-results.json)
and [UIU-SESSION-01](../../checklist.md#task-uiu-session-01) trace REQ-UIU-03/07.
The primary adopts preparation 20f4342: 55 owner files and one new test file
initially match exactly; only the appended handoff is combined with primary
history. Three new visitor blocks move before existing test modules after Clippy
rejects their placement; no body or scientific statement changes. Independent
review finds no production omission/undercount. Initial failed attempts and their
corrections are retained explicitly in the evidence, not counted as passing gates.

FullLoopRequest, FullLoopSweepPlan, FullLoopResult and FullLoopReceiverArchive
expose passive metadata through existing Rust/facade/native classes. Original
inputs, cached playback powers, retained stages/context/LLRs, protected pilot
cache, spare capacities and repeated shared payloads are included. Historical
archive retained_bytes and copying remain unchanged. The Copy helper audits
fixed Duration/FecCounters leaves without a new codec dependency.

Executed: **103 Rust cases/doctests** (10 common, six new private owner,
61 complete relevant downstream API/channel/protected and 26 facade), and
**34 installed native cases**, with no failures or skips in the final gates.
Eleven metadata/facade Rust cases, a compile-fail doctest and three native cases
are new. Native full-loop/observation/saved-project/archive/namespace/upstream
metadata consumers pass. Eight separate Rust/native single/multiple-RxMER,
observed/unobserved, complete/pre-cancelled reference cases agree on all three
quantities, original provenance, actual outcomes and historical archive metadata.
Warnings-denied owner/native Clippy, independent no-default downstream build,
formatting and fresh compiler/installed inventories pass. All previous 602/575
rows are preserved; the resulting operation baseline is 606 Rust/579 native.

This is Level 1 current-root metadata acceptance. Unchanged Qt presentation,
registration and consumers require no Qt/QML/executable rebuild here; unrelated
workspace/analysis/CLI/packaging regressions remain at their applicable gates.
The accepted executable is still current. Pre-production bounds, the
[construction boundary](retention-contract-decision.md#output-construction-boundary-requiring-clarification),
independent input/presentation charges and real host wiring remain open.
UIU-SESSION-01 stays unchecked; historical acceptance states remain unchanged.


## UIU-SESSION-01 independent input and preview metadata

[Owner design](design.md#independent-input-and-idle-preview-owner-closure),
[exact source/command receipt](session-input-metadata-evidence.json) and
[executed API cases](session-input-metadata-operation-test-results.json) trace
REQ-UIU-03/07. Adopted DS preparation `91d1e9b` and Designer preparation `c9ae6662`
are independently source-reviewed, then validated together in the primary checkout.
All original scientific statements, source bytes, archive statistics, constructors
and public behavior are preserved. The primary removes one unused import and
names distinct fixture identities/positions; no warning suppression is added.

Executed: **84 unique Rust cases** (nine private DS, 29 complete relevant DS API,
seven private Designer, 12 relevant PNM API and 27 facade) and **47 installed
native cases**, with no failures or skips in final counted results. Nineteen Rust
owner/facade and three native cases are new. Eight independent native owner cases
match a separate Rust process on all three quantities, original byte/provenance
and actual preview data. Rust-only NamedCapture/CaptureProvenance roots have no
separate Python class. The exact locally built wheel is installed only in the
existing isolated developer venv; its extension hash matches the wheel member.

Warnings-denied owner/native Clippy, formatting, independent no-default DS build,
fresh Rust/native inventories and locked resolution for all three Cargo manifests
pass. PNM adds only the existing common-contract leaf; all versions/pins stay
unchanged. The resulting matrix is **615 Rust / 585 native** rows, preserving all
previous 606/579 rows, types, class identities and signatures byte for byte.

The evidence recorder caught three old reference-dependent cases initially
skipped in admission/TX modules. Both modules were rerun with retained independently
generated references; only their no-skip results are counted. The initial Clippy
failure and native reference setup logs are retained as corrections, not passes.

The user also approved retained-owner aggregate accounting with separate finite
construction/numerical execution limits. Simultaneous source/target and candidate
owners remain charged. This unit does not claim pre-production output bounds or
host aggregate admission. Visible application DTO/JSON/queue/copy charges and real
session wiring remain pending. Unchanged UI registration/presentation needs no
Qt executable rebuild; unrelated full workspace/CLI/packaging regressions remain
at Level 2/3. UIU-SESSION-01 remains unchecked; completed history is preserved.


## UIU-SESSION-01 pre-production detached archive bounds

[Design](design.md#pre-production-detached-archive-bounds),
[exact source/command evidence](session-archive-bound-evidence.json) and
[executed API cases](session-archive-bound-operation-test-results.json) trace
REQ-UIU-03/07. Agent preparation `99351a2` was independently source-reviewed,
then adopted exactly into the primary checkout. All legacy copy/helper bodies,
selector/error ordering, scientific queries and info.retained_bytes are preserved.
The protected constructor leaves its private live pilot cache absent, matching
the legacy detached archive. New final nested Vec/String capacities are exact.

Executed: **79 Rust cases** (14 private archive, 37 complete relevant full-loop/
observation/archive/metadata/protected API and 28 facade) and **28 installed
native cases**, with no failures or skips. Ten Rust and four native cases are new.
All **24 independent Rust/native reference cases** cover both original saved
projects, observed/unobserved and complete/pre-cancelled runs, each with default,
reduced and omitted copy policies. Preflight equals actual complete metadata;
historical info and all six role/QAM/decision/soft/decoder views match.

Exact/insufficient byte/root dimensions, generic entry admission, arithmetic
overflow, original source preservation and detached alias lifetime pass. Warnings-
denied owner/native Clippy, independent no-default downstream check, formatting
and fresh compiler/installed inventories pass. The local wheel's installed native
extension hash matches its archive; installation is developer-venv-only.
The resulting matrix is **617 Rust / 588 native** rows; all earlier 615/585 rows,
types, aliases and methods/properties are preserved. The strict matrix validates
all 46 retained successful named execution records against the fresh inventories;
historical tests are not rerun to recreate evidence. The existing native common
RetainedFootprint class additionally accepts a required payload argument and
optional zero entry/root counts, mirroring Rust's public struct literal. Its
properties stay immutable, and negative/overflow integer conversions reject.

This Level 1 unit accepts target preflight and capacity-controlled copying.
Applications still reserve its source/target and old aliases independently.
Temporary builders have separate finite role/event/byte/input limits; no aggregate
scratch/heap/RSS guarantee is claimed. Full-run DS/US bounds, actual host charges/
workers and complete session Level2 remain pending. Unchanged UI/registration
needs no executable rebuild; unrelated full workspace/CLI/packaging regressions
remain at their applicable Level2/3 gates. UIU-SESSION-01 stays unchecked.

## UIU-SESSION-01 pre-production session output bounds

Primary Level 1 acceptance adds three paired operations through the existing
independent PHY APIs and re-export-only facade: downstream FullLoopRequest
result preflight/bounded run, upstream Simulation report preflight/bounded run,
and downstream TransmitterPreviewRequest preview preflight/bounded generation.
They trace REQ-UIU-03/07 and the selected
[owner designs](design.md#pre-production-downstream-result-bounds),
[upstream design](design.md#pre-production-upstream-report-bounds) and
[idle preview design](design.md#pre-production-idle-transmitter-preview-bounds).
The actual session task remains unchecked.

All three dimensions reject before control/source/RNG progress. Finite preparation
snapshots and source exports use the approved separate construction policies;
final output buffers have enforced capacities before publication. Original
scientific execution, partial outcomes, source bytes/provenance, cached/hidden
observations and old owner lifetimes remain intact. Accepted inputs/requests,
old/queued outputs, copy source/target and application projections still need
separate host charges. No aggregate scratch, heap or RSS guarantee is claimed.

Primary executed evidence: **115 unique Rust cases**, including **45 new**
owner/API/facade cases; **48 installed-native cases**, including **six new**, with
zero skips. The fresh local wheel's extension hash/import origin is verified in
an existing isolated developer venv. Native direction workers preserve GIL
release, Busy, pause/cancel, session restoration, original science and aliases.
Separate current Rust processes provide **18 bounded-output reference cases**
(two downstream projects, both upstream numerologies, observed/unobserved and
complete/pre-cancelled modes, plus PRBS/file idle previews). Receiver-estimate and
soft-view references are also refreshed for the affected native consumers.

Both PHY standalone feature checks, owner/facade/native Clippy with warnings
denied, format and fresh rustdoc/native inventories pass. Strict operation tracing
retains every prior **617 Rust/588 native** row and all type/class/member
contracts, adding exactly six operations in each language for **623/594** rows.
The two deliberately doc-hidden generic Serde helpers retain their existing
source/signature checks. Named passed-case execution records accompany the rows.

Initial synthetic TX fixture failures are preserved: the source capture supplied
no exclusion template; the next fixture revision also exposed the mandatory FFT
edge exclusion condition. An explicit decoded exclusion assignment and a named
whole-MHz active offset correct the fixture. Only the failed six-case TX target
was rerun. The new six-case downstream result contract also received a separate
exit-zero invocation for strict execution tracing; duplicate executions are not
included in the 115 unique count. No numerical assertion or production admission
was relaxed.

Source review preserves legacy DS run/scientific bodies. TX's original control
statements move verbatim into shared dispatch; the public generate body is
refactored. US report identity strings/version are centralized through exactly
three unchanged-value substitutions, with a reversal proof; scientific/report
bodies otherwise remain unchanged. A US cancellation seam exists only under
cfg(test), defaults unset and releases the progress guard before cancellation.
Synthetic US Failed/TimeLimit owner fixtures preserve real completed science,
but do not claim actual engine failure/timeout execution. New DS bound cases do
not directly execute failure/timeout, pilot/MCE or alternate imported-response/
playback modes; complete relevant acceptance remains Level 2. Active bounded TX
cancellation lacks a dedicated runtime case; source-reviewed shared dispatch and
legacy cross-thread cancellation are the scoped evidence.

Source-only [downstream preparation](agent-handoffs/result-retention.md) and
[upstream preparation](agent-handoffs/report-retention.md) retain isolated review
provenance; the primary executions above establish this unit's local acceptance.

The machine-readable [receipt](session-output-bound-evidence.json) and
[executed operation cases](session-output-bound-operation-test-results.json)
record exact commands, source/artifact hashes, omissions and unresolved gates.
Ignored logs/references live under
`docs/development/local/unified-qt-workbench/session-01-output-bound{,-native}`.
Unrelated complete workspace/Qt/QML/CLI/project/packaging regressions and a Qt
rebuild are not required for this additive API-only unit: no scientific algorithm,
saved schema, manifests/locks, registration or existing Qt call site changes.
The last accepted Qt executable remains current for unchanged UI sources.
Real application charges, bounded-call adoption, canonical host/registration,
complete session Level 2 and milestone/final Level 3 remain mandatory.

## UIU-SESSION-01 downstream Qt retained-target adoption

Level 1 acceptance adopts the already validated bounded operations in the three
[existing downstream consumers](design.md#downstream-qt-retained-target-operation-adoption):
full-loop execution after runtime narrowing, idle TX generation inside its worker,
and detached receiver history copy. Original callbacks, cancellation/stale results,
partial science, status/error handling and presentation data are preserved.
The backend bound itself is the target limit; this proves controlled retained
target capacities in real Qt paths, without claiming shared aggregate admission.

Executed: **18 actual Qt Rust cases**, Qt all-target Clippy with warnings denied,
formatting, final API boundary, fresh optimized executable, **four QML behavior
cases** plus two fixture hooks, and **six native startup/project probes** using
freshly discovered desktop credentials. No failures/skips. The QML runner emits
two activeFocusOnTab warnings during existing retained-filter navigation; no
presentation code changes. The [receipt](session-qt-target-adoption-evidence.json)
records commands, source/executable identities and exact scope.

Reuse backend/native scientific acceptance at local commit 1841d92: none of its
API, scientific, native, manifest/lock or persistence sources changes. Another
wheel, unrelated complete workspace/CLI/project/packaging and full QML suites
are not required for these three dispatch substitutions. Actual host charges,
upstream adoption/registration and complete session Level 2 plus final Level 3
remain mandatory; UIU-SESSION-01 stays unchecked.

## UIU-SESSION-01 upstream controller target admission

The [planned controller consumer](design.md#next-application-consumer-proven-upstream-controller-admission)
now replaces caller-asserted reservations with actual public Simulation metadata,
checked concrete wrapper quantities and session-bound BatchReport preflight.
Preparation stays off-GUI before the global lease; dispatch validates the exact
session/key and uses the already accepted bounded operation. The [source handoff](agent-handoffs/controller-admission.md)
records its isolated preparation and finite remaining host-owner map.

Executed in the primary checkout: **48 combined Rust cases**, comprising 35 shell
and 13 real upstream controller cases; warning-denied compilation and Clippy,
formatting and final API boundary. New cases reject each report dimension before
lease/control acquisition, reject foreign prepared owners/keys while returning
the genuine unstarted lease, and check wrapper root replacement/overflow. Existing
completion, partial cancel, paused Busy, stale callbacks, disconnected queue,
nonblocking destruction and unwind restoration still pass. The [receipt](session-upstream-admission-evidence.json)
records commands, source/artifact identities and scope. Initial compilation used
a stale cached facade artifact and failed; selecting the current Qt cache artifact
passed without rebuilding or changing sources. Both attempts are preserved.

The component remains unregistered. Backend/native evidence at 1841d92 and the
latest production Qt executable/probes at 171b7aa are reused: none of those linked
sources or public/persistence contracts changes. Another wheel, production Qt
build, full QML or unrelated workspace/CLI/packaging suites are not required for
this unregistered application-controller change. Outer application, budget,
control, transport, input/presentation/error owners and canonical host wiring
remain mandatory before shared aggregate/session acceptance. UIU-SESSION-01 stays
unchecked; complete Level 2 and milestone/final Level 3 gates remain intact.

## UIU-SESSION-01 actual downstream host and terminal adoption

The [concrete host/worker design](design.md#actual-downstream-host-and-terminal-adoption)
now registers the existing Rust application components in the native binary,
binds real downstream Context/Model ownership to canonical generations, and
migrates its scientific worker onto the existing shared terminal transport. Start
acquires the genuine global lease before clearing accepted results. Every data
adoption checks its actual worker token and the current host generation. Finished
is data only; actual terminal/spawn-failure disposal acknowledges admission.
Prepared project models rebind to the target host. The [worker source handoff](agent-handoffs/downstream-worker-resume.md)
retains the isolated preparation scope and its originally unexecuted test claims.

Executed in the primary checkout: **75 unique Rust cases through normal Cargo
binary tests** (48 existing shell/upstream, ten downstream compatibility, four
worker, four workbench-context, five project-load and four real shared-host cases), warnings-denied
all-target Clippy, formatting and final API boundary. The new tests run both real
engines through the same host, test pause/cancel exclusion, Busy preserving a
previous actual report, canonical stale-generation/rebind and destroyed-adapter
termination. Separate error/panic fixtures exercise transport only. The [receipt](session-host-dispatch-evidence.json)
records exact case discovery, commands, failures and source/artifact identities.

A fresh optimized executable passes **six native startup/project probes**, with
current desktop credentials discovered rather than hardcoded. **Four QML
behavior cases** plus two fixture hooks pass; two existing activeFocusOnTab
warnings remain in retained-filter navigation. Initial compilation exposed the
separate-library CXX registration-link dependency and a new Drop-type fixture
construction error. Keeping the existing binary target and normal field
assignment resolved those failures; no scientific assertion or CXX build/link
contract was relaxed. Final rejection-path review then found pre-admission result
clearing and Qt Apply/Restore option mutation. Configuration now commits only
after one successful host admission; the real-report regression preserves options,
verification, request, generation and report under synthetic counter pressure,
then proves successful adoption when pressure releases. QA corrected its settings
JSON fixture to the actual TransmissionSettings type, with independent successful
scientific preflight. Only the affected 23 Model/context/project and final four
host cases reran; unchanged 48 shell/upstream and four worker results are reused.
The failed logs and all candidate/result scopes remain preserved. Final method
placement in focused lifecycle.rs passes the four shared-host cases, all-target
Clippy, formatting, boundary and a fresh optimized build. Six desktop probes
repeat against that final executable; unchanged QML sources/signatures reuse
the recorded four behavior cases and two hooks.

Named desktop limits are independent application safety policies: two GiB payload,
one million ordered entries and 65,536 shared roots, excluding allocator/private
container/refcount bookkeeping and execution scratch. Explicit constructors
accept independent limits. This unit charges the host's actual fixed readiness
metadata; existing scientific roots, wrappers and application/transport/DTO
owners still require complete common-budget participation. No aggregate or RSS
acceptance follows from metadata flags or these defaults.

Scientific/public/native/persistence sources and contracts are unchanged; reuse
accepted backend/native evidence at 1841d92 without another wheel or unrelated
complete workspace/CLI/package regressions. Full affected-feature Level 2 and
milestone/final Level 3 remain mandatory. Upstream QML recovery remains isolated
at 6b9519d, not integrated/registered. Upstream QObject/configuration wiring and
remaining retained-owner closure are finite open gates; UIU-SESSION-01 remains
unchecked. Source-only independent QA found no concrete lifecycle defect; disk
is 80% used with 24 GiB free and 13% inode use, with no confidently obsolete
cleanup candidate and all active caches preserved.


## UIU-SESSION-01 upstream QObject and background preparation

The [concrete direction-adapter design](design.md#uiu-session-01-concrete-direction-adapter-and-request-owner-closure)
is now consumed by a thin registered UpstreamSession QObject. Main constructs it
on the same fixed desktop host as downstream. Its application model parses and
admits public configuration documents off-GUI, and preflights report targets in a
separate bounded preparation before acquiring the genuine global run lease.
Stop suppresses pending preparation adoption/dispatch without joining; the same
idle controller is restored after target preparation. Rejected candidates and
stale configuration completions preserve accepted owners. Actual reports validate
canonical tokens before UI adoption; every genuine terminal is acknowledged.
Scalar state preserves null BER and uses lossless decimal application IDs.

Executed Level 1: **59 unique Rust cases** (36 shell, 13 existing upstream
controller, six new upstream-model and four actual shared-host cases), warnings-
denied all-target Clippy, formatting and final API boundary. Six model cases use
real public configurations/results for complete recovery, invalid Apply, cancelled
target preparation, stale configuration, paused cancellation/partial retention,
abandoned preparation final-drop and pre-copy input admission rejection. Synthetic
counter pressure is explicitly an admission fixture, not allocated scientific data.
Independent identity text anchors cover numbers beyond JavaScript precision.
**91 QML runner cases**, including fixture hooks, pass with no failures/skips;
existing activeFocusOnTab warnings remain visible. The optimized executable passes
**six native startup/Open probes**, constructing the new QObject. The [receipt](session-upstream-qobject-evidence.json)
retains commands, case discovery, artifact hashes and failures. The initial compile
failed on a new fixture include depth and QString len representation; corrected
without relaxing any scientific assertion. The final test-only delta reuses the
unchanged production build; subsequent source edits are documentation banners only.

The retained preparation input uses an exact boxed byte slice admitted before its
copy. Existing session/report wrappers use their accepted public metadata/bounds;
outer model/preparation/control/transport/DTO ownership still requires closure.
Do not infer complete aggregate/RSS acceptance from adapter registration. No public
scientific/native API, calculation, project schema, default or existing invokable
changes; accepted backend/native evidence at 1841d92 remains applicable. Another
wheel/full unrelated core/CLI/package run is not required for this Qt-only unit.
Full relevant Level 2/milestone Level 3 remain open. Source-only QA found no concrete
adapter lifecycle defect and ran no tests. Disk remains 80% used, 24 GiB free and
13% inodes; active artifacts/caches are preserved. No visual control/layout changes
are exposed, so user-guide screenshots remain unchanged. UIU-SESSION-01 stays open.


## UIU-SESSION-01 downstream request and plan owner admission

The [concrete owner design](design.md#uiu-session-01-concrete-direction-adapter-and-request-owner-closure)
and [source handoff](agent-handoffs/downstream-request-admission.md) now admit
accepted public requests/plans in the common host budget. Actual public metadata
plus Retained<Arc<T>> storage and two real shared roots cover each immutable owner.
Model and Worker share outer Arcs; cross-host rebind reserves conservative target
coverage before aliasing the exact same inner scientific allocations. Source and
target reservations survive their respective queued/old aliases through final drop.
No scientific payload clone or clone-capacity assumption is needed for rebind.
Candidates are fully reserved before the host generation/configuration commits.

Executed in the primary checkout: **87 unique Rust cases**, all-target warnings-
denied Clippy, formatting/final boundary, optimized Qt build and **six native
startup/Open probes**. Five new functions cover real exact-root admission and
three-dimensional rejection, valid changed candidates preserving actual completed
reports, old worker/new candidate overlap, cross-host rejection/alias/final-drop,
and checked wrapper arithmetic. Original scientific assertions remain; existing
fixtures change only owner typing and explicit test ceilings to admit real roots.
A prior metadata-pressure fixture now uses explicit unbounded test-only counters
before filling them synthetically; it still proves rejection and old-state
preservation. New exact-owner tests independently exercise each finite ceiling.

The [receipt](session-downstream-request-admission-evidence.json) records exact
source-agent provenance and primary execution. Agent source/rustfmt checks did not
establish runtime acceptance. QML source, imports, registration and invokable shapes
are unchanged from the 91-case registered-adapter run at 6f4b3fe; reuse those QML
results with that scope. Backend/native scientific contracts are unchanged; reuse
1841d92 without a new wheel or unrelated full workspace/CLI/package regression.
Full task Level 2/milestone Level 3 remain mandatory. Per-profile working copies,
result/queued/DTO targets, input/editor/saved JSON, previews/history and application
infrastructure remain concrete open owners. This unit does not accept aggregate
retention or UIU-SESSION-01. Disk remains 80% used, 24 GiB free and 13% inodes;
active caches/artifacts are preserved.

## UIU-SESSION-01 existing observation-query copy envelope

The [query storage design](design.md#uiu-session-01-existing-observation-query-copy-envelope)
closes the concrete stage/inventory copy prerequisite without new public methods.
Exact final capacities preserve scientific values, source capacity, original
coordinates, missing/zero distinctions and errors. Exhaustive eight-variant and
fixed-layout tests prove the conservative per-return source-payload envelope;
all simultaneous queries and derived containers need independent reservations.
Independent QA reviewed the source proof; its review is not execution evidence.

[Exact command/artifact receipt](session-query-envelope-evidence.json) and
[named operation cases](session-query-envelope-operation-test-results.json) record
93 unique Rust cases: 32 owner/API, 29 facade and 32 actual Qt consumers. All pass,
with scoped all-target Clippy, formatting and final dependency-boundary checks.
One freshly built exact wheel passes 25 installed-native cases with no skips:
full-loop science/lifecycle, observed values/errors/ownership/direct-Rust parity,
bounded outputs, retained metadata and import/legacy namespace compatibility.
Four QML behavior cases/two fixture hooks pass; two pre-existing activeFocusOnTab
warnings remain unsuppressed. The fresh optimized executable passes six desktop
workbench/launch probes. Fresh Rustdoc/native inventories retain 623/594 operations,
all prior signatures/types, and historical operation records.

Existing independent process references remain valid because only owned capacities
change; native numerical comparisons executed against them. No algorithm, schema,
namespace, manifest, registration or operating-mode change. Unrelated full workspace,
analysis, CLI, upstream and packaging runs were not required for this focused unit.
Complete retained-owner integration, SESSION Level 2 and milestone/final Level 3
remain mandatory. UIU-SESSION-01 and all previous checklist states are unchanged.

## UIU-SESSION-01 downstream backend-result target admission

The [concrete result-owner design](design.md#uiu-session-01-actual-downstream-result-and-presentation-owners)
is implemented in the primary checkout from the isolated source preparation.
PreparedResult borrows the exact narrowed request and reserves its public result
bound plus the actual fixed ResultData layout/final Arc root before execution.
No completed science is rejected by a later size measurement. The same charge
survives queued Box, current and old Arc owners; Start keeps the previous report
until actual replacement. Input invalidation and original scientific values remain
compatible. Projection buffers and other enumerated owners remain open.

[Exact adoption/execution record](session-downstream-result-admission-evidence.json)
records 93 primary-worktree Rust cases, including six new real target tests:
all three pre-control rejections, exact complete/precancel science, Busy rollback,
genuine worker queue/terminal/final-alias lifetime, old-report preservation and
checked fixed-wrapper overflow. Existing request tests include the newly real
result charge without removing their historical assertions. Four QML behaviors
and two hooks pass with two existing unsuppressed activeFocusOnTab warnings.
All-target Clippy, formatting, final boundary/traceability, fresh optimized build
and six desktop workbench/launch probes pass. Agent source-only checks remain
separate from these primary runtime results.

No linked backend/binding code, signature, algorithm, schema, manifest or QML
changed; the exact 42d177e wheel/native scientific record remains applicable.
Rebuilding that wheel or unrelated complete scientific/CLI/packaging regressions
was not required. Full retained-owner closure, SESSION Level 2 and milestone/final
Level 3 remain open; no checklist checkbox or historical evidence was changed.

## UIU-SESSION-01 per-profile execution request admission

The [concrete execution-input design](design.md#uiu-session-01-per-profile-execution-request-admission)
reuses existing public preparation/metadata and request_retention::admit_request.
Selection/runtime narrowing are finite temporary preparation; the intermediate
request drops before final candidate admission. The final immutable charged owner
is borrowed throughout result preflight, execution and projection. Accepted source
request/plan and old results retain their independent charges. No new public API
or accounting abstraction was introduced.

[Exact case/command record](session-execution-request-admission-evidence.json)
records 95 unique selected Rust cases with passing results across the initial
expanded run and corrected seven-case request rerun. The initial command itself
failed: 93 passed/two test-fixture failures. Independent QA found a zero-entry
request underflow and a cancel-time lifetime race. The corrected tests assert
exact zero-entry admission, payload/root rejection before control, exact paused
ownership and permitted execution-input release after cancellation. Existing
plan/result tests retain genuine all-three-dimension rejection. All seven corrected
request tests pass; repeated cases are excluded from the unique total. Original
failure/source logs remain retained, and the failed command is not labeled passed.

Four QML behaviors/two fixture hooks pass, with two existing unsuppressed focus
warnings. Final all-target Clippy, formatting, dependency boundary/traceability,
fresh optimized build and six desktop probes pass. Existing scientific assertions,
source/old aliases, complete/partial outcomes and historical checkboxes survive.
No backend/binding/signature/schema/registration change required a new wheel or
unrelated full scientific/CLI/packaging runs; reuse the accepted 42d177e artifact.
Other retained owners, complete SESSION Level 2 and milestone/final Level 3 remain
open; this unit does not mark the parent complete.

## UIU-SESSION-01 stored observation and capture admission

The [stored projection design](design.md#uiu-session-01-stored-observation-and-capture-admission)
uses four individually reserved inventory/accumulator/page/replacement slots,
then an allocation-free first pass to admit exact final typed/JSON display storage
before the existing second-pass copies. Final capacities are normalized. Original
values/nulls/tags/coordinates, first/contiguous/phase/frame/decoder selection and
scientific reports survive. Dynamic guards drop after their buffers; the enclosing
ResultData backend/fixed-layout reservation drops last. Independent QA reviewed
both isolated source and primary wiring; neither source review is runtime evidence.

[Exact source/primary case record](session-observation-admission-evidence.json)
records 107 selected primary Rust cases with no failures/ignores. Eleven new
cases cover all eight JSON variants, arithmetic, phases, paging/slot overlap,
independent target measurement, exact source/query/target ceilings, alias disposal,
selection edges, precancel and actual worker fallback. One final one-case rerun
also verifies the completed result's static diagnostic through actual Model state;
it is excluded from the unique count. An unused fixture-only constructor warning
from initial compilation was removed before final all-target Clippy; no runtime
behavior relied on that helper.

Four QML behavior cases/two hooks pass, with two existing unsuppressed focus
warnings. Formatting, final dependency boundary/traceability, all-target Clippy,
fresh optimized build and six native workbench/launch probes pass. The completed
backend remains available when display admission rejects; no uncharged fallback
warning Vec/String is retained. The existing Receiver reason and warning response
borrow/project its static diagnostic. Exact pressure fixtures include real new
projection storage or assert stable retained floors during asynchronous query/target
construction, retaining all scientific and exact final-cleanup assertions.

No backend/binding/API signature, schema, registration or dependency change required
another wheel or unrelated complete scientific/CLI/packaging suites. Reuse the
accepted 42d177e backend/native record and unchanged 623/594 operation inventories.
Independent instrument/transport copies, input/editor/project, preview/history and
infrastructure owners remain open. SESSION Level2/milestone/final Level 3 acceptance
is still required; all historical receipts and checklist checkbox states survive.


## UIU-SESSION-01 idle transmitter preview owner admission

The [prepared/product design](design.md#uiu-session-01-idle-transmitter-preview-owner-admission)
reuses existing public metadata and bounded generation. Concrete timing, request
children, actual carrier/span capacities and both prepared Arc roots are admitted
before sharing immutable inputs with the worker. Product target admission precedes
public control claim/generation and follows Box queue, current/stale/cancelled
completion and final disposal. Target-host admission precedes cancellation or
replacement of the live analyzer; failed rebind preserves its accepted work.

[Exact primary execution record](session-preview-admission-evidence.json) records
23 selected Rust passes, including seven new owner cases, original preview science,
project/context transfer and actual shared-direction lifecycle tests. The initial
command passed 22 cases and failed one fixture assertion: transfer also released
the final source host's readiness metadata. The corrected fixture independently
retains that host, verifies unchanged source charge through rebind, then verifies
separate final drops of the prepared alias and host. The complete corrected run
passes all 23; the initial failed command remains recorded as failed.

Four QML behaviors/two hooks pass with the two existing unsuppressed focus warnings.
All-target Clippy, formatting, boundary/traceability, optimized build and six native
desktop probes pass. Original numerical assertions, values, invalid-input handling,
historical evidence and checklist checkbox states survive. Root changes also apply
the required comment spacing; the isolated source handoff remains source-only.

No backend/binding/public signature, project schema, registration or dependency
change requires another wheel or unrelated full scientific/CLI/packaging suites.
Reuse the exact accepted 42d177e backend/native artifact and unchanged operation
inventories. Independent input/editor/project, history/performance, instrument/
transport and outer infrastructure owners, complete SESSION Level2 and applicable
milestone/final Level3 remain open. This focused unit does not accept the parent.


## UIU-SESSION-01 detached receiver history admission

The [concrete archive design](design.md#uiu-session-01-detached-receiver-history-admission)
uses the existing public target bound/bounded copy and shared Context budget.
Checked root replacement includes the actual Snapshot layout and one outer Arc;
the backend's own detached root and original quantities remain included. Copying
starts only after admission. The archive moves without the former second clone;
its last guard survives registry replacement/clear, Context destruction and every
selected old alias until final drop. Host rejection preserves prior status/history;
ordinary backend errors retain historical unavailable behavior.

[Exact primary execution record](session-history-admission-evidence.json) records
29 selected Rust passes with no failures/ignores, including six new owner cases,
the original history oracle, receiver instruments, project/context transfer and
real shared-direction lifecycle consumers. Tests prove byte/root boundaries and
overflow, exact simultaneous source/target ownership, original QAM/decoder/info
parity, source passivity, selected-alias lifetime, rejected replacement and real
precancelled absence. Actual zero ordered entries remain zero.

Four QML behaviors/two hooks, all-target Clippy, formatting, boundary/traceability,
optimized build and six native probes pass. Two existing unsuppressed QML focus
warnings remain recorded. Independent QA reviews source only; source-agent checks
alone do not establish runtime acceptance. Root wording distinguishes excluded
allocator bookkeeping from separately owned accounting metadata; comment spacing
also follows coding-style. Original scientific values/assertions and historical
receipts/checklist states survive.

No public/backend/binding signature, science, schema, registration or dependency
change requires another wheel or unrelated complete scientific/CLI/packaging
suites. Reuse the exact 42d177e backend/native record and unchanged inventories.
Performance, Entry diagnostics/registry capacity, input/editor/project, independent
instrument/transport and infrastructure owners remain open, along with complete
SESSION Level2 and applicable milestone/final Level 3. This unit accepts no parent.


## UIU-SESSION-01 copied receiver performance admission

The [concrete performance design](design.md#uiu-session-01-copied-receiver-performance-admission)
reuses the existing finite public profile query and FEC builders as temporary
preparation, then normalizes and admits actual final Entry/text/Point/JSON storage
before registry adoption. It assumes no collect/clone capacity or multiplier.
Existing source and old entries remain charged. The last guard follows stored
children through replacement/clear/final drop; no payload Arc is introduced.
All initial entries must be admitted before either history is cleared. Host
rejection preserves prior entries; existing query errors keep their admitted
failed-entry behavior and complete error text.

The real Model returns its genuine unstarted lease on initial Entry failure,
before token/timing changes or dispatch. Profile callbacks surface display
admission failures and still publish completed science; setup callbacks retain
the original scientific error while logging secondary display rejection.
Independent QA reviews both source and integration wiring; its review is not
runtime evidence.

[Exact primary execution record](session-performance-admission-evidence.json)
retains an initial fixture import compile failure, then the wider 71-case run
(68 passed/three failed) and corrected 15-case rerun (all passed). The failures
were existing accounting fixtures omitting the newly admitted initial entries:
one paused exact-total assertion and two tight budgets preventing their intended
first scientific run. Actual initial/stored reservation helpers correct these
fixtures without guessed allowances or weaker scientific/lifetime assertions.
Those cfg(test) helpers read assigned ledger charges, explicitly separate from
independent storage-layout/scientific verification. Histories are explicitly
cleared before isolating the original final-backend-alias assertions.
All 71 selected unique cases have passing results across the runs; 12 repeated
cases are excluded from that total. The failed commands remain labeled failed.

Six new owner tests and one real Model no-dispatch/lease/history test pass, along
with the affected receiver/closed-loop/context/project consumers. Four QML
behaviors/two hooks, final all-target Clippy, formatting, boundary/traceability,
optimized build and six native probes pass. Two existing unsuppressed QML focus
warnings remain recorded. Public values, original plotting/PLC assertions,
scientific outcomes and historical receipts/checklist states survive.

No backend/binding/signature, science, schema, registration or dependency change
requires another wheel or unrelated complete scientific/CLI/packaging suites.
Reuse the exact 42d177e backend/native record and unchanged inventories. Accepted
input/editor/Designer/saved-project, registry spare capacity/QAM diagnostics,
independently returned query/UI transport and infrastructure owners remain open,
with complete SESSION Level2 and applicable milestone/final Level 3 gates. This
focused unit does not accept the parent.


## UIU-SESSION-01 atomic accepted input and saved-document admission

The [concrete design](design.md#uiu-session-01-atomic-accepted-inputs-and-prepared-projects)
couples matching accepted Inputs and saved sections in one immutable admitted
owner. Shared read handles preserve original data/charges through replacement and
final drop. Source/profile/measurement Apply validates a candidate, publishes the
complete input/document pair once, then commits its editor. Power/loss are paired
updates. Matching measurement presence refresh returns without new publication.
Save/recovery borrows the original charged map through unchanged validation,
serialization and atomic writing; complete and empty schemas 1/2 parity pass.

Queued project inputs carry the actual target budget, full canonical key and a
charged snapshot of its original accepted owner. This snapshot cannot form a
Context cycle. Adoption rejects foreign/stale targets before path/registry/input
mutation. The accepted-owner comparison and swap share one lock, also rejecting
an intervening Apply before the canonical generation advances. Independent QA
reviews concrete arithmetic/lifetimes/transactions only; runtime evidence comes
from primary integration checks.

[Exact primary execution record](session-input-admission-evidence.json) retains
all attempts: initial 102 Rust cases (100 passed/two old fixture failures), a
new test-helper visibility compile failure, corrected 17-case rerun, and final
27-case owner/project/shared-lifecycle rerun. All 115 selected unique cases have
passing results across those runs. The fixture corrections explicitly retain
the source Context through transfer assertions and include still-live accepted
Inputs when isolating final report aliases; no scientific assertion is weakened.
Six installed native saved-project cases pass with the unchanged wheel/reference;
the initial missing-reference environment failure remains recorded. Four QML
behaviors/two hooks pass, with two existing unsuppressed focus warnings. Final
all-target Clippy, format/boundary, optimized build and six fresh desktop probes
pass. Documentation/traceability checks are linked in the exact record.

Public scientific/binding APIs, data values, schema, dependency/registration and
CLI contracts remain unchanged; reuse their existing inventories/evidence and do
not rebuild another wheel or run unrelated full suites. The shared performance
JSON counter uses the same live actual-capacity leaf. Prepared editor/Designer
model graphs, all-target receiver/preview readiness before project publication,
registry/instrument/transport and infrastructure owners, complete SESSION Level 2
and applicable milestone/final Level 3 acceptance remain open. This unit leaves
UIU-SESSION-01 unchecked and preserves historical completed tasks.


## UIU-SESSION-01 prepared project target preflight

The [focused design](design.md#uiu-session-01-prepared-project-target-preflight)
closes concrete late receiver/preview admission after project publication.
Background Open now admits actual request/plan and transmitter input roots against
the destination budget while old and queued owners remain charged. Candidates
retain isolated Contexts. Adoption checks idle/root ownership and the captured
target, then holds the original accepted-owner lock through the guarded canonical
readiness commit. Only success publishes inputs/results/transfers and permits the
caller to update the native Save path. GUI attachment reuses those owners and
the genuine committed key without another reservation or generation change.
Successful edits/New discard old unconsumed transfers; failed edits preserve them.
The old worker retains its genuine global lease until known terminal even when
project adoption requests cancellation. Independent QA reviewed these paths
read-only; primary runtime evidence is separate from source preparation.

[Exact primary execution record](session-project-preflight-evidence.json) retains
the initial compile failure, 127 passes/one new rollback watchdog failure, and
the corrected 23-case pass. All 128 selected unique Rust cases have passing
results across those runs. The rollback fixture explicitly runs its applied
profile to obtain one real old report; existing legacy sweep coverage remains
unchanged. Six installed native saved-project cases, four QML behaviors/two
hooks, all-target Clippy, formatting/boundary, optimized build and six fresh
desktop probes pass. Two existing QML focus warnings remain unsuppressed.
Documentation and requirement/checklist history checks are recorded separately
in the same receipt.

Public scientific/binding APIs, algorithms, schemas, dependencies and registration
remain unchanged; reuse the exact native wheel/reference and export inventories.
No unrelated full workspace/CLI/packaging suite is claimed executed. Prepared
editor/Designer graphs, independently retained transport/infrastructure owners,
complete SESSION Level 2 and applicable milestone/final Level 3 gates remain open.
UIU-SESSION-01 remains unchecked; completed downstream history is preserved.


## UIU-SESSION-01 retained editor candidates and prepared transfer

The [focused design](design.md#uiu-session-01-retained-editor-candidates-and-prepared-transfer)
now admits actual independent source/profile/measurement editor candidates before
GUI replacement and coupled Apply publication. Original bytes, decoded frames,
catalog/JSON/options buffers and capture Designer Origin children participate in
the existing three-dimensional budget. Queued Open/New models retain their guards;
exclusive payload/guard transfer preserves data-before-reservation disposal and
does not reserve again during GUI reload. Rejected candidates preserve both editor
and accepted input/document state. New now preflights the complete reset before
changing the project or Save destination, preserving appearance. Untouched interval
text borrows the unchanged static default; owned parsed text remains charged.

[Primary execution record](session-editor-models-evidence.json) records 95 passing
unique Rust cases, then 13 passing affected cases after restricting the unused
legacy restore helper to tests. Initial compile and Clippy failures remain
recorded; no runtime failure was hidden. Eight QML behaviors/two hooks pass without
warnings, including rejected Source Begin and New preserving existing state.
Six installed-native saved-project cases, final all-target Clippy, formatting,
boundary, optimized build and six fresh desktop probes pass. The existing exact
wheel/reference/export inventories are reused because backend/binding APIs and
schemas did not change. Documentation/traceability checks are recorded separately
in the same receipt. Independent read-only review found the New/default gaps;
their concrete fixes and regression coverage are included here.

Source-only agent preparations are separately identified and do not establish
MERLIN runtime acceptance. Designer Session/work/export, registry/presentation/
infrastructure owners and full SESSION Level 2/milestone Level 3 remain pending.
UIU-SESSION-01 stays unchecked; no unrelated full workspace/CLI/packaging execution
is claimed and all historical tests, vectors and completed tasks are preserved.


## UIU-SESSION-01 receiver registry storage

The [focused design](design.md#uiu-session-01-receiver-registry-storage) now covers
QAM catalog slot/status buffers with actual-capacity admission before replacement.
Snapshot aliases keep their original scientific reservations. Performance retains
only occupied Vec slots already charged by live Entry guards; boxed-slice
normalization precedes publication and clear releases the complete buffer.
Initial Begin prepares both admitted catalogs before changing either history.
Late rejection preserves exact previous status, scientific identity and results.

[Primary execution record](session-registry-storage-evidence.json) records 74
passing cases/two new fixture failures, then three passing focused cases after
correcting the omitted existing host-readiness baseline. All 76 selected unique
Rust cases have passing evidence; the failed initial logs remain. No production
code changed for that correction. Four QML behaviors/two hooks pass without
warnings, as do final all-target Clippy, formatting/boundary, optimized build
and six fresh desktop probes. Read-only QA reviewed mutators, rollback, guard
lifetimes and lock order separately; source preparation did not establish runtime
acceptance. Documentation and requirement/checklist checks are recorded in the
same receipt.

No scientific/public API, binding, schema, dependency or registration changed.
Existing exact backend/native/export evidence is reused, without claiming new
Python, full workspace, CLI or packaging runs. Designer Session/work/export,
retained presentation and fixed infrastructure owners remain pending, together
with full SESSION Level 2 and applicable milestone/final Level 3. UIU-SESSION-01
stays unchecked and completed downstream history is preserved.


## UIU-SESSION-01 retained Designer session copies

The [focused design](design.md#uiu-session-01-retained-designer-session-copies)
now admits the actual GUI Designer Session and independently prepared Session
inputs for both generation and export workers. Preview borrows the same charged
owner; actual copies retain their guards through closure completion even after
GUI replacement/destruction. Metadata includes private original/profile/catalog/
design/request children and actual cloned capacities, preserving existing science.
Rejected replacement/copy leaves prior controls/profile/original metadata intact.
Export preparation no longer discards its old Ready snapshot before successful
input admission and worker creation. Existing document ceilings are named by
their independent recipe/playback/export purposes, with unchanged values/errors.

[Primary execution record](session-designer-input-evidence.json) contains 41
passing unique Rust cases and a final five-case bridge pass after naming the
unchanged limits. Seventeen QML behaviors/four hooks pass without warnings;
all-target Clippy, formatting/boundary, optimized build and six fresh desktop
probes pass. Read-only QA confirms Session ownership/lifetimes/rollback and no
Context cycle. The direct prior-Ready export rejection/spawn-failure runtime case
remains a documented gap for the next export Prepared/State acceptance; source
inspection alone is not reported as executed coverage. Source-agent preparation
and exact unchanged scientific/native/export evidence are separately identified.
No new Python, full workspace, CLI or packaging execution is claimed.

Request/control/worker envelopes, capture queues, export products, retained QML
and fixed infrastructure owners remain pending. This unit accepts Session input
copies only. Full SESSION Level 2 and applicable milestone/final Level 3 still
precede the parent checkbox; all completed downstream history remains preserved.


## UIU-SESSION-01 retained export products and ready-state rollback

The [focused design](design.md#uiu-session-01-retained-export-products-and-ready-state-rollback)
now admits complete Prepared frame/JSON/error capacity before terminal queueing.
One charged shared root permits immutable Ready/save-worker aliases without an
extra payload copy. Working preserves its old Ready snapshot until genuine
terminal reception; input/product rejection, parse/spawn failure, cancellation
and disconnection preserve or restore the exact original. Existing scientific
preflight/save bodies move behind thin QObject calls into a focused lifecycle
module, retaining format/limit/reply behavior and public Rust ownership.

[Primary execution record](session-export-product-evidence.json) records the
initial pin-conversion compile failure before runtime, followed by all 46 passing
Rust cases. Seventeen QML behaviors/four hooks, final all-target Clippy,
formatting/boundary, optimized build and six fresh desktop probes pass without
QML warnings. Direct actual-state tests close the previous Session-input unit's
prior-Ready input-rejection and prepare/save spawn-failure gap; no filesystem
save or native picker was executed by those deterministic failure tests. Queued
success/cancel/disconnect, all three late target dimensions, old/new frame equality
and final shared disposal are exercised. Source-only metadata preparation and
read-only review remain separate from primary execution. Documentation/traceability
checks are recorded in the same receipt.

No scientific/public API, binding, persisted schema, dependency or registration
changed; exact scientific/native evidence is reused without a new wheel or Python,
workspace, CLI or packaging run. Saved completion/error envelopes, other worker/
generated-queue data, retained QML and fixed infrastructure remain pending.
UIU-SESSION-01 stays unchecked until full relevant Level 2 and applicable milestone/
final Level 3 acceptance, preserving all completed downstream history.
