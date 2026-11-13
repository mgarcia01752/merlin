# Rust logging

The signal-processing libraries emit records through the standard `log` facade. The CLI
owns its `env_logger` backend; library callers can install a compatible backend
without the engine changing global logging configuration.

The CLI defaults to **info** and writes logs to **stderr**. Help/version remain
on stdout, and simulation JSON files keep the same contracts. Per-codeword
progress is debug output; normal runs show concise command and completion records.

```bash
RUST_LOG=debug ./scripts/cargo-local.sh run -p ofdm-fec-cli --locked -- \
  simulate --input examples/cli/awgn.json --output debug-result.json
```

To see only simulation-module diagnostics and application errors:

```bash
RUST_LOG=error,docsis_ofdm_fec::simulation=debug \
  ./target/release/ofdm-fec simulate \
  --input examples/cli/awgn.json --output selected-result.json
```

`RUST_LOG` accepts `error`, `warn`, `info`, `debug`, `trace` and `off`, plus
comma-separated module directives. Modules use Rust crate names:
`ofdm_fec`, `docsis_ofdm_fec` and `fec_codec`. Trace enables all current records;
there are no sample-by-sample trace records. Optional regular-expression message
filters, color and timestamps are disabled to keep the backend lean.

For an installed/built executable, append stderr to a local log:

```bash
RUST_LOG=debug ./target/release/ofdm-fec simulate \
  --input examples/cli/awgn.json --output saved-result.json 2>>doft.log
```

In PowerShell, set `$env:RUST_LOG = 'debug'` before launching the executable.
`RUST_LOG=off` suppresses records, including errors; it does not turn failure into
success or bypass validation. Existing result files are still never overwritten.
Cargo's own build messages are separate from application logging.

## Levels and ownership

| Level | Current use |
| --- | --- |
| Error | Terminal CLI errors and a separately suppressed output-cleanup failure |
| Warn | Runtime-budget exhaustion with completed work retained |
| Info | Command start, successful export/completion and cancellation |
| Debug | Simulation counts, optional progress, binary/LLR validation, payload/NCP/PLC non-convergence, PLC timestamp CRC rejection and probe failures/budgets |

Expected FEC non-convergence is part of simulation results. It is not an
application error and does not produce error/warn floods in a noisy run. Invalid
library inputs continue to return errors; the caller decides whether to log them
at its terminal boundary. The CLI logs a propagated failure once rather than
adding error records at every helper. Failed cleanup is a different error and
gets its own record because it would otherwise be discarded.

Records contain scalar status/counts/limits and error context. Capture bodies,
MAC addresses, sample arrays, full scenarios and provenance are not dumped.
Logging does not replace typed test points, progress callbacks, CRC checks,
truth comparison or result serialization.

## Library consumers and dependencies

Both libraries depend on the pinned facade through `[workspace.dependencies]`.
They never initialize a backend, inspect `RUST_LOG` or create log files.
`crates/cli/src/logging/mod.rs` is the sole backend initialization owner.
`try_init` reports initialization failure instead of replacing another logger or
panicking; the CLI has one stderr bootstrap fallback for that case.

Pinned versions are `log` 0.4.35 and `env_logger` 0.11.11. The backend is confined
to the executable, with default features disabled. Cargo.lock can include optional
package entries even when those features are not active. Check active dependencies
with `cargo tree -p ofdm-fec-cli --edges normal`.

The independent codec examples do not install a logger automatically. An embedding
application or future Python/Qt adapter owns the backend and filtering; simulation
APIs stay independent of it. See the upstream
[log facade documentation](https://docs.rs/log/0.4.35) and
[env_logger builder](https://docs.rs/env_logger/0.11.11/env_logger/struct.Builder.html).

## Validation

Spawned CLI tests cover default/debug/off/module filters, one terminal error
record, stderr routing, help/version output and destination protection. Their
simulation results match after excluding wall-clock elapsed time. An isolated
codec test installs a caller-owned sink after constructing the library codec,
checks diagnostic levels and verifies identical decisions/iterations with logs
on and off. Existing numerical tests remain required.

Logging can change execution time. A run close to its wall-clock budget can
complete a different number of words; completed results remain valid partial
results. Logging must not change the math or RNG sequence for the same work.

## Desktop logging

The Qt executable now owns a separate stderr backend, with a quiet warn default.
`--debug-ui` enables desktop debug records by default; RUST_LOG overrides this
without controlling timer visibility. The background workbench loader records
start/completion/cancellation and elapsed milliseconds. Terminal load failure is
logged once with its lifecycle event; detailed validation errors remain in the
UI. No capture bytes, arrays or capture paths are logged by the loader.
See the maintained [desktop debug inventory](debugging.md) for controls and
copy/paste examples. CLI behavior above is unchanged.
