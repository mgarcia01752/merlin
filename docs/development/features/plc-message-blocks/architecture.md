> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/plc-message-blocks/`
> Logs, captures and specification images are not published.

# PLC message boundary

Scope: fixed PLC frame geometry, actual timestamp block plus message-channel block,
validated parsing/serialization, bounded opaque MAC-message segments, and ordered
36-byte /288-bit information blocks. No PLC encoder, synthetic OCD/DPD packet,
EM/TR, acquisition scheduling or frame-wide signal generation.

Authority: PHY3.1 I20 §7.5.13.2,.4–.7, PDF149,152–156; MULPI3.1 I25 §6.5.1–4,
PDF215–218, and §7.1.5, PDF227–228. PHY4.0 I07 §7.3.5.13, PDF146 delegates
FDD PLC to3.1. Local text tables are unambiguous about message type and fields;
no claim that MULPI figures were rendered from a PDF. Use original PHY PDF for
bitstream/puncturing diagrams before codec implementation. Extended timestamp
is55 coarse bits,5 modulo20 sample ticks and4 fractional phase bits, not a Unix
nanosecond count. Timestamp points to the end of the PLC preamble.

Separate modules: geometry, timestamp, timestamp_block, message_channel,
information, frame; root only re-exports. CRC algorithm remains fec-codec::crc.
Geometry/timestamp/information support analysis-only builds. CRC-backed frame
operations use the existing simulation feature, as does NCP control coding.

Application subset: one TS then one MC exactly filling720/360 bytes. Reject other
message blocks explicitly. Caller MAC segments are opaque: no claim to validate
MAC packet FCS or descriptor contents. Pointer checks are strict lab bounds,
not a substitute for packet assembly. Default idle frame is not acquisition
signaling; OCD/DPD and scheduling remain TODO.

Tests: independent literal TS/CRC and frame vectors, timestamp carry/overflow,
reserved fields, corrupt CRC, exact frame lengths, pointer boundaries, maximum
payload, both spacings, MSB order, read-only inputs and analysis-only availability.

## Validation outcome

Implemented six focused PLC operations plus module wiring. Twelve new tests pass
in separate geometry, timestamp, TS block, MC block, FEC input and frame test files.
Independent polynomial long division agrees with TS CRC bytes50 BF9F for header10
and timestamp00 02 04 06 08 0A 0D3A. These are synthetic regression values, not
externally generated interoperability vectors. Both FFT frame capacities pass.
Full pre-push gate passed; analysis-only warnings-denied Clippy passed. The JSON
example validates20 information blocks,720 bytes and288 bits/block. Markdown
links/fences and git diff whitespace checks pass. No dependencies were added.

NEXT-02 and NEXT-07A are checked. NEXT-07 stays incomplete for actual acquisition
MAC packets and repetition; NEXT-08 remains the next simulation implementation.
No EM/TR support, observer producer, codec, signal or blind receiver is claimed.
