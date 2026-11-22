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
