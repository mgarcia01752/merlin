# Windows install and uninstall

Choose the x64 MSI, or the portable ZIP if no system installation is wanted.
Download `SHA256SUMS` from the same release. In PowerShell, compare the reported
hash with that installer's entry:

```powershell
Get-FileHash .\merlin-<version>-windows-x64.msi -Algorithm SHA256
```

Open the MSI and approve the normal Windows administrator prompt. Reopen your
terminal after installation, then run `ofdm-fec --version`. A newer MSI upgrades
the matching installed product; there is no need to uninstall first. Older MSI
versions are rejected. Windows/macOS native execution is pending the later CI run.

The current installer is unsigned. Windows may show an unknown-publisher or
SmartScreen warning. Confirm the release source and hash, and follow your
organization's approval policy; no script-execution-policy or security-setting
change is required. There is no desktop shortcut for this command-line program.

Uninstall through **Settings → Apps → Installed apps → MERLIN OFDM CLI → Uninstall**.
The MSI removes its machine PATH entry and installed resources. User-created
simulation outputs remain yours. Portable ZIP copies are independent: extract,
run `bin\ofdm-fec.exe`, and remove that extracted directory when finished.
Examples and guides are in `share\merlin` beneath the installation directory.
