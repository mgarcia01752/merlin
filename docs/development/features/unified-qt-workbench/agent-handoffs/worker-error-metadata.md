# Worker AdmissionError metadata preparation

Source-only base: `5edb05bca9d3b460f8f3e0db6895c4f10576e2d1`.
Branch: `feature/qt-worker-error-metadata`. This private owner supports
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07), under the
[approved retention contract](../retention-contract-decision.md).
No official acceptance is established by source preparation.
Primary adoption/runtime now links the
[worker-error validation](../validation.md#uiu-session-01-retained-worker-error-completions);
source-only execution statements below describe the frozen preparation.

Only `apps/qt/src/downstream/help.rs`, new `help/retention.rs`, new
`help/retention_tests.rs` and this handoff change. The application-private hook is:

```rust
AdmissionError::retained_resources(&self) -> Result<RetainedResources, String>
```

It delegates to owner-local `retention::resources`. Exhaustive AdmissionError
and Diagnostic patterns count the error fixed layout once, actual message
capacity, and optional Diagnostic fixed pointee once plus its code/title/tooltip/
faq_anchor capacities. The inline optional Box handle is already in the error
layout. All five numeric Diagnostic fields pass the existing Copy-constrained
leaf audit. An exclusive Box adds no shared roots or ordered entries. Existing
checked append is reused; no production clone, serialization, getter-based
reconstruction or scientific operation is introduced.

The lead can pass these actual root quantities to existing Retained/admit_model
for a detailed worker failure. Any surrounding fixed envelope/root adjustment,
guard lifetime, payload-free rejection fallback, bounded queue and reply behavior
remain lead-owned. Count the error fixed value once when composing a containing
owner. This passive operation adds no admission or future-error allocation bound.
Original error creation, wording, Display, help lookup and assertions are unchanged.

Two focused tests are prepared, both **UNEXECUTED**:

- A real registered export Diagnostic checks independent actual error/Box layout
  and every private String capacity. Message and four diagnostic buffer growth
  deltas preserve original message/guidance/numeric values and Box identity.
  Serialization is used only to compare values, never to infer byte quantities.
- Errors without Diagnostic count only their root and actual message buffer;
  an empty message still retains the fixed error value. Ordered entries and
  shared roots remain zero rather than inventing positive admission dimensions.

Only pinned rustfmt and direct source/diff/allowlist/link checks run locally.
Compilation, Clippy and all Rust/Qt/QML/native runtime checks remain
**UNEXECUTED**, lead-owned. No bridge, public scientific API, manifest, canonical
document, build artifact or shared cache is changed. Prior frozen branches remain
preserved.

Focused local save:

```bash
cd /home/dev01/Projects/merlin-worktrees/qt-session-shell &&
tools/git/git-save.sh --commit-msg "Feature: Measure retained worker error payloads
- Count private diagnostic storage at its owner
- Preserve original error text and help behavior
- Tests: source checks; runtime unexecuted"
```
