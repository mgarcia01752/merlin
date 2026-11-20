> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/ui.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Presentation implementation design

DEV-UI-PLAN-01 closes design only. REQ-OBS-03/04 and REQ-API-01..06 apply.
No GUI is delivered by this record. A future optional Qt6 presentation adapter
depends on the public Rust facade; native widget glue and its license stay in
that adapter. Scientific crates remain Apache-2.0, Qt/Python independent. Adopt
interaction patterns rather than copying any GPL application implementation.

## Owners and planned files

Create `apps/qt/` as a separate optional package with `src/main.cpp` for startup,
`src/workbench.{cpp,h}` for navigation, `src/session_bridge.rs` for facade-only
dispatch, `src/editors/` for drafts, and `tests/` for widget/event/lifecycle cases.
Create `apps/qt/src/instruments/` for passive QAM/SA views. Extend
`crates/morgana/src/api/` only for shared capabilities found missing during
implementation, then expose them in Python before the GUI uses them.
No Qt object, UI pointer or event loop enters the scientific worker. Do not add
these planned files as executable stubs before the separate UI implementation.

## Routes and accepted state

The main path is PRBS → CM-US-TX → CH-SIM → CMTS-US-RX → source comparison.
Side panels are US Configuration and PNM Files (RxMER/US-Pre-EQ). Every stage
row routes by its stable TX/CH/RX ID to the architecture section and a dedicated
detail view. PRBS seed/continuity is distinct from the PHY randomizer. Each
TX01..08, CH01..06 and RX01..09 detail shows accepted settings, applicability,
current readiness reason and any retained observations actually available there.
Stage configuration readiness and availability of retained historical taps are
separate statuses; do not fabricate an instrument at a stage without a tap.

Editors own a local draft copied from accepted Rust input. Apply performs Rust
validation and atomic configure, then publishes the new generation and readiness.
Cancel, close, parse failure or unsupported combination preserves the prior
generation. Loading a capture is idle; model association and readiness require
explicit CM/channel/grid/profile/time and replay settings. Preview uses no active
source/RNG state. Changing accepted settings invalidates current-run readiness
without deleting immutable old reports; old results retain generation labels.

QAM/SA actions carry the exact public `Tap`, filter, limit and report identity;
they query returned snapshots and never rerun a simulation. `TxOutput`,
`ChannelOutput` and implemented RX taps show actual complex samples/cells,
units, physical bins/sample origin and retained/absent coverage. Stage-internal
instrument proposals require new Rust observations and tests before exposure.

## Lifecycle, limits and independent acceptance

An application-owned bounded worker calls Rust `run_with_control`; the UI stays
responsive through progress/pause/resume/cancel. One session lease permits one
active mutation; Busy remains visible. Closing a running window requests cancel,
retains completed outcomes, joins the worker, then destroys presentation state.
No callback invokes Qt under a scientific lock. Poll at no more than10Hz, retain
at most one pending progress snapshot and bounded instrument results using the
existing maximum65536 points; stale queued generations are discarded explicitly.

Future widget tests SHALL exercise every route and every actual tap, valid/
invalid Apply, Cancel, load-without-run, and a changed-generation stale result.
Threaded tests SHALL verify responsive cancel/paused cancellation and safe close.
Screenshots SHALL verify text plus color for missing/ready/busy/unsupported states,
unavailable observations and complete/partial/discontinuous results. A completed
run with failed FEC/source decisions MUST remain a scientific failure. Accessible
names, keyboard navigation, focus and visible errors are part of UI acceptance.
Rust/native fixture parity remains mandatory for any shared setting introduced.

Design acceptance: all block/stage routes, actual tap dispatch, draft lifecycle,
readiness, worker ownership, planned files and tests are specified above.
Runtime GUI acceptance remains a separately tracked implementation milestone.
