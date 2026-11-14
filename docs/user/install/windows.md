# Windows desktop installation

[Installation index](README.md) · [Workbench guide](../workbench.md)

Choose `merlin-desktop_VERSION_windows_amd64.msi` from the
[MERLIN Releases](https://github.com/mgarcia01752/merlin/releases) assets.
Replace VERSION with the release number. This is the Windows x64 desktop UI;
Windows CLI packages and macOS delivery are outside the current release scope.
The MSI includes Qt, the Visual C++ runtime, public workbench examples and guides.
Rust, Python, Qt and a source checkout are not needed to run it.

[v0.2.7](https://github.com/mgarcia01752/merlin/releases/tag/v0.2.7) passes native
Windows CI build, MSI installation, icon/runtime, repair, shortcut and uninstall
checks. Interactive Windows 11 appearance/security-prompt review and cross-version
upgrade/downgrade checks remain pending. The MSI is unsigned.

## Install and launch

1. Download the MSI and `SHA256SUMS` from the same release.
2. In PowerShell, compare `Get-FileHash .\PACKAGE.msi -Algorithm SHA256` with
   that exact filename's entry in `SHA256SUMS`.
3. Close MERLIN, double-click the MSI and follow the prompts. Choose the install
   folder and optionally **Create a desktop shortcut** (unchecked initially).
4. Approve the normal Windows administrator prompt for installation.
5. Open **Start → MERLIN**. The application runs as your normal desktop user.

The default folder is `C:\Program Files\MERLIN`. Examples live under
`files\examples\workbench`; Open starts there regardless of the launch folder.
Store your own workbenches and exports in a writable folder such as Documents.
Full notices are under `notices` in the installation directory.

## If the MSI does not open

The installer is currently unsigned. Verify its origin and checksum. Keep Windows
security protections enabled and follow your organization's approval process.
Do not disable SmartScreen/Defender, change execution policy, remove download
security markings or run the application as administrator.

PowerShell can start Windows Installer with a diagnostic log:

```powershell
$Msi = (Resolve-Path "$env:USERPROFILE\Downloads\PACKAGE.msi").Path
Start-Process msiexec.exe -ArgumentList @('/i', "`"$Msi`"", '/L*V', "`"$env:TEMP\merlin-install.log`"") -Wait
notepad "$env:TEMP\merlin-install.log"
```

Replace PACKAGE.msi with the exact downloaded filename. This command retains
Windows approval prompts. Review logs for personal information before sharing;
include the failure and nearby `Return value 3` lines in an issue report.

## Upgrade, repair and uninstall

**About → Check For Updates** checks for a newer stable desktop release.
Press the green **Update** chip or **Update** in that dialog. MERLIN saves the
accepted workbench for recovery, downloads the exact MSI and verifies its size,
SHA-256, product identity and version. It closes only after the installer helper
is ready. Approve the normal Windows administrator prompt; MSI displays progress
and MERLIN reopens as your normal user with the saved workbench. Your install
folder and desktop-shortcut choice are retained. Windows is never automatically
rebooted; if installation needs a Windows restart, the result says so.

A canceled administrator prompt or failed MSI reopens the previous application
when it remains available, with the recovery workbench and an error message.
Failures before handoff leave MERLIN open. Recovery contains accepted settings
and inputs, not unsaved dialog drafts or simulation results. Updates are blocked
while a simulation or WB load is active. Source/extracted copies cannot install.

Older versions whose Update opens the release page need one manual MSI upgrade
to a version containing this updater. The new handoff's Windows UAC, cancellation,
upgrade and recovery lifecycle still require native verification.

The installer log is `%LOCALAPPDATA%\merlin\updates\last-install.log`.
The next launch consumes a bounded result from `last-result.json` in that folder.
Recovery stays at `%LOCALAPPDATA%\merlin\update-recovery\workbench.merlin.json`.
If MSI rollback leaves no runnable application, repair it with the full installer;
the recovery file remains available. Keep Windows security protections enabled.

To repair, reopen the same-version MSI and choose **Repair**. To remove MERLIN,
use **Settings → Apps → Installed apps → MERLIN → Uninstall**. MSI removes its
runtime, bundled examples and shortcuts. User files outside the installation
folder remain; do not put original captures or workbenches in Program Files.

The release lists compressed installer bytes. Packaging reports installed file
bytes separately. Windows system libraries remain external prerequisites;
package size and installed behavior need measurements from the native run.
