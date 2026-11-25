# Unified Qt compatibility matrix

Status: acceptance plan. Backend evidence at `23f63c9` is a historical baseline,
not evidence that new UI tasks passed. Existing import/behavior contracts remain
binding. [Canonical requirements](../../../requirements.md#unified-qt-workbench-requirements)
and the [official tasks](../../checklist.md#unified-qt-workbench-delivery) own scope
and status.

| Consumer / contract | Required retained or new behavior | Acceptance owner |
| --- | --- | --- |
| Existing DS Rust and CLI | `docsis_ofdm_fec::api::*`, `Simulation`, `Merlin`, existing requests/defaults/errors and outputs retain behavior; no scientific fixture changes merely to pass migration | UIU-API-01/02 and UIU-QA-01: focused core/API regressions, no-default-features, facade compatibility and CLI affected workflows |
| Independent PHYs and facade | Base direction features remain usable without Qt/Python/other PHY; optional input adapter has an explicit acyclic graph | UIU-BOUND-01 and UIU-QA-01: Cargo metadata, independent feature/build tests and negative boundary cases |
| Python distribution | Existing `merlin.Simulation` unchanged, genuine DS/US imports in one installed wheel; shared new scientific capabilities land in Rust first with matching errors/units/bounds | Every API task: installed-wheel imports and targeted operation parity; UIU-QA-01 final workflow regression |
| Existing DS Qt | Diagram, source/OCD/DPD/measurement/channel/receiver editors, apply/cancel/readiness, retained snapshots and PLC/NCP/payload views, PRBS, BER/FEC, source selection, update/About/native actions survive source moves and public API migration | UIU-BASE-01 establishes current cases; UIU-DS-01 migration comparison; UIU-QA-01 native and scientific presentation checks |
| QML/build identity | One executable, same `Merlin.Workbench` URI, Main resource URL, icons/notice bytes, all QObject entries together, every production QML registration exactly once | UIU-ORG-01/UIU-DS-01: registration/resource/import check, Qt Rust/QtTest, native build/launch |
| Tabs and dispatch | Independent config/dirty/path/selection/results; tab switching does not stop worker; Start/Stop/Pause scoped to selected session; second Start rejected visibly while either direction is active, including paused | UIU-SESSION-01/UIU-SHELL-01/UIU-US-02: delayed callback, stale generation, cancellation, tab-switch, focus and failed-admission tests |
| Legacy saved DS schema 1/2 | Recognize as DS without changing accepted settings, embedded bytes, filenames, hashes, defaults, original grids or Designer semantics; unsupported versions fail without state loss | UIU-API-05/UIU-FILE-01: current examples plus independent old fixtures, exact input/provenance comparisons and Rust/Python load/run |
| New direction-aware project | Proposed schema 3 envelope with explicit direction and version; DS and US payloads remain distinct. Finalize exact field inventory/version policy in UIU-API-05 before implementation. Never infer US from a legacy DS field | UIU-API-05: typed bounded parser, serializers, equality/round trip, malformed/wrong direction/unknown-version cases in Rust and installed Python |
| Active-direction Save/Open | Save writes only active project's file; dirty prompts and overwrite protection retained. Open routes to validated direction and guards that tab's dirty state. App close checks both. Failed/cancelled Open/Save leaves both states intact | [UIU-FILE-01 accepted evidence](validation.md#uiu-file-01-native-routing-and-dirty-transactions): two populated tabs, repeated Open/Save As, cancelled picker/write failure, no unrelated file overwrite |
| Updates/restart | Private recovery bundle contains both independently API-valid projects, active tab and required UI state; no normal combined-workspace file. Source checkout never installs/elevates. Failures preserve original process/state | UIU-RECOVERY-01: bounded bundle validation/restart args and recovery tests; installed polkit/UAC lifecycle remains unverified until actually exercised under separately authorized environment |
| US measured/synthetic modes | Preserve H/C/C×H and CM/CMTS reference planes, original PNM evidence and selected association; actual backend rejection of unsupported combinations. Measured RxMER presently requires unity physical H/effective C and excludes sample AWGN/delay/phase/CFO | UIU-US-01/02/03: supported/unsupported UI admission and matching backend/API errors; do not enable arbitrary combinations |
| Shared scientific instruments | Two real direction consumers of each extracted renderer, authoritative API data, correct direction labels and unavailable states; no false shared status for a copied DS implementation | UIU-SHARED-01/02: DS/US fixtures and actual selected run observations, axes/units/PSD/MER/FEC/partial/provenance assertions |
| Native desktop | Ubuntu 22.04 amd64 and Windows x64 Qt UI package contracts retained; Windows CLI and macOS remain deferred. Local evidence does not establish hosted or installed success | UIU-QA-01: available local native launch/build and package checks; exact untested platforms/lifecycles recorded separately |

Schema 3 is the selected next-version proposal, not a format already supported.
Existing parser byte/capture limits are ceilings, not permission to allocate the
maximum in every tab: UIU-API-05 must establish aggregate project/recovery bounds
without silently lowering established valid DS input behavior. Initial execution
is serial, but retained observations from both tabs also need a finite aggregate
budget and explicit eviction/admission behavior that preserves accepted inputs.
