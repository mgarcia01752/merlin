# Continuous payload and NCP transport

The fixed-profile TX/RX path now carries actual encoded payload and NCP through
cell interleaving and the [waveform](../waveform/README.md). Each subsystem has
one Rust owner; runners connect stages rather than reimplementing the math.

```mermaid
flowchart TB
    SOURCE["Payload bits"] --> FEC["TX: BCH → LDPC → bit processing"]
    FEC --> PACK["TX: bounded symbol packer"] --> ASSEMBLY["TX: actual NCP, data and filler mapping"]
    ASSEMBLY --> INTERLEAVE["TX: continuous time/frequency interleaving"]
    INTERLEAVE --> WAVE["Physical pilots/PLC → waveform TX → channel → waveform RX"]
    WAVE --> INVERSE["RX: inverse cell interleaving"]
    PLC["Actually received PLC configuration"] --> GATE["Ready snapshot and supported profiles"]
    GATE --> INVERSE
    INVERSE --> NCP["RX: NCP soft hypothesis, FEC and CRC"]
    NCP --> DATA["Received pointers → data soft bits and inverse randomization"]
    DATA --> COLLECT["Bounded word collection → inverse bit processing"]
    COLLECT --> DECODE["LDPC → BCH → recovered payload"]
    SOURCE -->|"Independent truth only"| COMPARE["Compare recovered bits"]
    DECODE --> COMPARE
```

| Subsystem | Design guide |
| --- | --- |
| Packed-symbol signal assembly | [Assembly](assembly.md) |
| Continuous TX queue/interleaver/session | [Transmitter](transmitter.md) |
| Received configuration and support checks | [Configuration](configuration.md) |
| NCP hypotheses and accepted boundaries | [Control recovery](control.md) |
| Pending word and actual decoder results | [Collector](collector.md) |
| Source-order receive processing | [Decoder](decoder.md) |
| Physical-to-source RX stream | [Receiver](receiver.md) |
| Scientific result meanings | [Results](results.md) |
| Bounded stage inspection | [Test points](observations.md) |
| Actual sample-domain demonstration | [Reference experiment](reference-experiment.md) |

Current support is short8/9 concatenated coding, uniform256-QAM ProfileA,
uniform positive QPSK/16-QAM/64-QAM NCP, and the fixed even selectors emitted by
the packer. Dynamic switching, odd selection, OPT tagging, mixed loading and
zero-loaded DPD positions are rejected explicitly. A Z-marked unused data field
is supported and carries BPSK filler; this is distinct from static zero loading
in a DPD.

Streams retain state across calls and are independent of simulation duration.
They do not reset at one-second boundaries or codeword boundaries. The new
[channel stepper](../channel-runner/README.md) selects a configurable observation
duration, initially one second; the original bounded demonstration remains
shorter. Both use the separate [PRBS15 source/verifier](../simulation/prbs.md).
The [job/CLI wrapper](../channel-runner/cli.md) adds cancellation, source-window
metrics and partial results. Blind acquisition and nonideal operation remain later work.
See [architecture](../../architecture.md), [checklist](../../development/checklist.md)
and [source evidence](../../reference/payload-transport-evidence.md).
