# Ubuntu install and uninstall

Use the DEB on Ubuntu 22.04, 24.04 or 26.04 x64. The release Linux build runs on22.04,
so its libc baseline is not raised by building on a newer Ubuntu release.
Download the matching `.deb` and `SHA256SUMS` from the release assets.

```bash
sha256sum --ignore-missing --check SHA256SUMS
sudo apt install ./PACKAGE.deb
ofdm-fec --version
```

Replace `PACKAGE.deb` with the downloaded CLI DEB filename
(`merlin-cli_VERSION_ubuntu22.04_amd64.deb` for new releases). Keep `./` so APT
uses the local file. Earlier releases used `merlin-ofdm_VERSION_amd64.deb`;
use the actual filename rather than copying an old version. Verify checksums
before installing; checksums detect changed downloads, not publisher identity.
Upgrade with the same `apt install` command for a newer package. Uninstall:

```bash
sudo apt remove merlin-ofdm
```

The command is `/usr/bin/ofdm-fec`; guides/examples/licenses are under
`/usr/share/merlin`. The package does not modify user data. A portable `.tar.gz`
can instead be extracted into a user-owned directory; run its `bin/ofdm-fec`
and remove the extracted directory to uninstall that copy.
