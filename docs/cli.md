# CLI

The CLI accepts decoded simulation scenarios as JSON and calls the public Rust
API. Binary capture adapters are separate from the simulation core. Run `ofdm-fec --help` for options.
Outputs must be new files; existing destinations are never replaced.

`import-pnm` converts current decoder JSON for RxMER, channel response and profiles
into a scenario. `channel --scenario` uses explicit measured selection, or the
channel request can select sample AWGN. See the [impaired waveform guide](user/impairments/experiments.md)
for executable commands, protected LTE sweep, noise policy and remaining limits.

## Diagnostic logging

Logs go to stderr at info level by default. Set `RUST_LOG=debug` for progress and
stage diagnostics or `RUST_LOG=off` for silence. JSON results and exit status are
independent of the filter. See [logging](development/logging.md) for module filters,
log capture and severity/error ownership.

## Seeded baseline

```bash
./scripts/cargo-local.sh run -p ofdm-fec-cli --locked -- \
  simulate --input examples/awgn.json --output awgn-result.json
```

`examples/awgn.json` selects seed1, ten codewords, 30LDPC iterations, a ten-second
trial budget and25dB MER. `noiseless: true` tests recovery without injected noise.
Ctrl-C requests cancellation; completed trials are exported as a partial result.
The budget is checked between codewords, so one bounded word may finish afterward.
Codec setup precedes the trial timer.

## Generate a candidate

The supplied scenario is **synthetic**, with three frames and four carrier roles.
Its names/hashes are synthetic metadata, not captured source authentication.

```bash
./scripts/cargo-local.sh run -p ofdm-fec-cli --locked -- \
  evaluate --input examples/scenario.json \
  --settings examples/conservative-16qam.json --profile 0 \
  --output candidate.json
```

`--frame INDEX` selects the scenario identity using that frame (default0).
Only exactly compatible frames participate; other devices/channels/grids are
kept separate. `--profile INDEX` explicitly chooses captured roles, matched to the
selected frame within `time_tolerance_seconds`. Without a profile, roles remain
unknown unless the supplied settings explicitly allow proposing data roles.

The generated profile preserves pilots, PLC and exclusions. Use the Rust
`CandidateProfile::edit_range` API for carrier/range edits; protected carriers are
not changed. Save the returned versioned document through the consumer adapter.

## Evaluate an existing candidate

```bash
./scripts/cargo-local.sh run -p ofdm-fec-cli --locked -- \
  evaluate --input examples/scenario.json --candidate candidate.json \
  --output reevaluated-candidate.json
```

The document's policy settings are used, saved results are recalculated, and
captured special roles are reverified. Inspect `evaluation.frames` for individual
intervals, missing readings, vulnerable carriers and the worst timestamp. A gross
rate excludes cyclic prefix, FEC, pilots/PLC, MAC and other PHY overheads.

## Capture-derived residual noise

```bash
./scripts/cargo-local.sh run -p ofdm-fec-cli --locked -- \
  simulate --input examples/awgn.json --scenario examples/scenario.json \
  --candidate candidate.json --frame 0 --output measured-model-result.json
```

Only the selected frame is simulated. The first coding mode requires every
selected data carrier to use16-QAM, with a present finite MER. Verified special
carriers are skipped; unknown/reserved roles are rejected. Results identify the
source frame and selected carrier offsets. No reliability is inferred for other
frames. “Measured model” means synthetic trials conditioned on supplied MER,
not observed modem errors.

Library callers can choose a bounded subcarrier interval through `MeasuredNoise`.
The CLI currently uses the entire selected source frame.

## File payloads

`channel --payload-file FILE` sends exact text or binary bytes when the request
selects `source.kind=file`. Optional `--recovered-file NEW_FILE` exports actual
recovered original bytes. See the [file-source guide](user/simulation/file-source/README.md)
for optional PRBS15 fill, size bounds, segment BER and partial-result behavior.

## Decoder import and native installation

New `import-pnm` requests use schema2 and an explicit decoder contract pinning
pnm-decode0.3.6. Legacy schema1 remains supported without a producer declaration.
See the [import contract](user/impairments/pnm-import.md#explicit-decoder-contract-input-02)
and the updated measurement demo request.

Native CLI archives/installers and per-OS install/uninstall instructions are
described in the [installation guide](user/install/README.md). Workflow files are
prepared locally; Windows/macOS execution and publication remain pending.
