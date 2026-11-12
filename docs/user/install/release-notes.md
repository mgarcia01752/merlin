MERLIN includes a native desktop workbench and a separate `ofdm-fec` command-line toolkit.
Choose the desktop DEB to run the UI; the CLI assets are optional.

## Release assets

| Asset | Purpose |
| --- | --- |
| `merlin-desktop_@VERSION@_ubuntu22.04_amd64.deb` | Ubuntu 22.04 desktop, bundled Qt, examples and guides |
| `merlin-cli_@VERSION@_ubuntu22.04_amd64.deb` | Standalone command-line toolkit installed by APT |
| `merlin-cli_@VERSION@_linux_amd64.tar.gz` | Portable command-line toolkit; extract and run |
| `SHA256SUMS` | Checksums for verifying the downloaded assets |

## Ubuntu desktop install or upgrade

From the folder containing this release's downloaded desktop DEB and checksums:

```bash
sha256sum --ignore-missing --check SHA256SUMS
sudo apt install ./merlin-desktop_@VERSION@_ubuntu22.04_amd64.deb
```

Keep `./`: APT needs a local file path. Use the exact downloaded filename.
[Generic APT installation and update instructions](https://github.com/@REPOSITORY@/blob/@TAG@/docs/user/install/ubuntu-desktop.md).

The asset `merlin_@VERSION@_ubuntu22.04_amd64.deb` is an identical desktop
compatibility copy for automatic updates from 0.1.4 and earlier clients. It is
not another application; new manual downloads should use `merlin-desktop`.

## Standalone CLI

The CLI DEB/archive target Ubuntu 22.04, 24.04 and 26.04 x64. Desktop delivery on
24.04/26.04 remains deferred for this phase.
[CLI installation, upgrade and removal](https://github.com/@REPOSITORY@/blob/@TAG@/docs/user/install/linux.md).

Packages include public examples, documentation and full license notices.
Passing CI does not establish DOCSIS device interoperability.
