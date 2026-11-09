# PLC message boundary evidence

Pinned sources: CM-SP-PHYv3.1-I20-230419, CM-SP-MULPIv3.1-I25-230419 and
CM-SP-PHYv4.0-I07-250627. PHY3.1 Figure63/PDF153 was inspected in the local
original PDF. MULPI field definitions and bounds below were checked against the
versioned text extraction; no rendered MULPI figure verification is claimed.

| Source | Endpoint and requirement | Implementation / status |
| --- | --- | --- |
| PHY3.1 §7.5.13.2, PDF149; MULPI3.1 §6.5.1/Table74, PDF215 | CMTS constructs eight preamble and 120 data symbols; frame holds ten or twenty FEC blocks | `PlcFrameGeometry`; dimension tests pass; waveform pending |
| MULPI3.1 §6.5.1–2/Table75, PDF215–216 | CMTS places exactly one TS block first after preamble; header type1, reserved zero, eight-byte timestamp and CRC-24-D | `TimestampMessageBlock`, TS-first frame composition; independent CRC division tests |
| MULPI3.1 §7.1.5, PDF227–228 | Extended timestamp uses55 coarse bits, modulo20 sample phase and four lower phase bits | `ExtendedTimestamp`; explicit sample units and overflow checks |
| MULPI3.1 §6.2.1.2, PDF83 | Multioctet numeric fields use byte-big-endian order | Timestamp and pointer network-order vectors |
| MULPI3.1 §6.5.4/Table77, PDF218 | CMTS places MC last when present, fills idle channel with0xff; optional two-byte packet pointer | `MessageChannelBlock` and exact TS+MC frame; strict pointer policy validates buffer bounds only |
| PHY3.1 §7.5.13.5/Figure63, PDF153 | CMTS converts bytes to bits MSB first | `PlcInformationBlock`; independent literal bit vectors |
| PHY3.1 §7.5.13.6, PDF153 | CMTS uses (384,288) punctured PLC LDPC from (480,288) rate3/5 mother | Dimensions documented; encoder/decoder not implemented |
| PHY3.1 §7.5.13.4, PDF152; MULPI3.1 §6.4.40–41, PDF197–201 | CMTS communicates fixed OFDM and profile configuration via OCD/DPD | Descriptor MAC packets, packet CRCs and repetition scheduling pending; opaque MC input does not establish this requirement |
| PHY4.0 §7.3.5.13, PDF146 | FDD PLC delegates to PHY3.1 §7.5.13 | Same message-boundary implementation; no FDX claim |

The current supported subset has no EM or trigger blocks. It deliberately rejects
those between TS and MC instead of parsing them as MAC data. A valid TS+idle-MC
frame alone does not establish OFDM acquisition or a complete CMTS transmitter.

Reserved-field rejection is an explicit strict laboratory parsing policy. MULPI
§6.1 (PDF82) requires a transmitting CMTS to zero reserved fields and a receiving
CM to ignore them. These validators do not claim that strict rejection implements
the normative CM receive policy.
