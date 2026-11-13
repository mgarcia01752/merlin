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

**About → Check For Updates** identifies the verified Windows desktop MSI.
**Download MSI** opens the release page. Download the newer MSI, close MERLIN
and run it; Windows Installer replaces the older desktop installation and rejects
downgrades. Windows automatic download/install/restart is deferred.

To repair, reopen the same-version MSI and choose **Repair**. To remove MERLIN,
use **Settings → Apps → Installed apps → MERLIN → Uninstall**. MSI removes its
runtime, bundled examples and shortcuts. User files outside the installation
folder remain; do not put original captures or workbenches in Program Files.

The release lists compressed installer bytes. Packaging reports installed file
bytes separately. Windows system libraries remain external prerequisites;
package size and installed behavior need measurements from the native run.
