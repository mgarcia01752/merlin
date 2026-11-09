# QAM mapping and soft bits

The public Rust mapper supports every downstream data order in the pinned PHY:
16,64,128,256,512,1024,2048,4096,8192 and16384-QAM. BPSK and QPSK references are
also available for future control-channel work; they do not become downstream
payload modes. Data modulation starts with256-QAM for the upcoming channel path.

The tables are reused from PNM Viewer Toolkit's preserved PyPNM resources.
**PyPNM - Proactive Network Maintenance Toolkit** attribution and its LICENSE,
NOTICE and source hashes are retained under
[resources/qam-table](../../crates/ofdm/resources/qam-table/README.md).

## Public API

```rust
use docsis_ofdm_fec::signal_processing::modulation::{map_symbol, qam_llrs};

let point = map_symbol(256, 0x35)?;
let soft_bits = qam_llrs(256, point, 0.01)?;
assert_eq!(soft_bits.len(), 8);
# Ok::<(), docsis_ofdm_fec::LabError>(())
```

`order` is the constellation size, not a PNM enum. `SymbolLabel` is a `u16` alias,
large enough for a14-bit cell word. The first bit is the most significant bit;
label0x35 means00110101 for256-QAM. Labels must be smaller than the chosen order.
Unsupported orders and invalid labels return structured errors.

`reference_for_order` returns a static `QamReference` with order, bit count, scale
and borrowed I/Q coordinates. Its `iq` field is now a slice so one representation
covers all sizes, rather than a16-element array. Existing `map_qam16` and
`qam16_llrs` entry points remain available; both use the shared resources/math.

## Coordinates, scaling and likelihoods

For data orders16..16384, resource row index is the label. BPSK/QPSK resource
rows require explicit build-time label permutations verified against PHY
Figures99–100; the original resource files are unchanged.
Do not sort points by amplitude or assume a
generic square constellation:128,512,2048 and8192-QAM use non-square mappings.

For integer coordinates $a_l,b_l$ and constellation order $M$:

$$
E_{raw}=\frac{1}{M}\sum_{l=0}^{M-1}(a_l^2+b_l^2),\qquad
x_l=\frac{a_l+j b_l}{\sqrt{E_{raw}}},\qquad
\frac{1}{M}\sum_l|x_l|^2=1.
$$

For256-QAM, $E_{raw}=170$. For16384-QAM it is10922. The build checks all
imported energies against the scaling resource and compiles normalized static
arrays. Mapping does not read files at runtime. Received samples are already
on the normalized scale and are not normalized again by the demapper.

For received equalized sample $y$ and complex noise variance $N_0=E[|n|^2]>0$,
max-log soft demapping uses

$$
L_k\approx\frac{
\min_{l:b_k(l)=1}|y-x_l|^2-
\min_{l:b_k(l)=0}|y-x_l|^2}{N_0}.
$$

Outputs are MSB-first natural-log LLRs, clipped to±100. Positive favors bit0;
negative favors bit1. Each real noise component has variance $N_0/2$. Supply
linear complex noise power, not dB or per-axis variance. Finite I/Q, positive
finite variance and finite squared distances are checked.

A noiseless mapper test still supplies a finite positive reference variance to
compute finite LLRs. It does not inject noise; zero variance is rejected.

## What is implemented

Mapping and reference max-log demapping work through16K-QAM. The demapper searches
all points, with work proportional to constellation size times bit count. It is
a correctness reference; streaming high-order performance optimization remains
separate from mapping correctness.

The full coding CLI still runs the original short8/9 uniform16-QAM experiment.
Other orders require their verified bit interleavers/demultiplexers, padding and
symbol packing before enabling a full coding mode. High-order mapping availability
does not imply completed channel simulation. No256-QAM CLI mode, IFFT or actual
NCP message generation is enabled by this change.

The subsequent [256-QAM cell-loop implementation](qam256-coding-loop.md) now adds
its short8/9 bit processing and randomized noiseless recovery as a standalone API
and JSON example. The general CLI simulation job and physical channel stages
remain separate.

## Evidence and validation

Local CM-SP-PHYv3.1-I20-230419 AnnexA, PDF197–206, defines labeling and scaling.
Tests independently transcribe all64 first-quadrant256-QAM labels from Figure108
(PDF201), then check all256 labels using the reflection rules in Figures106–107
(PDF200). Table54 (PDF206) supplies independent expected energies. Other orders
have diagram anchor checks, full coordinate uniqueness/energy checks and selected
noiseless soft-decision checks; these are not exhaustive independent labeling
vectors for every high-order point. Full external interoperability vectors remain
pending.

All256 ideal points recover their eight bits with zero errors. Tests also check
soft-bit ordering, variance scaling, clipping and invalid inputs. The capability
registry marks mapping experimental independently of full coding simulation.
