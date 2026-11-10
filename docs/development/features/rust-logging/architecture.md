> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/rust-logging/`
> Logs, captures and specification images are not published.

# Standard Rust logging

Use pinned log0.4.35 facade in all three crates. CLI alone owns env_logger0.11.11
initialization, stderr sink and RUST_LOG filter; default info. Disable env_logger
default features to avoid optional regex/color/time dependencies. Both advertise
Rust1.71 MSRV, compatible with pinned1.85.1; verify actual locked graph in gate.
No new shared logging crate or custom facade; library consumers provide backend.

Retrofitting: CLI command lifecycle/failure, JSON read/write context and suppressed
cleanup errors; simulation lifecycle/limits; payload/NCP/PLC decoder diagnostics;
PLC message CRC rejection and optional probe failures/budget exhaustion. Log scalar
counts/status/error codes, no data arrays, frame bytes, MACs or provenance dumps.
Input/CRC rejection and expected decoder failures are debug, not application error.
Terminal failures log once at CLI boundary; only separately suppressed failures
need their own error record. Preserve Result and simulation outputs; logging is
not a success flag, cancellation or probe replacement. No loop info/warn floods.

Testing: spawned CLI stderr/stdout/filter/error behavior, no-clobber output, same
simulation results with logs enabled/disabled (exclude elapsed time), facade record
ownership/levels and no global library initialization. Whole existing simulation
suite and analysis-only gate remain required. Update architecture/checklist/docs
and coding-style skill; no commit/push or global system installation.

## Completion and verification

Implemented standard facade/backends and targeted error/lifecycle/decoder/probe
instrumentation without adding logging to generic error constructors. Shared
facade version lives in workspace dependencies; only CLI depends on env_logger.
The active dependency graph has no enabled regex/color/time backend feature;
Cargo.lock also records optional regex packages from resolver metadata.

Four new tests pass:3 isolated CLI tests and1 consumer-owned facade test. CLI
filters include default,debug,off and module selection; simulation JSON matches
apart from elapsed_ms. Error status/destination protection remain unchanged.
Full pre-push gate passed, along with analysis-only warnings-denied Clippy.
Coding-style skill validates through canonical and both discovery paths. Changed
Markdown local links/fences and git diff whitespace checks pass. No system
packages installed. Parent shell currently has RUST_LOG=warn, honored by the
smoke run; explicit default tests remove that inherited setting per child process.

Updated architecture, checklistQA-08, coding style, CLI docs, build docs and the
logging guide. No log files are automatically created and no logger is installed
by either simulation library. NEXT-09 PLC block interleaving stays pending.
