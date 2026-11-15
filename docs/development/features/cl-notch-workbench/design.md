# WB-CL-01: CableLabs-style 192 MHz notch workbench

Create self-contained main/notch and flat-baseline WBs using Rust's physical
layout/pilot generator, decoded synthetic JSON and the existing saved-WB loader.
Match V05 Appendix II's 192 MHz occupied / 190 MHz modulated envelope, nominal
PLC center 333 MHz, ingress center 375 MHz and 8 MHz width, fixed 4096-QAM.
Set the PLC's first carrier to 332.8 MHz (16 bins at 25 kHz); the six-MHz PLC
neighborhood is 330–336 MHz. The guide does not pin channel edges: select
299–491 MHz occupied and 300–490 MHz active envelope as explicit MERLIN choices.

Other explicit choices: flat 42 dB background, rectangular 10 dB notch over
[371,379) MHz, 25 kHz spacing, CP512, roll-off128, interleaver16, unity response,
CM channel correction (unity without a supplied estimate), PRBS15 seed32767,
noise seed41, 10 ms observation and 300 s runtime cap. No gain/loss/echo additions.
This is localized Gaussian noise from RxMER, not an LTE waveform, calibrated
0 dBmV hardware test or claimed reproduction. Companion flat WB isolates baseline.

The reusable Rust engine supplies carrier roles. The generator packages those
synthetic documents with hashes/base64 and validates both WBs through pnm-input.
No copied field capture bytes or altered decoder algorithms. Retain a manifest,
reproducible generator, user instructions, native admission and bounded run evidence.

## Validation

Both saved files passed Rust preparation. The notch WB also passed native Qt
`--check-workbench`. The existing Python WB runner (native MERLIN 0.3.1 on this
Ubuntu host) completed both 10 ms experiments: 1,273 total words each, zero UFEC
and zero post-FEC BER. Notch: 1,273 corrected, 10.225 s wall time; flat baseline:
1,057 corrected, 8.105 s. These are local simulated observations, not CableLabs
results or a field reliability guarantee. The scoped Rust fixture test verifies
PLC first/last carriers, exact notch positions, active envelope, modulation codes
and unity selection. Full suite and other platforms were not run for this fixture.
