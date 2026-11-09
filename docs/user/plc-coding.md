# PLC LDPC coding

PLC information blocks now pass through a separate Rust LDPC encoder and soft
decoder. This code is independent of downstream payload BCH/LDPC and NCP control
LDPC. It uses the existing safe Rust `ldpc-toolbox` dependency; no new dependency
or native codec is introduced.

```bash
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example plc_codec_loop --locked
```

The example recovers one timestamp-plus-idle-MC frame at each spacing. It reports
actual iterations per block, compared information bits, bit errors, recovered
message equality and the first block's mother/transmitted arrays. Its receiver
gets only transmitted likelihoods; original bytes are used afterward for truth
comparison. It is a bounded coding experiment, not a modulated waveform run.

```mermaid
flowchart LR
    SOURCE["PLC information bytes · TS and MC"] --> BITS["36 bytes → 288 MSB-first bits"]
    BITS --> MOTHER["Mother LDPC encoding · 480 bits"]
    MOTHER --> PUNCTURE["Delete coordinates 48–95 and 384–431"]
    PUNCTURE --> TX["384 transmitted bits"]
    TX --> SOFT["Noiseless finite soft values"]
    SOFT --> RESTORE["Restore punctures as zero-LLR erasures"]
    RESTORE --> DEC["LDPC soft decoder · actual iterations"]
    DEC --> CHECK["Complete mother syndrome and convergence"]
    CHECK --> BYTES["288 information decisions → 36 bytes"]
    BYTES --> MESSAGE["Reassemble frame and validate timestamp CRC"]
    MESSAGE --> TRUTH["Compare original and recovered information"]
    SOURCE -->|"Independent truth only"| TRUTH
```

## Matrix and puncturing

The mother code is a systematic $(480,288)$ rate-$3/5$ binary LDPC code. The
parity-check matrix $H$ has 192 rows and 480 columns, built from the $4\times10$
compact matrix in PHY3.1 I20 Table21 (PDF96), lifted by $L=48$. For nonnegative
shift $s$ in block $(i,j)$:

$$
H_{48i+r,\,48j+(r+s)\bmod48}=1,\qquad r=0,\ldots,47.
$$

A dash denotes an all-zero block. Encoding solves for parity $p$ such that

$$
x=[a_0,\ldots,a_{287},b_{288},\ldots,b_{479}],\qquad Hx^T=0\pmod2.
$$

PLC performs **no shortening**. It deletes these mother-coordinate ranges:

$$
\mathcal P=\{48,\ldots,95\}\cup\{384,\ldots,431\}.
$$

The retained ascending sequence is

$$
\mathcal R=(0,\ldots,47,\;96,\ldots,383,\;432,\ldots,479).
$$

Output is $c_i=x_{\mathcal R_i}$, containing 240 directly transmitted information
bits and 144 parity bits. All 288 input bits still belong to the code; the decoder
must recover the 48 punctured information bits from parity constraints. The
transmitted code has rate $288/384=3/4$.

The explicit ranges in §7.5.13.6 determine the implementation. That paragraph
refers to Table20, but the delegated fine-ranging mother matrix is Table21.
Figures64/65 also label the final retained parity coordinate as469, inconsistent
with the stated dimensions and ranges. This implementation ends at479, as
required by the complete coordinate definition and 384-bit length. The versioned
specification extracts have not been edited; these source discrepancies are
recorded in [PLC evidence](../reference/plc-evidence.md).

## Soft decoding and result interpretation

LLRs use natural-log $P(0)/P(1)$, so positive values favor zero:

$$
\lambda_j=\begin{cases}
\operatorname{clip}(\ell_i,-100,100),&j=\mathcal R_i,\\
0,&j\in\mathcal P.
\end{cases}
$$

A puncture has zero LLR because it is unknown; it is not assigned a known-zero
prior. Saturation at 100 is an application numerical policy, not a mandated
receiver setting. Inputs must be finite. Decoder work is bounded to 1–100
iterations, also an application policy.

The codec uses horizontal-layered sum-product decoding and reports actual
iterations. `success` requires decoder convergence and an independently evaluated
zero syndrome for the complete mother word. On failure the final decisions and
iteration count remain available as diagnostics.

Parity success is distinct from truth recovery and message CRC success. For
example, LLRs favoring a different valid codeword can converge while every
information bit differs from the original. A test records that distinction. The timestamp
CRC protects the timestamp block; it does not validate the entire MC segment.
Embedded MAC packet CRCs and descriptor interpretation remain separate work.

## Public Rust operations and modules

| Owner | Operation/data |
| --- | --- |
| `fec_codec::plc::parameters` | Dimensions, retained coordinates and application limits |
| `fec_codec::plc::PlcFecCodec::encode_mother` | 288 binary bits to complete 480-bit mother word |
| `fec_codec::plc::puncturing::puncture` | Standalone validated 480-to-384 coordinate selection |
| `fec_codec::plc::puncturing::restore_llrs` | Standalone finite-input validation, saturation and erasure placement |
| `fec_codec::plc::PlcFecCodec::{encode, decode, valid_syndrome}` | Composed coding, soft reception and independent parity check |
| `fec_codec::plc::PlcBlockResult` | Information/mother decisions, actual iterations, convergence; byte packing helper |
| `transmission::plc::PlcBlockCodec` | Thin adapter from typed `PlcInformationBlock` to serializable stage results |
| `transmission::plc::{EncodedPlcBlock, DecodedPlcBlock}` | Direct mother/transmitted arrays or diagnostic receive results |

The primitive keeps matrix, setup, encoding, decoding and puncturing in separate
modules. Setup is reused per job. Inputs are borrowed and unchanged; outputs own
fixed-size arrays or bounded vectors. There is no filesystem, GUI or transport
dependency in the codec. The message adapter uses the `simulation` feature and
maps primitive errors to structured `LabError` categories.

Direct arrays provide test points at the mother, transmitted, restored LLR and
receive-decision boundaries. Registration with the common optional observer and
CLI export remain TODO; this step does not silently claim those producers exist.

## Validation and next stage

Independent triangular parity equations verify all 288 information basis vectors
plus zero, all-one and patterned words. Separate tests cover exact puncture
boundaries, zero-erasure restoration, complete syndromes, noiseless recovery of
punctured information, actual iterations, a specified weak wrong-sign correction,
a contradictory one-iteration failure, invalid inputs and parity-versus-truth.
Complete 360/720-byte message frames also recover with zero information bit errors
at both spacings, followed by timestamp CRC validation.

These are specification-derived tests, not externally generated interoperability
vectors or a general error-correction-distance guarantee. There is no PLC BCH
stage. [PLC block interleaving](plc-interleaving.md) and its hard/soft inverse
are now implemented separately. Acquisition descriptors, preamble,
16-QAM mapping and physical-bin insertion remain separate stages. The next coding
boundary is **NEXT-11: PLC preamble**;
[PLC randomization](plc-randomization.md) now has independent state and a soft inverse. See the
[message guide](plc-messages.md), [architecture](../architecture.md) and
[checklist](../development/checklist.md).
