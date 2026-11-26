# Detached receiver archive admission preparation

Source base: `0fe979fe3da6bff0c691526841c767cf66260212`.
Branch: `feature/qt-receiver-archive-admission`.
Owner: UIU-SESSION-01 / REQ-UIU-03, REQ-UIU-06, REQ-UIU-07.
This is source-only preparation; no official task/runtime acceptance is claimed.

## Concrete owner and integration

`qam_history::retain` uses the existing canonical
`context.application.budget()`. Before producing a detached history,
`qam_history::retention::snapshot` queries the existing public
`receiver_archive_retained_bound`, reserves all three dimensions, then calls
`receiver_archive_bounded`. It moves the resulting archive into `Arc<Snapshot>`;
no scientific clone remains. Snapshot declares its `Retained<()>` guard last,
so archive data drops before the reservation. Every registry/current/old Arc
alias keeps that reservation through final disposal.

The checked payload is the public bound minus
`size_of::<FullLoopReceiverArchive>()` plus `size_of::<Snapshot>()`.
The actual Snapshot layout includes its guard and padding. Ordered entries
pass through unchanged (the current backend target has zero); shared roots
are the backend bound plus the one actual Snapshot Arc. No allocator,
refcount-header or growth-factor estimate is introduced. The independently
charged source remains alive throughout copying. Existing public copy policy,
historical `info().retained_bytes`, scientific selections and query errors stay
unchanged.

A new host-admission failure leaves the accepted entry status/snapshot untouched.
Original backend failures retain the prior unavailable-history/status behavior.
No root callsite change is required for this snapshot owner. Entry status/error
Strings, catalog/query response copies, the outer Context Vec registry and
control/accounting/transport infrastructure remain separate consumers.

## Prepared validation

Six focused tests are added, plus the original detached-query test with its
scientific assertions preserved:

1. Exact wrapper replacement, fixed-root underflow and byte/root overflow.
2. Exact simultaneous source/target charge, legacy info/QAM/decoder parity,
   source passivity and final-alias disposal after source drop.
3. One-unit rejection in every positive target dimension before copying;
   the actual zero ordered-entry dimension is asserted without an invented charge.
4. Replacement, clear and Context destruction preserve selected old aliases.
5. Explicit synthetic accounting pressure preserves the same accepted entry,
   status, catalog and scientific source after host rejection.
6. Real pre-cancelled empty archive preserves absence and legacy query errors.

Executed here: pinned rustfmt parse/format check, whitespace check and a scoped
source/ownership audit. **ALL compilation, Clippy, Cargo/runtime tests, Qt/native
checks and integration acceptance are UNEXECUTED in this worktree.** Independent
legacy/comparator fixtures are outside the ledger assertions; these are concrete
owner tests, not an aggregate session-memory proof. Root owns execution after
selective adoption.

Source receipt and exact SHA-256 list are in ignored
`docs/development/local/session-shell/receiver-archive-admission/`.

## Next copied-record consumer (no source edits)

The existing caller is
`apps/qt/src/downstream/instruments/receiver/performance.rs::retain`.
It invokes `FullLoopResult::profile_performance` at
`crates/ofdm/src/api/full_loop/performance.rs:59`, then retains copied Point
storage, status/error Strings, fixed FEC/BER/counters and
`receiver::fec::snapshot` JSON independently of ResultData.

The public performance DTO has a points Vec and optional error String; its
other current fields are fixed scalar/key/FEC/BER values and a borrowed static
reference-plane string. Each point contains original offset/frequency/role,
fixed CarrierMer and optional ideal MER. Its query currently collects a Vec and
formats the existing scientific error. No collect/clone capacity equality is
assumed. Performance production, diagnostics and JSON targets remain frozen
pending a separate concrete finite-preparation/adoption proposal; no API
extension or bound is introduced in this unit.
