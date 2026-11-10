# Stage-observation design

Test points let an experiment inspect intermediate values without duplicating
numerical calculations or adding print statements inside DSP stages. They can
support diagnosis, plots and later exports.

```mermaid
flowchart LR
    STAGE["Signal-processing stage computes its result"] --> RESULT["Completed typed result"]
    RESULT --> NEXT["Next signal-processing stage"]
    RESULT -.-> SELECT["Selected stage, time and element window"]
    SELECT --> LIMIT["Event and element budgets"] --> VIEW["Borrowed read-only event"]
    VIEW --> SINK["Caller observer: inspect or copy"]
    OFF["Disabled observation"] --> SKIP["No callback"]
```

Models describe stage identity, direction, coordinate domain, units, timing,
normalization and data shape. Selection decides which stage/time/element ranges
are visible. The observer consumes an immutable borrowed view; it does not receive
mutable codec, interleaver or randomizer state.

Simulation results are computed before observation. Disabling observations,
exhausting a budget or failing a callback must not change those completed values.
Callback failures still consume the dispatch budget and are reported to the caller;
they are not silently retried. The channel stepper ends a failed session on
observer error and retains completed summary comparisons. Disabled/exhausted
probes do not end a session. The [job wrapper](../channel-runner/job.md) now
returns explicit Failed/Cancelled/RuntimeExceeded results with completed science.

Windows retain original element offsets and total source size. PLC windows also
retain the full local grid; physical-bin windows retain spacing and frequency
origin. Missing history stays missing, including JSON null when serialized.
A selected window is not a new, shorter physical channel.

Current producers cover pilot stages, PLC transmit stages, assembled physical
bins and thirteen [waveform TX/channel/RX boundaries](../waveform/observations.md).
Waveform windows retain full numerology, sample clock, absolute sample origin
and direction as well as physical frequency coordinates. Ten [payload/NCP TX/RX producers](../payload-transport/observations.md) add
source/channel cell records and actual control/data recovery boundaries.
Other processing stages expose some results directly but are not all wired
to this common observation mechanism. A registry entry is not permission to claim
that every stage in the architecture already has a producer.

The observer does not write files or own the simulation result. A caller may copy
a selected window while the callback is active; borrowed data cannot outlive that
call. Selection and finite budgets bound the amount of optional data collected.
See [observation limits](../pilot-signals.md#direct-apis-and-observation) and
[the architecture's stage map](../../architecture.md#test-points-and-data-access).

Source and existing coding-noise outputs now have producers with independent RNG
continuity tests. [Versioned bounded JSONL export](probe-export.md) and the
[channel CLI](../channel-runner/cli.md) provide maintained file output. The
[separate bit-processing stages](../bit-processing/README.md) expose direct
intermediates without requiring the common observer.

## TX QAM role metadata

The mapped TX cell observation carries optional `transmit_cell_roles` in its
context, derived from the actual packing plan: data, NCP, filler and scattered-pilot
placeholders. The vector covers the complete source-rank grid, at most8192 cells;
selected event data still uses its original element offset. Consumers must not
index this vector as a selected-slice-relative array. This metadata is TX-only
and does not alter transported I/Q or enter receiver decoding. An interleaved
consumer must transport roles through the matching interleaver or mark them
unknown; constellation coordinates do not establish their role.
