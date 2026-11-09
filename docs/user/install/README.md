# Install the MERLIN CLI

MERLIN currently provides `ofdm-fec`, a command-line program. A Qt application
is not packaged. Native release automation is prepared locally; Windows/macOS
execution and public release publication await the later workflow push.

| Platform | Install / uninstall |
| --- | --- |
| Ubuntu22.04 or newer, x64 | [DEB or portable archive](linux.md) |
| Windows, x64 | [MSI or portable ZIP](windows.md) |
| macOS, Apple Silicon or Intel | [PKG or portable archive](macos.md) |

A release contains four native installers (Ubuntu, Windows, and two macOS CPU
architectures), matching portable archives and one combined `SHA256SUMS`.
The CLI, example inputs and applicable licenses are included in each package.
Source builds remain independent of native installer tools:

```bash
scripts/cargo-local.sh build -p ofdm-fec-cli --release --locked
```

Development package creation uses an already-built native binary whose reported
version must match root `VERSION`. Pick a new output directory:

```bash
python3 tools/packaging/package.py --binary target/release/ofdm-fec \
  --output dist/local-package --native
```

On Windows use `python` and `target/release/ofdm-fec.exe`. Native tooling is
`dpkg-deb` on Linux, Apple's `pkgbuild` on macOS, or WiX3.14.1 on Windows
(`WIX_BIN` selects its bin directory). Bash/Git Bash and the existing Cargo source
cache are needed only during development packaging to collect pinned dependency
notices; neither is a runtime dependency of the installed CLI. Omitting `--native` creates only the
portable archive and checksums. Packaging installs nothing and refuses existing
output directories. Full native build execution on each target remains required
before claiming support on that platform.
