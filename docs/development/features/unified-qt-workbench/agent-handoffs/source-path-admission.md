# Original source-path admission preparation

Base: `cd0166457c69576294f339f3124d75c0164b3d39`.
Owner: UIU-SESSION-01, REQ-UIU-03/06/07. The lead's canonical
[design](../design.md#uiu-session-01-original-source-path-ownership)
precedes this source-only implementation; acceptance/canonical records stay lead-owned.

Replace the thread-global source-path set with one guarded registry on the actual
ApplicationSessions host. Both direction flags protect the same canonical path;
Save queries their union. Only successful guarded downstream project adoption
releases downstream flags; failed/stale/cancelled Open preserves protection.
Current import callers remain downstream, and successful bounded reads register
before scientific/editor staging. No scientific/schema/QML/public API changes.

Each actual registry pointee and its shared root is admitted before Arc creation;
each distinct path record admits its complete layout, actual PathBuf capacity and
one ordered entry before insertion. The path and final reservation belong to the
same concrete owner; immutable path alone defines lookup/ordering. Clones share
the same guards, and registry reservations retain the counter without Context or
ApplicationSessions cycles. The original distinct-path cap gains one named shell
constant. Existing accounting/gate/Sessions core assertions remain, with a fourth
fixed host root explicitly measured rather than overlapping those layouts.

Compile, Cargo/runtime tests, Clippy and builds remain UNEXECUTED here. The lead
owns selective adoption and actual acceptance. This does not close other path/GUI
copies, channel policy or the complete SESSION retained-owner inventory.

## Concrete hooks and source coverage

`ApplicationSessions` adds `register_input_path(Direction, PathBuf)`,
`protects_input_path(&Path)` and `clear_input_paths(Direction)`. Only the existing
bounded filesystem reader registers actual canonical paths in production; all
three current importer callers pass their Context host and Downstream explicitly.
`Project::write(path, application)` queries that host's union, including recovery
Save. PreparedProject clears Downstream only after the existing accepted-state
and readiness guards succeed. Scientific validation, atomic writer contents,
original bytes, saved JSON and CXX/QML signatures remain unchanged.

The concrete `BTreeSet<Record>` keeps path then Cell flags then final guard;
Borrow/Eq/Ord inspect only the immutable path. The documented
`mutable_key_type` allowance applies only to `impl InputPaths`, not the module.
Mutex access recovers poison, holds no Context/host, and orders its short counter
reservation consistently. Registry contents drop before its own retained guard;
records release payload before their individual guards. Each record has zero
new shared roots. Duplicate direction registration also succeeds at full limits
without another charge. `MAX_ORIGINAL_SOURCE_PATHS` names the old distinct cap
once, independently of unrelated equally valued text/configuration limits.

Nine new source cases cover exact/below registry bytes/roots; actual PathBuf
spare capacity and record byte/entry rejection preserving older records; cap and
duplicate/direction union; foreign counter/final shared alias; host isolation and
clone flags; real canonical symlink reads (Unix); New preserving shared Upstream
protection; Save refusing the actual host union without altering originals; and
successful guarded Open releasing only Downstream. Existing stale/foreign Open,
late-readiness rejection, constructor rollback and original/scientific assertions
remain and gain path checks. The background-loader pressure fixture now registers
its real source before byte exhaustion, preserving its intended loader boundary.
Three core host roots remain independently asserted; the registry is a measured
fourth root with an added final-prefix rollback check. Existing Context fixed
layout tests already measure actual host/type sizes and need no source edit.

Executed source-only checks: pinned rustfmt/check; `git diff --check`; current
Python integration-guide check; exact 19-path/base/index/NUL/SPDX audit; 15
unchanged read/scientific/lifecycle body comparisons; routing-only comparisons
for the existing atomic write and guarded adoption bodies. Compilation, Cargo
runtime tests, Clippy, builds and desktop probes are **UNEXECUTED** here. Lead
must run the complete affected Qt Rust regression and integration gates; no
scientific/native/QML acceptance is claimed from this preparation.

Ignored source receipt and exact path/hash manifest:
`docs/development/local/unified-qt-workbench/source-path-admission-source-cd01664/`.
Other persistent path/GUI copies, transport policy and complete SESSION ownership
remain pending; this unit covers only the concrete original-source registry.
