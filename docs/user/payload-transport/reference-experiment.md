# Actual payload waveform experiment

Run the shared Rust verifier:

```bash
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example payload_waveform --locked
```

The example first acquires actual OCD/ProfileA/NCP packets through PLC waveform
samples. Acquisition uses explicit known timing/layout and reference filler in
the unconfigured data plane. Once the received gate is Ready, a new synchronized
payload session starts at the next128-symbol boundary.

```mermaid
flowchart TB
    ACQ["Actual PLC waveform acquisition"] --> GATE["Ready received configuration"]
    GATE --> RX["Construct independent payload RX"]
    SOURCE["Continuous PRBS15 payloads"] --> TX["Actual BCH/LDPC, packing, NCP and cell TX"]
    TX --> WAVE["Pilots/PLC and windowed waveform samples"] --> RX
    RX --> DECODE["Received NCP → payload LDPC/BCH"]
    SOURCE -->|"Truth supplied only after decoding"| COMPARE["Actual bit comparison"]
    DECODE --> COMPARE
    TX --> DRAIN["Drain delayed cells and final window tail"] --> RX
```

At both spacings it sends four scheduled payloads over160 source symbols at
maximum interleaver depth, splits RX sample blocks into137-sample chunks, crosses
the128-symbol reset and retains words completed during drain. The JSON contains
real payload bit errors, independent decoder flags, LDPC iterations, BCH correction
counts, accepted NCP iterations, pending bits, warm-up/drain counts and waveform/
PLC comparisons. It does not estimate throughput from a guessed bit rate.

The verifier permits1–512 source symbols and at most16 scheduled payloads.
Acquisition is bounded to eight PLC frames. An interval too short to place/recover
all scheduled words fails explicitly. Samples and signal-processing stage histories are
streamed; no entire waveform is retained. Output summaries remain bounded.
The demonstration uses a synthetic24MHz active envelope with explicit pilot
positions and a unity channel. A positive soft-reference variance generates
finite LLRs but adds no channel noise.

This is a reproducible short proof of production transport modules, not the
configurable-duration job runner. Streams themselves use continuous64-bit symbol
identities without per-second resets. The separate [channel stepper](../channel-runner/README.md)
now selects duration, initially one second. Its separate [job/CLI facade](../channel-runner/cli.md)
adds cancellation, progress, window accounting and partial results.
The example now selects [PRBS15](../simulation/prbs.md), seed `0x7fff`, explicitly.
JSON includes the source kind/seed and independent `prbs_comparison` totals.
The legacy `run_payload_waveform` API still uses `experiment.seed` for ChaCha8;
`run_payload_waveform_with_source` takes an explicit source and ignores that
legacy seed. Truth streams are never receiver inputs.
