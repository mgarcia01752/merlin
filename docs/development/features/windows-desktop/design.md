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

## UI-MSI-07: cache a valid Installed apps icon

Run 43476334867 creates and installs the MSI but lacks `DisplayIcon` in the
uninstall registration. The authored Icon table identifier `ApplicationIcon`
lacks a filename extension. Follow the
[WiX icon guidance](https://docs.firegiant.com/wix3/tutorial/com-expression-syntax-miscellanea/add-or-remove-programs-entries/)
and give the `.ico` resource an `.ico` identifier, consistently referenced by
ARPPRODUCTICON and both shortcuts. Preserve product/component identity.
Retain the icon assertion, handle missing registry properties explicitly under
PowerShell strict mode and record registration fields with installer evidence.
Validate icon references in focused MSI authoring tests; native installed
registration/repair/removal remains a hosted acceptance requirement.

Local verification: all five focused Windows packaging tests pass, including
consistent `.ico` references. Native icon registration remains pending CI.

## UI-MSI-08: verify the registered MSI product icon

Run 43480254153 installs successfully and its log confirms ProductIcon and
IconCreate for ApplicationIcon.ico; the uninstall registry lacks DisplayIcon.
Do not treat an optional uninstall value as the MSI icon contract. Query the
registered product code via Windows Installer ProductInfo(ProductIcon), require
a cached icon file and compare its SHA-256 with the shipped branded ICO. Retain
registration uniqueness, uninstall metadata, runtime/repair/shortcut/removal
checks and record product/icon evidence. Querying metadata does not install or
repair products. Native verification occurs only on disposable CI.

References: [ProductInfo](https://learn.microsoft.com/en-us/windows/win32/msi/installer-productinfo),
[MsiGetProductInfo](https://learn.microsoft.com/en-us/windows/win32/api/msi/nf-msi-msigetproductinfow)
and [uninstall registry mapping](https://learn.microsoft.com/en-us/windows/win32/msi/uninstall-registry-key).

## UI-MSI-02 and UI-MSI-09: native result and remaining review

[v0.2.7 run 43482850185](https://github.com/mgarcia01752/merlin/actions/runs/43482850185)
completes native build/MSI/default install/icon/runtime/repair/shortcut/uninstall
checks and publishes the full stable release. This closes automated UI-MSI-02.
UI-MSI-09 retains interactive Windows 11, upgrade/downgrade and security-prompt
review. See the validation record for test counts, bytes and verification limits.

## UI-WIN-THEME-01: select palette-aware controls through Qt

The installed Windows screenshot shows native white ComboBox/SpinBox surfaces
with dark-theme foreground text. Rust startup currently sets a process environment
variable; hosted launchers already set Fusion before process launch, masking the
installed-path discrepancy. Configure Fusion through QQuickStyle before creating
QGuiApplication/loading controls. Keep the Qt interop in the GPL platform adapter.
Require the existing packaged startup smoke to run without inherited style hints
and confirm the selected style; retain all QML startup error checks. Verify
readability of enabled/disabled dropdowns and spin editors in both themes with
focused QML checks. Native Windows visual confirmation remains required.
Reference: [QQuickStyle](https://doc.qt.io/qt-6.8/qquickstyle.html).

Validation: five packaging smoke tests and both QML shading/contrast checks
pass. Native Linux Qt build and startup smoke with absent and conflicting style
environment hints report Fusion. Formatting and scoped Clippy pass. Windows
interactive visual verification remains pending; the full suite was not run.

## UI-WIN-UPDATE-01: Install a verified MSI from Update

The user's request supersedes the earlier Windows in-app installation deferral.
Update saves private accepted WB recovery, downloads the exact newer stable
Windows MSI under LOCALAPPDATA with bounded HTTPS redirects/size/SHA-256 checks,
and prepares a normal-user PowerShell handoff outside Program Files. Its
preflight checks registered MSI ownership, package UpgradeCode/name/version/x64
identity and hash before signaling ready. No source checkout may install.
Only after ready does the app commit the handoff and quit. The helper waits for
that process to exit, requests ordinary UAC for msiexec only, preserves install
location/desktop shortcut preference, disables OS auto-reboot and records the
exit code/log. Success or cancellation/failure reopens MERLIN unelevated with
WB recovery and a bounded result notice. Restart-required results stay explicit.
Never use execution-policy bypass, unblock files, suppress OS security or launch
the application elevated. Failure before handoff keeps the original app open.

Rust owns download, staging, recovery and state. A fixed embedded PowerShell
script owns Windows Installer COM inspection, parent-exit coordination, UAC and
normal-user relaunch. Inputs are JSON data, never interpolated command source.
Keep Ubuntu installation behavior unchanged. Add focused metadata/download,
handoff/status and QML tests plus native PowerShell contract checks. Local Linux
validation cannot establish Windows UAC/upgrade/rollback/runtime behavior; record
those as pending until actual native evidence exists.

Sources: [Start-Process](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/start-process?view=powershell-5.1),
[msiexec options](https://learn.microsoft.com/en-us/windows/win32/msi/standard-installer-command-line-options),
[MSI return codes](https://learn.microsoft.com/en-us/windows/win32/msi/error-codes),
[registered component paths](https://learn.microsoft.com/en-us/windows/win32/msi/installer-componentpath).
Automation uses explicit property/method dispatch, matching the existing hosted
MSI icon validation helper.

Local validation: 12 focused updater Rust tests pass, including shared download
limits/hash checks, exact Windows MSI metadata, bounded/BOM result parsing,
source-run rejection, existing Ubuntu package identity and state failures. Four
QML update tests pass (six including setup/cleanup): direct click, Windows WB
recovery, run/recovery guards and preferences. Scoped Qt Clippy with warnings
denied, formatting and native Linux build pass. The coding-style skill and both
discovery links validate. The full test suite was not run.

## UI-WIN-UPDATE-02: Native upgrade lifecycle verification

Pending: run the prepared native PowerShell contract checks through Windows CI
and test the installed upgrade/handoff on Windows. Explicitly exercise UAC
approval/cancel, MSI failure and rollback, 3010 without forced reboot, normal-user
relaunch, WB recovery, custom install location and desktop shortcut preservation.
PowerShell is unavailable on this local Linux host, so these checks were not run
locally. Earlier v0.2.7/v0.2.8 packaging results do not verify this new updater.

## WIN-UPDATE-03: native paths and user-reviewed error reports

A field MSI log reports error 1324 for `merlin/updates`: Rust joined a compound
path containing a forward slash and MSI rejected it after the COM preflight.
Build paths component by component and normalize every installer argument with
Windows GetFullPath before launching msiexec. Keep hash, identity and UAC checks.
Add native contract coverage for mixed separators and spaces.

On update errors, offer Report Issue. Rust reads a bounded tail of the installer
log (UTF-8/UTF-16), redacts the current user-profile path, and constructs a report.
QML copies it only on the user's click and opens a fixed GitHub new-issue form.
The user pastes, reviews and submits; no automatic posting or log-bearing URL.
Missing logs produce a usable status-only report. Report text explains that the
user must review for private information. Keep reports out of saved workbenches.
Test failed/success state visibility, log bounds/encoding/redaction, and browser
failure. Native Windows install verification remains required separately.

The focused QML suite passed all 7 cases, including clipboard paste-back and
browser failure. Dark/light dialog captures were reviewed and added to the
Windows install guide. The UI skill and both discovery links validated. Rust
Qt-adapter tests passed all 85 cases in 47.40 s; the full QML suite passed
83 cases in 28.12 s. Formatting, Clippy and the desktop build passed. No Windows PowerShell runtime
is available on this host; its new native path contract remains unexecuted here.
