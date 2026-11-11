# Receiver Workbench

Click **Receiver** for the processing path and instruments. Use **Configure
Receiver / Run…**, accept the inputs and enable PRBS-RX comparison, then use main
Start. Instruments inspect the same retained complete/partial run and never
advance the simulation. Each popup has X and its existing Close/Cancel controls.
See the [run/save guide](closed-loop-workbench.md).

![Receiver Workbench and FEC Statistics shortcut](../images/workbench/receiver-workbench-dark.png)

## FEC statistics

Open **Receiver → FEC Statistics** in the upper-right header, or **FEC Monitor**
in the Diagnostic Instruments row. Both open the same window. After a run it shows:

- Payload words generated/recovered, LDPC/BCH failures, BCH corrected bits and mean
  LDPC iterations, with the actual iteration-count distribution.
- CRC-accepted NCP fields and total NCP LDPC iterations; PLC frames, iterations
  and compared bit errors. These are separate control and payload paths.
- Independent payload compared bits, bit errors, BER, codeword errors and
  unrecovered generated words.
- The first 64 retained completed word outcomes, source-symbol intervals,
  iterations and correction/error decisions; omitted-row count is explicit.

No compared bits means unavailable BER. A cancelled/failed run retains completed
outcomes; missing control-decode observations are not invented failure counts.

![Actual FEC statistics](../images/workbench/receiver-fec-dark.png)

## QAM analyzer and retained selectors

**Signal** selects Data Profile, NCP, PLC, Continuous Pilots or Scattered Pilots.
**QAM View** selects Raw, Equalized or Paired independently. For PLC, choose Data
or Preamble. These groups retain their different modulation and pilot boost.
The paired panels use matching received cells and equal I/Q scale.

Select an available received profile and one modulation order, then optionally
enter inclusive **First/Last Bin** and **First/Last RX Symbol** bounds. **All**
means all retained coordinates, and **Reset Filters** clears the filters. The
shown bounds describe the actual retained data, not all transmitted history.
An empty/invalid range clears the chart; different QAM orders are never pooled.
Each independent run uses wire Profile A/ID0. The Receiver title identifies the
original captured profile used for its loading; QAM metadata retains the actual
wire identity. Profile Performance selects results by original captured ID.

![Matched NCP receive constellation and selectors](../images/workbench/receiver-qam-ncp-dark.png)

NCP points now come from CRC-classified received cells, using the accepted field's
consumed-cell count and the same inverse coordinate transport as payload. The
plot never regenerates received points from decoded labels or TX reservations.
A failed CRC can leave NCP role attribution unavailable. The core retains up to
65,536 payload pairs, 8,192 NCP pairs and 8,192 per protected stream for this UI;
at most 2,048 selected pairs are displayed. Protected captures cover the first
1,024 physical channel symbols, including acquisition. Payload/NCP pairs carry
actual source and receive coordinates for the configured observation interval.
Changing a filter cannot retrieve data outside these retained windows.

## Channel response and echo

Open **CH-EST / Equalizer** or **Channel Response / Echo**. Select **Applied
Reference H** for the channel's supplied/unity response, or **Receiver H-hat**
for actual pilot-estimated snapshots. H-hat requires a run with pilot-estimated
correction; the UI never substitutes supplied H when no estimate was retained.
Choose among up to four retained observation-symbol snapshots.

Views include gain, wrapped phase, real/imaginary H and the band-limited impulse
response. Excluded/unknown frequency bins remain gaps in response curves and are
zero-filled for the impulse transform. Band limits produce ringing, so plotted
peaks are not automatically identified physical echoes. Signed delay is relative
to the receiver's timing reference. The displayed uniform-cable velocity factor
is explicit; path/reflector distances are estimates, not reach or protection.

![Actual receiver-estimated impulse response](../images/workbench/receiver-response-dark.png)

## Slicer and LLRs

Open **Soft Demap / Decode** or **Slicer / LLR**. It begins with the current QAM
selection; its filters can then be changed independently. The first 128 selected
cells show physical nearest-symbol labels, received I/Q, residual squared distance
and actual post-descrambling NCP/payload LLRs. Physical labels precede
descrambling, so do not compare their bits directly with those LLR signs.
Pilot slicing explicitly accounts for its existing amplitude boost.

Select **Payload LDPC Input** to inspect the first retained coding-order
16,200-value decoder-input window and its likelihood distribution. Cell filters
do not relabel this separate word boundary. Positive LLR favors zero; negative
favors one. These are diagnostics: production FEC continues to consume soft
values, and hard decisions never feed back into decoding.

Protected PLC/pilot streams provide physical decisions but do not expose a
retained per-cell soft boundary in this view. They show that distinction explicitly;
LLRs are not synthesized from decided labels.

![Actual decisions and likelihoods](../images/workbench/receiver-slicer-dark.png)

## Spectrum Analyzer

Click **RX Input · SA**, **FFT Bins** or **Spectrum Analyzer**. Its Test Point
selector is independent of QAM selection: RX Input / Channel Output, RX Framed,
RX Useful and RX FFT Bin Power. Time spectra use retained contiguous samples;
useful symbols are not joined across prefix gaps. FFT-bin power has distinct
units and physical frequency coordinates. The local Run button is disabled:
main Start owns the full loop. TX timing graphics remain hidden in RX instruments.

![Actual retained RX spectrum](../images/workbench/receiver-spectrum-dark.png)

Accepted input changes clear every retained instrument result. Retention and
query limits are shared across windows. Known-boundary timing, the admitted
coding/profile set and initial host limits remain explicit implementation limits;
these instruments do not add arbitrary RF acquisition or external-interoperability
claims. See the [design](../development/features/receiver-instruments/design.md)
and [workbench guide](workbench.md). Dark screenshots are actual Ubuntu VM captures.

## Profile performance and independent runs

**Diagnostic Instruments → Profile Performance** combines measured receiver MER,
the ideal profile reference and TotalCW/CCW/UFEC counters. Configure Receiver
selects all imported profiles as separate runs by default. The performance
selector reviews each result; other instruments remain labelled with the latest
completed profile. See the [profile performance guide](receiver-profile-performance.md)
for counter scope, Profile A mapping and sweep controls.
