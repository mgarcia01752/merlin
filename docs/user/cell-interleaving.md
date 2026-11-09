# OFDM cell time and frequency interleaving

The Rust engine now interleaves mapped cells across time and frequency, then
reverses both operations at the receiver. This spreads a short burst or narrow
interferer across different codewords instead of concentrating its damage.
The current integration is noiseless; no burst-noise performance result is claimed.

This stage is separate from [codeword bit processing](qam256-coding-loop.md).
Bit processing rearranges bits within a codeword. Cell interleaving transports
already mapped I/Q points, NCP cells and scattered-pilot placeholders across a
stream of OFDM symbols. It leaves continuous pilots, PLC and excluded bins out
of that stream.

## Run the example

From the repository root:

```bash
./scripts/cargo-local.sh run -p docsis-ofdm-fec --example cell_interleaving --locked
```

This bounded example sends four symbols of synthetic integer tags through a
matched transmitter and receiver. JSON shows two startup symbols with unavailable
cells, then the four original symbols restored after a two-symbol delay. The
example uses NI=129 and depth M=3 to exercise dummy padding and a partial frequency
column; it is an algorithm demonstration, not a valid complete RF allocation.

```mermaid
flowchart LR
    MASK["Physical pilot pattern → inverse reference"] --> PACK["Packer and NCP codec"]
    PACK --> MAP["Shared randomization → QAM cells"]
    MAP --> TIME["TX time delays"] --> FREQ["Frequency permutation"]
    FREQ --> PHY["Physical data/pilot bins"]
    PHY --> INVF["Inverse frequency permutation"]
    INVF --> INVT["Complementary RX time delays"]
    INVT --> RX["Original symbol phase → inverse randomization"]
    RX --> NCP["Decode NCP and CRC → payload boundaries"]
    NCP --> FEC["Soft bits → LDPC/BCH → compare payload"]
```

## Fixed settings and ownership

`signal_processing::interleaving::InterleaverSettings::new(spacing, cells, depth)`
validates the fixed reduced domain. NI includes data, NCP and scattered-pilot
placeholders. NI is at most 7537 for 25 kHz or 3745 for 50 kHz. Depth M supports
all integers 1..16 or 1..32 respectively. Small NI is allowed for algorithm tests;
it does not establish a valid channel layout by itself.

`SymbolInterleaver<CellValue>` composes time then frequency for `Transmit`, and inverse
frequency then complementary time delays for `Receive`. Create independent
objects for each direction and keep them for the entire job. Their inputs are
borrowed slices of `Option<CellValue>`; outputs own cloned values. `None` marks unavailable
startup/drain history, never a zero sample or payload bit. The generic transform
does not modify numerical values, labels or metadata inside CellValue.

Call `process_symbol(index, input)` with consecutive indices beginning at zero
and exactly NI cells. Length, sequence and overflow errors leave history
unchanged. The frequency-only API also exposes the read-only output-to-input
index map and its inverse operation. No codec, Qt, Python or filesystem dependency
is needed for these algorithms, including analysis-only builds.

## Time interleaver equations

For depth M and NI reduced positions, define

$$
J=\left\lceil\frac{N_I}{M}\right\rceil,
\qquad N_{\mathrm{dummy}}=JM-N_I.
$$

Branch b has bJ cell-storage positions. The commutator visits each branch J times
per padded symbol and starts each symbol at branch zero. Therefore reduced rank
p stays at the same rank but is delayed by b=p mod M whole symbols:

$$
y_t[p]=x_{t-b}[p],\qquad b=p\bmod M.
$$

The implementation clocks conceptual dummy positions at the end of each symbol
and discards their outputs. They never become exported payload cells. Each
receiver branch uses (M-1-b)J storage positions, giving the matched pair

$$
\widehat{x}_t[p]=x_{t-(M-1)}[p].
$$

Thus pair latency is M-1 complete symbol periods, not a fixed sample delay shared
by every transmitter branch. M=1 gives no time delay. Neither time state nor its
commutator is reset per codeword or at the 128-symbol randomizer boundary.

Each direction retains

$$
B=J\sum_{b=0}^{M-1}b=J\frac{M(M-1)}{2}
$$

optional cell positions, independent of job duration. The memory occupied by an
individual generic CellValue remains a caller concern.

## Frequency permutation equations

For the 128-row store,

$$
K=\left\lceil\frac{N_I}{128}\right\rceil,
\qquad C=N_I-128(K-1),\quad1\le C\le128.
$$

