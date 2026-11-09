# NCP control encoding and receiver discovery

Next Codeword Pointer (NCP) messages identify codeword starts within an OFDM
symbol. The Rust implementation now creates real message bytes, CRC and control
FEC, maps them into QAM labels, and decodes the field back into data intervals.
This closes the signaling gap between the [symbol packer](symbol-packing.md)
and a receiver that needs to discover payload boundaries.

This is a pre-interleaving, noiseless reference path. A full OFDM waveform,
PLC signaling and dynamic profile switching remain outside this stage.
[Cell interleaving](cell-interleaving.md) now transports these mapped NCP cells
and reverses the operation at the receiver. The ordinary `ofdm-fec simulate` CLI
remains its separate16-QAM experiment; the public API and example below exercise NCP functionality.

## Run the example

From the repository root:

```bash
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example ncp_field --locked
```

The example sends a profile-A start at data position 0 and a zero-loaded filler
marker at position 2025. It prints JSON containing recovered messages, CRC bytes,
consumed field cells, each control block's actual LDPC iteration count and
pointer-derived data spans. With QPSK, two messages plus CRC consume 72 cells.
The isolated example bypasses cell randomization; the cross-symbol integration
test includes shared data/NCP randomization and actual payload recovery.

```mermaid
flowchart LR
    PLAN["Packer: starts, NULL or Z"] --> MSG["Three-byte messages"]
    MSG --> CRC["CRC-24-D over message bytes"]
    MSG --> FEC["Each message: NCP LDPC"]
    CRC --> FEC
    FEC --> LABEL["QPSK / 16-QAM / 64-QAM labels"]
    LABEL --> RAND["Shared symbol randomization"]
    RAND --> IQ["Noiseless mapping and soft demapping"]
    IQ --> INV["Inverse randomization and label grouping"]
    INV --> DEC["NCP LDPC decode until L, then CRC block"]
    DEC --> CHECK["Accept complete field only if CRC matches"]
    CHECK --> SPANS["Decoded pointers → payload boundaries"]
```

## Message layout

Each unencoded message is three bytes, in MSB-first byte order:

| Byte | Bits, MSB to LSB |
| --- | --- |
| 0 | Profile ID (4), Z, C, N, L |
| 1 | T, U, R, pointer bits 12..8 |
| 2 | Pointer bits 7..0 |

Z starts zero-loaded filler; C selects even/odd data profile; N selects even/odd
NCP profile; L marks the last message before CRC; T is codeword tagging; U signals
an NCP profile update. R is reserved. Figures 71–72 show an older reserved label
where the later Figure 92/Table 53 defines U; the serializer preserves U at bit 6
of byte 1 and treats only bit 5 as R.

Pointers count pre-interleaving data positions, skipping scattered-pilot
placeholders. They are not physical FFT-bin indices. `0x1fff` is NULL; other
13-bit values represent starts. Within a chain pointers increase, and a NULL or Z
marker can appear only last. N and U must agree across the chain.

`encode_packed` selects even data/NCP profiles, U=0 and T=0 for this fixed-setup
experiment. Generic message APIs preserve control flags but do not implement the
128-symbol profile-update sequence, OPT state or PLC profile announcements. The
strict laboratory parser rejects R=1 and Z+NULL together. A session also needs
spacing-specific rolling start limits: the packer enforces ten starts across
any two consecutive 4K symbols; a field decoder alone has no previous-symbol state.

## CRC and control FEC equations

CRC-24-D covers the unencoded message bytes in field order, excluding FEC parity.
The generator is

$$
g(x)=x^{24}+x^{22}+x^{20}+x^{19}+x^{18}+x^{16}+x^{14}
+x^{13}+x^{11}+x^{10}+x^8+x^7+x^6+x^3+x+1.
$$

For message polynomial $m(x)$, append the remainder

$$
r(x)=x^{24}m(x)\bmod g(x).
$$

The implementation uses zero initialization, no reflection and no final XOR.
The CRC is three MSB-first bytes and gets its own NCP FEC block.

The NCP code is independent of the payload BCH/LDPC chain. Its mother code is the
rate-1/2 (160,80) binary LDPC from PHY Table 19, with 16×16 right-shifted identity
blocks. Its systematic codeword satisfies

$$
H\begin{bmatrix}a\\b\end{bmatrix}=0\quad\text{over GF}(2).
$$

