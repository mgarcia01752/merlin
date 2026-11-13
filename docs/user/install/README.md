# Install MERLIN

Choose the desktop package for the simulation workbench:

| Platform | Install / uninstall |
| --- | --- |
| Ubuntu 22.04 amd64 desktop | [Desktop DEB](ubuntu-desktop.md) |
| Windows x64 desktop UI | [Desktop MSI](windows.md) |
| Ubuntu 22.04, 24.04 and 26.04 x64 CLI | [CLI DEB or portable archive](linux.md) |

Windows native build/MSI/install/repair/shortcut/uninstall checks pass in v0.2.7.
Interactive Windows 11 and upgrade/downgrade review remain pending. Windows CLI
assets are excluded.
macOS delivery remains deferred.

Release inventory includes the Ubuntu desktop and CLI DEBs, Linux CLI archive,
Windows desktop MSI and `SHA256SUMS`. Publication requires every configured
native/security job to pass. Download the exact asset for your platform from
[Releases](https://github.com/mgarcia01752/merlin/releases), not a source archive.

Desktop packages include Qt, curated [workbench bundles](../workbench-examples.md),
documentation and full notices. Native Open starts at the installed examples.
Keep user workbenches/exports in a writable folder outside the installation.
The CLI has its own executable and bundled resources; it does not contain Qt.

## Linux CLI source builds and development packages

```bash
scripts/cargo-local.sh build -p ofdm-fec-cli --release --locked
python3 tools/packaging/package.py --binary target/release/ofdm-fec \
  --output dist/local-package --native
```

Packaging requires a version-matched binary and a fresh output directory.
`dpkg-deb` builds the Linux CLI DEB; omit `--native` for an archive/checksums only.
Python and a cached locked Cargo graph collect dependency notices during the
build. These tools are not runtime dependencies of the installed CLI.
Packaging does not install anything or replace existing outputs.

Native desktop builds: [Qt development](../../development/qt.md) ·
[Windows build/MSI](../../development/windows-desktop.md).
