# Windows desktop MSI

## UI-MSI-01: native Windows desktop delivery

Add Windows x64 desktop UI delivery alongside Ubuntu, using OSCAR-UI's WiX3
packaging, runtime relocation and disposable-runner lifecycle checks as guidance.
The user explicitly excludes Windows CLI delivery and defers macOS. No commit,
push, publication, external signing or local system installation is authorized.

The native Qt adapter gains conditional GTK linking, Windows Common Item
Dialogs, executable icon/version resources and portable configuration storage.
Linux's authenticated APT updater stays Linux-only; Windows updates use a full
MSI downloaded from Releases. Scientific code and input contracts are unchanged.

`tools/packaging/windows_desktop.py` owns fresh staging, Qt deployment, complete
public examples/docs/notices, stable WiX identities, shortcuts and MSI authoring.
Install to Program Files, add a Start menu entry and an unchecked optional desktop
shortcut. No application launch is elevated. Major upgrades replace older MSI
versions and reject downgrades; repair/removal are owned by Windows Installer.
Payload path hashes determine stable component IDs/GUIDs independently of file
ordering. Files are immutable inputs and output collisions are rejected.

Generate the multi-resolution ICO, 493×312 dialog and 493×58 banner reproducibly
from the existing MERLIN icon and navy/teal palette. No new logo or image model is
needed. Resource compilation embeds the same icon into the Windows executable.

Master and Release remain the only workflow entry points. A native windows-2022
job uses Rust1.85.1, Qt6.8.3 MSVC2022 x64 and WiX3.14.1. Master validates the
adapter/QML/build. Tag validation additionally stages/deploys/packages and verifies
install, GUI launch, repair and uninstall with developer paths removed, preserving
user files. Publication waits for Windows and all existing Ubuntu/security jobs.
No Windows CLI archive/MSI or macOS artifact enters the required inventory.

Local tests cover WiX identity/upgrade/shortcut authoring, input safety, icon/BMP
formats, staging contracts and release collection. Linux Qt checks guard existing
behavior. Windows compilation, interactive picker/installer appearance, signing,
and installed lifecycle remain unverified until native evidence exists.

References: OSCAR-UI `scripts/build/windows-installer.py` and
`scripts/windows/windows-release.ps1` (read-only workspace references, absent
from a fresh MERLIN clone),
[Qt deployment](https://doc.qt.io/qt-6.8/windows-deployment.html),
[Common Item Dialog](https://learn.microsoft.com/en-us/windows/win32/shell/common-file-dialog),
[WiX3 UI](https://docs.firegiant.com/wix3/wixui/dialog_reference/wixui_installdir/).

Implementation and local checks are recorded in [validation](validation.md).
Native Windows execution remains a separate unchecked acceptance item.

EXAMPLES-01 narrows desktop example staging to `workbench/` and `measurements/`.
CLI requests move to `examples/cli/` and remain only in source/CLI distributions.
See [example layout](../example-layout/design.md) for the README and validation.
