# Synthetic channel-width examples

These ideal inputs test **24 MHz minimum occupied width** and **90 MHz mid-width**.
They are synthetic JSON in the supported decoder shape, not measured captures or
external interoperability vectors. No original capture has been modified.

| Directory | Occupied width | Active envelope | Grid samples at 25 kHz |
| --- | --- | --- | --- |
| [24mhz](24mhz/import.json) | 24 MHz | 22 MHz | 880 |
| [90mhz](90mhz/import.json) | 90 MHz | 88 MHz | 3,520 |

Each directory has RxMER, channel estimation and a modulation-profile document
with the same four IDs/data orders as the single-channel example:
0 = 256-QAM, 1 = 1024-QAM, 2 = 2048-QAM, 3 = 4096-QAM.
Every profile covers the complete active grid, including the common PLC and
continuous-pilot assignments. Pilot positions are regenerated with the shared
Rust PHY placement routine; simply truncating the wide capture could leave too
few pilots at the minimum width.

The grid starts at 640 MHz. RxMER is a synthetic flat 48 dB, channel correction is
unity, and timestamps are identical. The channel requests use **no added noise**
and unity response to isolate channel-width/bit-loading behavior. Importing the
48 dB RxMER does not automatically enable noise in the standalone channel requests.
An engineering choice of 1 MHz taper allowance at each edge gives the active
widths above; occupied width and active carrier span are different quantities.

Load and run, using new output filenames:

```bash
cd /home/dev01/Projects/DOCSIS-OFDM-Channel-Sim &&
scripts/cargo-local.sh run -p ofdm-fec-cli -- import-pnm \
  --input examples/measurements/channel-widths/24mhz/import.json \
  --output /tmp/merlin-24mhz-scenario.json
```

```bash
cd /home/dev01/Projects/DOCSIS-OFDM-Channel-Sim &&
scripts/cargo-local.sh run -p ofdm-fec-cli -- channel \
  --input examples/measurements/channel-widths/24mhz/channel-profile-3.json \
  --output /tmp/merlin-24mhz-profile-3-result.json
```

Replace `24mhz` with `90mhz` or the profile suffix with `0`, `1` or `2`.
Channel requests carry the selected profile's exact per-bin loading. The fixed
runner announces it as its internal Profile A (ID 0), while the imported scenario
preserves the four original IDs. These tests do not expand the measured-selector
API beyond ID 0 or claim dynamic switching between the four profiles.
Schema-1 import is deliberate: these are generated fixtures, not exports from
an actual `pnm-decode` producer; no producer-version declaration is fabricated.

Regenerate into a **new** directory; existing output roots are rejected:

```bash
cd /home/dev01/Projects/DOCSIS-OFDM-Channel-Sim &&
scripts/cargo-local.sh run -p docsis-ofdm-fec \
  --example prepare_channel_widths -- /tmp/merlin-new-channel-widths
```

Source: CM-SP-PHYv3.1-I20-230419, section 7.3.1/Table 6, printed pp.47–48,
and section 7.5.9, PDF p.136 (occupied versus encompassed spectrum). The supported occupied
range is 24–192 MHz in 6 MHz increments. This example covers 25 kHz spacing;
it does not establish the same width matrix at 50 kHz.
