# Getting started with MERLIN

[Documentation index](../README.md) · [Toolkit overview](../../README.md)

## Desktop and CLI

**Desktop:** follow the [Ubuntu DEB](install/ubuntu-desktop.md) or
[Windows MSI](install/windows.md) guide. Check the exact version's
[release assets](https://github.com/mgarcia01752/merlin/releases) and checksums
before installing. A Git tag alone does not establish published packages or
successful native validation. macOS delivery remains deferred.

**CLI from source:** configure the pinned toolchain using the
[development setup guide](../development/setup.md), then run these commands
from the repository root:

```bash
scripts/cargo-local.sh build -p ofdm-fec-cli --release --locked
mkdir -p local

# Run the downstream waveform/AWGN example.
target/release/ofdm-fec channel \
  --input examples/cli/channel-awgn.json \
  --output local/downstream-report.json

# Validate the upstream plan without running the engine, then execute it.
target/release/ofdm-fec upstream preview \
  --input examples/cli/upstream/us-50khz.json \
  --output local/upstream-preview.json
target/release/ofdm-fec upstream run \
  --input examples/cli/upstream/us-50khz.json \
  --output local/upstream-report.json
```

Choose new output paths on subsequent runs; existing files are never replaced.
Inspect recovery and FEC outcomes as well as run status. See the
[downstream CLI](channel-runner/cli.md),
[upstream CLI](upstream/cli.md) and
[CLI example inventory](../../examples/cli/README.md) for measured runs, probes and limits.

## Rust and Python APIs

The thin `merlin` facade exposes both independent engines and shared contracts:

```rust
use merlin::downstream::Simulation as DownstreamSimulation;
use merlin::upstream::Simulation as UpstreamSimulation;
use merlin::common;
```

The native `merlin-ofdm` Python distribution mirrors those namespaces:

```python
from merlin.downstream import Simulation as DownstreamSimulation
from merlin.upstream import Simulation as UpstreamSimulation
import merlin.common
```

Existing `docsis_ofdm_fec::api::{Simulation, Merlin}` Rust imports and Python
`merlin.Simulation` retain downstream behavior. Loading and configuration remain
idle until an explicit run. Python calls the Rust engine directly.

Start with the [Rust API guide](../reference/public-api.md),
[Python installation guide](python/integration.md),
[upstream Python guide](python/upstream.md) and
[paired Rust/Python examples](../../examples/api/README.md).
