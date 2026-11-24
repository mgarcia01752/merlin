# Designer Session admission preparation

Source-only base: `25e6d26019bd90e1036353559a7f2c2f63446736`.
Branch: `feature/qt-designer-session-admission`. Ownership remains with
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01) and
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06),
[REQ-UIU-07](../../../../requirements.md#req-uiu-07). The
[editor design](../design.md#uiu-session-01-retained-editor-candidates-and-prepared-transfer)
keeps Session/worker/export admission separate from measurement editor guards.
No official task is accepted by this preparation.

## Exact source and interface

Only `apps/qt/src/downstream/inputs/measurement_input/designer.rs`, its existing
`designer/retention.rs`, `designer/retention/tests.rs`, new
`designer/admission_tests.rs` and this handoff change.

```rust
Session::retained_resources(&self) -> Result<RetainedResources, String>
Session::admit(self, &RetentionBudget) -> Result<Retained<Session>, String>
Session::clone_admitted(&self, &RetentionBudget)
    -> Result<Retained<Session>, String>
```

The existing exhaustive Session visitor is enabled in production. It counts the
Session fixed value once; hash Vec/String capacities; the shared profile's fixed
pointee, dynamic children and root; complete original catalog metadata; Origin
Vec storage and every independent Record/design original/cache/recipe child;
and the Request's source capacity and recipe children. Existing Origin and record
walkers, scientific bodies, numerical assertions and original fixtures are
unchanged. Shared payloads are conservatively repeated by independent owners.

Admission delegates only to existing `inputs::retention::admit_model`, including
its actual queued/GUI storage-layout adjustment. The existing Retained field order
drops Session data before releasing its guard. No new Session Arc wrapper or root
is introduced. `clone_admitted` makes one finite temporary clone, measures that
actual clone, then admits it before returning retained ownership. It never predicts
Clone capacities from source metadata. Rejection drops the candidate and preserves
the independently charged source and all its values/reservations.

## Lead caller hooks and remaining owners

Store `Session::begin(...)?.admit(canonical_budget)?` only after success, preserving
the old Session on error. A future worker copy must use
`session.value().clone_admitted(canonical_budget)?` and move the returned Retained
owner through dispatch/queue/disposal. Borrow its `.value()` for existing preview,
build or export operations; do not detach shared children and drop their last
covering owner. Old and current Session copies keep their own guards until final
drop, including when their immutable profile/catalog/design payloads overlap.

The existing finite input and RxMerDesign capture/value/recipe limits govern
temporary Session construction and clone preparation. This unit adds no temporary
byte or preparation-peak bound. It does not wire `bridge/rxmer_designer.rs`, admit
worker Work/request/hash/profile-hash/options/control/channel infrastructure, or
bound build/output/export/Qt JSON targets. Those are remaining concrete consumers,
not exclusions from the aggregate contract. The lead owns their transactional
integration and runtime acceptance.

## Source-prepared checks

Three new tests are prepared, **not executed**:

- Independent hash slots/text, Origin slots, source text, Request/Record edit slots
  and imported original JSON text capacities change only the measured payload
  delta. Original bytes, Request/Record values and public profile/catalog metadata
  remain identical. The original flat-mask/imported-recipe passive case is preserved.
- Source and actual admitted clone coexist with separately measured layout/child
  charges; shared profile identity survives movement to an old owner and current
  disposal. Final Session drop releases the final covering charge; Send is checked.
- All three positive dimensions reject worker-copy admission under actual remaining
  host pressure, preserving source bytes, hashes, Request, metadata, Arc count and
  ledger usage. No estimated scientific allowance or clone-capacity equality is used.

Only pinned rustfmt and direct source/diff/allowlist checks run here. Compilation,
Clippy and all Rust/Qt/QML/native runtime tests are **UNEXECUTED**, lead-owned.
No shared cache, build, manifest, bridge, model or canonical document is modified.

Focused local save:

```bash
cd /home/dev01/Projects/merlin-worktrees/qt-session-shell &&
tools/git/git-save.sh --commit-msg "Feature: Admit immutable Designer sessions
- Measure and retain actual Session copy ownership
- Preserve prior sessions on host rejection
- Tests: source checks; runtime unexecuted"
```
