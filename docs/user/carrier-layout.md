# Downstream carrier layout

`transmission::layout` defines physical PHY-bin roles before waveform assembly.
It does not generate QAM samples or run interleavers. This API is available
without the optional simulation feature.

Supply fixed [transmission settings](transmission.md) and `CarrierLayoutSettings`.
The constructor builds an immutable owned map without changing inputs.
`symbol_roles(symbol_index)` returns one role per physical bin: Data, Excluded,
PLC, ContinuousPilot or ScatteredPilot. Indices follow increasing-frequency PHY
order, not an FFT library's wrapped output convention.

## Layout inputs and validation

| Field | Meaning |
| --- | --- |
| `zero_frequency_hz` | Absolute frequency of PHY bin zero |
| `first_active_bin` | First bin in the active envelope, inclusive |
| `active_end_exclusive` | First bin beyond the active envelope |
| `plc_first_bin` | First PLC bin; width follows spacing |
| `excluded_bins` | Individual internal exclusions; expand bands into indices |
| `additional_continuous_pilots` | Caller-selected pilots beyond the eight predefined PLC pilots |

Validation checks FFT bounds, frequency overflow, active-envelope width against
occupied bandwidth, PLC geometry/grid alignment, duplicates and collisions.
Bins outside the envelope are excluded. Internal exclusions cannot enter the
PLC's entire 6 MHz neighborhood.

PLC width is 8 bins for 4K and 16 for 8K. Its neighborhood has 56 bins on each
side for 4K, or 112 for 8K. The lowest-frequency bin of the 6 MHz neighborhood
must lie on a 1 MHz grid. The eight predefined continuous pilots are added at
distances 15, 24, 35 and 47 from each nearest PLC edge for 4K, or 30, 48, 70 and
94 for 8K. Supply only the additional pilots: 8–120 distinct locations outside
that neighborhood.

Additional-pilot **generation** is pending. Count bounds and collision checks
do not certify the full §7.5.15.2.2 algorithm: density, merged-spectrum placement,
edge adjustment, isolated-region coverage and periodicity perturbation still
need verification. This representation also does not certify spectral masks or
encode PLC messages. It is not a complete operational CMTS channel validator.

## Scattered-pilot progression

Symbol zero is the first symbol after the PLC's eight-symbol preamble. Continue
the index across codewords and subsequent preambles; the period is 128 symbols.
Do not assume a run starts at that phase without explicitly establishing it.

For the bin $a$ immediately above the highest PLC bin and symbol phase $s$:

$$
k\equiv a+d(s)\pmod{128},\qquad
d(s)=\begin{cases}
s,&\text{4K},\\
2(s\bmod64)+\lfloor s/64\rfloor,&\text{8K},
\end{cases}\qquad s=\text{symbol index}\bmod128.
$$

Only data-eligible bins become scattered pilots. PLC, exclusions and continuous
pilots take precedence. In 8K mode, the first 64 symbols visit one parity of
positions; the second 64 visit the other. Every eligible bin is visited once per
cycle. Data counts can vary by symbol, while the combined data/scattered-pilot
domain stays constant.

## NCP is a separate domain

`layout::ncp::select_ncp_cells` accepts typed **pre-interleaving** cell roles.
It selects data cells from the highest index downward, skipping scattered-pilot
placeholders and zero-loaded cells. Results are in assignment order. Inputs are
unchanged; insufficient capacity returns an error instead of partial output.

Returned indices are not physical FFT-bin positions. The future packer must
inverse-map physical pilots through the actual frequency/time interleavers first.
PLC, continuous pilots and exclusions are outside this cell domain. Required
cell count comes from future NCP encoding/packing; the helper does not invent
pointer contents or reserve a fixed physical band. A physical Data role is a
candidate location, not a final payload-capacity claim after NCP overhead.

## Small geometry example

This synthetic 24 MHz example demonstrates the API. Its supplied pilot list is
not a certified continuous-pilot generator.

```rust
use docsis_ofdm_fec::transmission::{
    TransmissionSettings,
    layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole},
};

let settings = TransmissionSettings {
    occupied_bandwidth_hz: 24_000_000,
    ..Default::default()
};
let geometry = CarrierLayoutSettings {
    zero_frequency_hz: 100_000_000,
    first_active_bin: 3000,
    active_end_exclusive: 3800,
    plc_first_bin: 3312,
    excluded_bins: vec![3760],
    additional_continuous_pilots: vec![3040, 3080, 3120, 3160, 3480, 3520, 3560, 3600],
};
let layout = CarrierLayout::new(&settings, &geometry)?;
let first = layout.symbol_roles(0);
assert_eq!(first.len(), 8192);
assert_eq!(first[3328], PhysicalCarrierRole::ScatteredPilot);
assert_eq!(layout.symbol_roles(128), first);
# Ok::<(), docsis_ofdm_fec::LabError>(())
```

## Evidence and tests

Pinned source: CM-SP-PHYv3.1-I20-230419:

- §7.5.13.1–2, PDF 148–149: PLC width, clear neighborhood and grid.
- §7.5.15.1.1–2, PDF 164–168: scattered-pilot phase and role precedence.
- §7.5.15.2.1, Table 50, PDF 169: predefined pilot offsets.
- §7.5.15.2.2, PDF 170–171: additional-pilot generation, pending here.
- §7.5.14.4–5, PDF 163–164: NCP placement and interleaving.

Tests cover independent 4K/8K vectors, both halves of the 8K cycle, full-cycle
coverage, constant interleaver-domain size, invalid geometry, collisions,
frequency overflow and NCP eligibility. Waveform and full-channel decoding remain
upcoming work.
