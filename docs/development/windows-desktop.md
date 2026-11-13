# Windows desktop build and MSI

[Native delivery](native-delivery.md) · [Design](features/windows-desktop/design.md)

Use native Windows x64 and PowerShell 7 in a Visual Studio 2022 x64 Native Tools
environment. Install the C++ desktop workload/Windows SDK (cl.exe, link.exe and
rc.exe), Rust1.85.1 with rustfmt/Clippy, Python3.12, CMake and Ninja. WiX3.14.1
is needed only for the MSI. These are development prerequisites, not installed
application dependencies. No execution-policy bypass is needed or supported.

```powershell
rustup toolchain install 1.85.1 --profile minimal --component rustfmt --component clippy
./scripts/windows/qt-desktop.ps1 setup
./scripts/windows/qt-desktop.ps1 check
```

Setup explicitly downloads the pinned Qt6.8.3 MSVC2022 x64 SDK through
`aqtinstall==3.3.0` into checkout-local `local/`. Build/check reuse existing tools.
A complete explicit `QT_SDK_DIR` remains read-only. SDKs/caches are not release
payload. The native job uses windows-2022 and caches by platform/toolchain/locks.

With WiX3.14.1 installed and `WIX_BIN` pointing to its bin directory:

```powershell
./scripts/windows/qt-desktop.ps1 package
```

This compiles release `merlin-qt.exe`, embeds the MERLIN icon/version resource,
uses windeployqt to resolve QML/native plugins, copies app-local redistributable
CRT files and stages the shared examples/docs/notices. The relocated GUI/examples
are exercised with SDK paths removed. No Windows CLI is built or packaged.
Desktop staging excludes standalone requests in `examples/cli/`; the
[example index](../../examples/README.md) explains the retained UI/shared data.
The MSI uses the normal installation UI, selectable folder, unchecked desktop
shortcut and Start menu entry. It never launches the app elevated.

Fresh output `dist/windows-desktop/` contains only the canonical MSI and
SHA256SUMS. Existing output/stage/report directories are rejected. Retain the
sibling `windows-desktop-stage`, `windows-desktop-wix` and
`windows-desktop-report.json` as local evidence. The report separates installer
bytes, installed file bytes, prerequisites and unsigned status. No Windows size
measurement is claimed before native packaging. No package is installed here.

Regenerate the tracked ICO/BMP artwork with `python tools/branding-windows.py`
using Pillow (development only; Ubuntu development setup already provides it).
`packaging/windows/provenance.json` records source/output hashes. WiX's adapted
installer UI retains its upstream notice, reciprocal license and source provenance.
Original input artwork remains untouched; no macOS assets are generated.

On disposable GitHub Windows runners only, `scripts/windows/test-installed.ps1`
checks default install, repair, removal, opted-in shortcut install, offline GUI/
examples and final removal while preserving a user WB. It retains MSI logs.
Do not run machine-changing tests on a normal development workstation.

Local Linux Python/Qt checks validate source contracts and Linux regression
behavior. Windows compilation/runtime, Common Item Dialog interaction, the MSI
wizard appearance, SmartScreen behavior and upgrade/downgrade lifecycle still
need native evidence. Signing, automatic Windows updates/restart and macOS are
out of scope. Master/Release retain their existing Ubuntu/security gates and
publish only after every required job succeeds; no push is implied by this work.
