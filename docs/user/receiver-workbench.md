# Receiver Workbench

Click Receiver to open **Receiver Workbench**. The navigation and selectors are
implemented; actual receiver charts and full-loop UI runs remain pending.

The Receiver Workbench places processing blocks across the top, clickable
test points below them, and instrument actions along the bottom, following the
supplied sketch. Processing stages use aligned, equal-size rounded chips with
subtle pin accents; test points line up below them. Pin accents are decoration,
not readiness indicators. Instrument buttons use matching rounded styling.
Each instrument opens its own focused dialog. Close the instrument to return to
the receiver workspace; Escape closes only the topmost dialog.

![Receiver Workbench](../images/workbench/receiver-workbench-dark.png)

## QAM analyzer

Use **Signal: Data Profile | NCP | PLC** to choose the received signal.
Use **View: Raw | Equalized | Paired** independently to inspect it before/after
receiver correction. Equalized is before slicing so noise remains visible.

When receiver results are connected, Data Profile will allow a supported received
profile, carrier or compatible same-QAM group and symbol interval. For PLC, select data or preamble separately.
The **Data Profile QAM**, **NCP QAM** and **PLC QAM** buttons open the same analyzer
with that signal selected. Raw / Equalized / Paired is an independent control.
PLC exposes a separate Data / Preamble selector. Received Profile is blank and
disabled until real receiver metadata is available.

![NCP paired QAM selection](../images/workbench/receiver-qam-ncp-dark.png)

NCP requires its own received-control I/Q selection; decoding an NCP message does
not by itself make a received constellation available. Unsupported selections
show why they are unavailable. Mixed modulations are not combined silently.

## Spectrum Analyzer

The SA has its own **Test Point** dropdown. Click **RX Input · SA**, **FFT Bins**
or **Spectrum Analyzer** to open it. The selected tap is retained independently
of the QAM signal. Switching taps clears stale viewport content. Start with **RX Input / Channel
Output**, which includes channel impairments. Framed and prefix-removed samples
can be added with explicit boundary handling. **RX FFT Bin Power** is a separate
frequency-domain view with its own units. Channel-response/echo inspection is
separate from spectral power.

![Receiver SA test-point selection](../images/workbench/receiver-spectrum-dark.png)

Choosing NCP or PLC in the QAM analyzer does not change the SA's full-channel
input. Selecting a test-point marker opens the instrument at that location;
switching instruments or selectors reads the same retained run and never starts
a new simulation. If that run did not retain the requested data, the UI explains
it rather than substituting other samples or leaving an old plot mislabeled.

Receiver SA **Run simulation** is disabled until receiver-run dispatch exists.
It does not call the transmitter preview. TX-only window timing is hidden for
receiver instruments. Response/echo, slicer/LLR and FEC buttons currently explain
their pending data connections. Receiver remains yellow; opening instruments
does not satisfy run readiness.

The existing TX SA remains available. Actual receiver plots require the pending
full-loop UI connection and bounded retained-result session. The first payload
collector supports a limited profile selection; general profile and NCP views
must be enabled only as their Rust APIs support them.

See the [task design](../development/features/receiver-workbench/design.md) and
[workbench guide](workbench.md). Screenshots show the actual dark Qt UI on the Ubuntu VM.
