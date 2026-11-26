# Scientific control metadata: Rust source preparation

Base: `fe04f22a35b2a1650d26288a9e3a6cecf49289a7`.
Branch: `feature/qt-control-retention-api`; frozen de9 and prior branches preserved.
This source-only UIU-SESSION-01 prerequisite implements the scoped
[scientific-control decision](../design.md#uiu-session-01-scientific-control-allocation-metadata-prerequisite)
under [REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-06](../../../../requirements.md#req-uiu-06) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07), preserving the
[session lifecycle](../../../../architecture.md#unified-qt-session-and-file-lifecycle).
Lead owns canonical records, Rust execution, subsequent Python parity and acceptance.

## Exact operation and owner map

| Existing owner / edited source | Additive contract |
| --- | --- |
| `crates/phy-api/src/control.rs` common RunControl | Only existing RetainedPayload trait implementation for the private Arc<AtomicBool> child. No fourth inherent operation/getter or facade extension. |
| `crates/ofdm/src/api/full_loop/control.rs` FullLoopControl | `retained_footprint(&self) -> ApiResult<RetainedFootprint>`; full root + private Shared + common stop atomic, two roots/zero entries. |
| `crates/ofdm/src/api/transmitter/control.rs` TransmitterPreviewControl | Same signature, its own actual root/Shared layouts + stop atomic, two roots/zero entries. |
| `crates/ofdma/src/simulation/control.rs` directional RunControl | `retained_footprint(&self) -> PhyResult<RetainedFootprint>`; root + private State, one root; optional common stop adds its atomic and one root. |

Each source adds only its new `control/retention_tests.rs` child; this handoff is
path nine. Exhaustive owner field/enum audits and existing checked sums cover all
current children. Containing root/Arc payload layouts count each inline field once;
shared stop children do not recount the inline common handle. Shared aliases are
conservatively repeated when independently queried, without strong-count sizing.
Private refcount/allocator bookkeeping stays outside the approved payload quantity.

Metadata borrows source owners without cloning, serializing, running science,
claiming a control, storing atomics, reading elapsed clocks, waiting on Condvar or
notifying workers. Progress mutexes recover poison consistently. Upstream progress
and pause locks are short and separate, so metadata introduces no nested lock order.
Generic synchronization visitors, new dependencies, defaults, schemas and numerical
algorithms are unnecessary. Current mode/status/progress behavior remains intact.
Overflow uses existing downstream ResourceLimit errors and upstream ResourceLimit
with field `retained_footprint`.

## Source preparation and tests

All compilation, Cargo, Clippy and runtime checks are UNEXECUTED here. Only pinned
rustfmt and source/allowlist/original-body preservation checks are authorized.
Prepared tests (13 total, all UNEXECUTED):

| Owner | Cases |
| --- | --- |
| Common (2) | Independent atomic/layout formula and clone/Weak final drop; repeated alias quantities and cancellation passivity. |
| Full loop (4) | Independent Shared/stop formula and aliases; idle/requested/acknowledged pause/progress/cancel/terminal passivity; poison recovery; real complete/pre-cancelled request science and source invariance. |
| TX preview (3) | Independent Shared/stop formula and aliases; Running/Completed/Failed/Cancelled one-shot passivity; real preview metadata, sample/PSD and source invariance. |
| Upstream (4) | Plain/optional-stop formula and alias delta; pause/progress/claim/cancel passivity; separate poison recovery; real complete/shared-pre-cancelled report science. |

Real runs reuse the established single-channel profile/RxMER and upstream
`us-50khz.json` fixtures. Upstream comparison removes only host `elapsed_ms` fields;
scientific values remain compared. Its unit-test State formula includes the existing
cfg(test) cancellation hook; a native reference must measure its actual production
layout. Original tests and all original production bodies remain unchanged.
No fake backend or new scientific claims follow from metadata fixtures.

Executed source checks: pinned Rust 1.85.1 rustfmt --check on all eight Rust files;
`git diff --check`; exact nine-path/base/empty-index/text-source audit; three-method
and 13-test inventory; original-body preservation review and fixture existence.
The generated Python integration guide --check reports current. No Cargo, build,
Clippy or runtime command was invoked; there are no build/runtime artifacts.

## Remaining lead gates

Run the focused common/downstream/upstream Rust tests before native exposure.
Lead then owns native methods in existing PyFullLoopControl, PyTransmitterControl
and upstream PyControl, returning existing merlin.common.RetainedFootprint through
current error mapping/GIL release; update existing root/upstream stubs. No common
Python method or new class/namespace registration is proposed. Installed-wheel
parity must use a matching direct Rust reference; old wheels establish no support.

Qt admission must separately cover its outer wrapper/control allocations and keep
charges through every execution/queued/old alias. Public metadata does not reserve
application policy, predict future results or establish SESSION aggregate closure.
No Qt, facade, Python, manifest, canonical record or matrix edits are included.
Final focused save receipt supplies exact paths, commit/parent, source hashes and
clean-tree verification. No runtime/build artifacts are produced.


## Native adapter source phase

Parent: frozen Rust preparation `ff062c13b4c005e4df4422ce86f1295d43fcca5f`.
Branch: `feature/qt-control-retention-native`; the Rust branch remains preserved.
The lead reports all 13 owner tests passed after exact primary adoption, including
real-run invariance. This phase does not rerun them or claim native acceptance.

Exactly three native methods reuse the accepted Rust operations:
PyFullLoopControl in `bindings/python/src/full_loop/control.rs`,
PyTransmitterControl in `bindings/python/src/transmitter.rs`, and PyControl in
`bindings/python/src/upstream/simulation.rs`. Each `retained_footprint(self)`
returns existing `merlin.common.RetainedFootprint` via PyRetainedFootprint, with
`py.detach` interpreter release and existing structured error mapping. Upstream's
existing generic error conversion accepts PhyError into ApiError, retaining the
ResourceLimit field. No scientific owner is cloned or rebuilt. The two existing
downstream controls in `__init__.pyi` and upstream RunControl in `upstream.pyi`
receive matching declarations; all old declarations and native bodies remain.
There is no common control method, class/namespace registration or dependency.

This six-path phase appends only this note beyond those five adapter/stub files.
Executed pinned Rust 1.85.1 rustfmt --check on the three Rust files, diff --check,
exact base/allowlist/old-body audit and source-only AST checks of both stubs. The
generated integration guide --check reports current. Compilation, native import,
Cargo/maturin, Clippy and runtime checks remain UNEXECUTED in this worktree. Lead owns the native parity test file,
fresh wheel, direct Rust reference comparison, matrix/guides and acceptance.
The earlier Rust-phase native gate is superseded only by this prepared source;
installed/native/runtime evidence and Qt owner admission remain lead gates.
