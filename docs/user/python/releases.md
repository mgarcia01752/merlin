# Python wheel releases

[Python installation and API guide](README.md) · [Downstream installation](integration.md) · [Release overview](../install/README.md)

This page describes MERLIN's Python release assets and their required checks.
It is separate from the installation guide so package delivery rules and native
validation evidence have one maintained location.

## Release targets and filenames

The configured release matrix builds **CPython 3.10, x64** wheels:

| Native runner | Release wheel pattern | Library inside |
| --- | --- | --- |
| Ubuntu 22.04 x86_64 | `merlin_ofdm-<version>-cp310-cp310-linux_x86_64.whl` | `merlin/merlin.cpython-310-x86_64-linux-gnu.so` |
| Windows 2022 x64, MSVC | `merlin_ofdm-<version>-cp310-cp310-win_amd64.whl` | `merlin/merlin.cp310-win_amd64.pyd` |

Install the wheel, not the raw `.so` or `.pyd`. Each wheel includes the Rust
engine, native Python bindings and complete dependency notices. The consumer
needs neither Qt, a MERLIN CLI executable nor a Rust compiler. CPython ABI and
architecture must match. A native `linux_x86_64` wheel is not a manylinux promise;
other Linux distributions/Python versions and macOS are outside this release matrix.

## Release gate

Only a version-tag Release run enables the shared workflow's `python-wheels`
job. Ordinary master/PR validation skips this matrix. Ubuntu and Windows each:

1. Install CPython 3.10 x64, Rust 1.85.1 and maturin 1.8.3. Windows selects MSVC
   and uses PowerShell; Ubuntu uses Bash. Git Bash must not precede MSVC with its
   unrelated `link.exe` utility. Both Windows jobs use the shared native
   `scripts/windows/setup-msvc-ci.ps1` helper; no Node-based MSVC action is needed.
2. Fetch the locked binding graph and build the optimized wheel through
   `tools/python/build_wheel.py`, preserving licenses/notices.
3. Verify filename, distribution/version, CPython ABI/platform tags, native
   extension type and required legal files.
4. Create a fresh temporary venv, install that wheel with no dependencies/network,
   and verify the imported native module resides inside that environment.
5. Run the actual `examples/api/python/run_single_capture.py` using isolated
   Python from outside the checkout. It must complete with this fixture's
   82 total codewords, zero corrected/uncorrectable words and BER `0.000e+00`.
   These are actual engine outcomes, not replacement estimates.
6. Stage only the successful wheel and its `SHA256SUMS`, retaining a separate JSON
   report with platform, native path, output, checksum, elapsed time and status.

Both native wheel artifacts are required by release collection. Missing artifacts,
failed jobs or modified checksummed bytes block publication. Existing CodeQL,
RustSec and application test/package gates remain required. Wheel jobs add no
Windows CLI package or desktop dependency.

The configured native jobs are not evidence of successful hosted execution.
Linux local installed-wheel checks pass; Windows runtime and actual release
asset availability remain unverified until the matching native release job
succeeds and the resulting wheel is published. The test proves a clean Python
environment on that runner, not installation on every pristine OS version.

## Download, verify and install

Select the wheel matching your CPython 3.10 interpreter and OS from the chosen
MERLIN GitHub release. Download its `SHA256SUMS` alongside it. Pin the exact
version and verified hash in the consuming project; do not infer asset existence
from a tag or this filename pattern.

On Ubuntu, verify downloaded assets with `sha256sum --ignore-missing --check
SHA256SUMS`. On Windows, compare `Get-FileHash <wheel-file> -Algorithm SHA256`
with that wheel's entry in `SHA256SUMS`. Then use the consuming environment's
interpreter to install the downloaded file:

```text
python -m pip install --no-index --only-binary=:all: <downloaded-wheel-file>
python -c "import merlin; print(merlin.__version__)"
```

Use a venv rather than a system-wide install. The [downstream integration guide](integration.md)
explains local environments, exact version/tag requirements and hash-pinned
consumer dependencies. The native API [examples](../../../examples/api/README.md)
require their documented fixture files from the matching source checkout;
these measurement examples are not embedded in the wheel.

## Local validation helper

After building a fresh wheel directory with the matching Python interpreter:

```text
python tools/python/smoke_wheel.py --wheel-dir <new-wheel-directory> --output <new-asset-directory> --report <report.json>
```

The helper creates/removes its temporary venv, leaves inputs untouched and rejects
an existing output directory. It publishes nothing. Reports/artifacts belong in
ignored `local/`, `dist/` or `docs/development/local/` folders. Native Windows
validation must run on Windows; a passing Linux check cannot verify its `.pyd`.
