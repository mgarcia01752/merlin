# Qt workbench development

The native workbench includes OCD/DPD/source editors, measurement inputs,
transmitter/channel/receiver stage navigation, SA/QAM and decoder instruments,
closed-loop Start/Stop/Pause and self-contained workbench Open/Save.
See the [UI checklist](ui-checklist.md) and
[architecture](../architecture.md) for supported behavior and remaining work.
Channel conditioning, shared TP instruments, optional electrical references and
WB persistence are implemented; detailed physical RF front-end limits remain
follow-up work. See [UI-RF-01](features/rf-power-budget/design.md).

## Source organization

```text
apps/qt/
├── qml/
│   ├── Main.qml
│   ├── shell/                 Workbench, menus and global settings
│   ├── theme/                 Theme and palette
│   ├── components/            Dialog shell, chips, connectors and tree grid
│   ├── about/                 About and licenses
│   ├── inputs/                ofdm_settings, profile_input, source_input,
│   │                          measurement_input
│   ├── workbenches/           transmitter, channel, receiver, verification
│   └── instruments/           spectrum, qam, response, slicer, fec,
│                              messages, stages
└── src/
    ├── main.rs                Startup and module wiring
    ├── bridge/                All ten QObject adapters, one QML module
    ├── inputs/
    │   ├── channel_model.rs   Additional-loss editor model
    │   ├── profile_input/     Decoder, draft/accepted state, restoration
    │   ├── source_input/      Source model and restoration
    │   └── measurement_input/ Shared capture models and restoration
    ├── workbench/             Accepted session and project JSON persistence
    ├── simulation/            Closed-loop admission and bounded worker
    ├── platform/              Native picker, icon header, files and examples
    └── instruments/
        ├── channel/           TP routing, calibrated SA and shared raw QAM
        ├── receiver/          Received-cell/response/decoder providers
        ├── transmitter/       Preview, retained stages and message providers
        └── spectrum/          Shared spectrum viewport formatting
```

Inputs use their Rust owner names; workbenches compose editors/instruments.
Instrument windows and their charts live together. Shared Qt components remain
under components; signal-processing algorithms stay in the independent core.
Each window retains its own QML file. Ordinary actions use
`components/WorkbenchActionButton.qml`, including shared dialog button-box
delegates and themed Apply/Cancel footers. `ReceiverChip` wraps that action for
instrument navigation; `StageChip` retains the decorated diagram symbol.
See the required [UI skill](../../skills/ui-design/SKILL.md). Relative directory imports make dependencies
explicit and let local QtTest fixtures run without the compiled Rust module.

Register every QML file in build.rs, preserving the Main.qml embedded entry URL.
Pinned CXX-Qt0.7.3 permits one directory of Rust QObject bridge files per QML
module, so the bridges stay together in `src/bridge/` and import grouped providers.
Do not move an individual QObject bridge into an instrument directory. The existing
root instrument aliases remain; other private modules use canonical owner paths.
Project validation calls restoration helpers under inputs, with no dependency
on QObject bridges. Native C++ headers stay in platform; build.rs includes src
and registers the native picker separately from the QML module.
See the [organization design](features/qt-source-organization/design.md).

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
cover all nine blocks, closing dialogs, run readiness/controls, theme
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
