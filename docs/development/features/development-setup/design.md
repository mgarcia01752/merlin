# Development setup

## DEV-01: Explicit Ubuntu development installer

Provide `install.sh --development` for Ubuntu 22.04/24.04/26.04 x64. Install only
missing apt prerequisites, then project-local Rust 1.85.1 with rustfmt/Clippy,
Qt 6.8.3 gcc_64 through aqtinstall 3.3.0, and locked Cargo dependency sources.
Do not compile, test, change shell profiles, install MERLIN system-wide or start
services. Only apt uses sudo; run the script as the normal development user.
Reject redirected project-local tool roots before installation writes.

`--check` reports readiness and returns nonzero if incomplete. `--dry-run` prints
the plan. Both are read-only: no downloads, elevation or directory creation.
Qt SDK checks include the version, QML controls/dialogs and QML test tools.
An explicit complete QT_SDK_DIR is reusable read-only; an incomplete external
SDK override is rejected rather than installed into another checkout.

Packages cover C/C++ Qt interop tooling, Rust build prerequisites, XCB/OpenGL,
GTK native dialogs, fonts, Python development tools, headless QML tests, desktop
validation and local package-building tools. The Qt SDK, not Ubuntu's potentially
different Qt release, supplies Qt libraries and QML modules. There is no C/C++ DSP
or Python production runtime. See scripts/install-development.sh for the list.

## DEV-02: Pinned decoder source fetch

This section retains the original fetch-stage history. The current implementation
uses the ingested source under `third-party/pnm`, verified by
`tools/development/check_sources.py`; its
[source/provenance guide](../../../../third-party/pnm/README.md) supersedes the
former Git-cache prerequisite described below. No sibling checkout is required.

The Qt Cargo manifest and lockfile pin pnm 0.3.6 to upstream revision
5fbd5561f3e06b73486546b5ffa520279080a5eb. Cargo fetches that source into ignored
local/merlin-cargo/git; no PNM Viewer source is vendored in tracked MERLIN files.
Retain dependency notices under apps/qt/third-party and include applicable notices
in future distributions. The application links the Rust library and needs no
upstream checkout, decoder subprocess, service or network at runtime.

Check cached source identity against the manifest without fetching. Existing
parser evidence remains upstream; focused adapter tests compare the sanitized
binary and its existing JSON export through MERLIN's shared profile validator.
Internet access is required for initial setup; successful source fetching does
not claim support for pristine offline installation.

## Validation

Exercise safe modes and malformed options in a disposable MERLIN-local checkout
with fake host commands that cannot install/download. Check incomplete/wrong SDK
modules. Run host syntax checks and a real dry-run; do not install packages merely
to validate setup tooling. A full clean-machine provisioning run on each Ubuntu
version remains a separate evidence requirement.

Validated safe modes on Ubuntu 22.04: three isolated setup tests pass, shell
syntax/ShellCheck pass, existing SDK checks pass (including libexec/qmlcachegen),
and the real dry-run reports readiness/missing packages without installation.
All 70 declared packages are present in this host's configured Ubuntu metadata.
Pinned cached Git source identity matches the manifest. No apt install or fresh
Rust/Qt provisioning was executed. Pristine setup on all three Ubuntu versions
remains DEV-03 and is not claimed by these checks.

## DEV-01 correction: VM display and standalone launch references

The XCB development launcher must discover `DISPLAY` when absent, including
Wayland desktop sessions that already supply `WAYLAND_DISPLAY`. Preserve an
explicit display/authorization for forwarded sessions and reject an unavailable
XCB display before invoking Cargo. Explicit offscreen execution remains valid.
Owner: `scripts/qt-desktop.sh`; acceptance:
`tools/development/tests/test_qt_display.py` and the existing binary's real XCB
startup smoke. This repairs the existing setup contract rather than changing
scientific, API, persistence or UI behavior. DEV-03 provisioning remains open.

The ignored VM launcher must use MERLIN's existing binary and an owned Qt
runtime, without Cargo or a sibling repository. Remove stale dangling toolchain
references to the former viewer checkout. RAVEN is the current upstream product
name; immutable GitHub source identities, imported bytes and copyright holders
remain historical evidence. The architecture retains its former section anchor.

Validation passed: four isolated launcher regression cases, `bash -n`,
ShellCheck and requirement/architecture/feature-link checks. The real existing
Qt binary passed `--smoke-test` through the repaired VM launcher with `DISPLAY`
and `XAUTHORITY` initially absent and `WAYLAND_DISPLAY` present. The launcher
discovered the live desktop and exited zero with `MERLIN_QT_STYLE=Fusion`.
`xdpyinfo` confirmed access; `libxcb-cursor.so.0` was already installed and no
system installation was needed. The ignored launcher uses MERLIN's installed
`/usr/lib/merlin` runtime, not a reference repository SDK. Its configuration and
the removed dangling links remain VM-local changes.

No Rust/Qt source, scientific algorithms or public API changed. Full simulation,
Qt rebuild/QML regression, Python wheel and release suites were not required
or executed. This launch smoke does not establish that the prebuilt executable
was rebuilt from the current checkout or establish pristine provisioning.
