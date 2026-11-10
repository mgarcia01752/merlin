# Qt workbench development

UI-01 currently provides a native Qt Quick/QML shell with the clickable system
diagram and section-preview dialogs. The OCD timing editor, DPD capture importer and PRBS-TX source editor are implemented, together with a reusable Spectrum
Analyzer at actual generated TX output; other editors,
analyzers and run integration remain pending. Start/Stop/Pause are visible and disabled.
See the [UI checklist](ui-checklist.md) and
[design/provenance](features/visual-workspace/design.md).

## Build and launch in the VM

Use Qt 6.8.3 gcc_64 and the pinned Rust 1.85.1 toolchain. The desktop adapter has
its own Cargo workspace/lockfile under apps/qt; CLI builds remain Qt-independent.
CXX-Qt 0.7.3 and CXX 1.0.158 are pinned. The Qt adapter now enables the existing
Rust simulation feature for TX sample generation; source-only configuration users
can still use `source-input` without FEC/runner dependencies. Shared dependency
versions in the Qt lock follow the reviewed root lock, without broad upgrades. A C++ compiler is needed for Qt interop
only; no DSP/FEC algorithms use C++. Use [development setup](setup.md): `./install.sh --development` installs missing
Ubuntu prerequisites and prepares the project-local pinned Rust/Qt toolchain.
Its check/dry-run modes do not install packages.

Set QT_SDK_DIR to an existing SDK or place it under local/qt/6.8.3/gcc_64.
For the current development VM, the existing reference SDK can be read without
modifying the reference checkout:

```bash
cd /home/dev01/Projects/merlin
export QT_SDK_DIR=/home/dev01/Projects/PNM-Viewer-Toolkit/local/qt/6.8.3/gcc_64
scripts/qt-desktop.sh build
scripts/qt-desktop.sh run
```

This is a development SDK selection, not a production dependency on the viewer.
The helper uses MERLIN's local/merlin-cargo cache and apps/qt/target outputs;
it does not write into the reference repository. It uses an available Cargo or
the existing project-local toolchain. Missing SDK/toolchain prerequisites stop
with a diagnostic. The run helper can read DISPLAY/XAUTHORITY from the user's
active VM desktop session, without changing the session configuration.

The VM is the interactive rendering/review target. Build success and offscreen
rendering alone do not establish interactive approval or support on other OSes.
Initial targets remain Ubuntu 22.04, 24.04 and 26.04 x64.

## Fast UI iteration

Keep `apps/qt/target/` and the development Cargo cache between iterations.
The build helper uses stable profile settings; Cargo reuses unchanged compiled
dependencies and rebuilds/relinks affected code. Embedded QML changes still need
an application build before the VM executable reflects them. Leave that executable
ready to run after each UI change. Do not routinely clean targets or change profile
settings, which would discard useful build reuse.

UI-only work uses Qt-focused build/tests. Non-Qt workspace compilation and tests
are needed only when changes affect shared signal-processing code, dependency contracts
or integration behavior. Required push/release checks remain unchanged.

## Validation

```bash
scripts/qt-desktop.sh check
```

Checks Rust formatting, warnings-denied Clippy, adapter tests/build and QML
navigation/theme/render tests using Qt's offscreen software renderer. QML tests
cover all nine blocks, closing dialogs, disabled lifecycle controls, theme
switching and minimum-window navigation. Rendering outputs stay in ignored
`docs/development/local/visual-workspace/`; this directory must exist before the
render tests. The helper creates it. Numerical tests remain the commands in
[build.md](build.md). No UI test fabricates simulation results.

## Desktop branding

The square dark/light MERLIN icons are embedded in the executable for side
branding and the window icon. Theme changes update both. The approved light
icon was regenerated from the dark design; the dark asset is unchanged.

Prepare a checkout-local launcher with the same dark icon:

```bash
python3 tools/branding-workbench-desktop.py
```

It writes `local/workbench/merlin.desktop` and `launch.sh`, using QT_SDK_DIR.
Launch the script from the VM terminal or open the desktop entry where supported.
This does not install an application-menu entry, modify the VM desktop, or create
an installer. GNOME trust/launcher registration and native packaging remain
separate work; no OS protections are bypassed.

The Qt application is GPL-3.0-only; see apps/qt/LICENSE and apps/qt/NOTICE.
Signal-processing libraries and independent development tooling retain Apache-2.0.