Place the 24 message bits at $a_0,\ldots,a_{23}$ and set
$a_{24},\ldots,a_{79}=0$. Here $a_0$ is byte 0's MSB, following the diagram's
coordinate labels. Transmit retained coordinates

$$
c=(a_0,\ldots,a_{23},b_{104},\ldots,b_{111},b_{128},\ldots,b_{143}).
$$

There are 48 transmitted bits: 24 information and 24 retained parity bits.
The other 56 mother parity coordinates are punctured. At the receiver those
punctures receive zero LLR; known shortened zeros receive +100. Observed finite
LLRs are clipped to ±100. Positive LLR favors zero. The decoder reports actual
iterations and success; a convergence failure rejects the field.

## QAM label order and placement

NCP grouping differs from ordinary MSB-first grouping. For $w=\log_2 M$, label $j$
uses retained-coordinate bits in reverse order within each group:

$$
\ell_j=\sum_{k=0}^{w-1} c_{jw+k}\,2^k.
$$

Thus the first QPSK label is $(a_1,a_0)$, the first 16-QAM label is
$(a_3,a_2,a_1,a_0)$, and the first 64-QAM label is $(a_5,\ldots,a_0)$.
The soft inverse reverses each cell's MSB-first LLR group before FEC decoding.
Shared constellation references provide the normalized I/Q coordinates.

One encoded block occupies $48/w$ cells: 24 for QPSK, 12 for 16-QAM or 8 for
64-QAM. Field labels fill the highest eligible position downward, skipping
pilot placeholders. Use each `PackedCell::NcpReserved::field_cell` index to pick
its label before shared randomization. The packer adapter verifies the reserved
region, its count and its pointer domain before returning labels.

## Public Rust API and ownership

- `fec_codec::crc::crc24d` calculates CRC over borrowed bytes.
- `fec_codec::ncp::NcpFecCodec` owns reusable control-code encoder/decoder setup.
- `transmission::ncp::NcpMessage` owns typed fields and serializes three bytes.
- `NcpFieldCodec::encode_packed` converts existing packer plans into real control
  labels, discarding diagnostic codeword IDs. `encode_messages` accepts explicit
  validated chains when control flags are supplied by the caller.
- `decode_field` takes high-to-low, descrambled cell LLRs for the known NCP
  modulation. It reads until L, then consumes one CRC block. Trailing hypotheses
  for payload cells may follow; exact transmitter field length is not required.
- `DecodedNcpField::data_spans` validates pointers against the remaining data
  count and returns ordered intervals. A prefix before the first start continues
  a previous codeword; its profile comes from receiver session state.

The receiver input is bounded to 7537 cells and at most 12 blocks are decoded.
The per-block iteration cap is 1..100. Invalid input, truncation, FEC failure,
CRC mismatch or malformed chains return errors without partial messages. Decoder
scratch state is job-owned; no transport, filesystem or GUI objects are required.
Codec-backed field APIs require the existing `simulation` feature. Message
serialization remains available without that feature.

## Validation and limits

Tests verify both published Annex E vectors (`01..07 → cd ef 27` and
`01..ff → 2c a8 8b`), all 24 information basis cases against an independently
written triangular parity solver, a hand-solved impulse parity vector, selected
error correction, bounded decoding failures and malformed chains. A valid FEC
block containing the wrong CRC is rejected as a whole-field CRC failure.

A separate cross-symbol test uses real NCP FEC/CRC and shared randomizer state.
It recovers four BCH/LDPC payloads without bit errors for each NCP modulation,
across the 128-symbol reset. Receiver boundaries come from received NCP pointers,
not packer codeword IDs, offsets or reservation counts. Its pilot mask and small
interleaver domain are synthetic; this is not external DOCSIS interoperability
or full RF-channel validation.

Three major stages remain: pilot/PLC signal generation;
IFFT/prefix/windowing and inverse receiver;
and streamed one-second noiseless integration with warm-up and drain.

Evidence: CM-SP-PHYv3.1-I20-230419 §7.4.3.2, PDF56; Table19, PDF91;
§7.5.14.1–5 and Figures71–79, PDF159–164; §8.3.4.1–3 and Figure92/Table53,
PDF180–184; AnnexE, PDF210. The local original PDF was inspected for the matrix,
message layout and bit/label diagrams. External interoperability vectors remain
outstanding.
