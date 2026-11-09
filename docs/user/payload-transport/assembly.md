# TX: packed-symbol signal assembly

Assembly turns an existing bounded packer plan into actual transmitted cells.
It first encodes NCP messages, CRC and control FEC, validating the reserved
positions. Then it processes cells in increasing pre-interleaving order.

```mermaid
flowchart TB
    PLAN["Validated packer plan"] --> CONTROL["Encode NCP messages and CRC"]
    PLAN --> DATA["Data labels / filler / pilot roles"]
    CONTROL --> RAND["Shared data/NCP randomizer, one clock per eligible cell"]
    DATA --> RAND --> QAM["Map data, NCP or BPSK filler"]
    QAM --> CELLS["Sample cells and typed pilot placeholders"]
```

Data uses256-QAM, NCP uses its configured control order, and Z-marked filler uses
the specified BPSK sequence. Scattered-pilot placeholders do not clock the
randomizer. The existing randomizer owns all mask arithmetic; no duplicate LFSR
is added here. A candidate copy commits only after successful assembly, so a
malformed label or reservation cannot partially advance the stream.

Interleaver values distinguish `Sample`, `PilotPlaceholder` and an outer `None`
for unavailable history. They contain no codeword ID, cell offset or reserved
NCP marker. Those remain TX-only plan diagnostics; the receiver must discover
field length and codeword starts from received control signaling.
