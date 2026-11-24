# Upstream presentation integration preparation

Primary SHELL adoption selects the thirteen startup/drill-in QML files and its
startup test. The two resource-grid QML files, grid test and two grid fixtures
remain preserved exactly at the source commit/worktree below for UIU-US-04.
They are not part of the SHELL implementation commit. The complete hashes and
grid fixture results below describe the frozen source candidate; they do not
claim primary adoption or native grid acceptance. Primary registration also
renames upstream VerificationDialog to UpstreamVerificationDialog to avoid the
pinned QML cache basename collision.

This is a bounded source integration prerequisite for
[UIU-SHELL-01](../../../checklist.md#task-uiu-shell-01),
[UIU-US-01](../../../checklist.md#task-uiu-us-01) and
[UIU-US-04](../../../checklist.md#task-uiu-us-04), using the accepted
[SESSION C5 presentation contract](../design.md#uiu-session-01-c5-existing-persistent-presentation-owners)
and [REQ-UIU-07](../../../../requirements.md#req-uiu-07). SESSION is already
accepted at the base below. These fixture checks neither reopen that acceptance
nor accept SHELL/US tasks. Root owns source registration, actual host adapters,
Main integration, canonical documentation and runtime task acceptance.

## Exact recovery and ownership

Branch `feature/qt-upstream-presentation-integration` starts at accepted SESSION
`50339f01e2c0829350ac39d4be0fe893eee74da6`. Incoming
`6b9519dccf4be3576f404793241855908eeb1470` has parent
`c33e4b273c3680e6499fb2b1be579b8cf303d79d`. Its exact 15 QML and four fixture/test
paths were verified against Git objects before adaptation and copied alone.
The old upstream-ui-resume.md was not copied; this is the only new handoff.
Later `7a0cd0c` draft-dialog source is deliberately excluded. All prior branches,
including upstream originals, 15d80e5, f341906, 44e1da8 and 2129577, are preserved.

Only three production files adapt: UpstreamWorkbench.qml, UpstreamDetailDialog.qml
and ResourceGridView.qml. The other twelve production files and both fixture
files remain byte-identical to 6b9519d. Both existing QML tests adapt to the
accepted explicit PresentationBudget fixture seam. Every new file is listed
with its SHA-256 below; this handoff makes exactly 20 allowed paths. No existing
root/session/shared source, build/registration file, Cargo, lock, Main, science,
canonical document or checklist is changed. Existing GPL Qt notices remain.

## Stable host and signal contract

`UpstreamWorkbench` still requires injected `theme` and `host`:

- `host.presentation.blocks[sectionId] = {status, reason}` supplies authoritative
  readiness display. The component admits this plain snapshot before retaining
  it; missing blocks keep the existing attention/unavailable state. No readiness
  or scientific result is synthesized.
- `host.presentation.details[sectionId] = {summary, rows:[{label,value}]}` supplies
  display strings only. Each drill-in independently retains its admitted snapshot
  and derived row strings, so rejected child replacement can keep the matching
  old details after the parent changes. Child plot bindings do not create a
  second data owner.
- Existing section identifiers are `ucd`, `profiles`, `grants`, `transmitter`,
  `channel`, `receiver`, `source`, `preEqualization`, `rxmer`, `verification`.
  `openSection(sectionId)` preserves `sectionRequested(sectionId)` navigation
  and now returns the shared dialog's `openChecked()` result. Unknown identifiers
  remain inert. Configure can reuse `openSection("ucd")`.

`ResourceGridView`/`ResourceGridDialog` retain the original `host.resourceGridState`
contract: status/reason, accepted public snapshot, authoritative Rust `max_cells`,
opaque session/snapshot-session tokens, lossless configuration-generation string
and actual configured grant choices with grant_id/label/session_token. The
unchanged public view request has only frame_index, first_symbol, symbol_count,
first_carrier, carrier_count and max_cells. `viewRequested(request,context)`,
`grantRequested(grantId,context)` and `cellInspected(cell,context)` retain their
original context envelopes; grant requests select an actual configured session.
Wide identities/timing stay original strings/tokens, without Number conversion.
Only admitted bounded screen indices participate in existing geometry operations.

The resource dialog remains separately instantiable. No new startup route,
scientific API, allocation reconstruction, pilot placement or timing inference
is added. Actual observation/draft/Apply adapters and grid wiring remain US task
work; the read-only UCD/profile/grant inspectors are not configuration editors.

## Actual C5 consumers and lifecycle

All new owners and the frame text Binding are explicit typed properties with
distinct property names and ids. They do not become implicit Dialog contentData
children. All inherited dialogs use accepted WorkbenchDialog visibility accounting;
owned open calls use openChecked. The ten named detail windows share the guarded
base dialog; ResourceGridDialog delegates its data owner to ResourceGridView.

Workbench guards the retained host presentation. Detail guards its independently
retained presentation plus flattened label/value strings before Repeater model
conversion. Grid guards one coherent bundle containing the original host state,
pending window/grant/context, selected/hover cell/context, derived grant choices,
role/count labels, metadata label, tooltip and inspection JSON. The grid model is
measured before ComboBox conversion; no Qt sequence readback is counted. Palette
and geometry remain the existing fixed presentation style, separate from DTOs.

The shared owner reserves candidate logical bytes/entries while the old owner is
held. Rejection retains the old snapshot/context and shows bounded static feedback.
Grid requests and inspections emit only after their own candidate is admitted;
rejected incoming state freezes actions until that exact host state is admitted.
Existing unavailable/loading/stale-generation/pending-grant rules still suppress
scientific actions and preserve original identity matching. No native accepted
configuration or completed science is mutated by these presentation operations.

The actual noneditable frame selector contentItem also uses reserveTextFields,
covering its actual maximumLength before showing dynamic displayText or enabling
actions. A typed Binding retains normal SpinBox formatting when admitted. No
maximumLength, validator or backend frame bound is invented/changed. There are
no editable draft fields in this recovered unit. Hidden/closed DTOs and text keep
coverage; explicit owners clear on destruction and final parented adapter disposal
owns remaining reservations. Source-only logical accounting is not a Qt heap/RSS
or physical-allocation deduplication claim.

The approved startup diagram, all ten purpose/reference-plane/processing labels,
connector role/style, C-before-H relation, H versus C×H labels, and receiver
observation semantics are unchanged. Resource rendering remains batched Canvas,
with frame-symbol X and physical-carrier Y, all six reported roles, original
holes/origins, full backend capacity/counts and no object per cell.

## Executed bounded evidence

Both isolated runs passed 36 Qt cases (32 functional rows plus four init/cleanup),
0 failed/skipped/blacklisted. The second/final run includes the frame contentItem
coverage and is the frozen source result. The four additional functional cases
exercise rejected host readiness, checked-open rejection/recovery, rejected grid
snapshot/context and rejected pending request/inspection without emitted actions.
The original mouse/keyboard/focus, dark/light, resize/scroll, role-color/coordinate,
stale generation, grant session isolation, bounded item count and metadata checks
remain. Grid has 71 visual items for either 204 or 396 actual reported cells.

The two original snapshots/600 cells are byte-identical public Rust-process
fixtures; their reviewed producer/provenance stays in the unchanged fixture README.
No Rust producer was rerun. Full wrapper SHA-256 is
`e527493ed9bcfe45bbe85c407e22928db32f9ed946ad7563f4aa01c90e3f344c`;
original payload SHA-256 remains
`653f27fc675a9c97e7673c6bbf422e9bd94e28aada82e9fc3b90bed8ac3b7ab6`.
Readiness strings in startup cases are explicitly presentation fixtures, not
scientific acceptance or a fake runtime adapter. The reused fixture counter is
application-private test transport, not native application-counter evidence.

Command environment: existing Qt 6.8.3 SDK, QT_QPA_PLATFORM=offscreen,
QT_QUICK_BACKEND=software, QSG_RHI_BACKEND=software, QML_DISABLE_DISK_CACHE=1,
Fusion style; input apps/qt/tests/upstream only. Working/output/cache directory:
ignored docs/development/local/unified-qt-workbench/upstream-presentation-integration
inside this isolated worktree. Initial/final logs remain there, with final result
SHA-256 `2aafa3d68a2a68ce81d007be0b709bef8c8e7e9e08f8025d22c243146cb375af`.
Normal dark startup and actual-grid fixture renders were visually inspected;
these are qmltestrunner captures, not an application/native screenshot claim.

All 15 production QML and two QML tests parsed using qmlformat without rewriting
formatting. Staged/unstaged/untracked scope and all adapted diffs were reviewed;
git diff --cached --check passed. Python guide --check reported current; helper
refresh must leave its SHA-256 unchanged. SDK artifacts remain read-only:

- qmltestrunner SHA-256: `636d7a9a050891bb36d918d2159fd621a87e188761390ce88160b5789e223d0b`.
- qmlformat SHA-256: `6a83f4367c098e1f0069ce88483f5dec76817054db8bef7efa63eb14241230fc`.

Disk before and after: 79% used, 25 GiB free; inodes 12%. No Cargo target, build,
wheel, native executable, full regression, global cache edit or external deletion
occurred. Cargo/Clippy/native/shared runtime acceptance and qmllint are UNEXECUTED
for this unit. Root owns actual shell registration/host execution and US acceptance;
fixture passes alone establish none of those gates.

## Exact recovered source inventory

| Repository-relative path | SHA-256 |
| --- | --- |
| `apps/qt/qml/upstream/BurstProfilesDialog.qml` | `bb48727854c1762e09e2e5a4989c68890321ae5eeff8fa28a679b840fa6c9613` |
| `apps/qt/qml/upstream/ChannelDialog.qml` | `3609b2cb7b10f73e55b93dcfd11c724baf5d06da1d226b2cf82911d593dbab6e` |
| `apps/qt/qml/upstream/GrantsDialog.qml` | `1c3d7f841d29167ae526176252b68174222ae6224af02fe699ae5a074ec25422` |
| `apps/qt/qml/upstream/PreEqualizationDialog.qml` | `f74c9259cefdef90d9c863895eff730919e5191d8319e2ec99bcd8bf4ce4eaef` |
| `apps/qt/qml/upstream/ReceiverDialog.qml` | `8807c368205eff35ddb6d3da2b77e3aa2f7406c588236945283b3edc9252914a` |
| `apps/qt/qml/upstream/ResourceGridDialog.qml` | `81b71ab8a77198dc305c5e1572e7037482f30583a3af61d2bdd9fa73f5bb9574` |
| `apps/qt/qml/upstream/ResourceGridView.qml` | `a4dd09d89ed7484da58637f5a0144524daae959f742d388f9a6fc86fd124b8b7` |
| `apps/qt/qml/upstream/RxmerDialog.qml` | `0c49ac043a35f8de48d0a5fadb364e3e1f7583cc53d1bf300c37a0f93c6abd2c` |
| `apps/qt/qml/upstream/SourceDialog.qml` | `313442d568d64379a3583584bdb7b29f88c495ca5e6ff87d4b70e190aa0af294` |
| `apps/qt/qml/upstream/TransmitterDialog.qml` | `75ab6f87f41dbc308f4a8d5fef37adef4a6d33a7532b3d3a9a71f467d4e105f2` |
| `apps/qt/qml/upstream/UcdConfigurationDialog.qml` | `9937af805e01e73277ee3f8d8c49095b10d846304223d23afeb7b6c86f51800a` |
| `apps/qt/qml/upstream/UpstreamConnection.qml` | `9167d353dda8a5023a958eda55954988d94a04a58ea34b950fbd588af5f8de82` |
| `apps/qt/qml/upstream/UpstreamDetailDialog.qml` | `b45c958895e7c6a7d92a4571228b3a1f892c0a14170d0bf374889bbf6e8dd285` |
| `apps/qt/qml/upstream/UpstreamWorkbench.qml` | `382e8ba526b907b32669e7a8e98764c32281d81947df109fa2cdda5a79f218ad` |
| `apps/qt/qml/upstream/VerificationDialog.qml` | `cc28492549922058ba408b60e7807eb7fa4ee539060e2c287185ec5d2ff53a30` |
| `apps/qt/tests/upstream/fixtures/README.md` | `0f10f702a27073f294effa2c6c9ffa34fca82c3987726650671f2584b2c2ccbe` |
| `apps/qt/tests/upstream/fixtures/ResourceGridFixture.js` | `e527493ed9bcfe45bbe85c407e22928db32f9ed946ad7563f4aa01c90e3f344c` |
| `apps/qt/tests/upstream/tst_resource_grid.qml` | `2658aa1227ef72f1e3baf6c643115e7fa9c609cb3768464c76bde5602c81ad3c` |
| `apps/qt/tests/upstream/tst_upstream_workbench.qml` | `7779c4e002e8dc40bf4644be8c432a7d0651541b3619dff771f6a95f0ca599d9` |

The unchanged Python guide SHA-256 before the authorized save helper is
`ea767a75658ff4b8b8121aedf751a4e2cfc10ebbbc08a0384c874e4a55501591`.
