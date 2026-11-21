# Upstream CLI

[Upstream guide](README.md) · [CLI guide](../../cli.md) ·
[Configuration examples](../../../examples/cli/upstream/README.md)

The existing `ofdm-fec` executable calls the same upstream Rust API as Python.
Build it with `scripts/cargo-local.sh build -p ofdm-fec-cli --locked`.
Every operation requires a local input and a new output path:

```bash
target/debug/ofdm-fec upstream preview \
  --input examples/cli/upstream/us-50khz.json --output upstream-preview.json
target/debug/ofdm-fec upstream run \
  --input examples/cli/upstream/us-50khz.json --output upstream-report.json
target/debug/ofdm-fec upstream measured \
  --input examples/measurements/upstream/measured/capture-driven.json \
  --output upstream-measured.json
```

`preview` validates and exports the idle Rust plan and discovered profile IDs.
`run` executes the selected generic configuration. `measured` admits the complete
explicit capture-association plan before running. The 25 kHz examples use the same
commands. Loading does not execute; input fields and supported modes follow the
[configuration](configuration.md) and [measurement](measurements.md) contracts.

For raw capture inspection use `upstream ucd`, `upstream rxmer` or `upstream pre-eq`.
UCD input is the complete raw MAC management message; RxMER and Pre-Eq input is a
supported binary PNM file. The `.hex` measurement fixtures are textual test vectors;
decode them to bytes before choosing them as raw capture inputs. Inspection writes
the Rust-decoded document with original bytes/provenance and performs no simulation.

Inputs and serialized outputs are bounded to 16 MiB. Existing output paths are
never replaced, including inputs selected again as destinations. Invalid or
unsupported admission produces no result. Output-budget failures preserve inputs
and create no destination. Downstream-only options are rejected on upstream commands.

Exit status zero means successful inspection/preview or complete source recovery.
An explicit run with incomplete or failed recovery writes its actual report and
returns status 1. `Completed` means processing reached its normal end; inspect
`reference_recovered`, source coverage, bit errors and actual decoder decisions
separately. Ctrl-C requests cooperative cancellation through the existing handler;
the owned report retains completed science and original configuration. Cancellation
may occur before any source is generated and also returns status 1. An I/O or
serialization failure can prevent report export and remains visible on stderr.

There is one executable and no CLI scientific implementation or Python subprocess.
Existing downstream commands and report schemas retain their behavior. Qt upstream
presentation, native installer behavior and hosted validation are separate gates.
