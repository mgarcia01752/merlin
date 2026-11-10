# Content-based development build and test cache

Status: **proposal only; no caching implementation or gate changes authorized by
this document**. The user requested a plan before implementation. This plan serves
fast local iteration, initially on the Ubuntu development VM. It does not change
signal-processing algorithms or replace Cargo's compiler/dependency tracking.

## Objective and current behavior

Reuse compiled artifacts and successful test results when their inputs are
unchanged, following the useful part of Yocto sstate's content-based approach.
Avoid repeating a roughly ten-minute full suite for ordinary UI work. Explain
every selected build/test and every reuse decision.

Currently scripts/qt-desktop.sh check always runs formatting, Clippy, Rust tests,
build and QML tests; run invokes cargo run. The Qt app has its own manifest,
lockfile and apps/qt/target. The simulation workspace has a separate root target.
Both retain artifacts, but there is no task-level successful-test cache.
scripts/check-before-push.sh and CI define separate broad validation policies.

Rust compilation granularity is normally a crate, not a source subdirectory.
Cargo remains responsible for compiling/reusing objects and linking compatible
artifacts. Subdirectory mappings select affected tests; they cannot promise that
only one Rust source file will compile. Embedded QML/resource edits still require
rebuilding the Qt application before running that updated executable.

## BUILD-01: Fingerprints and read-only task planning

Introduce a development task planner under tools/build/ with a small shell entry
point. Keep implementation in project-owned Python standard-library tooling;
Python is development-only and never part of the application runtime.

Store local records under ignored local/build-state/. Preserve both existing
Cargo target directories and stable profiles. Do not delete caches, relocate
current outputs or install compiler-cache services as part of this first phase.

Each task has an explicit ID, inputs, prerequisites, command, expected outputs
and validation policy. Compute a SHA-256 fingerprint from a canonical, sorted
manifest containing:

- Repository-relative paths, file content hashes and relevant executable bits.
  Include untracked source, added/removed files, renames, fixtures, resources,
  generated-input producers and task/mapping definitions. Do not rely on Git HEAD,
  timestamps or file size to decide that source content is unchanged.
- Task command/arguments, profile, features, target triple, compiler/build-tool
  identities and relevant environment/configuration inputs such as RUSTFLAGS,
  Cargo config, C/C++ compiler, linker and Qt renderer selection for UI tests.
- Applicable Cargo.toml/Cargo.lock, rust-toolchain.toml, build.rs, QRC/resource
  dependencies, script dependencies and transitive prerequisite fingerprints.
- Qt SDK identity for Qt tasks, using a verified content manifest/provenance for
  consumed SDK inputs. Version strings alone are insufficient for a modified SDK.
  Define verification and refresh explicitly; do not treat equal timestamps as
  proof of equal SDK bytes. Scope external identity work to relevant tasks.

Hash the source inventory once per invocation, then reuse those hashes across
task keys. Exclude target outputs, caches and unrelated private capture/evidence
folders; declare any fixture or generated input actually consumed by a task.
Record symlink resolution and referenced input content without traversing arbitrary
external trees. Unsupported external/generated inputs make a task uncacheable
until its input contract is defined.

The planner reports RUN, REUSE, NOT AFFECTED or UNKNOWN, with changed inputs and
reasons. Unknown source ownership expands to the relevant crate and consumers,
not silent success. An absent successful record requires execution when that task
is selected; it does not trigger unrelated work on a Qt-only first invocation.

## BUILD-02: Build artifacts and success records

Wrap existing build commands rather than implementing a second compiler cache.
A build hit requires a matching task fingerprint and existing outputs with matching
recorded hashes. Missing/corrupt outputs rebuild even if the source is unchanged.
Keep normal Cargo freshness checks underneath any cache miss.

Cache build, formatting, Clippy and individual test groups separately. A matching
build does not imply tests passed; changing a test reruns that test group without
invalidating an unrelated application build. Record exact command, tool identity,
input key, result, log reference, duration and output hashes. Display cached tests
as previously passed for matching inputs, never as newly executed.

Only publish success after exit status zero, required output validation and a
second input fingerprint check. Edits during execution invalidate that result.
Failed/interrupted tasks never create success entries. Use atomic record updates,
per-task locks and Cargo's build locking for concurrent invocations. Reverted
content may reuse its prior success only if the full current key matches; a
recent failed attempt must remain visible rather than being hidden by old logs.

The launcher uses a verified current executable directly on a build hit. Otherwise
it builds first and refuses to launch stale code on failure. Document that replacing
a binary does not update an already-running VM application; restart is separate.

## BUILD-03: Explicit affected-test mapping

Maintain a reviewed, tracked mapping, proposed as tools/build/tasks.json. Define
crate dependencies from Cargo metadata and explicit cross-module test ownership;
file-name similarity alone is not sufficient evidence of test coverage.

