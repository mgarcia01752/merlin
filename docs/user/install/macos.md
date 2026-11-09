# macOS install and uninstall

Choose `macos-arm64.pkg` for Apple Silicon or `macos-x64.pkg` for Intel.
The matching portable `.tar.gz` is an alternative. Verify downloads against the
same release's `SHA256SUMS`:

```bash
shasum -a 256 --ignore-missing --check SHA256SUMS
```

Open the matching PKG and follow Apple's installer prompts. The current packages
are unsigned/not notarized; follow macOS and your organization's approval policy.
Native build/package execution is pending the later CI run. Reinstall a newer
PKG to upgrade, then run `ofdm-fec --version` in a new terminal.

The command installs as `/usr/local/bin/ofdm-fec`; examples/guides/licenses are
under `/usr/local/share/merlin`. To remove this exact installed CLI package:

```bash
sudo rm /usr/local/bin/ofdm-fec
sudo rm -r /usr/local/share/merlin
sudo pkgutil --forget org.merlin.ofdm.cli
```

These commands do not remove simulation output elsewhere. For a portable copy,
extract the archive, run its `bin/ofdm-fec` and remove only that extracted directory
to uninstall. No Finder application or DMG is included for the CLI-only stage.
