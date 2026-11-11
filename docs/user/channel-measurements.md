# Channel Estimate and RxMER inputs

The **DS Channel Est.** and **RxMER** blocks open separate dialogs built around
one shared measurement viewer. A capture tree on the left retains original
file metadata; the chart on the right supports zoom, full span and hover
readouts. All parsing, calculations and chart reduction happen in Rust.

Scroll over the plot to zoom around the pointer, or left-drag horizontally to
select a range. **Full Span** resets the view. There are no +/- zoom buttons.
Click the plot for keyboard +/- zoom and Home reset. Changing a capture or metric
resets the range; zoom only queries retained input data.

![Shared measurement mouse navigation](../images/workbench/measurement-mouse-zoom-dark.png)

Apply a DPD first. Inputs must match its device, channel, frequency origin,
subcarrier spacing and complete source grid. Merely overlapping frequencies do
not qualify. Active carrier coverage is checked before Apply. Missing/excluded
values remain distinguishable; no frequency resampling or replacement is hidden.
A configurable capture association tolerance defaults to 90000 seconds as an
application policy, not a specification requirement. Original timestamps stay
unchanged.

## Channel response

The default is **Flat / unity H = 1 + 0i**, equivalent to **0 dB gain / 0° phase**
per subcarrier. This synthetic configuration needs no file and becomes ready
once the DPD/OCD grid is valid. It is not a measured capture or receiver estimate.
A linear response of 0 + 0i would suppress the signal.

![Flat channel response](../images/workbench/channel-response-flat-dark.png)

Use **Open file…** for binary channel-estimate PNM or supported decoded JSON,
or **Load demo** for the bundled decoded example. Select gain, wrapped phase,
real H or imaginary H in the shared viewer. Zero/missing active gains and
incompatible grids reject Apply. **Flat / unity** returns the draft to the
synthetic default; Cancel preserves the accepted configuration.

![Captured channel response](../images/workbench/channel-response-capture-dark.png)

## RxMER captures

Use the OS-native **Open file(s)…** chooser for one or multiple binary RxMER PNM
or decoded JSON files. Detection uses contents rather than extensions. The pinned
PNM Viewer Rust library handles binary decoding; MERLIN owns simulation policy.
Expand a tree row for identity, frequency, capture time, format and original hash.
Select a row to inspect its original values, or **Effective RxMER / playback**
to inspect the proposed combination. Duplicate byte-identical files reject.

![Single RxMER input](../images/workbench/rxmer-single-dark.png)

| Mode | Effective model |
| --- | --- |
| Single capture | One per-carrier observation held for the run. Requires one file. |
| Average | Equal-capture-weight mean of linear residual error power, held for the run. |
| Sequence · hold | Hold each capture until the next explicit symbol boundary. |
| Sequence · interpolate power | Linearly interpolate error power between adjacent captures at the selected symbol. |

The simulation uses normalized error power `v = 10^(-RxMER/10)`. Average and
interpolation operate on v; the chart converts back with `-10 log10(v)`. They
never average/interpolate frequency grids or silently discard missing required
values. This differs from the viewer's arithmetic-dB display statistic.

For a sequence, enter **OFDM symbols between captures**. At 25 kHz spacing and
CP512, 1000 symbols corresponds to 42.5 ms. **Preview symbol** selects the point
in the synthetic schedule to display. Move selected files earlier/later or
explicitly **Sort by capture time**; filenames never determine playback order.
The final capture is held after the last transition, with no extrapolation or
implicit looping. Recorded capture times are separate from this playback clock.

![Interpolated-power preview between the first two captures in the bundled collection](../images/workbench/rxmer-interpolation-dark.png)

RxMER uses **Open file(s)…** for capture selection; Load demo and Add notch demo
have been removed from its toolbar. The Channel Estimate editor retains its own
Load demo action. Adding a second file requires selecting Average or
one of the sequence modes. The combined chart always identifies the effective
residual model, not a new measured capture. RxMER does not establish real burst
correlation or reconstruct measured I/Q.

## Readiness and current scope

Apply validates the entire draft before replacing accepted state. Invalid edits
and Cancel preserve accepted inputs. Replacing DPD or OCD revalidates dependent
configuration; old incompatible inputs report an error instead of silently being
realigned. Channel Simulator is green when both accepted RxMER and channel
response and additional-loss configurations are valid. Opening Channel Model shows their summary
and links back to the editors.


Green means **configured**, not a running simulation. This task provides shared
viewers, accepted input state and reusable Rust combination/preview operations.
Main Start dispatches the frozen configuration through the closed-loop worker,
with actual payload/PLC QAM and channel-output SA observations. The TX SA's local
**Run simulation** remains a separate preview before these impairments.
Receiver Apply checks that the playback transition span fits the selected run
duration; a valid editor schedule can still be too long for that request.

The editor accepts at most 256 RxMER files, 64 MiB original RxMER bytes and four
million decoded carrier values. One response file has a separate 16 MiB ceiling;
each input file is at most 16 MiB. Originals/full arrays remain in Rust; each chart
returns at most 1024 min/max groups, preserving narrow extrema and missing-data
breaks. Workbench JSON Save/Open preserves accepted original files and settings; see the
[run/save guide](closed-loop-workbench.md).

See the [design and validation](../development/features/rxmer-workbench/design.md),
[existing engine playback](impairments/measurement-timeline.md) and
[paired receiver constellation](impairments/paired-constellation.md).

## Channel Simulator stages

Click **Channel Simulator** to open **Channel Input → Channel Model → Channel
Output**, with separate **RxMER** and **DS Channel Estimate** input blocks.
Click either measurement block to open its existing input frame.

