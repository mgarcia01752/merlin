> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/waveform/`
> Logs, captures and specification images are not published.

# Waveform reference milestone

Implement WAVE-TX-01..03, WAVE-RX-01..04, WAVE-CH-01 and WAVE-QA-01..02.
Keep nonideal acquisition WAVE-RX-05 and production data/NCP WAVE-RX-06 pending.
Pinned source: PHY I20 7.5.7.1 PDF133 centered unitary IDFT; 7.5.8.1 PDF135-136
CP/RP; delegated7.4.10.1 PDF69 taper/suffix and PDF70 overlap. Local PDF pages133
and69 rendered and visually inspected. PHY4.0 I07 7.3.5.7/8 delegate to PHY3.1.
No native FFT dependency: safe Rust radix2 behind TX/RX facade. Immutable geometry,
bounded sample blocks, pure window/prefix and stateful bounded overlap/framing.
RX receives sample blocks and explicit bootstrap layout/timing; decoded PLC then
builds independent received configuration. No input truth enters RX.
