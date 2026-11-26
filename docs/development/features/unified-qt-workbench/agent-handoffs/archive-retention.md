# UIU-SESSION-01 detached archive retained-target preparation

Source-only preparation from `71505d97c0c2bf430942d0a8b3a5e20b2709fe61` on
`feature/qt-retention-archive`; input `91d1e9b`, downstream `20f4342` and original
session `665d758` branches remain preserved. The lead owns
[the design](../design.md#pre-production-detached-archive-bounds),
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07). No official runtime, native
or aggregate-retention acceptance follows from this preparation.

## Interface and exact target

Only two public methods are added to existing FullLoopResult:

```rust
receiver_archive_retained_bound(
    &self, policy: &FullLoopReceiverArchivePolicy,
) -> ApiResult<merlin_api::RetainedFootprint>

receiver_archive_bounded(
    &self, policy: FullLoopReceiverArchivePolicy,
    limit: merlin_api::RetainedFootprint,
) -> ApiResult<FullLoopReceiverArchive>
```

Both select the same first actual nonempty LDPC-input event and original floor
cell indices as receiver_archive. Preflight preserves original logical-byte,
protected-selection and decoder-representation error order. The bounded copy
recomputes the complete requirement and rejects any insufficient dimension
before constructing payloads. It does not itself reserve an application budget.

Final payload bytes equal checked historical retained_bytes plus
size_of::<FullLoopReceiverArchive>(); the legacy statistic already includes the
fixed Retained value and copied payload/context lengths. The target has zero
ordered entries and exactly one Retained Arc edge. Every final vector, including
nested payload/NCP LLRs, decoder prefix and context roles, is normalized through
Box<[T]> into Vec. Context strings go through Box<str> into String. Thus final
capacities equal copied lengths for the actual nonzero-sized payload types.
No source spare capacity is copied into the target.

All accessible copied records, selections and context/grid headers use exhaustive
field patterns. ProtectedConstellation's private outer pilot_sequence cannot be
exhaustively destructured from this owner under the allowed scope. The lead
explicitly approved reusing its existing constructor and auditing every current
public field/stream: the live cache remains absent, matching the historical copy.
WaveformGeometry is an existing fixed TransmissionSettings owner, cloned without
heap children. Future private ownership changes require another owner audit;
exact metadata/bound regression cases are prepared here.

## Compatibility and construction boundary

Existing receiver_archive and info().retained_bytes semantics remain unchanged.
mod.rs adds only module registration. copy.rs changes only selected/bytes sibling
visibility and documentation; every legacy function body is byte-for-byte intact.
No original algorithms, source records, statuses, selectors, defaults, schemas,
scientific values or query restrictions change. No unsafe or new shared type.

The caller keeps the live source and all old archives charged while reserving the
new target before copying, and retains that target charge through aliases/queues.
Temporary collect/shrink builders and copied query DTOs are separate ownership.
Construction uses the existing finite policy fields and their named ceilings:
MAX_RECEIVER_ARCHIVE_PAYLOAD_CELLS, MAX_RECEIVER_ARCHIVE_NCP_CELLS,
MAX_RECEIVER_ARCHIVE_PROTECTED_CELLS, MAX_RECEIVER_ARCHIVE_DECODER_LLRS and
MAX_RECEIVER_ARCHIVE_BYTES. Selected nested LLR/role/UTF-8 byte lengths participate
in the historical byte preflight. No allocator growth multiplier, scratch-byte
promise, fallible-allocator behavior or total preparation-peak bound is invented.
Future full-run output bounds and actual host admission remain separate gates.

## Prepared evidence and remaining work

All nine new cases are **UNEXECUTED**, including compilation and Clippy. Seven
private cases in bounded/tests.rs cover:

1. Exact final capacity/metadata for every copied buffer and literal floor indices,
   with nested LLRs, Unicode/context-role spares and historical info compatibility.
2. Exact and one-extra-required-unit admission for each dimension; the ordered-entry
   case is an independent predicate anchor, not a claim that archives own entries.
3. The empty target's required shared root, one-byte rejection and old alias/source
   preservation after rejection.
4. Omitted roles/decoder with original Some headers and zero final buffer capacities.
5. Original protected-selection and wrong-decoder-representation errors.
6. Checked fixed-handle addition overflow before copy.
7. Skipping empty/non-LLR events without relabeling the selected original context.

Two public cases in api_receiver_archive_bounded.rs use the established supported
small real simulation fixture: complete source/target metadata, actual private
pilot-cache omission, all six QAM/decision/soft query planes and decoder parity,
pre-copy byte/root rejection, preserved source/old aliases and detached lifetime;
and a real pre-cancelled partial result with exact zero-entry omitted retention.
Private fixtures are copy/storage anchors, not scientific acceptance.

Executed here: pinned rustfmt parse/format check, whitespace/allowlist/source
review, exhaustive visible field audit, unchanged-legacy-body audit, admission
ordering/banner/import/comment/numeric review and Python-guide currency. No
Cargo/shared-target/wheel/full-suite, runtime or Clippy check was executed. Disk
remains 80% used, 25 GiB free and 13% inodes; no targets/caches were cleaned.

The exact file/hash manifest, audit and source-only receipt are in ignored
`docs/development/local/session-shell/archive-retention/` (`source-sha256.txt`,
`review.txt`, `receipt.json`). Adopt only this archive owner/test/handoff unit;
the lead owns primary compilation, complete affected tests/Clippy, paired native
operations/parity, inventories and canonical acceptance evidence. No Qt, shared
API leaf, manifest, lock, native adapter or canonical document is edited here.
No push, release or installation occurs; the verified SHA/clean result is reported
separately after the focused helper save.