Click **Channel Input** or **Channel Output** to choose **Spectrum** or **QAM**.
Input Spectrum uses TX output samples; input QAM shows the retained physical TX
FFT symbol with bin-range filters. It does not identify Data/NCP roles from point
positions. Output Spectrum uses channel-output samples at RX input; output QAM
uses actual received FFT cells **before equalization**, with the existing received
role/profile/bin/symbol selectors. Empty selections stay unavailable.

Received I/Q uses circular dots at the actual retained coordinates, before hard
decisions. These are scatter markers, not intensity bins. Channel impairments
can shift, rotate or spread the points; changing their marker shape does not
change those samples. High-order QAM can still have a square overall footprint.
The display limit reports when only a subset of retained cells is shown.

![Raw Channel Output QAM](../images/workbench/channel-output-qam-dark.png)

There is no separate SA block. Opening an instrument does not run the simulation;
use the main workbench Start/Stop/Pause controls. After changing accepted settings,
run again to populate current observations.

Channel Model shows input summaries and opens the separate
[Additional Tilt / Cable Loss editor](impairments/additional-loss.md). The default
is Off. Manual tilt or full additional cable loss composes with the supplied
response, preserving the imported files. These settings save in the WB JSON.

![Channel Simulator staging](../images/workbench/channel-inputs-ready-dark.png)

See the [staging design](../development/features/channel-workbench/design.md)
and [additional-loss design](../development/features/channel-tilt/design.md).

## Launch amplifier, attenuator and TapPoints

Click the **amplifier triangle** to set gain, or the **variable attenuator** to
set receiver-input attenuation. Each opens its own editor. **Apply** accepts;
**X / Close** discards changes. Defaults are **0 dB gain**, **0 dB attenuation**
and **Tilt Off**, so no initial setup is required. Tilt and additional cable loss
remain under **Channel Model → Additional Loss**.

![Channel Simulator with amplifier, attenuator and TapPoints](../images/workbench/channel-inputs-ready-dark.png)

The channel workspace expands with the application window. Maximize MERLIN for
more space: the diagram uses the available width with balanced outer margins,
equal gaps and aligned connectors. RxMER and DS Channel Estimate remain centered
above and below Channel Model. Controls stay readable at the minimum window size.

The circles marked **TP** mean **TapPoint**. Click either to select **Spectrum**
or **QAM** before or after the amplifier. Channel Input also opens the first tap;
Channel Output opens instruments at receiver input, after attenuation. These
routes use the same SA and channel QAM windows with their selected boundary in
the title. Input QAM shows retained physical FFT bins; output QAM shows actual
received FFT cells before equalization, with received role/bin/symbol selectors.
After-amplifier points include the accepted gain; automatic axis scaling keeps
them visible and the displayed axis extent reports their actual scale.

![Raw received QAM at channel output](../images/workbench/channel-output-qam-dark.png)

### Electrical reference and amplifier target

Leave **TX Electrical Reference** unchecked for an ordinary relative experiment.
To obtain electrical readings, enable it in the amplifier editor and enter the
nominal **Input Data Level (dBmV / 6 MHz)** before amplification. This is an
explicit model reference, not a level inferred from a PNM capture. Imported
channel estimates remain relative multipliers.

Use **Gain (dB)**, or select **Set Output Data Level** for a target nominal DATA
level after amplification. The target requires a reference and must resolve to
0–60 dB gain. The reference/target range and gain bounds are model controls, not
a universal DOCSIS CMTS output-power specification. Receiver attenuation accepts
0–60 dB. Receiver AGC and physical overload/clipping are separate, unmodeled effects.

![Launch amplifier settings](../images/workbench/channel-amplifier-dark.png)

### Independent receiver noise

The attenuator editor optionally enables **Add Independent Receiver Noise** with
a density in **dBmV/Hz**. This requires the TX electrical reference. It adds a
fixed frequency-bin noise floor after attenuation, in addition to the supplied RxMER residual.
Account for noise already represented by RxMER; leave this option off when the
measured residual is the entire intended receiver budget. Removing calibration
while independent noise is enabled is rejected until noise is explicitly disabled.

With independent noise enabled, increasing cable loss or attenuation lowers SNR;
matched equalization cannot undo that loss. With it off, the existing post-EQ
RxMER model can retain the same MER despite cable attenuation. Actual samples
and decoding determine recovery; a level warning does not invent bit errors.

![Receiver input attenuation and independent noise](../images/workbench/channel-attenuator-dark.png)

### Channel Power and saved workbenches

The SA reports **Channel Power** integrated over the fixed occupied channel band,
including actual pilot energy, internal gaps and noise. It separately shows the
band-average **dBmV / 6 MHz** level and integration bandwidth. Mouse zoom and
**Full Span** change the display, not that power integral. Without calibration,
power is explicitly relative; with a TX reference it is in **dBmV** and PSD is
in **dBmV/Hz**. The channel-output SA warns outside the−12…+12 dBmV/6 MHz
band-average operating target. This is not a per-subband or composite RF-port
compliance measurement.

![Calibrated Channel Power after the amplifier](../images/workbench/channel-power-sa-dark.png)

**File → Save / Save As / Open** preserve accepted gain, attenuation, target,
TX reference, independent noise and existing tilt/cable choices along with the
original PNM inputs. Supported older workbenches default missing power controls
to0 dB gain/attenuation with no electrical reference or added noise, preserving
existing tilt/cable settings. Unsupported versions or invalid values reject the
load without changing the current WB. Derived power readings, zoom, tap selection
and unapplied drafts are not saved.

See the [model and implementation record](../development/features/rf-power-budget/design.md#ui-rf-01-implemented-scalar-conditioning-and-channel-instruments)
for normalization assumptions and remaining RF front-end work.

CLI users can use the same conditioning model with the
[standalone channel-power example](../../examples/channel-power.json).
