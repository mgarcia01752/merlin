# C5 receiver presentation consumers

Source-only UIU-SESSION-01 C5 / REQ-UIU-07 preparation under the lead-owned
[design](../design.md#uiu-session-01-c5-existing-persistent-presentation-owners).
Exact base: `6b32dcfc401929f087c9871933c088a77ac94ec8`.
No runtime acceptance or C5/SESSION completion is claimed.

## Scope and shared seam

The exact source owners are ReceiverQamDialog, ReceiverResponseDialog,
ReceiverSlicerDialog, ProfilePerformanceDialog, ReceiverFecDialog, FecControlTable
and FecCodewordTable, plus this handoff. The lead supplies PresentationOwner and
`theme.presentationBackendFactory`; its current source is inspected read-only and
is not copied into this branch. Each of the five dialogs uses one compound plain
candidate: original DTO(s), original scalar query context and independently
retained selector/display strings. Admission overlaps old/candidate ownership.
Failure keeps the old whole view, restores its requested selectors and shows the
shared bounded static feedback. Hidden/closed data retains its witness. Each
nonvisual owner is an explicit typed property with a distinct property name/id;
it does not enter Dialog contentData or create an orphan default contentItem.

- QAM: exact selection, original points and correction/MER metadata, admitted
  panel-title array and display strings. Existing public selection aliases and
  rawOnly remain. Rejected selection restoration suppresses intermediate queries.
- Response: exact source/snapshot/view/VF query, original response/comparison and
  admitted source/view/snapshot labels and MCE text. Existing missing-result
  labels can update from settings without an additional scientific query. The
  actual VF SpinBox TextInput maximumLength is admitted at completion/retried on
  open; enabled/editable/dynamic text are gated by textAdmitted. No validator or
  domain limit changes. Its separate text complement lasts until owner disposal.
  SpectrumPlots load only after initial admission because the unchanged shared
  plot dereferences points directly; successful layout/points remain unchanged.
- Slicer: original slicer plus optional QAM-metadata responses, exact query/tab,
  admitted cell/decoder selectors, precision-formatted decision text and both
  near/full summaries. Symbol/Confidence switching selects the same scientific
  data with admitted display context and no extra backend query; Decoder Input
  transitions query as before. Failed refresh preserves selected cell/tab/filter.
- Performance: original viewport/profile/range, selector labels, counter strings
  and existing runtime-status wording. Status-only updates reuse the accepted
  viewport without querying; failure preserves it. Status-only replacements keep
  the accepted query token, as do Response/Slicer display-only updates.
- FEC: original statistics plus profiles/counters/error text and BOTH converted
  table projections form ONE candidate, so overview/table/profile cannot publish
  partially on rejection. All original chart values, words, order and nulls stay
  in the unmodified statistics DTO.

FecControlTable now accepts a `projection` alias and exposes pure `prepare(stats)`.
FecCodewordTable similarly accepts `projection` and prepares original headings,
summary, all rows and Errors Only rows before the parent admits them. All converted
cell strings and both independent arrays are counted in the parent compound
candidate before Repeater/ListView conversion. Errors Only switches admitted
aliases without allocating a retained filter. The original error predicate,
formatting and row order remain. `filtered` now aliases converted row arrays;
internal raw statistics/words/omitted component inputs were replaced by projection.
Tables do not create independent reservations or duplicate scientific data.

## Lead integration and remaining owners

Supply the shared import/native factory. These seven files contain no dialog-open
invocation; ReceiverWorkbench's existing callers must use openChecked (lead-owned).
DTO properties are readonly admitted aliases, so direct-assignment fixtures should
use an actual backend response and refresh. QAM/Slicer/FEC admit their initial
empty candidate after existing controls/tables have been constructed; other
initial defaults use the shared consumed initialValue seam.

ReceiverCellControls selector/text complements, shared plots/chart-derived strings
and readouts, open-slot implementation/callers, registration and tests are owned by
the lead and remain outside this isolated unit. These are integration dependencies,
not exclusions from the overall C5 contract. Scientific algorithms/APIs, control
operations, original query arguments, saved behavior and numerical limits remain
unchanged. Canonical architecture/checklist are lead-owned and remain unchecked.

## Source evidence

All seven QML files parse with exit 0 and empty stderr via the existing read-only
Qt 6.8.3 qmlformat --ignore-settings FILE. This checks syntax only: imports/types
and actual runtime behavior are UNEXECUTED. Exact eight-path, raw-NUL, SPDX, source
body/formatting and staged/unstaged/untracked checks are recorded in the ignored
receipt directory:
`docs/development/local/unified-qt-workbench/receiver-presentation-6b32dcf/`.
It contains parser commands/results, source hashes, reviewed diff and final commit
receipt. No Cargo/runtime/Clippy/native/build/wheel/full-suite checks run here.
Lead owns actual success/rejection/rollback/filter/tab/text/open-slot tests and
unchanged numerical/JSON values, Qt/import validation and final C5 acceptance.