| Changed area | Development work selected |
| --- | --- |
| Nothing, or touching a file without changing bytes | Reuse valid task records; no compiler/test process |
| QML components | Relevant QML suites and embedded Qt application build; reuse Rust checks whose inputs did not change |
| Icons/QRC assets | Resource/application build and branding/render checks |
| Qt Rust/native bridge | Qt build, applicable Rust style/Clippy/tests and adapter/QML integration checks |
| Test file or test fixture | Consuming test groups and their build prerequisites |
| A known simulation subdirectory | Owning crate compilation, mapped unit/integration tests and known affected consumers |
| Shared types, re-exports or public API | Owning crate plus reverse-dependent compile checks and mapped consumer tests |
| Crate root or unmapped Rust source | Broader owning-crate validation and conservative consumer coverage |
| Cargo features, manifests, lockfiles, build scripts or toolchain | Recompute affected dependency/task closure; conservatively broaden when uncertain |
| CLI-only implementation | CLI build/checks and affected CLI tests; reuse unchanged science test results |
| Documentation only | Relevant documentation/link checks; no application compilation |
| Development tool or mapping | Tool tests and affected tasks; invalidate changed task definitions |

Start with coarse Qt/science/CLI/tool groups, then add reviewed simulation
subdirectory mappings incrementally. An internal function can change public
behavior without changing a signature; do not limit consumer checks to API text
diffs. Shared primitives, macros and global configuration need broad mappings.
Tests with unknown coverage stay in an owning-crate fallback group.

Selected Rust test commands must match actual integration-test binaries or explicit
unit-test names. Detect empty/mistyped selections; zero tests cannot establish
success for a group expected to contain tests. Keep known slow end-to-end groups
identified separately so they run only for their declared affected inputs or a
full validation request. Do not remove or weaken their assertions.

## Proposed commands (not implemented)

- scripts/dev-check.sh plan: print fingerprints, affected tasks and cache reasons;
  no builds/tests or cache-success writes.
- scripts/dev-check.sh check: run affected development checks, reuse valid records.
- scripts/dev-check.sh build --scope qt: ensure the current Qt executable exists.
- scripts/dev-check.sh check --scope qt: limit intent to Qt while including any
  necessary changed prerequisites; report relevant changes outside the request.
- scripts/dev-check.sh check --fresh: rerun selected checks without trusting prior
  test success; retain Cargo compiler artifacts.
- scripts/dev-check.sh check --full: explicitly execute the complete declared
  validation suite, including uncached tests; retain compiler artifacts.

Integrate qt-desktop.sh with the same planner after the cache contract is proven.
Do not add a second independent implementation of fingerprinting in that wrapper.
Full validation retains separate science and Qt environment prerequisites; report
unavailable required tooling rather than claiming full success.

## BUILD-04: Verification, measurement and rollout

Test the cache with lightweight fake build/test commands and temporary source
fixtures first. Cover unchanged reruns, same-size byte edits with preserved mtime,
new/deleted/renamed files, test-only changes, shared dependency changes, feature/
toolchain changes, missing/tampered outputs, malformed records, failures, concurrent
runs, source changes during execution and empty test selection. No ten-minute
science suite is necessary to validate the planner's own logic.

Then verify real Qt cold/warm runs, one QML edit, one icon edit and one bridge
change. Validate a small representative simulation mapping against broader known
coverage before enabling finer selection. Seed success records only from observed
successful executions; do not backfill them from informal historical reports.

Measure hash/planning overhead, build time, test time and cache-hit counts. Acceptance:
unchanged warm runs spawn no Cargo/compiler/test processes and hashing is small
relative to the avoided work. Establish an actual VM timing baseline rather than
promising an unmeasured duration. Qt edits must not run the full science suite.
A deliberately affected consumer must be selected, and unknown mappings must
produce an explicit conservative fallback.

Roll out in this order:

1. Planner and fingerprints, with explain output and fake-command tests.
2. Verified Qt build reuse and separately cached Qt check groups.
3. Reviewed crate/subdirectory mappings, starting with areas actually being edited.
4. Benchmarks, documentation, cache diagnostics and explicit fresh/full controls.

## Boundaries

This is local development caching, not a portable/remote binary archive service.
Cross-machine sstate distribution and sccache are later options if measurements
justify them. Do not load another machine's test-success record as local runtime
validation. Avoid copying private inputs or credentials into cache manifests/logs.

Existing push/release gates and hosted CI are unchanged by this proposal. A later
proposal may apply caching to those gates, but development cache hits do not bypass
them. Security scans depend on changing advisory/query data, so source-only hashes
must not suppress RustSec/CodeQL execution. Interactive VM approval is not a cached
automated test result.
