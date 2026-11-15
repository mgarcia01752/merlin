# Install MERLIN's Python API

Install only `merlin-ofdm` (import `merlin`) in a virtual environment.
The desktop application, standalone CLI and Qt are not installed.

## From a checkout

```bash
./install.sh --install-pymerlin --check
./install.sh --install-pymerlin --venv .venv-merlin
.venv-merlin/bin/python -c 'from merlin import Simulation; print(Simulation())'
```

The entry point delegates to `install-local.sh` and `install.py`. It checks
Ubuntu x64, CPython 3.10, venv/ensurepip, the destination, Rust 1.85.1 and a C
linker. It installs pinned maturin in the private environment, builds an optimized
wheel through the existing notice-generating packaging helper, installs it and
checks the native import/version. Builds need network access for dependencies.
See [build prerequisites](../../docs/development/build.md).

## From a published release (no compiler)

Download [install.py](install.py), then run:

```bash
MERLIN_TAG=v0.3.1
python3 install.py --tag "$MERLIN_TAG" --check
python3 install.py --tag "$MERLIN_TAG" --venv .venv-merlin
```

On Windows, use `py -3.10 install.py --tag v0.3.1 --venv .venv-merlin`.
Use `.venv-merlin\Scripts\python.exe` afterward. Windows execution remains
unverified locally. Supported release-wheel hosts are Ubuntu 22.04, 24.04,
26.04 x64 and Windows x64, with 64-bit CPython 3.10. A release must actually
contain the matching wheel and SHA256SUMS; version numbers do not prove availability.

Release mode verifies the wheel's SHA-256 before creating the environment or
installing it. `--check` / `--dry-run` performs local checks without downloading,
installing or creating directories. Use `--python PATH` to select CPython 3.10.
Nonempty destinations must already be compatible virtual environments.

On Ubuntu 22.04, explicit `--install-prerequisites` allows sudo apt to install
`python3.10` and `python3.10-venv`. Without that flag, missing requirements cause
an actionable error. Newer Ubuntu needs separately provisioned Python 3.10;
the installer does not replace system Python. No automatic Rust or linker
installation occurs. Source builds may leave their private environment/build
artifacts after failure; correct the error and rerun.

See [Python installation and dependency pins](../../docs/user/python/integration.md).