Write input cells along rows addressed by seven-bit reversal. Rows below C have
K cells; the remaining rows have K-1. All columns except the last rotate toward
increasing row indices. Column zero rotates by 17. Subsequent rotations use

$$
s_0=17,\qquad
s_{j+1}=(s_j\gg1)\oplus((s_j\mathbin{\&}1)\ll5)
\oplus((s_j\mathbin{\&}1)\ll4).
$$

This follows the bit orientation of Figure 57's GF(64) register. The initial
rotations are 17, 56 and 28. A cell at row r moves to (r+s_j) mod 128. The last
column is never rotated, even when full.

Read columns in bit-reversed order using ceil(log2 K) bits. For a non-power-of-two
K, fall back to the original column when its reversed index is outside K.
For example K=3 reads 0,2,1 and K=5 reads 0,4,2,3,1. The resulting mapping is a
bijection and is identical every symbol for fixed NI: the LFSR restarts at 17.
It is not a continuously changing frequency shift.

The normative text's aside saying C=0 for exact multiples conflicts with its
formula. This implementation uses C=128 in that case, consistent with the formula
and Appendix I's sample algorithm. The column bit width follows ordinary binary
reversal for power-of-two K and the sample's explicit ceil(log2 K) construction.
Appendix I is an informative cross-check; Figures 56–58 and section 7.5.6.2 are
the normative basis.

## Physical bins and pilot reference

`transmission::layout::PilotReference::new(layout, depth)` uses the existing
validated `CarrierLayout`. Its `physical_bins()` associates reduced ranks with
physical bins in ascending order; PLC, continuous pilots and exclusions are
absent. Its `settings()` supplies the matching NI/spacing/depth configuration.
Its borrowed `roles(symbol_index)` feeds the [symbol packer](symbol-packing.md).

For each of 128 physical phases, remove protected bins and inverse-frequency-map
the scattered-pilot pattern to U. Compensate each transmitter branch's delay:

$$
P_t[p]=U_{(t+(p\bmod M))\bmod128}[p].
$$

After time interleaving this becomes U at the physical symbol's phase, then the
frequency permutation returns the required scattered-pilot locations. This is
equivalent to the specification's inverse time/frequency reference with the
matched-pair delay removed to select the proper 128-symbol segment. Phase zero
uses the existing convention: first symbol following the PLC preamble.

The reference transports **placeholders**. Actual scattered-pilot BPSK signs and
boost are available from the separate [pilot generators](pilot-signals.md); full
[physical-bin insertion after interleaving](plc-channel.md) is now implemented,
including PLC signals. Automatic additional continuous-pilot placement remains pending. PLC message/coding and
[block interleaving](plc-interleaving.md) are implemented separately.
The caller supplies the existing layout's
additional continuous pilots; the reference does not certify their generation.

## Startup, drain and verification

Cold transmitter history contains `None`. After M-1 symbols every transmitter
branch has sufficient history; a matched receiver returns all original input
cells after the same M-1-symbol pair delay. Continue the pair with M-1 all-None
symbols to drain a finite input stream. Do not count those markers as payload.
A future continuous waveform runner must establish valid channel startup/filler
signals and separate warm-up from its one-second observation.

The receiver must use the recovered **source-symbol index** for inverse cell
randomization, rather than its later channel-output index. Otherwise LFSR reset
phase can be wrong at the 128-symbol boundary.

Tests verify hand-calculated frequency indices, permutations at partial/full
column boundaries and both maximum NI values, every supported time depth, exact
cell delays, startup/drain and invalid-call atomicity. The pilot-reference test
checks all 128 physical phases at several depths and both spacings, including
protected-bin exclusion.

A separate integration test transports real NCP and four actual BCH/LDPC payloads
through mapped cells and the interleaver pair. At both spacings and maximum time
depth it recovers every payload with zero bit errors across a 128-symbol reset.
Receiver codeword boundaries come from NCP/CRC, not transmitter diagnostic IDs.
The test uses a 24 MHz geometry with explicit synthetic additional pilot positions
and transports pilot placeholders, not actual pilot signals. External DOCSIS
interoperability and full waveform validation remain outstanding.

Pilot signals are available separately. Major remaining stages are PLC and
physical-bin assembly; IFFT, prefix/windowing
and inverse receiver; and a streamed one-second noiseless integration.

Evidence: CM-SP-PHYv3.1-I20-230419 section 7.5.6.1–3, Figures 55–58,
PDF129–133; Appendix I sample code, PDF211–212 (informative). The original local
PDF diagrams were inspected for branch lengths, shift-register orientation,
rotation direction and pilot-reference phase selection.
