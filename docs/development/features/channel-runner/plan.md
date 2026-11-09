> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/channel-runner/`
> Logs, captures and specification images are not published.

# RUN-01 / RUN-02

Generalize sample-clock timing with checked64-bit accounting while preserving
one_second_timing. Add a symbol-at-a-time channel owner with frozen settings,
actual PLC acquisition, continuous PRBS payload supply, existing payload TX/RX,
shared actual waveform link and explicit observation/completion/drain phases.
No full-waveform or duration-sized word-history allocation. Return per-step
completed comparisons and cumulative finite statistics. Invalid/failed sessions
are explicit. Default observation is one second; caller can select up to one hour.
Duration floors to sample ticks and complete source symbols; report unused ticks.
Observe the configured source-symbol interval then stop generating payloads,
place queued words and drain delayed cells/window tail separately.
The first stage is a public stepping API, not the later CLI/cancellable job facade.
