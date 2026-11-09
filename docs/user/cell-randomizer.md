# Cell-word randomization and receiver inverse

`signal_processing::modulation::randomization::CellRandomizer` implements the
downstream **data/NCP** randomizer in PHY I20 §7.5.5.3. Each transmitter and receiver
owns a separate instance. The state persists across cells, codewords and symbols,
with a synchronized reset every 128 OFDM symbols.

The randomizer runs after bit interleaving/demultiplexing and before QAM mapping.
It is not the test-data generator, a payload PRBS source or encryption.

## Supported operations

| Operation | Effect on registers |
| --- | --- |
| `begin_symbol(index)` | Start the next symbol; reset at indices 0,128,256,… |
| `apply_data(order, label)` | XOR a data cell through16K-QAM; clock once |
| `apply_ncp(order, label)` | XOR an NCP cell using QPSK,16-QAM or64-QAM; clock once |
| `zero_loaded_bpsk()` | Return the specified BPSK label; clock once |
| `skip_scattered_pilot()` | Leave the registers unchanged |
| `descramble_data_llrs(order, llrs)` | Flip data LLR signs according to mask; clock once |
| `descramble_ncp_llrs(order, llrs)` | Flip NCP LLR signs according to mask; clock once |
| `state()` | Return an independent diagnostic register snapshot |

Call `begin_symbol(0)` before processing any cells. Index zero means the first
**pre-interleaving** symbol following the PLC preamble, not an arbitrary capture
start. Later indices must be consecutive. The future runner must supply correct
cell order, pilot placeholders and this synchronization reference. The randomizer
does not infer codeword boundaries, count a symbol's expected cells or locate a
PLC preamble. Invalid calls leave both registers and symbol progression unchanged.

Continuous pilots, exclusions and PLC cells are outside this data/NCP stream.
A zero-loaded cell is transmitted as BPSK and advances the sequence, despite
carrying no payload bits. A scattered-pilot placeholder does not advance it.
The receiver must account for the same cells to stay synchronized.

## Field arithmetic and bit ordering

The two registers hold elements of $\mathbb F_{2^{12}}$. The field is defined by

$$
p(\alpha)=\alpha^{12}+\alpha^6+\alpha^4+\alpha+1.
$$

The LFSR polynomial is $x^2+x+\alpha^{11}$. From the old state, one clock computes

$$
D_0'=\alpha^{11}D_1\pmod{p},\qquad D_1'=D_0\oplus D_1.
$$

The synchronized initial values are $D_0=\mathtt{0x555}$ and
$D_1=\mathtt{0xAAA}$. Each register's bit zero represents the coefficient of
$\alpha^0$.

For a $b$-bit data cell, the mask bits are

$$
r_k=\begin{cases}
D_0[k],&0\le k<\min(b,12),\\
D_1[k-12],&12\le k<b,
\end{cases}
\qquad z_k=y_k\oplus r_k.
$$

Here $y_0$ is the **most significant** bit of the mapper's integer cell label.
The integer mask is therefore

$$
R_b=\sum_{k=0}^{b-1}r_k2^{b-1-k}.
$$

Do not directly XOR the integer D0 register with an MSB-first label. At the seed
state, a256-QAM cell uses mask `0xAA`, not `0x55`. The13-bit mask uses all12 D0
bits followed by D1[0]; the14-bit mask adds D1[1]. NCP cells use2,4 or6 D0 bits.

The register is clocked once **after** consuming each data/NCP/zero-loaded cell.
A zero-loaded cell uses $z_0=D_0[0]$: label0 maps to $(+1,0)$ and label1 to $(-1,0)$.

## Receiver processing

XOR is its own inverse, so applying the same mask to a randomized hard label
recovers the original label. For soft bits with $L_k=\ln(P_0/P_1)$:

$$
L_k^{plain}=(-1)^{r_k}L_k^{randomized}.
$$

The inverse preserves confidence magnitudes and changes only the signs selected
by the mask. It does not make premature hard decisions. LLR inputs must have the
correct length and finite values; the returned vector is owned and inputs remain
unchanged.

```rust
use docsis_ofdm_fec::signal_processing::modulation::{
    map_symbol, qam_llrs, randomization::CellRandomizer,
};

let mut transmitter = CellRandomizer::new();
let mut receiver = CellRandomizer::new();
transmitter.begin_symbol(0)?;
receiver.begin_symbol(0)?;
let transmitted_label = transmitter.apply_data(256, 0x35)?;
let point = map_symbol(256, transmitted_label)?;
let llrs = qam_llrs(256, point, 0.01)?; // Finite variance reference; no noise added.
let original_soft_bits = receiver.descramble_data_llrs(256, &llrs)?;
let recovered = original_soft_bits.iter()
    .fold(0u16, |label, value| (label << 1) | u16::from(*value < 0.0));
assert_eq!(recovered, 0x35);
# Ok::<(), docsis_ofdm_fec::SimulationError>(())
```

## Evidence, tests and integration boundary

Source: CM-SP-PHYv3.1-I20-230419 §7.5.5.3, Figure52, PDF123–124. Its published
example has cells `{A0,SP,A1,Z,A2}` and register states `D1 & D0` before each cell:

```text
AAA555 → FFFADF → FFFADF → 520799 → 2B9828
```

Tests match these exact states, check MSB/LSB masks through16K, verify symbol-reset
and skip behavior, and recover all256-QAM labels through mapping, soft demapping
and descrambling. Mixed data/NCP/zero-loaded streams verify synchronization and
invalid-input tests verify unchanged state.

The zero-loaded example also checks BPSK label1 maps to(-1,0), as stated on PDF124
and AnnexA Figure99, PDF197. The imported BPSK/QPSK geometry tables need explicit
build-time label permutations to match Figures99–100; original files are retained
unchanged. Independent tests verify all control labels.

**[PLC randomization](plc-randomization.md) is implemented separately.**
Section7.5.13.8 uses seeds D0=0x4A7,D1=0xB4C and one clock after each PLC
carrier, including the final carrier before the next symbol; it must not use this
data/NCP instance. PLC preamble randomization is prohibited. No actual NCP message
is created by this API; the separate [NCP codec](ncp.md) creates messages.
The existing16-QAM CLI experiment remains at its earlier
coding boundary. The separate [payload transport](payload-transport/README.md)
now wires this module into fixed256-QAM TX/RX with actual waveform recovery. See [transmission scope](transmission.md) and
[QAM mapping](qam-mapping.md).
