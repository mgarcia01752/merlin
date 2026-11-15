# CableLabs-style 192 MHz / 8 MHz notch experiment

Open **workbench.merlin.json** in MERLIN for the notched case, or
**baseline.merlin.json** for the flat comparison. Opening does not start a run.
Both files are self-contained; press **Start** to run the actual Rust decoder.

This synthetic experiment approximates CM-GL-PNM-3.1-V05-230927, Appendix II,
pp.208–210 (especially Figure 192). It is an indirect comparison, not a
reproduction of CableLabs hardware measurements or an LTE waveform generator.
[Source/download and page map](../../../skills/docsis-pnm-rxmer-fec/references/source-map.md).

## Configuration

| Setting | Value | Basis |
| --- | --- | --- |
| Occupied / active envelope | 192 / 190 MHz | CableLabs experiment |
| Data modulation | 4096-QAM, captured Profile 3 | CableLabs modulation; ID chosen here |
| PLC nominal center | 333 MHz | CableLabs frequency plan |
| PLC first / last carrier | 332.800 / 333.175 MHz | 16 carriers at 25 kHz; 400 kHz allocation |
| PLC neighborhood | 330–336 MHz | MERLIN physical layout |
| Notch center / width | 375 / 8 MHz | CableLabs experiment |
| Notch frequency interval | 371 MHz inclusive to 379 MHz exclusive | Exact 320-position grid interval |
| Occupied edges | 299–491 MHz | MERLIN choice; source does not specify edges |
| Active grid | 300.000–489.975 MHz, 7,600 positions | 190 MHz envelope, MERLIN edge placement |
| Background RxMER | 42 dB | Chosen near the guide's 41–42 dB operating discussion |
| Notch RxMER | 10 dB | Starting experiment point, not a claimed failure threshold |
| Channel / receiver | Unity; CM channel estimation mode | No supplied estimate; correction is unity |
| Timing | 25 kHz, CP 512, roll-off 128, interleaver depth 16 | Explicit MERLIN choices |
| Continuous pilots | Existing Rust generator, default density 48 / seed 1 | Explicit MERLIN choice |
| Source / noise | PRBS15 seed 32767 / noise seed 41 | Reproducible MERLIN choices |
| Observation / limits | 10 ms / 50 decoder iterations / 300 s runtime | Bounded MERLIN choices |

The notch spans 8/190 = 4.21% of the active envelope, including any pilot
positions in that interval. This is not a count of unusable data carriers.
The PLC is outside the notch. No additional tilt, cable, echo, launch gain or
Common Path Attenuator loss is applied. The synthetic JSON epoch is shared
across inputs; it does not describe a field acquisition.

RxMER drives localized Gaussian error power in the existing engine. It does
not reproduce LTE timing, bursts, RF power calibration, modem acquisition or
vendor decoder behavior. Input RxMER and measured receiver MER are different
quantities. FEC is obtained from the actual simulated decoder, never inferred
from a below-target percentage. The flat WB changes only the RxMER input.

The captured Profile 3 is evaluated through MERLIN's existing Profile A wire
transport; the UI retains the captured identity in its report. A short run with
zero UFEC does not establish a field error-rate guarantee or validate a universal
notch threshold. Longer runs and multiple noise seeds are separate experiments.

## Reproduce the inputs

The Rust generator uses existing carrier-layout and pilot operations and rejects
an existing output directory. Run from the repository root with a new destination:

```bash
scripts/cargo-local.sh run -p pnm-input --example prepare_cl_notch_workbench -- /tmp/merlin-cl-notch-new
```

The five generated JSON files are deterministic. The checked-in provenance
records their hashes. The JSON captures use the existing decoded-input contract;
these are synthetic inputs, not binary PNM exports or borrowed modem captures.

To run either saved WB using an installed MERLIN Python API:

```bash
python examples/api/python/run_multi_rxmer_cmce_workbench.py examples/workbench/cl-192mhz-8mhz-notch/workbench.merlin.json
python examples/api/python/run_multi_rxmer_cmce_workbench.py examples/workbench/cl-192mhz-8mhz-notch/baseline.merlin.json
```

[Validation record](../../../docs/development/features/cl-notch-workbench/design.md).
