# Upstream configuration draft presentation preparation

Primary adoption note: source `aa90a179719ea3f2d64b39e13641f305ed890f5a`
is integrated selectively, preserving the primary Verification type rename.
The actual native adapter exposes resolved backend channel defaults when an
optional channel document is absent; only a changed field materializes that
document option. The original source-preparation absence wording below is
historical. Native integration/acceptance is recorded by the lead separately;
this source receipt alone accepts no official task.

## Scope and acceptance boundary

This isolated source preparation follows [UIU-US-01](../../../checklist.md#task-uiu-us-01),
[REQ-UIU-07](../../../../requirements.md#req-uiu-07),
[REQ-UIU-10](../../../../requirements.md#req-uiu-10),
[REQ-UIU-11](../../../../requirements.md#req-uiu-11) and
[REQ-UIU-27](../../../../requirements.md#req-uiu-27), under the
[upstream architecture](../../../../architecture.md#unified-qt-upstream-scientific-presentation)
and [approved startup layout](../upstream-startup-layout.md).
Architecture and checklist were reviewed; no ownership, schema, scientific API or
acceptance status changes belong to this unit. Accepted SESSION/SHELL remain
lead-owned baselines; actual native US01 integration/acceptance is still pending.

Branch `feature/qt-upstream-configuration-drafts` starts directly from frozen
`aaddc5ba61698f7b346116484b5d6be4a24fe57a`. The presentation and fixture source
was recovered from `7a0cd0c64d86b43dd378dfa3f27cb5db342cddac`, then adapted to the
accepted C5 seams. All earlier branches and commits remain preserved.

| Disposition | Exact owned path | Responsibility |
| --- | --- | --- |
| Add | `apps/qt/qml/upstream/UpstreamConfigurationDraftDialog.qml` | Seven-section draft lifecycle, guarded local values/models/errors, finite actual-text complement and shared Apply/Cancel footer. |
| Edit | `apps/qt/qml/upstream/UcdConfigurationDialog.qml` | Approved UCD field whitelist; existing title/reference-plane labels. |
| Edit | `apps/qt/qml/upstream/BurstProfilesDialog.qml` | Existing IUC/profile-group whitelist; other burst attributes untouched. |
| Edit | `apps/qt/qml/upstream/GrantsDialog.qml` | Explicit synthetic grant whitelist; represented MAP/probe inspection only. |
| Edit | `apps/qt/qml/upstream/SourceDialog.qml` | Source-seed whitelist, preserving the separate PRBS-TX block/reference plane. |
| Edit | `apps/qt/qml/upstream/ChannelDialog.qml` | Existing optional channel scalar whitelist; physical H/effective C×H wording unchanged. |
| Edit | `apps/qt/qml/upstream/PreEqualizationDialog.qml` | Existing initial-bank ID only; full-bank/lineage remains accepted-state inspection. |
| Edit | `apps/qt/qml/upstream/ReceiverDialog.qml` | Existing receiver scalar whitelist; received-observation semantics and processing stages unchanged. |
| Edit | `apps/qt/qml/upstream/TransmitterDialog.qml` | Configure Source navigation through the existing source editor; existing processing stages unchanged. |
| Edit | `apps/qt/qml/upstream/UpstreamWorkbench.qml` | Inject the existing host and forward the three draft signals and configuration acquisition request only. Diagram, connectors and verification component line remain unchanged. |
| Add | `apps/qt/tests/upstream/tst_upstream_drafts.qml` | Explicit presentation-host/counter fixtures and focused lifecycle/C5 interaction cases. |
| Edit | `apps/qt/tests/upstream/tst_upstream_workbench.qml` | Find the visible title-X under `popup.header`; the inherited replaced header can remain an orphan QObject child. |
| Add | This handoff | Scope, exact interfaces, receipts and remaining native prerequisites. |

No Main, build/registration, Rust/CXX, Cargo/lock, shared components, scientific
owners, canonical architecture/requirements/checklist or global limits changed.
Root's `UpstreamVerificationDialog` rename is deliberately outside the Workbench
hunks; retain that root variant during adoption.

## Host and field contract

The injected host retains `presentation.blocks/details` and its existing resource
grid contract. The new application-only alias is:

```text
host.draftState = {
  section: "ucd" | "profiles" | "grants" | "source" | "channel" | "preEqualization" | "receiver",
  token: <fresh nonempty string>,
  baseSession: <decimal string>, baseGeneration: <decimal string>,
  values: {<opaque field key>: <string or ordered string[]>},
  options: {fields: [{
    key, field, label, kind: "choice" | "integer" | "decimal" | "selection",
    editable, choices: [{value: <string>, label: <string>}],
    max_length, max_items, help
  }]},
  can_apply, pending, error,
  outcome: "editing" | "accepted" | "rejected" | "cancelled" | "stale"
}
```

The original signals remain `draftRequested(sectionId)`,
`draftApplyRequested(sectionId, token, changedValues)` and `draftCancelled(token)`.
`configurationLoadRequested()` requests the lead's existing Rust OS-native picker
and bounded read. The static Load Configuration secondary footer action is available
without an editable baseline, but disabled during local/host pending or authoritative
`host.upstreamState.busy/application_busy`. It changes no draft/readiness itself;
the host publishes actual acceptance/refresh. All seven draft dialogs forward it through
Workbench. No sample/default document or QML file reader is supplied.

The adapter supplies unique opaque keys, bounded descriptor/choice counts and
labels, authoritative editability/choices, positive integer text/selection
transport bounds and a guarded host envelope. `max_length` must be representable
by Qt TextField's integer `maximumLength`; QML reserves from the actual resulting
control property. No scientific option catalogs or limits are invented here.

| Section | Approved existing public document destinations |
| --- | --- |
| UCD | `selected_preset` String; bound preset's channel CP, rolloff and frame symbols (`usize`), spacing (`u8`), carrier-zero frequency, randomization seed and encompassed bandwidth (`u32`). |
| Profiles | `simulation.selected_iuc` and bound existing burst/profile-group modulation index, pilot pattern and additional minislots (`u8`). |
| Grants | Synthetic `simulation.grant_id`, `epoch_sample`, `first_frame` (`u64`), `frames` (`usize`) and ordered `minislots` (`Vec<usize>`). |
| Source | `simulation.source_seed` (`u16`), via `source_seed`. |
| Receiver | `simulation.estimator` (choice string), `maximum_pilot_gap`, `ldpc_iterations` (`usize`), `reference_variance`, `equalizer_power_floor` (`f64` decimal text). |
| Channel | Existing optional `simulation.channel.delay_samples` (`usize`), `noise_seed` (`[u8; 32]`), `noise_variance`, `phase_radians`, `frequency_offset_hz` (`f64` decimal text). |
| Pre-Eq | Existing optional `simulation.synthetic_pre_eq.initial_id` (`u64`), via `initial_id`; existing full coefficient bank/lineage is inspection-only. |

For `noise_seed`, native descriptors use unique opaque per-byte keys and labels
with semantic field `noise_seed`; QML never concatenates/parses a seed array.
Optional channel/Pre-Eq absence supplies no editable fields or invented defaults.
The native host may still supply actual accepted full-bank/lineage display strings.
Capture/full-bank selection remains US03. Transmitter Configure Source closes its
inspection window and opens the same source editor; it owns no duplicate draft.

All identifiers/integer/decimal edits remain exact strings; there is no scientific Number
conversion, JSON document parse/stringify, range validation, allocation/pilot/timing
reconstruction or accepted-readiness mutation. Only changed fields are emitted;
reverting an edit removes its key. Explicit selected identities retain their order.
The native adapter binds keys to the original preset/burst/group, parses exact Rust
types and merges the full document without retargeting keys or discarding any
unselected preset, null/optional field, original source owner or represented control.

## Lifecycle and presentation ownership

Each real open uses inherited `openChecked()` and requests a fresh draft. Old
host tokens cannot initialize a reopened editor. Exact token/section/base identity
gates all state consumption. Queued Apply freezes editing and stays open until
matching adoption. Matching accepted/cancelled closes; rejected/stale preserves
the visible draft and original plain-text diagnostic. Stale prevents Apply.
Pending Cancel/X/Escape emits the exact token once and awaits matching suppression
or a terminal acceptance that already won the race; no global Stop is called.
Forced external close/destruction requests targeted cancellation once before
clearing local aliases. If local identity admission failed, cleanup transiently
uses only the fresh exact host token for this section; it never retains an
unadmitted token or cancels an unrelated/previous host envelope. The native owner must suppress that late operation.

Nonvisual owners, palette and shortcut are explicit typed properties, avoiding
implicit Dialog content creation. The draft owner admits one coherent identity,
field-wrapper/derived object-name, original-value, changed-value and diagnostic
bundle before persistent replacement. Rejection preserves the old bundle and
disables Apply. Observation aliases plus independently derived row strings have
separate coverage. Optional diagnostic rejection cannot block a matching native
terminal transition. Repeated strings/wrappers are conservatively counted as
logical occurrences, without claiming physical allocation deduplication.

One text-complement owner reserves the combined capacities of all actual loaded
integer/decimal TextFields via `reserveTextFields`; it retains no field references. This
avoids a view owner per profile scalar. Dynamic text and editing remain blank/
disabled until that form's reservation succeeds. An earlier successful reservation
cannot authorize a later larger rejected form. These conservative text capacities
remain reserved until owner disposal, as required by the accepted seam.

Never-opened/closed dialogs do not copy incoming host draft/observation data.
Closed data bundles clear after requesting any needed targeted cancellation;
direct parent aliases remain under their parent's witness. The host still owns
its input DTO; independently retained local wrappers/changes/labels are admitted
here, including old/candidate replacement overlap.

## Focused validation

Final standalone results: **49 draft cases and 13 affected startup cases passed**,
zero failures/skips/warnings. These totals include four init/cleanup cases, giving
58 functional case rows. All 12 owned QML files parsed with Qt 6.8.3 qmlformat;
GPL notices and `git diff --check` passed. Dark/light fixture renders were inspected.
The generated Python integration guide passed its read-only currentness check.
These are Qt presentation fixtures, not native/Rust accounting or scientific
acceptance. No official UIU task is accepted by this preparation.

The existing read-only Qt 6.8.3 GCC64 SDK is used with offscreen/software/Fusion,
`QML_DISABLE_DISK_CACHE=1`. Commands run from this worktree's ignored
`docs/development/local/unified-qt-workbench/us01-draft-qml` directory:

```bash
QT_QPA_PLATFORM=offscreen QT_QUICK_BACKEND=software \
QSG_RHI_BACKEND=software QML_DISABLE_DISK_CACHE=1 \
/home/dev01/Projects/raven/local/qt/6.8.3/gcc_64/bin/qmltestrunner \
  -input /home/dev01/Projects/merlin-worktrees/upstream-qt-ui/apps/qt/tests/upstream/tst_upstream_drafts.qml \
  -style Fusion
```

Run the affected startup check with the same command and
`tst_upstream_workbench.qml` as input. Draft cases cover exact wide identities,
changed-only scalar/selection strings, source immutability, whitelist/opaque keys,
queued/adopted/rejected/stale responses, unrelated identities, cancellation races,
initial pending cancellation, forced-close cleanup, fresh reopen, missing metadata,
byte/entry pressure, combined dynamic text capacities and larger-form rejection,
checked-open rejection, never-opened/closed data, unsupported NaN rejection,
original plain errors, terminal progress under optional-display rejection,
keyboard/focus, scrolling, narrow resize and both themes. Extended cases cover
all four additional scalar sections, exact exponent/negative-zero/nonfinite text
forwarding to native validation, absent optional data and transmitter navigation
without changing either processing-stage list. Startup cases exercise
all existing section routes, reference planes, connector roles and admission guards.

Exploratory failures were resolved without weakening production guards: missing
JSON descriptor members now omit properties rather than retaining `undefined`;
the aggregate text test permits reusing its existing owner; startup title-X lookup
uses the visible header. One test name ending `_data` was renamed so QtTest
executes its intended closed/never-opened lifecycle case. The keyboard fixture uses
QtTest's character-string event API so exact uppercase `NaN` is forwarded unchanged;
its numeric key-code path produced lowercase text. Exploratory logs remain
ignored alongside final logs and bounded dark/light renderer images.

No Cargo/Rust compile/Clippy/tests, native application build/execution, Python,
wheel, full Qt/regression, installed or hosted tests were run. Those gates and
native GUI/accounting behavior remain pending with the lead. No shared caches or
existing targets were changed or removed. Disk before/after rendering: 79% used,
25 GiB available; inode usage 12%.

## Remaining native prerequisites

Reuse accepted `ConfigurationDocument::{from_json,validate,prepare}`,
`Simulation::document`, `UpstreamController::inspect` and existing off-GUI
configuration preparation/adoption. Root owns real-document acquisition for an
unconfigured session; no sample/default document is fabricated here. Root also
owns descriptor projection and bounded host DTO coverage, exact changed-field
parsing/merge, token/base-correlated pending completion and targeted cancellation,
plus QObject/model methods, Main/shell signal wiring and new QML registration.
Wire `configurationLoadRequested()` to `upstreamBackend.choose_configuration()`.
Picker Cancel preserves the visible unsaved draft. As agreed with the lead, the
host may close that section only after an actual queued import, then reopen through
`sectionDialogs/openSection` after preparation settles so a fresh token/base is
requested. Its emitted old-token cancellation must not cancel the separate raw
import. No additional dialog reset API or optimistic refresh is introduced.

A successful terminal envelope must retain the draft's original base identities;
publish the new accepted generation/readiness separately. Rejected retries must
not receive an older submission's terminal response. Preserve imported original
source bytes/hash by typed clone/edit followed by `PreparedSession::from_document`;
do not serialize/reparse the candidate just to apply edits. This follows the lead's
accepted native provenance rule. Typed-only sessions with no retained document remain
honestly unavailable to these editors. Actual public-validation rejection,
configuration adoption, cancellation and preservation of accepted native owners
still require lead integration tests.

## Exact source and artifact receipts

The following SHA-256 values describe the reviewed source/test bytes. The final
commit and this handoff's own digest are reported separately after save.

| Path | SHA-256 |
| --- | --- |
| `apps/qt/qml/upstream/BurstProfilesDialog.qml` | `340a0f8aeef618b7eeea8bd31a83b8b9eb18e907a3c08cff2ac36cd8fd9151e6` |
| `apps/qt/qml/upstream/ChannelDialog.qml` | `92aecbf41bd62be4e4efcd85f19d32351a9bd171300b5762c7dbd314cabe4dae` |
| `apps/qt/qml/upstream/GrantsDialog.qml` | `9780ee4e0da486e923e0ac8a7e1aceee9c5a70a93cc229c036108e607a5aae76` |
| `apps/qt/qml/upstream/PreEqualizationDialog.qml` | `7e5b875e8fec22ddd82957c2a1ab139e9500161e75d0dd982066b9ce9ac80ce3` |
| `apps/qt/qml/upstream/ReceiverDialog.qml` | `b8e4d89eaaed60f1dcac2d6226d9babcde3f3dd5b235aa28772797c2b5cddafa` |
| `apps/qt/qml/upstream/SourceDialog.qml` | `3f1c1e282097e477014d6a68e9886f13289b41a7221bc69d6863b05b0ac71bfa` |
| `apps/qt/qml/upstream/TransmitterDialog.qml` | `c40dbc878fa5559146208104eeec2716415673f833a1c9f73fac0aa92b5466cc` |
| `apps/qt/qml/upstream/UcdConfigurationDialog.qml` | `f7fabcf05e95767cf8ee07d285757da13aab54f8c95e579c353e38684530660b` |
| `apps/qt/qml/upstream/UpstreamConfigurationDraftDialog.qml` | `4620e47ddfe1c446680f743e33e71a6fd55f201b6e7f063869fe46322d550cfc` |
| `apps/qt/qml/upstream/UpstreamWorkbench.qml` | `4e473726496578b0f679e4a5b71868e90fcfe36196368af9fc7fbff763b04d10` |
| `apps/qt/tests/upstream/tst_upstream_drafts.qml` | `fd07bcd2a6d95aa08d076802b6d4183883dc72281cc9159d893502563d3487f7` |
| `apps/qt/tests/upstream/tst_upstream_workbench.qml` | `b19a152f94156589bc4494b2b0ad6a136cb49bc3c259abf7076ac414524fcf4a` |

Read-only SDK and ignored evidence SHA-256 receipts:

- `qmltestrunner`: `636d7a9a050891bb36d918d2159fd621a87e188761390ce88160b5789e223d0b`.
- `qmlformat`: `6a83f4367c098e1f0069ce88483f5dec76817054db8bef7efa63eb14241230fc`.
- `drafts-final.log`: `5998f1362644b4a33529116da8144232a897e7272b42005cb90c583dd7b8b1bb`.
- `startup-final.log`: `2fcc0ea7c8c3672cc399b6b76e325cde720f80d2bf26c161a89c55d567a83445`.
- `upstream-draft-grants-dark.png`: `8cba1e7d75cc3d29edd0f7478e77b8aa6bc5876d85542844e574147f99b6ef22`.
- `upstream-draft-grants-light.png`: `aca76f93773dca30906decef6d4529ba274b61876b1067ccdf7aa0b0ddc179dc`.

Ignored local evidence totals 432 KiB; no cache/target cleanup was performed.
The authorized local save uses `tools/git/git-save.sh --commit-msg`; no push,
merge, publication or release is included. Native US01 acceptance stays open.
