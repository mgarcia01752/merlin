# Runtime Apply transaction preparation

Base `7a554ed39454431b54267d36b9e3c7ee1a00aa5b`;
branch `feature/qt-runtime-apply-transactions`.
UIU-SESSION-01 / REQ-UIU-03, REQ-UIU-06, REQ-UIU-07.
This source unit depends on the lead's new `session::update` and borrowed
snapshot/saved guard contract, absent from this isolated historical base.
ALL compilation, Clippy, Cargo/runtime, Qt/native checks are UNEXECUTED here.
No runtime/session acceptance is inferred.

## Helpers and transaction boundary

All helpers return `Result<(), String>`:

- `SourceModel::apply_transactional(&mut self, &Context, document: &str)`
- `ProfileModel::apply_transactional(&mut self, profile_id: i32, document: &str)`;
  uses the model's existing exact Context.
- `measurement_input::Model::apply_transactional(&mut self, &Context,
  rxmer: bool, options: Options, settings: &TransmissionSettings)`
- `measurement_input::Model::refresh_transactional(&self, &Context)`

SourceModel/its Accepted state, ProfileModel and measurement Model gain Clone
without layout changes. Candidates are finite independent preparation. They use
existing constructors/validators, prepare the matching exported section, call
`session::update` once, then infallibly replace the accepted editor. The update
owns actual complete Inputs/section target admission. Host rejection preserves
accepted editor, scientific Inputs and matching saved section, while retaining
the staged draft. No clone-capacity equality or scientific reconstruction is
used for admission. Existing editor graph admission remains explicitly separate.

Profile selected frame, full catalog and all original profiles publish together.
Measurement Apply validates the selected kind using the existing public playback/
response checks and exact candidate Inputs profile/catalog, then publishes full
playback, response, tolerance and matching measurement section together. Ordinary
source/profile/measurement scientific errors retain their existing diagnostics.
No source, waveform or receiver advances.

Refresh derives its original dependency from ACTUAL current playback presence
versus `profile.is_some() && !accepted_rx.is_empty()`. If equal, it returns before
cloning/public reconstruction or publication. A mismatch republishes derived
playback/response/tolerance in one update; saved/editor state does not change.
No adapter Cell/cache is added, so external Context restoration is observed.
The bridge returns a channel diagnostic on refresh admission failure, keeping
accepted state. Existing model readiness computations remain unchanged.

## Bridge changes and integration

The three owned bridge Apply callers delegate to those helpers. All production
`set_*`/`put_saved` calls are removed from these bridge files. Measurement channel
loss and power each validate as before and publish their complete Inputs field
plus matching saved section in one update. Scientific channel_model owners are
unchanged. Read-only profile uses now borrow the root Snapshot guard; existing
reload calls borrow the saved Value through Deref without a payload clone.
No Context/session/loader/project/registration/Cargo or frozen retention child
is edited. Root integrates the helper contract and runs actual combined callers.

Six focused non-Qt cases are prepared: source exact bytes/matching saved data
and validation rejection; source pressure rollback in all three dimensions;
profile selected/all/catalog/saved removal; profile pressure rollback in all
three dimensions; actual RX/H coupled Apply and no-op/external-presence refresh;
measurement pressure rollback preserving RX/H/tolerance/section in all three
dimensions. Synthetic pressure fills actual remaining usize::MAX host policy;
it supplies no invented scientific byte estimate. Old read guards stay alive.
The original existing model tests/assertions remain unchanged.

Executed: pinned rustfmt parse/format with skip_children for exact owned files,
whitespace/source/allowlist audits, fixture paths, raw-NUL check and current
generated guide check. The source byte fixture uses textual escapes preserving
its null/newline bytes. ALL runtime/compile/Clippy checks remain UNEXECUTED by
explicit delegation. Root owns accepted-state admission integration and expanded
Model/Qt/instrument/project validation. Model graph/draft/candidate/bridge/control/
transport owners and complete aggregate/session acceptance remain open.

Ignored source receipt/hashes:
`docs/development/local/session-shell/runtime-apply-transactions/`.
