# C5 configuration presentation consumers

Source-only UIU-SESSION-01 C5 / REQ-UIU-07 preparation under the lead-owned
[design](../design.md#uiu-session-01-c5-existing-persistent-presentation-owners).
Exact parent: `6b32dcfc401929f087c9871933c088a77ac94ec8`.
C5/SESSION and actual runtime acceptance remain open.

## Exact scope and shared contract

Six existing files only: WorkbenchConfigurationDialog, OfdmSettingsDialog,
SourceInputDialog, ProfileInputDialog, ProfilePlotDialog, ProfileBandwidthDialog;
this handoff is the seventh path. Each uses an explicit typed PresentationOwner
property (distinct property name/id), avoiding Dialog implicit contentData. The
lead supplies the read-only shared helper/native factory/openChecked seam. No
shared source is copied, and no scientific/public API, schema or algorithm changes.

Whole native DTO/draft/context and independently retained derived arrays/strings
are admitted before persistent assignment/model conversion. Old/candidate charges
overlap; failed replacement keeps the prior coherent view and bounded static
feedback. Hidden data stays charged. Readonly aliases share the parent witness.

- Workbench configuration: native options/catalog plus selected IDs, checkbox
  labels/counts and errors. Numerical edits use already covered fixed slots;
  adding an absent key requires complete candidate admission. Selected arrays are
  admitted before publication. Unchanged catalog models retain their identity.
- OCD: original registry/enum choices/draft/profile summary, derived group/row
  arrays, descriptions/default/source/accessibility text and errors. Ordinary
  edits/description/error updates reuse the registry models to preserve focus.
  Row/choice actions require actual current registry membership. No enum values
  or native default semantics change.
- Source: original options/file metadata/describe counts plus help/errors and
  converted byte/bit/fill strings. The original Number(text) conversion may yield
  nonfinite invalid drafts: only those use their same String(value) display form
  inside the plain candidate. Transient describe/apply wire construction restores
  Number for those numeric fields, so JSON still sends null for NaN/Infinity and
  native validation remains authoritative. Finite options/bytes/hash are unchanged.
- Profile input: original preview/catalog, collapsed-key map, independent visible
  row wrappers/labels/accessibility/descriptions and errors. Native staged-input
  changes publish only after complete candidate admission; stale row/ID actions
  are rejected. Unchanged tree models are reused for description/error updates.
- Profile plot: original inspect viewport and exact profile/role/metric/range
  query plus admitted selected arrays, reversed toggle rows, role tooltip strings,
  identity/BW text and finite original dark/light palettes. Plot bindings alias
  original numerical data. Selection arrays are temporary until viewport admission;
  rejection restores the metric/toggle state. No profile/role/metric enums change.
- Bandwidth: one coherent parent-admitted profiles/selected-IDs context, then its
  own admitted snapshot/row labels/highlights. Failed context replacement preserves
  the entire old table. Its app-private inputs are now one context property,
  replacing separate profiles/selectedIds bindings to prevent mixed snapshots.

## Native synchronization and actions

Source/Profile keep a separate contextCurrent flag after rejected native
begin/choose_file/preview/clear output. Cached option/help/selection admission
cannot restore it. Only actual native metadata reload/adoption restores it;
transactional native cancellation/error may preserve an already current context.
Apply and indexed native actions stay disabled while unsynchronized, with static
reload feedback even if a later help update clears the owner's rejected flag.
Cancel still calls the existing native cancel method and invalidates this flag.

ProfilePlot receives sourceCurrent from ProfileInput. Unsynchronized source or
catalog blocks indexed/range queries; actual fresh open metadata admission
restores local readiness. Controls are gated individually, preserving Close.
ProfilePlot closes its nested bandwidth popup when closed. Owned open calls now
use openChecked. Root owns Workbench/root/shared open callers and all registration.

Real native Apply success emits the original success signal and closes immediately
based on its actual return value, independently of optional detail admission.
Native Apply errors are admitted while sources stay held; no guessed error is
substituted. Apply/Cancel ordering, original request conversions and validators
remain unchanged. Existing edited TextFields and numeric SpinBox contentItems use
reserveTextFields with their actual maximumLength; fields/controls/dynamic text
are gated until admitted and retried on open. No maximumLength or domain limit
changes. Text coverage persists until disposal independently of DTO clear/hide.

## Source evidence and lead hooks

The native factory/shared helper must be integrated by the lead. DTO properties
are now readonly admitted aliases; direct-assignment fixtures should use actual
backend begin/preview/refresh paths. ProfilePlot's app-private selected arrays are
readonly; its refresh accepts optional temporary profile/role arrays. Parent
Workbench/root guards, shared chart readouts, globals and native state acceptance
remain lead-owned and are not excluded from the full C5 contract.

All six QML files pass syntax parsing with the existing read-only Qt 6.8.3
qmlformat --ignore-settings FILE (exit 0, empty stderr). Exact seven-path,
SPDX/raw-NUL/direct-assignment/open-routing/typed-owner and diff checks are recorded
with source hashes and commit receipt under ignored
`docs/development/local/unified-qt-workbench/configuration-presentation-6b32dcf/`.
Import/type/runtime/Cargo/Clippy/build/native/wheel/full-suite checks are UNEXECUTED.
Lead owns actual success/rejection/stale-mutator/text/focus/theme/Apply/Cancel
checks, original numerical/JSON compatibility and final C5 acceptance.
