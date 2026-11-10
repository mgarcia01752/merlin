# Receiver Workbench

Click Receiver for the stage diagram and instruments. Rounded stage chips share
equal widths and aligned test-point rows; decorative pins do not indicate status.
Each editor/instrument has its own QML implementation or shared reusable dialog.

![Receiver Workbench](../images/workbench/receiver-workbench-dark.png)

Use **Configure Receiver / Run…** to choose supplied-H reference correction or
pilot-estimated H-hat and finite duration/decoder/runtime limits. Apply validates
the accepted OCD, DPD, source and channel inputs. Receiver becomes green when
this request is supported, before any results exist. PRBS-RX separately enables
independent source comparison. See the [run/save guide](closed-loop-workbench.md).

## QAM analyzer

**Signal: Data Profile | NCP | PLC** is independent of **Raw | Equalized | Paired**.
Equalized is before hard slicing, so the received residual remains visible.
Payload and PLC views now show actual bounded matched observations from the last
complete or partial run; paired panels use shared axes. PLC Data / Preamble stay
separate. The current payload view uses supported Profile 0 and its first
compatible modulation group. General profile/carrier/symbol selection is pending.

![NCP selection with an explicit support diagnostic](../images/workbench/receiver-qam-ncp-dark.png)

Matched NCP I/Q collection is not implemented. NCP control decoding still runs
in the closed loop, but its LLRs or accepted messages are not received scatter
points. The analyzer reports this gap rather than inventing samples.

## Spectrum Analyzer and diagnostics

Click **RX Input · SA**, **FFT Bins**, or **Spectrum Analyzer**. The independent
Test Point selector offers RX Input / Channel Output, RX Framed Samples, RX Useful
Samples and RX FFT Bin Power. The first three use retained contiguous time samples;
useful symbols are not joined across prefix gaps. FFT-bin power is a distinct
frequency-domain view with its own units and physical frequency grid.

![Actual retained RX input spectrum](../images/workbench/receiver-spectrum-dark.png)

Receiver SA's local Run button is disabled: use main Start to run the full loop.
Changing QAM or SA selection only reads retained Rust results and clears stale
viewports. TX window timing is hidden here. FEC Monitor displays real job totals;
response/echo and slicer/LLR result renderers remain pending. Supplied response H
and receiver-estimated H-hat are always distinct.

Dark/light presentation and 960×640 layout are checked on the Ubuntu VM. See the
[task design](../development/features/receiver-workbench/design.md) and
[workbench guide](workbench.md).
