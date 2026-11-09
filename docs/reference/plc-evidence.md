# PLC message boundary evidence

Pinned sources: CM-SP-PHYv3.1-I20-230419, CM-SP-MULPIv3.1-I25-230419 and
CM-SP-PHYv4.0-I07-250627. PHY3.1 Table21/PDF96 and Figures63–68/PDF153–157 were inspected in the local
original PDF. MULPI field definitions and bounds below were checked against the
versioned text extraction; no rendered MULPI figure verification is claimed.

| Source | Endpoint and requirement | Implementation / status |
| --- | --- | --- |
| PHY3.1 §7.5.13.2, PDF149; MULPI3.1 §6.5.1/Table74, PDF215 | CMTS constructs eight preamble and 120 data symbols; frame holds ten or twenty FEC blocks | `PlcFrameGeometry`, generated data/preamble columns and full frequency-grid tests; waveform pending |
| MULPI3.1 §6.5.1–2/Table75, PDF215–216 | CMTS places exactly one TS block first after preamble; header type1, reserved zero, eight-byte timestamp and CRC-24-D | `TimestampMessageBlock`, TS-first frame composition; independent CRC division tests |
| MULPI3.1 §7.1.5, PDF227–228 | Extended timestamp uses55 coarse bits, modulo20 sample phase and four lower phase bits | `ExtendedTimestamp`; explicit sample units and overflow checks |
| MULPI3.1 §6.2.1.2, PDF83 | Multioctet numeric fields use byte-big-endian order | Timestamp and pointer network-order vectors |
| MULPI3.1 §6.5.4/Table77, PDF218 | CMTS places MC last when present, fills idle channel with0xff; optional two-byte packet pointer | `MessageChannelBlock` and exact TS+MC frame; strict pointer policy validates buffer bounds only |
| PHY3.1 §7.5.13.5/Figure63, PDF153 | CMTS converts bytes to bits MSB first | `PlcInformationBlock`; independent literal bit vectors |
| PHY3.1 §7.5.13.6, PDF153 | CMTS uses (384,288) punctured PLC LDPC from (480,288) rate3/5 mother | Separate `fec_codec::plc` encoder/decoder; mother basis/parity, puncture and noiseless tests pass |
| PHY3.1 §7.5.13.7/Figures65–67, PDF154–156 | CMTS groups384 bits into96 MSB-first nibbles, writes rows and reads columns:4K8x12,8K16x6 | Separate validated grouping/permutation; full coordinate/basis tests, exact soft inverse and full-frame coding recovery; bounded label probes |
| PHY3.1 §7.5.13.8/Figure68, PDF157 | CMTS uses independent PLC seeds4A7/B4C, clocks after each carrier, resets after preamble and never randomizes preamble | Shared GF primitive with independent PLC instances; MSB-first mask0xE, continuous block cadence, soft sign inverse and multi-frame tests |
| PHY3.1 §7.5.13.4, PDF152; MULPI3.1 §6.4.40–41, PDF197–201 | CMTS communicates fixed OFDM and profile configuration via OCD/DPD | Fixed OCD/ProfileA/NCP descriptors, management MAC header/HCS/FCS,200ms-default repeat schedule and MC reassembly; generated-subset receiver tests |
| PHY4.0 §7.3.5.13, PDF146 | FDD PLC delegates to PHY3.1 §7.5.13 | Same message-boundary implementation; no FDX claim |

The current supported subset has no EM or trigger blocks. It deliberately rejects
those between TS and MC instead of parsing them as MAC data. A valid TS+idle-MC
frame alone does not establish OFDM acquisition or a complete CMTS transmitter.

Reserved-field rejection is an explicit strict laboratory parsing policy. MULPI
§6.1 (PDF82) requires a transmitting CMTS to zero reserved fields and a receiving
CM to ignore them. These validators do not claim that strict rejection implements
the normative CM receive policy.

## Coding source discrepancies and interpretation

§7.5.13.6/PDF153 names Table20 while referring to the fine-ranging (480,288)
mother code. §7.4.15.2.3/PDF96 explicitly defines it in Table21, with lifting48;
Table20 is a different CP/roll-off table. The implementation uses Table21.

Figures64/65/PDF154 print final retained parity coordinates ending at469. The
explicit §7.5.13.6 ranges instead delete48..95 and384..431 from a480-bit mother
word, leaving384 bits and ending at479. The code uses these explicit ranges and
ascending retained coordinates. This interpretation is recorded rather than
silently changing the specification extracts. No shortening is applied to PLC.

The reference decoder is an application implementation, not a prescribed CM
algorithm: horizontal-layered sum-product, finite LLR saturation +/-100 and1..100
iterations. Tests distinguish parity convergence, message CRC and payload truth.

## Completed signal and acquisition boundaries

| Source | Endpoint and requirement | Implementation / status |
| --- | --- | --- |
| PHY3.1 §7.5.13.3/Tables48/49, PDF151–152 | CMTS uses the specified eight BPSK preamble columns;0→+1,1→−1 | Separate preamble module; all columns independently transcribed from rendered local PDF and tested |
| PHY3.1 §7.5.13.9/PDF158, AnnexA | CMTS maps randomized nibbles using16-QAM with1/sqrt(10) scaling | Shared label table, explicit mapper/soft inverse and energy/sign tests |
| PHY3.1 Figure62/PDF151 and §7.5.15/PDF171 | Combine PLC/data/pilots only at the physical-bin boundary | Protected assembler, actual cell-interleaver history and both-spacing full-cycle tests |
| MULPI3.1 §6.2.3.2/PDF88–89; §6.4.1/PDF99–103; AnnexA/PDF586 | Management MAC framing, HCS, multicast/LLC/version/type and message FCS | Single-part acquisition packets; CRC check strings, fixed header vectors and byte-corruption rejection |
| MULPI3.1 §6.4.40–41/Tables69/70/PDF197–203 | OCD fixed parameters/protected assignments and ProfileA/NCP modulation | Bounded range-TLV builder, mandatory edge checks and received OCD validation using the existing physical map |
| MULPI3.1 §6.5.4/PDF218; AnnexB/PDF592 | MC continuation, first-new-start pointer, idle fill;200ms default/250ms maximum repeats | Integer sample-clock scheduler and bounded receiver-owned reassembly; segmented real FEC/QAM recovery |

The acquisition subset uses range TLVs and single-part MMMs, with uniform256-QAM
ProfileA and uniform2/4/6-bit NCP. Oversized packets and unsupported received
formats fail explicitly. MC segmentation is separate from multipart MMMs. EM/TR
modes and a live modem's channel-acquisition state machine remain outside this
fixed reference scope. MULPI requirements above were checked against text; no
rendered MULPI-figure verification or external interoperability is claimed.

The full reproducible boundary is documented in [PLC channel](../user/plc-channel.md).
