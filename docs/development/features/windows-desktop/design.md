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

## UI-MSI-03: portable offline FAQ lookup

Release run 43365042088 passed Ubuntu/security checks but failed two Windows
adapter FAQ tests before MSI packaging. Heading matching in `apps/qt/src/help.rs`
assumed LF, while Windows embeds CRLF Markdown from checkout. Normalize CRLF
inside the bounded FAQ renderer before matching headings; preserve the catalog,
unknown-code handling and offline-only behavior. Test every catalog section with
both line endings, including final-section boundaries and the full index.
Run only focused FAQ tests locally, as requested; Windows packaging/lifecycle
remains pending a new hosted run. Hosted validation gates remain unchanged.

## UI-MSI-04: retain Windows QML runner diagnostics

Run 43370737351 passes all 72 Windows adapter tests, including the FAQ repair,
but the QML runner exits without recorded test output. The root cause is not
yet established. Explicitly wait for the GUI executable, capture stdout/stderr,
and request Qt Test text/JUnit files. Print those logs and decimal/hex exit
status before failing. Upload diagnostics on both Master and Release failures;
keep MSI evidence conditional on release. Do not skip tests or alter assertions.
Only focused local checks are requested; native runner behavior requires CI.
Qt documents multiple output loggers in the
[Qt Test options](https://doc.qt.io/qt-6.8/qtest-overview.html#options).

## UI-MSI-05: portable About layout and screenshot output

Run 43373385198 identifies two Windows QML failures: action widths smaller than
text plus padding, and screenshot saving to an invalid `/D:/` path. Size About
actions equally from the largest rounded text-based implicit width; stack the
actions when the row cannot fit. Preserve labels, keyboard behavior and theme.
Run screenshot capture from a prepared output directory on both platforms and
save relative filenames instead of stripping file URL prefixes. Keep original
layout/render assertions and verify larger fonts with focused QML tests only.
The adapter/scientific core and hosted validation gates remain unchanged.

Validation: both targeted QML functions pass locally, including 24-pixel fonts
in both themes and actual PNG output. The dark About capture is inspected and
linked from the updated user guide. Shell syntax/ShellCheck, native Qt build and
offscreen startup smoke pass. Full suites were not rerun; Windows CI is pending.

## UI-MSI-06: preserve reviewed notice bytes on checkout

Run 43470708630 passes Windows adapter/QML validation and optimized compilation,
but notice collection rejects a reviewed license checksum before MSI authoring.
Git can convert tracked license LF bytes to CRLF on Windows. Pin packaging
resource bytes with a scoped `-text` attribute; keep original provenance digests
and exact-byte verification. Include source path and both hashes in errors.
Reproduce with a disposable Git index under `core.autocrlf=true`, verify the
complete notice collector succeeds with the attributes and rejects tampering.
No full simulation/Qt suite or native packaging run is required locally.

Local validation: both focused notice tests pass, including the reproduced
unpinned checkout failure, exact pinned copied bytes and altered-byte rejection.
Original reviewed resource bytes/digests are unchanged. Native MSI remains pending.
