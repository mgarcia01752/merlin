# RX subsystem composition

The reference receiver owns its geometry, a newly validated physical map,
known-timing framer, FFT plan and known-reference equalizer. It accepts received
sample blocks, never transmitter stage objects or original payload values.

```mermaid
flowchart LR
    SAMPLES["Received indexed sample block"] --> FRAMER["RX framing state"]
    FRAMER --> CP["Prefix removal"] --> FFT["FFT plan"] --> EQ["Reference equalizer"]
    EQ --> EXTRACT["Protected-role extraction"] --> RESULT["Complete received stage results"]
```

Initial timing/layout/channel knowledge is explicit bootstrap input. After PLC
acquisition, decoded OCD/profile settings can configure new consumers through
the existing [readiness gate](../receiver-configuration/README.md). This milestone
does not dynamically replace RX settings during an active partial symbol.

Incomplete periods stay in the framer. Received results expose period, useful
samples, raw bins, normalized bins and extracted domains. If later numerical
processing fails, framing has already advanced; start a new RX session rather
than retrying the same sample block as though it were unconsumed. Completed
results remain independently usable when a later observation callback fails.
The separate [payload receiver](../payload-transport/receiver.md) now consumes
these actual reduced cells under WAVE-RX-06, using the received PLC gate.

The [Receiver Channel Estimation (CH-EST-RX) subsystem](../receiver-estimation/README.md)
now adds pilot-derived correction and a separate bounded CP timing-acquisition
operation. The channel runner retains explicit timing/PLC phase; supplied-H
reception remains selectable for independent comparison.
