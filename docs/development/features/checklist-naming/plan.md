> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/checklist-naming/`
> Logs, captures and specification images are not published.

# Component checklist naming

Rename the maintained PLC-series NEXT identifiers to PLC, preserving their numbers,
status and existing07A extension. Apply stable component-local prefixes to future
checklist groups through the architecture and coding-style skill. Keep
RX-CONFIG-01 as the next prerequisite; no scientific or scheduling change.
Also correct the stale common-observer producer count in the checklist.

Validation: check unique IDs and unchanged checkbox statuses, no old numbered
NEXT references in maintained docs/skills, relative links and whitespace; validate
canonical coding-style skill and both discovery links. No Rust source changes.

Follow-up: split the scientific foundation into seven component sections, place
each user-guide link with its own items and preserve every checkbox/ID/item text.

Follow-up: separate transforms, prefix, transmit window/overlap, reference channel,
receiver, observation and waveform integration, preserving all item IDs/order/status.
Link planned stages to existing design references rather than implying waveform
implementation or creating placeholder model guides.
