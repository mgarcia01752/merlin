# Install MERLIN's Python API

Install only `merlin-ofdm` (import `merlin`) in a virtual environment.
The desktop application, standalone CLI and Qt are not installed.

## From a checkout

```bash
./install.sh --install-pymerlin --check
./install.sh --install-pymerlin --venv .venv-merlin
.venv-merlin/bin/python -c 'import merlin; print(merlin.__version__)'
```

The entry point delegates to `install-local.sh` and `install.py`. It checks
Ubuntu x64, CPython 3.10 or newer, venv/ensurepip, the destination, Rust 1.85.1 and a C
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

On Windows, use `py -3 install.py --tag v0.3.1 --venv .venv-merlin`.
Use `.venv-merlin\Scripts\python.exe` afterward. Windows execution remains
unverified locally. The installer accepts Ubuntu x64 without a release allowlist and Windows x64,
with 64-bit CPython 3.10 or newer. Source builds use that interpreter.
Release mode requires a wheel matching its ABI; existing cp310 wheels only
work with Python 3.10. Missing assets cause an error, not an interpreter downgrade. A release must actually
contain the matching wheel and SHA256SUMS; version numbers do not prove availability.

Release mode verifies the wheel's SHA-256 before creating the environment or
installing it. `--check` / `--dry-run` performs local checks without downloading,
installing or creating directories. Use `--python PATH` to select CPython 3.10 or newer.
Nonempty destinations must already be compatible virtual environments.

On Ubuntu, explicit `--install-prerequisites` allows sudo apt to install
`python3`, `python3-venv` and `python3-dev` for that Ubuntu release.
Without that flag, missing requirements cause an actionable error. The installer
does not replace system Python. No automatic Rust or linker
installation occurs. Source builds may leave their private environment/build
artifacts after failure; correct the error and rerun.

See [Python installation and dependency pins](../../docs/user/python/integration.md).

The shared CI/CD workflow tests source installation on Ubuntu 22.04/Python 3.10,
24.04/3.12, 26.04/3.14 and Windows/3.14. It checks the installed version without
running simulation. These are configured gates; hosted success must be verified.
