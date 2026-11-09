# Waveform source evidence and validation scope

| Pinned source | Direction / requirement | Implementation and validation |
| --- | --- | --- |
| CM-SP-PHYv3.1-I20-230419 §7.5.7.1–2, PDF133–135; Table38 | CMTS centered IDFT, increasing physical-bin order, unitary scale, 204.8Msamples/s and4K/8K geometry | Shared safe Rust radix2 behind TX IFFT/RX FFT; independent DC/quarter-tone/impulse/direct-sum and energy vectors |
| PHY3.1 I20 §7.5.8.1, PDF135–136, Tables39–40 | CMTS prefix/windowing delegation; permitted CP/RP; RP smaller than CP; zeroRP evaluation-only | Frozen geometry uses existing settings validator; TX/RX prefix tests and TX window tests cover every permitted pair at both spacings |
| PHY3.1 I20 §7.4.10.1, PDF69, Figure18 | Delegated cyclic suffix and symmetric half-sample taper | Separate prefix/window owners; coefficient, symmetry, complement and independent window-energy tests |
| PHY3.1 I20 §7.4.10.1–2.1, PDF70 | Overlap final/firstRP samples; cadence N+CP | Stateful overlap owner with explicit tail drain; independent shifted-sum, rejected-call and clock tests |
| CM-SP-PHYv4.0-I07-250627 §7.3.5.7–8 | DOCSIS4.0 FDD delegates IDFT and CP/windowing to PHY3.1 | Same pinned baseline; no FDX-specific inference |

The authoritative local PHY3.1 PDF pages133 and69 were rendered and visually
inspected for transform sign, centering, scaling and the half-sample window
formula. Neighboring source paragraphs and tables were read from pinned local
text. PHY4.0 delegation was checked in its pinned local text. No latest-issue
or external interoperability claim follows from these checks.

Receiver FFT inversion, exact-boundary framing, supplied-reference equalization,
unity sample transfer, frozen-session policy and application bounds are explicit
engineering/reference choices. They do not claim a complete normative modem
receiver or blind synchronization implementation. Header/cell/pilot/PLC rules
remain owned by their existing source mappings in [PLC evidence](plc-evidence.md).

[Waveform guides](../user/waveform/README.md) document each subsystem, mathematical
convention, data lifetime and remaining integration boundaries. The bounded
example demonstrates actual PLC decoding through waveform samples, not encoded
payload/NCP transport or the future one-second production runner.
