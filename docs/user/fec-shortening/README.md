# Downstream FEC codeword shortening

FEC-03 adds true variable-length DOCSIS payload coding while reusing the existing
BCH and16200-bit rate8/9 LDPC mother codecs. A smaller original payload retains
all168 BCH and1800 LDPC parity bits. This differs from appending user-source
padding and transmitting a full word.

| Component | Design |
| --- | --- |
| Validated lengths and effective rates | [Parameters](parameters.md) |
| Six transmit operations and intermediate vectors | [Transmitter](transmitter.md) |
| Known-zero restoration, decoding and comparison | [Receiver](receiver.md) |
| Reusable codec lifecycle and test points | [Composition](codec.md) |

```mermaid
flowchart LR
    SOURCE["K original bits"] --> TX["BCH prefix shortening → LDPC tail shortening"]
    TX --> WIRE["K+1968 transmitted bits / received LLRs"]
    WIRE --> RX["Restore zeros → LDPC → restore BCH prefix → BCH"]
    RX --> RESULT["K recovered bits + actual decoder outcomes"]
    SOURCE -. "Independent truth only" .-> COMPARE["Payload error comparison"]
    RESULT --> COMPARE
```

The local DOCSIS3.1 PHY I20 §7.5.4, PDF112–114 uses only the short16200-bit
rate8/9 downstream mother mode; normal64800-bit frames are not used. The extension
is shortening, not adding other DVB frame sizes/rates. DOCSIS4.0 PHY I07
§7.2.5.4, PDF77 (FDX) and §7.3.5.4, PDF134 (FDD) delegate downstream FEC to
that3.1 section. This codec feature does not implement FDX channel behavior.

The [task design](../../development/features/fec-shortening/design.md) records
verified local PDF diagrams, ownership, numerical policy and validation scope.

## Run the independent codec example

```bash
scripts/cargo-local.sh run -p fec-codec --example shortened_codec_loop --locked
```

The default is K=6136, matching the PHY's8096-omitted-zero example:

```text
Payload bits: 6136 · transmitted bits: 8104 · omitted zeros: 8096
LDPC iterations: 0 · bit errors: 0
```

Select a different original payload length, for example1024 bits:

```bash
scripts/cargo-local.sh run -p fec-codec --example shortened_codec_loop --locked -- 1024
```

This deterministic-pattern example is independent of PRBS, QAM and waveform
simulation. It fails if decoding or the independent comparison fails. Tests also
introduce errors and verify actual iterative correction and non-convergence.

## Integration boundary

The public Rust `fec_codec::shortening` module supports the codec boundary now.
The channel CLI and file source still use full16200-bit words. They do not
silently select shortening at file EOF. Shortened bit interleaving, QAM cell
packing and received length/signaling must be connected and tested under QAM-03.
Direct intermediate vectors are available now; no new channel JSONL stage IDs or
shortened waveform result contract is claimed. See the [API reference](../../reference/public-api.md)
for entry points and the [checklist](../../development/checklist.md) for scope.
