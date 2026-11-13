# CLI examples and use cases

[Examples index](../README.md) · [CLI guide](../../docs/cli.md) ·
[Channel runner guide](../../docs/user/channel-runner/cli.md)

These JSON files are **inputs for the `ofdm-fec` command-line program**. They let
you reproduce a simulation or analysis without clicking through the desktop UI.
They are also used by development tests and smoke checks. They are not saved UI
settings and are not included in desktop installers.

## Which file to use

| File | Use case | Command |
| --- | --- | --- |
| `awgn.json` | Run seeded FEC trials with additive white Gaussian noise; compare recovery under a chosen MER. | `simulate` |
| `scenario.json` | Supply a small synthetic measurement scenario to candidate evaluation or measurement-driven trials. It is not a captured modem recording. | `evaluate` or `simulate --scenario` |
| `conservative-16qam.json` | Configure conservative candidate evaluation capped at four bits per symbol (16-QAM); pair it with a scenario. | `evaluate --settings` |
| `channel.json` | Start with the baseline OFDM transmitter/channel/receiver simulation. | `channel` |
| `channel-awgn.json` | Exercise the waveform receiver with seeded sample noise. | `channel` |
| `channel-constellation.json` | Retain bounded transmit/receive constellation pairs for inspection. | `channel` |
| `channel-file.json` | Send a finite file payload, optionally fill to an explicit bit count, and compare recovered original bytes. | `channel --payload-file` |
| `channel-pilot-estimated.json` | Exercise pilot-derived receiver equalization and protected constellation observations. | `channel` |
| `channel-mce.json` | Exercise MERLIN Channel Estimation (MCE) with a weak echo and sample noise. | `channel` |
| `channel-power.json` | Inspect amplifier, attenuation and receiver-noise power settings. | `channel` |
| `channel-probes.json` | Export bounded intermediate test-point observations as JSONL. | `channel --probes-output` |
| `payloads/message.txt` | Provide known original bytes for the file-payload example and an independent byte comparison. | `--payload-file` |

## Run an example

Run these commands from the repository root. Build the standalone CLI once:

```bash
scripts/cargo-local.sh build -p ofdm-fec-cli --locked
```

Run seeded FEC trials:

```bash
target/debug/ofdm-fec simulate --input examples/cli/awgn.json --output awgn-result.json
```

Evaluate a candidate against the synthetic scenario:

```bash
target/debug/ofdm-fec evaluate --input examples/cli/scenario.json \
  --settings examples/cli/conservative-16qam.json --profile 0 --output candidate.json
```

Run the baseline OFDM channel or substitute another `channel-*.json` request:

```bash
target/debug/ofdm-fec channel --input examples/cli/channel.json --output channel-result.json
```

Send and recover the sample file:

```bash
target/debug/ofdm-fec channel --input examples/cli/channel-file.json \
  --payload-file examples/cli/payloads/message.txt \
  --output file-result.json --recovered-file recovered-message.txt
cmp examples/cli/payloads/message.txt recovered-message.txt
```

Export test points:

```bash
target/debug/ofdm-fec channel --input examples/cli/channel-probes.json \
  --output probes-result.json --probes-output probes.jsonl
```

Use a new output filename for every run: existing outputs are deliberately not
overwritten. Results are written to the explicit output paths, not back into the
example JSON. On Windows source builds the executable ends in `.exe`; the Windows
desktop MSI does not include the standalone CLI. See each linked guide for the
supported model, work limits and interpretation of the results.

Shared measurement/import workflows remain under
[measurement examples](../measurements/README.md). Desktop workbench examples are
separate under [workbench](../workbench/README.md).
