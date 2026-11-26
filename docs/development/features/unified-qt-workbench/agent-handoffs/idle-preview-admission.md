# Idle transmitter preview admission preparation

Source-only descendant of frozen observation `ccc85da07838b8301b3ccd925ce20027aad21b25`
on `feature/qt-idle-preview-admission`. UIU-SESSION-01 / REQ-UIU-03/06/07 owns this
partial application consumer; no runtime, aggregate or SESSION acceptance here.

## Concrete owners and interface

PreparedPreview retains accepted TransmissionConfiguration plus optional actual
TransmitterPreviewRequest and PhysicalLayoutView. The unused configured layout
key/provenance wrapper drops after existing finite layout preparation. Share
`Arc<Retained<Arc<PreparedPreview>>>` with the existing worker: no deep request
clone occurs at Generate. The two actual shared roots and complete retained
wrapper layout are charged before either final Arc allocation.

Opaque request children use public retained_footprint, replacing its inline root
with the actual PreparedPreview layout (including Option/padding/timing/layout).
TransmissionConfiguration's audited owner in api/admission.rs contains only
TransmissionSettings; its exhaustive public fields are fixed Copy leaves, also
confirmed by the existing backend visitor. Public PhysicalLayoutView owns only
carriers and role_spans Vec payloads, counted at actual current capacities times
their fixed public element layouts. Current carrier/span fields are audited Copy
leaves. These externally non_exhaustive types and the private configuration owner
require re-auditing if new owned children are introduced; no private getters or
new scientific operations are added.

The worker calls existing preview_retained_bound, reserves its full target plus
the checked Retained<TransmitterPreview> fixed-wrapper delta, then uses
existing generate_bounded. Preflight runs off the GUI thread and before backend
control claim/source progress. No post-science oversized rejection. The same
retained product moves through Box queue, current slot, stale/cancel/disconnect
and final drop. Old products/configuration aliases remain independently charged.
Retention failure preserves accepted source/configuration, revision, active job
and result; ordinary invalid-input invalidation keeps legacy behavior.

`Analyzer::rebind_prepared(Context) -> Result<(), PreviewError>` is the mandatory
lead integration hook for bridge/transmitter_analyzer.rs project-load adoption:
replace the direct `model.context = ...` assignment before replacing the live
model. The destination coverage is reserved before aliasing the identical inner
Arc; rejection changes neither context nor accepted state. Only the actual idle
prepared transfer is supported; active or result-bearing transfer rejects.
Root owns this one bridge callsite, all registration and acceptance. Do not use
direct context assignment for an already prepared owner after adoption.

## Checks and remaining owners

Executed: pinned recursive rustfmt parse/check; git diff --check; guide check;
exact allowlist and frozen-source audit; unchanged original three test-body and
cancel/poll/scientific-query value-equivalence review. Compilation, Clippy,
ALL Cargo/Rust runtime cases, Qt/QML, builds, native/wheel and acceptance checks
are UNEXECUTED here by assignment. The original three readiness/cancel/stale,
timing/velocity and actual spectrum/science assertion bodies are preserved.

Seven added cases prepare: actual layout spare-capacity/fixed-option delta and
checked product-layout overflow; exact prepared admission/all positive dimension
rejection/final alias; cross-budget same-root alias/pressure lifetime; future
product rejection before public control claim plus real pre-cancel; accepted
configuration/result/revision preservation under labelled synthetic pressure;
genuine Box queue/final drop and stale disposal; transactional idle rebind and
busy/result-bearing rejection. Zero ordered entries, where returned by the real
fixture, are not turned into fictitious scientific charges to invent a test.

Temporary candidate/layout/bound/waveform/PSD builders retain existing finite
input, FFT/geometry, Welch-window/source and cancellation limits; no preparation
peak, allocator multiplier, RSS or scratch aggregate claim. Outer Analyzer/Job,
control/thread/channel/closure handles, diagnostic Strings, Context/accounting
metadata graphs (including budget Arc edges), info/calibrated-spectrum/timing/
viewport/JSON/bridge copies and independently escaping aliases remain distinct
consumers. This closes only the concrete prepared source and backend product.
The existing shared retention primitive is unchanged.

Exact commands/status, source SHA256 inventory and save log are in ignored
`docs/development/local/session-shell/idle-preview-admission/`. Root must wire
rebind then compile and execute affected real preview/lifecycle/project/UI tests.
Frozen observation/result/request/worker/shared/scientific sources are untouched.
