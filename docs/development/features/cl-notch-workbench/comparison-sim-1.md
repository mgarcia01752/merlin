# WB-CL-03: saved Designer comparison

All runs completed using the real Rust engine through the local Python wheel.
The original `wb-sim-1.merlin.json` is unchanged. Rust independently ran that
file and matched Python FEC/profile results exactly. All cases used Profile 3,
4096-QAM, 192 MHz occupied / 190 MHz active envelope, PLC at 333 MHz,
10 ms observation, 50 decoder iterations and noise seed 41.

| Workbench | Total | Corrected | UFEC | Clean | Wall seconds |
| --- | ---: | ---: | ---: | ---: | ---: |
| wb-sim-1.merlin.json | 1273 | 1220 | 53 | 0 | 19.630 |
| cl-unity-notch-0db.merlin.json | 1273 | 1273 | 0 | 0 | 8.594 |
| cl-unity-notch-10db.merlin.json | 1273 | 1273 | 0 | 0 | 8.849 |
| cl-unity-notch-20db.merlin.json | 1273 | 1273 | 0 | 0 | 8.391 |
| cl-unity-notch-30db.merlin.json | 1273 | 1273 | 0 | 0 | 7.478 |
| cl-unity-notch-40db.merlin.json | 1273 | 731 | 0 | 542 | 6.394 |
| cl-unity-notch-48db.merlin.json | 1273 | 0 | 0 | 1273 | 5.556 |
| cl-unity-notch-5db.merlin.json | 1273 | 1273 | 0 | 0 | 8.365 |
| user-cl-notch.merlin.json | 1273 | 1269 | 4 | 0 | 14.329 |
| user-materialized.merlin.json | 1273 | 1220 | 53 | 0 | 24.679 |

The user design has a 48 dB flat starting level, 0.75 dB random variation,
−3 dB tilt, 2 dB peak-to-peak ripple at 20 MHz spacing, and an 18 dB-deep
10 MHz notch centered at 394.9875 MHz with 1 MHz edges. It uses the adapted
measured response and MCE. `user-cl-notch` changes only the notch center to
375 MHz and width to 8 MHz. Its four UFEC versus 53 in the original is a
combined position/width comparison; it does not isolate either variable.

The unity cases use CM supplied-channel correction with no supplied response,
a flat 48 dB background, no random variation/tilt/ripple, and a rectangular
8 MHz notch at 375 MHz. The number in each filename is the notch MER, not
its depth. This sweep retains zero UFEC through 0 dB notch MER in this finite
simulation. It therefore **does not reproduce the CableLabs failure transition**.

CableLabs CM-GL-PNM-3.1-V05-230927, Appendix II.3.2, pp.209–211, specifies
192 MHz / 190 MHz, 4096-QAM, PLC 333 MHz and LTE ingress at 375 MHz / 8 MHz.
Figure 192 (p.210) varies LTE interference and plots average MER over the LTE
band. The text distinguishes CM and VSA measurement planes and their threshold
disagreement. These RxMER-conditioned Gaussian-noise simulations do not reproduce
the LTE waveform, hardware receivers, acquisition behavior or laboratory exposure
duration. A 48 dB background is our explicit assumption, not a published fixed
value for Figure 192. No universal unusable-carrier percentage follows from these
results, and parameters were not tuned to manufacture a matching failure threshold.

[CableLabs document portal](https://www.cablelabs.com/specifications/CM-GL-PNM-3.1),
[pinned source map](../../../../skills/docsis-pnm-rxmer-fec/references/source-map.md),
[machine-readable results](comparison-sim-1.json). Exact graph values were not digitized.

The initial pre-fix Python run gave 54 UFEC. API-WB-03 corrected JSON float
round-trip parsing; the final direct Rust, direct Python and materialized runs
all give 53. Preserve that distinction when comparing earlier screenshots.

Run a comparison with:

```bash
local/api-python-test/bin/python examples/api/python/run_multi_rxmer_cmce_workbench.py examples/workbench/cl-192mhz-8mhz-notch/comparison-sim-1/user-cl-notch.merlin.json
```

The materialized cases were created by `prepare_cl_comparison`; the public
Rust/Python Workbench loader now accepts the original Designer recipe directly.
