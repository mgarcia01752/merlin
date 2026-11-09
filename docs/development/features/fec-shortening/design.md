# FEC-03 design: downstream variable-length shortened codewords

Status: implemented and validated; full pre-push gate passes.
Waveform shortened-word integration remains QAM-03. Existing full-block codec callers and SRC file padding remain unchanged.

## Scope and pinned evidence

DOCSIS3.1 PHY I20 §7.5.4, PDF112–114 limits downstream to the short16200-bit
rate8/9 mother code; normal64800-bit frames are explicitly not used. Table23
sets BCH14232/14400 and LDPC14400/16200. FEC-03 therefore covers shortened
lengths without introducing unsupported alternative DVB rates/frame sizes.
DOCSIS4.0 PHY I07 §7.2.5.4 (FDX, PDF77) and §7.3.5.4 (FDD, PDF134) delegate FEC
to that3.1 section; this adds no broader FDX simulation claim.

PHY I20 §7.5.4.2.3 Figures46–47, PDF114–115 defines the six encoder operations.
The local original PDF page115 was rendered and visually checked, including the
zero positions and lengths. §7.5.4.4, PDF119 defines RX zero-valued LLR
restoration and reverse decoding. §7.5.4.2.3.1, PDF116 provides the8096-removed /
8104-transmitted example; its6304 LDPC information bits include168 BCH parity,
so original payload K=6136. Do not call6304 original payload bits.

## FEC-03 module ownership and flow

```mermaid
flowchart LR
    INPUT["K original bits"] --> BCHPAD["Prepend S=14232-K zeros"]
    BCHPAD --> BCH["Existing BCH encoder:14400 bits"]
    BCH --> REMOVE["Remove first S zeros: K+168 bits"]
    REMOVE --> LDPCPAD["Append S zeros:14400 bits"]
    LDPCPAD --> LDPC["Existing LDPC encoder:16200 bits"]
    LDPC --> WIRE["Omit systematic zeros: K+1968 bits"]
    WIRE --> SOFT["Received transmitted-coordinate LLRs"]
    SOFT --> RESTORE["Restore known-zero LLRs before1800 LDPC parity bits"]
    RESTORE --> DECODE["Existing16200-bit LDPC decoder"]
    DECODE --> BCHRESTORE["Reinsert leading BCH zeros"]
    BCHRESTORE --> BCHDEC["Existing BCH decoder"]
    BCHDEC --> OUTPUT["K recovered bits + actual statuses and iterations"]
```

`fec-codec/src/shortening/` owns this independent codec boundary:

- `parameters.rs`: validated K and derived stage sizes.
- `encoding.rs`: separately callable prefix/tail insertion and removal.
- `decoding.rs`: known-zero LLR restoration and BCH input reconstruction.
- `codec.rs`: thin composition of those stages with existing BCH/LDPC codecs.
- `mod.rs`: exports only; no QAM, source, files, bindings or channel dependency.

The same mother matrix/kernel remains authoritative. Intermediate vectors are
returned directly for test points: BCH mother output, padded LDPC input, LDPC
mother output, transmitted word and restored LLRs. Outputs are bounded by mother
sizes. No new algorithms/dependencies or hidden decoder truth inputs are added.

## Geometry and decoding policy

Let S=14232-K, with application admission1≤K≤14232. The nonempty policy is an
API decision, not a claimed CableLabs minimum. K=14232 is the existing full word.
BCH parity remains168, LDPC parity1800, and transmitted N=K+1968. Original-data
rate is K/N; the inner effective rate is (K+168)/N. Those rates differ.

Finite known-zero LLRs use+100 and retained likelihoods saturate to±100 to bound
decoder arithmetic. This is a numerical receiver policy, not a normative RF
threshold or a claim that finite priors cannot be overturned. Any recovered
nonzero omitted LDPC systematic bit or restored BCH prefix makes the shortening
consistency check fail. Preserve actual decoder flags, iterations and failed
hard decisions; overall success also requires that consistency check. Truth BER
is measured independently over K original bits only.

## Validation and remaining integration

Validate all admissible geometries, exact Figure47 coordinates, independent BCH
polynomial/LDPC accumulator reference encodings, mother syndromes, restoration
signs and finite bounds. Exercise full-word compatibility, varied K, boundary
correction, demonstrated non-convergence, actual iterations, malformed inputs
and independent truth comparison despite success flags. Include a runnable
shortened-codeword example and component design/math guides.

Shortened24-column interleaving, QAM final-cell rules, NCP length discovery and
waveform/session admission must be implemented/verified separately under QAM-03.
The current channel does not infer K from TX metadata or silently switch coding
mode for a partial SRC file. External full-channel interoperability remains QA-05.

See the [validation record](validation.md) for independent test evidence and
current host/scope boundaries.

Current integration note: QAM-03 has since connected explicit shortened words
through bit processing, received NCP length discovery and waveform samples. See
the [QAM-02/03 design](../qam-completion/design.md); the codec results above remain
the FEC-03 milestone's validation record.
