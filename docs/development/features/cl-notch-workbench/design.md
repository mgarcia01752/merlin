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

## WB-CL-02: adapted measured channel companion

Create `cmce.merlin.json` alongside the unchanged unity experiments. User-approved
frequency alignment maps the existing single-channel response's 640–826.975 MHz
endpoints (7,480 samples) to 300–489.975 MHz (7,600 samples). For target index j,
source coordinate u = j × 7479 / 7599; linearly interpolate complex I and Q
between floor(u) and ceil(u). Exact knots retain their values; missing endpoints
produce missing output, never zero filling. No extrapolation or phase unwrapping.
This stretches the response shape and its delay scale; it is a synthetic adapted
response, not a measurement at the destination frequencies or a CL test setting.

The Rust example generator owns this fixture-only transformation; production
capture/grid validation remains unchanged. Preserve source bytes, timestamp and
SHA-256 in provenance. The derived capture takes the synthetic WB identity/time
for deliberate association. Supply the same adapted response to channel impairment
and CM channel correction. Keep RxMER, PLC, seeds and run limits unchanged.
Validate interpolation independently, Rust WB preparation, embedded hashes,
reproducibility, native UI admission and one bounded engine run.

### WB-CL-02 validation

Interpolation/missing-value/bounds unit checks passed; both CL fixture preparation
tests passed (0.13 seconds). Native Qt `--check-workbench` accepted the companion.
The native Python WB runner completed in 10.187 seconds: 1,273 total codewords,
1,273 corrected, zero uncorrectable and zero post-FEC BER. This is a finite
10 ms simulation with known supplied correction, not a field reliability claim.
Scoped Clippy and workspace formatting passed. The original five generated
files are byte-identical; original capture bytes are unchanged. No UI code or
engine behavior changed, and no full-suite/platform certification is claimed.

All eight generated JSON files reproduce byte-for-byte; source, embedded and
manifest hashes passed. Existing output directories are rejected. Five desktop
packaging tests passed (0.038 seconds); the companion is included by the existing
example directory packaging rule.
