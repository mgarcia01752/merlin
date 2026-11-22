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
