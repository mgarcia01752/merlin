# QAM mapping and soft bits

The public Rust mapper supports every downstream data order in the pinned PHY:
16,64,128,256,512,1024,2048,4096,8192 and16384-QAM. BPSK and QPSK references are
also used for protected control paths; they do not become downstream payload
modes. Supported full downstream data orders have their own coding-chain tests.

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
# Ok::<(), docsis_ofdm_fec::SimulationError>(())
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
Do not reorder the label-indexed source table or assume a generic square
constellation:128,512,2048 and8192-QAM use non-square mappings. Derived search
indexes preserve the original bit-label partitions.

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

Mapping and exact indexed max-log demapping work through16K-QAM. For verified
Cartesian labels, the demapper searches the I and Q coordinate levels once each,
then looks up the nearest allowed levels for each bit hypothesis. This changes
per-cell work from O(M log M) to O(log M), including the log2(M) output bits.

Cross-QAM keeps its original geometry through exact balanced spatial searches
for each bit hypothesis. Search can still visit every point in the worst case;
it is not a guaranteed logarithmic algorithm for irregular constellations.
BPSK, QPSK,16-QAM and cross-128 retain small direct scans to avoid search overhead.

Indexes are initialized once per supported order and shared safely between
threads. They never change the mapper, labels, transmitted points, noise draws,
LLR sign/scaling or clipping. Both I/Q squared terms are retained in the original
arithmetic order before subtraction. Extreme-coordinate overflow checks fall
back to the old full scan when an enclosing rectangle is inconclusive.

The optimization reduces receiver computation during acquisition, observation,
completion and drain; it does not shorten the interleaver or simulated duration.
LDPC iterations and other stages can still dominate total run time.

A reproducible kernel comparison is available as:

```bash
scripts/cargo-local.sh run -p docsis-ofdm-fec --release --locked \
  --example qam_demapper_benchmark -- 4096
```

It checks matching output checksums and reports cold initialization plus median
warm timings from three equivalent batches. These are kernel measurements, not
promises of the same whole-simulation speedup. Detailed results and verification
are recorded in the [performance design](../development/features/qam-demapper-performance/design.md).

The full coding CLI still runs the original short8/9 uniform16-QAM experiment.
All ten downstream orders now have their full uniform bit-interleaver and
padding rules, with actual soft/FEC and waveform transport tests under QAM-03.
See [QAM processing](qam-processing/README.md). Explicit shortened words use the
separate24-column path and received-boundary length discovery. The original
[256-QAM independent cell loop](qam256-coding-loop.md) remains available; the
general CLI `simulate` selector is still the original16-QAM coding experiment.
All32,720 data labels now have complete pinned-table preservation tests.
QAM-04 is complete within MERLIN's documented closed-loop scope. External
interoperability is not a minimum acceptance requirement.

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

The indexed implementation is compared bit for bit with an independent full-scan
oracle for every supported order, perturbed/edge/tie/outside samples, tiny/large
variance, overflow and invalid inputs. Existing label/energy tests remain in
place; numerical agreement with the old demapper is not external interoperability
evidence.
