# DEV-05: Parallel local validation

The user reports 32-minute local validation and requests parallel execution on
this PC before another release. Configure process-per-test scheduling across
the root workspace with pinned cargo-nextest, starting at four concurrent tests
on the eight-CPU Ubuntu VM. Preserve Cargo artifacts and all existing assertions.
This task changes local scheduling, not simulation algorithms or test-result
caching. Hosted workflow commands and publication gates remain unchanged.

Own the checksum-verified tool bootstrap under tools/testing and the local
launcher under scripts/. Download only an exact Linux x64 musl release into
ignored local storage, with TLS, size bounds, pinned archive hash and checked
member extraction. Never install globally or compile the runner from source.
Retain original Rust 1.85.1 and stable Cargo profiles. First use needs a network;
verified cached bootstrap and test execution work offline afterwards.

Split the uniform-order test into 16 independent spacing/order cases and the
CLI width/profile test into eight independently isolated temporary directories.
Preserve the full live PLC/NCP/waveform and CLI import/channel assertions.
Use the local nextest profile only; one process per test isolates loggers and
mutable process state. Serialize the pause/timing-sensitive case globally.
Keep doctests, analysis-only builds and example phases in dependency order;
Qt/QML and existing release smoke commands remain outside this parallel pool.

The pre-push gate calls the local launcher, while explicit --skip-test still
omits these Rust tests and retains its existing checks. Default execution runs
every existing test phase. Expose worker-count override, live case timings and
JUnit results; do not retry failures automatically or treat ignored tests as run.
Validate bootstrap/scheduling/error handling with offline tooling tests, compare
test inventories, then measure a real full local run. Report wall-clock time,
worker count, pass/fail totals and limitations without inventing a baseline.

Runner provenance: cargo-nextest 0.9.151, official
[Linux x64 musl release archive](https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-0.9.151/cargo-nextest-0.9.151-x86_64-unknown-linux-musl.tar.gz),
SHA-256 `6b284d6fde9670dfb6ee10ea9d9abbc85f95643d3f592472d420d82e257b3a6c`.
The prebuilt executable runs the project's existing Rust 1.85.1 builds; no
toolchain upgrade or Cargo.lock change is required. See upstream
[execution](https://nexte.st/docs/running/) and
[concurrency](https://nexte.st/docs/configuration/threads-required/) guidance.

Initial inventory: 343 tests across 116 root-workspace binaries, including 16
uniform-order cases and eight CLI synthetic width/profile cases plus two
existing import-contract tests. Splitting the former two tests adds 22 named
tests without dropping combinations. No fixed shared file/port dependencies
were found in the root integration tests; process-local logger state remains
isolated. The real pause/resume test has a full-budget override and passes.
Seven offline bootstrap/launcher tests and 41 Git-helper tests pass, including
the retained --skip-test behavior. Warnings-denied workspace Clippy, rustfmt,
ShellCheck and Bash syntax checks pass. Full timing results follow below.

## Measured local result

On this Ubuntu 22.04 x64 VM (eight available CPUs, Rust 1.85.1, existing
unoptimized/debuginfo Cargo profile), `scripts/test-local.sh all` completed with
four test workers and exit status zero in **5 minutes 19.07 seconds**:

| Phase | Result | Execution time |
| --- | --- | --- |
| Workspace | 343 passed, zero skipped, 116 binaries | 286.713 s |
| Preparation example | 4 passed, zero skipped | 0.015 s |
| Analysis-only | 113 passed, zero skipped, 96 binaries | 0.799 s |
| Doctests | Both workspace libraries and analysis checked; zero examples | Commands succeeded |

Total: **460 tests passed**, with no retries or ignored cases. The whole-command
measurement includes 11.37 s example compilation and 17.21 s analysis-only
compilation. Workspace binaries had already been built during inventory
validation (67 s, separate from the timed command). Average aggregate CPU use
was 336%; the pause case passed under its exclusive worker override. The two
slow-test notices are progress reports, not timeouts or failures. That run emitted an
analysis-only dead-code warning for SymbolPacker.coded_bits; PACK-03 later gates
the simulation-only state without suppressing diagnostics.

The user's 32-minute report is a historical baseline, not a same-commit controlled
benchmark. This measurement covers the full Rust test phases, not the complete
pre-push gate's tooling/style checks, release CLI build or sequential simulation
smokes. Qt/QML was not rerun because this task changes no UI implementation.
No simulation assertions, tolerance, seeds, work limits or compilation profile
were relaxed. Hosted test execution with the split cases remains unverified.

Ignored evidence: `docs/development/local/parallel-local-tests/` contains the
inventory, full run log, `/usr/bin/time -v` report and Clippy log. Nextest's latest
phase report is `target/nextest/local/junit.xml`. No commit, tag, push or release
was executed. The content-based result cache remains a separate proposal.
