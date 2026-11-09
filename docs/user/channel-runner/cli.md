# Channel CLI and JSON requests

Build only the standalone driver with the pinned local toolchain:

```bash
./scripts/cargo-local.sh build -p ofdm-fec-cli --release --locked
```

Run the one-second reference request:

```bash
./target/release/ofdm-fec channel --input examples/channel.json --output channel-result.json
```

The request omits `duration_ms`, so the default is1000ms. It uses an explicit
synthetic24MHz50kHz layout with maximum interleaver depth. Actual time-domain
samples, PLC/NCP and data FEC are processed; host execution can take longer than
simulated time. Existing `simulate` and `evaluate` commands keep their older roles.

For a quick3ms run with bounded selected test-point export:

```bash
./target/release/ofdm-fec channel --input examples/channel-probes.json --output channel-probes-result.json --probes-output channel-probes.jsonl
```

Every destination must be new. Existing result/probe files are never overwritten.
The core schema handles scientific values; the CLI only validates paths, adapts
requests, connects the process Ctrl-C token and writes typed JSON/JSONL. Input
and result JSON are limited to16MiB. [Probe export](../simulation/probe-export.md)
has separate record/stream and event/element limits.

The version1 request requires `schema_version`, `transmission` and `layout`.
Optional defaults are `duration_ms=1000`, `prbs_seed=32767`,
`ncp_modulation="qpsk"`, `llr_variance=0.01`, `max_iterations=50`,
`receive_block_samples=4096`, and the [job limits](job.md). The reference variance
only produces finite LLRs; it adds no waveform noise. Unknown fields, unsupported
versions and invalid geometry/work bounds reject before processing. `probes`, when
present, must have an accompanying `--probes-output` path, and vice versa.

The output has its own schema-version1 result contract:

| Field | Meaning |
| --- | --- |
| `status`, `error` | Job outcome and optional terminal diagnostic. |
| `settings`, `limits` | Frozen scientific configuration and host/output policy. Duration in settings uses serialized seconds/nanoseconds; the request adapter uses milliseconds. |
| `summary` | Actual FEC/PRBS totals, decoder histogram, source-window accounting, intervals and sample identities. |
| `retained_words`, `omitted_word_details` | Bounded per-word details and explicit retention omissions. |
| `unrecovered_generated_payloads` | Admitted source words without recovered comparisons at termination. |
| `probe_stats` | Event/element counts and callback failures. |
| `elapsed_ms` | Host elapsed time; excluded from deterministic scientific equality tests. |

A saved result is not an input request. Inspect `status` even when the process
exits successfully: Ctrl-C and host-budget stops preserve valid partial output.
Stage/export failures also preserve completed results when result-file writing
succeeds, then return status1. Export final flush/sync is checked before the
result is written, so a final export failure also produces Failed status while
retaining completed scientific totals. A failed final disk write is reported and the
existing bounded JSON writer removes its own incomplete result file. JSONL may
retain a valid prefix or an incomplete final record on I/O failure; it is not
silently reported as a complete export.

The pre-push gate runs a release-mode channel/probe smoke check alongside the
older coding CLI check. Its integration tests verify shared calculations, schema
validation, partial behavior and non-overwriting destinations. No GUI, device
capture, network service or additional OS package is required.
