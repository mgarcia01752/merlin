# TX: continuous payload stream

The transmitter owns a bounded symbol packer, concatenated encoder, bit-processing
permutation, data/NCP randomizer, pilot reference and cell interleaver. Payload
input remains separate from waveform assembly and from receiver truth.

```mermaid
flowchart LR
    PAYLOAD["14232 payload bits"] --> CODE["BCH → LDPC → 2025 labels"]
    CODE --> QUEUE["At most16 queued words"] --> PLAN["Next source-symbol plan"]
    PLAN --> ASSEMBLE["Actual mapped data/NCP/filler cells"]
    ASSEMBLE --> INTERLEAVE["Continuous cell interleaver"] --> CHANNEL["Reduced physical-time grid"]
```

A payload enqueue returns coding-stage outputs for optional inspection. The packer
keeps a partial word until its final cell is placed. No codeword boundary resets
randomizer or interleaver state. Source symbols, channel symbols and the session
origin are explicit; a call can emit cells delayed from earlier source symbols.

Unavailable TX branch history is preserved in the direct interleaver output.
The physical-time view deliberately substitutes unity BPSK at unavailable data
positions and records their ranks. This is an explicit reference startup/drain
policy, not silently treating missing history as valid payload or measured zero.
Actual scattered pilots are inserted by the physical assembler. RX does not
receive the list of replaced ranks.

After the queue empties, drain advances only the interleaver for $M-1$ periods
when source input has been transmitted; an untouched empty session emits no drain.
It does not create new source/NCP symbols or discard pending queued words. New
payload or ordinary source calls are rejected after drain begins. The matched RX
can finish valid words during drain; those words must be retained and compared.
