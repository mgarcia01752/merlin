# Prepared RxMER export product metadata

Source-only base: `6f2e4e3286ceaab3961811507385c260df22b7a4`.
Branch: `feature/qt-export-product-retention`. This concrete pending product
belongs to [UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07), under the
[approved retained-owner contract](../retention-contract-decision.md).
No task or runtime acceptance is established here.

## Exact files and interface

Only two new source files and this handoff are added:

- `apps/qt/src/bridge/rxmer_export/retention.rs`
- `apps/qt/src/bridge/rxmer_export/retention/tests.rs`

The unregistered owner-local module exports:

```rust
pub(super) fn resources(
    prepared: &super::Prepared,
) -> Result<crate::shell::RetainedResources, String>;
```

It exhaustively audits `frames`, `policy`, `context`, `summary`, `json_summary`,
`pnm_error`, `diagnostic` and `collection`. It counts `size_of::<Prepared>()` once,
the actual frames Vec capacity including spare slots, and each live frame's
existing `inputs::retention::rxmer_frame_children`. The four inline Value owners
reuse `json_children`; only their dynamic children are added. Optional error
capacity is counted separately. RangePolicy is exhaustively matched and both it
and collection pass the existing Copy-constrained fixed-leaf audit. Fixed frame
and Value handles are already inside slots or Prepared and are not counted twice.
There are no shared payload roots; ordered JSON map entries follow the existing
leaf contract. No serialization, frame rebuilding, scientific query, capacity
normalization or mutation occurs in the visitor.

## Lead integration and remaining owners

Register `mod retention;` in `bridge/rxmer_export.rs` and use the complete actual
product metadata when admitting retained Prepared ownership. The lead owns the
concrete Retained/Outcome/State/queue wrapper adjustment and admission boundary,
data-before-guard drop order, old/queued product overlap and cancellation/disposal.
The passive visitor adds no reservation or pre-production bound and does not
change the existing finite preparation boundary. Preserve the previous Ready
product on rejected preparation or failed dispatch. Error/control/request,
returned save-reply JSON, channel, native picker/filesystem and Qt presentation
owners remain separate consumers of the same aggregate contract. This unit
does not change publication paths, scientific APIs or export behavior.

## Prepared tests and local evidence

Two source tests are included, both **UNEXECUTED**:

- A genuine existing RxMER JSON is publicly decoded with the same explicit null
  and 42.75 dB anchors used by existing Qt input coverage. Public PNM/JSON preflight
  creates real summaries; independent display metadata exercises nested context
  arrays and diagnostics. A direct fixed-layout/capacity oracle checks every
  containing value once, complete map population, zero roots and repeated-query
  passivity, including original keys, nulls, values and buffer identities.
- Independent spare frame/readings slots, all provenance Strings, context array/
  text, each JSON owner's map key and the diagnostic/error text change the exact
  measured capacity delta without changing their values. Optional error absence
  removes only its buffer; fixed policy/collection changes add no dynamic storage.

Pinned rustfmt and direct source/allowlist/fixture/link/diff checks are the only
local validation. Compilation, Clippy and every Rust/Qt/QML/native runtime check
remain **UNEXECUTED**, lead-owned. No existing source, registration, shared leaf,
manifest, canonical document, build artifact or cache is changed.

Focused local save:

```bash
cd /home/dev01/Projects/merlin-worktrees/qt-session-shell &&
tools/git/git-save.sh --commit-msg "Feature: Measure prepared RxMER export storage
- Reuse actual frame and JSON child metadata
- Preserve immutable export product values
- Tests: source checks; runtime unexecuted"
```
