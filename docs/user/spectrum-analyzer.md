# Spectrum Analyzer (SA)

The SA is a reusable workbench window for time-domain complex I/Q test points.
Its first connection is **TX output**, after cyclic prefix, windowing and
overlap-add and before channel impairments. Other channel/RX tap connections
remain pending. The FFT and all signal calculations run in Rust; Qt draws the
bounded trace and handles selection/zoom.

Accept OCD, DPD Profile A (ID 0) and PRBS-TX source settings. Transmitter turns
green when these form a supported Rust request. Click **Transmitter** to open
the SA and generate its first sample snapshot. **Run simulation** regenerates
from the same frozen source settings. This does not start the full closed loop;
its Start gate and other workbench blocks remain separate.

![Actual shared SA showing TX baseband output](../images/workbench/sa-tx-baseband-dark.png)

The signal is **complex baseband**, centered at DC. There is no RF upconversion.
The default axis shows positive and negative frequency offset: at 25 kHz spacing
the 8192-point FFT spans −102.4 through +102.375 MHz. **Capture frequency mapping**
instead labels the same bins using the selected capture's frequency origin.
Changing the axis never changes or upconverts samples.

![The same PSD with capture frequency labels](../images/workbench/sa-tx-mapped-dark.png)

The vertical axis is normalized power spectral density, **dB re one sample unit
squared per Hz**. It is not calibrated dBm, dBmV or dBFS. No RF output level or
reference impedance is assumed. This is a simulation analyzer, not RF compliance
evidence. The same unit/reference-plane distinction must be preserved when more
workbench taps are connected.

Generation uses actual selected PRBS/file bits, existing FEC/control coding,
mapping, interleaving, pilots and OFDM synthesis. Idle priming establishes
interleaver history before consuming source bits. A finite file is sent once;
accepted optional fill is honored, with idle/filler after EOF. Generated source
bit counts include queued preview words and do not prove full transmission or
recovery. No receiver runs for this preview.

The SA uses 32 periodic-Hann FFT windows with 50% overlap, averaging **linear
power before conversion to dB**. FFT size is 8192 or 4096 at 204.8 MS/s; displayed
bin spacing and equivalent noise bandwidth follow that geometry. This analysis
window is separate from the transmitter's OFDM roll-off window.

Hover or use Left/Right on the plot to select a bin group for zoom. The trace shows each
group's peak and a min/max envelope; full-resolution FFT data stays in Rust.
**Zoom in**, **Zoom out** and **Full span** query new bounded views without
regenerating the signal. At close zoom, individual bins become visible. Zero
power remains explicit in retained Rust data; the finite drawing floor is visual only.
The area below the frequency axis now contains the symbol-timing and cable panel.
Sample counts, selection text and completion messages remain omitted. Generation
status and diagnostics are available in the Run simulation button tooltip.

**Cancel** or closing the SA requests cooperative worker cancellation. The UI
remains responsive. Previous complete results stay available if inputs have not
changed. Applying new source, profile or OCD settings cancels older work and
invalidates its spectrum; rejected edits and Cancel preserve accepted state.
Only one preview worker runs at a time. Settings/results are session-only.

```mermaid
flowchart LR
    INPUT["Accepted source / OCD / DPD"] --> TX["Rust TX stages"]
    TX --> TAP["tx.overlap_add.samples: complex baseband"]
    TAP --> SA["Shared Rust FFT / PSD"]
    SA --> VIEW["Bounded min/max view"] --> QT["Shared SA window"]
    FUTURE["Future channel/RX time-sample taps"] -.-> SA
```

See [workbench readiness](workbench.md), [PRBS-TX](source-input.md) and
[design and validation](../development/features/transmitter-spectrum/design.md).

## Symbol timing and cable echo budget

Below the PSD, the left panel shows the actual **transmit-window coefficient**
near symbol start. This is a dimensionless window view, not the time-domain I/Q
signal amplitude. The overview reports CP, useful FFT length/time and symbol
cadence. Yellow marks the roll-off/overlap boundary; the CP-end marker identifies
the useful FFT start. The flat guard label subtracts roll-off from CP duration.
All values use the frozen settings that produced the displayed spectrum.

Select **Belden 1694WB RG-6**, **CommScope P3 500** or **Custom cable / velocity
factor** on the right. Custom VF must be greater than zero and at most one; an
empty value leaves distances unknown. Cable selection updates the estimate
without rerunning the simulation or changing any samples.

The ideal timing budget is `(CP samples − roll-off samples) / sample rate`.
For CP512/RP128, it is 1.875 µs. With VF0.82, this corresponds to about 1512 ft
of **extra path**, or 756 ft of **two-way reflector spacing**. The estimate assumes
one homogeneous cable and zero reserved receiver timing margin. It is not maximum
plant reach, a general distance-to-fault measurement, or guaranteed echo
protection. Actual echoes also depend on topology, strength and receiver behavior.

Applying new accepted inputs clears the panel until a new snapshot completes.
Full time-domain I/Q trace inspection remains pending. The
[design and source evidence](../development/features/transmitter-spectrum/design.md#ui-sa-04-symbol-timing-and-cable-echo-budget)
records the PHY derivation and manufacturer VF references. The
[original graphic](../images/workbench/sa-timing-proposal-dark.png) is a historical
proposal; screenshots above show the implemented compact panel.
