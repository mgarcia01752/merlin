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

![Interpolated-power preview between two captures](../images/workbench/rxmer-interpolation-dark.png)

**Load demo** adds the sanitized binary baseline; **Add notch demo** adds the
existing decoded alternative. Adding a second file requires selecting Average or
one of the sequence modes. The combined chart always identifies the effective
residual model, not a new measured capture. RxMER does not establish real burst
correlation or reconstruct measured I/Q.

## Readiness and current scope

Apply validates the entire draft before replacing accepted state. Invalid edits
and Cancel preserve accepted inputs. Replacing DPD or OCD revalidates dependent
configuration; old incompatible inputs report an error instead of silently being
realigned. Channel Simulator is green when both accepted RxMER and channel
response configurations are valid. Opening Channel Simulator shows their summary
and links back to the editors.

![Accepted channel inputs](../images/workbench/channel-inputs-ready-dark.png)

Green means **configured**, not a running simulation. This task provides shared
viewers, accepted input state and reusable Rust combination/preview operations.
Main Start dispatches the frozen configuration through the closed-loop worker,
with actual payload/PLC QAM and channel-output SA observations. The TX SA's local
**Run simulation** remains a separate preview before these impairments.
Receiver Apply checks that the playback transition span fits the selected run
duration; a valid editor schedule can still be too long for that request.

The editor accepts at most 64 RxMER files, 64 MiB original RxMER bytes and four
million decoded carrier values. One response file has a separate 16 MiB ceiling;
each input file is at most 16 MiB. Originals/full arrays remain in Rust; each chart
returns at most 1024 min/max groups, preserving narrow extrema and missing-data
breaks. Workbench JSON Save/Open preserves accepted original files and settings; see the
[run/save guide](closed-loop-workbench.md).

See the [design and validation](../development/features/rxmer-workbench/design.md),
[existing engine playback](impairments/measurement-timeline.md) and
[paired receiver constellation](impairments/paired-constellation.md).
