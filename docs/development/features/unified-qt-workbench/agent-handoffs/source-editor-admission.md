# Source editor retained-owner admission

Source-only preparation from exact base
`1921ba95dd2a93caa40540535f9a5189af0d490c`. The source passive visitor/test pair
is selected from frozen `6d20fa7a3a6b05398b766529531ef3e7fb0b70a1`;
other frozen visitor/preview branches remain unchanged.

[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07) own this concrete consumer
under [accepted-input ownership](../design.md#uiu-session-01-atomic-accepted-inputs-and-prepared-projects).
No task, runtime or aggregate SESSION acceptance is established here.

## Exact files and interfaces

- `apps/qt/src/downstream/inputs/source_input/model.rs`: register the passive
  child; add `retained_resources(&self) -> Result<RetainedResources, String>` and
  `admit(self, &RetentionBudget) -> Result<Retained<Self>, String>`.
- The selected `model/retention.rs` and `model/retention/tests.rs` count actual
  source file/configuration children, repeated shared payloads and mode capacity;
  their exhaustive model patterns and original assertions remain unchanged.
- `begin_admitted(&self, &RetentionBudget)` returns a retained cloned candidate
  and existing begin metadata. `stage_admitted(&self, &RetentionBudget, String,
  Vec<u8>)` returns the admitted exact-byte staged candidate. Original begin,
  stage, validation and scientific operations retain their existing bodies.
- `apply_transactional(&mut self, &Context, &str)` now returns
  `Result<RetentionReservation, String>`. It accepts a finite candidate, admits
  its full actual model, then performs the existing coupled Inputs/saved-section
  update. Retained owns the candidate through every fallible step. Only after
  success does it replace the editor and return its still-live reservation.
- `apps/qt/src/bridge/source_input.rs` keeps `model_reservation:
  Option<RetentionReservation>` immediately after model. Begin, stage, cancel and
  fallback reload prepare/admit before replacing model and guard. Apply stores
  the returned guard before returning to Qt. A prepared reload validates exact
  budget ownership, transfers existing parts and never re-admits the payload.
- `model/admission_tests.rs` prepares four new real model cases. One additional
  bridge-local case tests the actual private prepared-transfer helper without
  calling a native picker or QObject invokable.

## Shared integration dependencies owned by the lead

This preparation intentionally refers to lead-owned hooks absent from its base:

```rust
Retained::into_parts(self) -> (T, RetentionReservation)
inputs::retention::admit_model<T>(
    model: T,
    resources: RetainedResources,
    budget: &RetentionBudget,
) -> Result<Retained<T>, String>
PreparedModels.source: Option<Retained<SourceModel>>
```

The shared admission hook replaces the visitor's once-counted `size_of::<T>()`
with `max(size_of::<Retained<T>>(),
size_of::<(T, Option<RetentionReservation>)>())`, checked. The tuple preserves
padding between the actual model/guard fields. The two actual layouts are
queued retained data and the adapter model followed by its optional guard.
No wrapper arithmetic is duplicated here, no shared root is invented, and no
clone-capacity equality or multiplier is assumed. Root queued preparation calls
`source.admit(budget)` and checks ownership before accepted project adoption.

Both runtime replacement paths drop old model data before replacing/releasing
its old reservation. The reservation's own budget handle keeps its accounting
alive independently of Context destruction. Current/queued/admitted candidates
coexist and stay charged until their actual final drops. Temporary bounded source
construction and returned JSON/QString/native-path data remain separately scoped;
this source-editor unit does not close the outer Context/CXX/transport/accounting
graph or Designer Session/worker/export owners.

Legacy non-Qt fixtures may immediately drop the returned Apply guard as explicitly
authorized for those existing fixtures. The production adapter always stores it.
No new framework, public scientific API, persisted schema, default, algorithm,
source-byte interpretation, manifest or registration change is introduced.

## Error and caller behavior

Admission/validation rejection preserves the previous staged and accepted model,
its old reservation and coupled accepted Inputs/saved source section. Begin and
file selection return their existing JSON-error style; Apply returns existing
plain error text. Cancel and reload preserve state and log a warning because their
existing invokables return void. Native chooser cancellation makes no model change.

One exact lead-owned QML hook is required: base SourceInputDialog.qml onOpened
reads `initial.options`/`initial.file` unconditionally and clears its error message.
Handle `initial.error` before those assignments/refresh so a rejected begin stays
visible and cannot install undefined draft values. QML is outside this source
unit's allowlist and is unchanged here. The lead reports this consumer fixed and
targeted QML checks executed in primary; that evidence is outside this source commit.

## Prepared cases and source evidence

Five new admission cases plus the selected passive case are prepared:

1. Actual old and staged candidates coexist; into_parts preserves their charge;
   data drops before the transferred reservation's final release.
2. Positive model-dimension pressure rejects begin, stage and fallback reload,
   preserving the previous draft, accepted model, saved section and old charge.
3. Apply rejects each of payload/ordered-entry/shared-root pressure. File models
   have zero ordered entries, so entry-only pressure specifically reaches the
   coupled accepted-state update after model admission and rolls it back.
4. Successful Apply preserves original null/newline bytes and keeps the separate
   model and accepted snapshot charges through their respective final drops.
5. Actual bridge prepared transfer rejects a foreign host; same-host transfer
   succeeds under full pressure without a second reservation and releases old data.
6. Passive source capacity/alias coverage is retained byte-identically from 6d.

Executed here: pinned direct rustfmt/check, complete path/staged/untracked review,
raw-NUL/license/body-preservation/field-drop-order/source-caller audits, generated
Python guide check, focused helper save and clean/SHA verification. **Compilation,
Clippy, all runtime cases, Qt/QML/native and broad regressions are UNEXECUTED**.
The lead owns shared-hook adoption, QML error handling, compilation and actual
consumer/runtime acceptance. No build or shared cache was touched here.
