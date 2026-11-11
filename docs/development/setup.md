# Development setup on Ubuntu

Local full tests use `scripts/test-local.sh all`. First use downloads a pinned,
checksum-verified nextest binary into ignored local storage; no extra host
package or global installation is required. See [parallel validation](build.md#parallel-local-tests).

Initial targets are Ubuntu 22.04, 24.04 and 26.04 x64. The current VM is Ubuntu
22.04; clean-machine provisioning and native runtime on 24.04/26.04 remain
unverified. Run from a MERLIN checkout as your normal user:

```bash
./install.sh --development --dry-run
./install.sh --development
./install.sh --development --check
```

The install action uses sudo apt-get for missing system packages. It prepares
Rust 1.85.1 (from rust-toolchain.toml), rustfmt/Clippy, Qt 6.8.3 gcc_64 and locked
Cargo dependencies. It does not compile, test, install the application, change
shell profiles or start services. Check and dry-run never install or download.
The exact package list lives in scripts/install-development.sh.

| Development area | Included prerequisites |
| --- | --- |
| Builds and Qt bridge | GCC/C++, CMake, Ninja, pkg-config, Clang, linker tools |
| Qt rendering/dialogs | OpenGL/EGL, XCB/XKB, GTK/GLib, fonts and related headers |
| Local tooling | Python, virtual environments, PIL/GTK screenshot support, shellcheck |
| GUI tests and packaging | Xvfb, Xauth, X11 utilities, desktop-file validation, dpkg tools |
| Downloads/source | CA certificates, curl, Git and archive tools |

The Qt/XCB list follows the [Qt Linux requirements](https://doc.qt.io/qt-6.8/linux-requirements.html).
Qt archives include qtbase, qtdeclarative (Quick/Controls/Dialogs), qtshadertools,
qtsvg, qttools and ICU. The pinned aqtinstall 3.3.0 runs only in the development
virtual environment under local/qt-tools; no Python sidecar is used by MERLIN.

All downloaded SDKs/toolchains and sources live under ignored local/. The PNM
library is fetched from PNM Viewer Toolkit at the exact Git revision in
apps/qt/Cargo.toml/Cargo.lock (currently pnm 0.3.6). No viewer source copy or
runtime viewer installation is required. Cargo's source cache is a download,
not tracked project code. License/NOTICE files remain tracked for attribution.
The sanitized binary demo is an intentional capture fixture, not a source copy.

Build and run after setup:

```bash
scripts/qt-desktop.sh build
scripts/qt-desktop.sh run
```

The helper defaults to local/qt/6.8.3/gcc_64 and launches through XCB/XWayland
unless QT_QPA_PLATFORM is explicitly selected. Native Wayland-specific validation
is not claimed. To reuse an existing complete SDK,
set QT_SDK_DIR before running setup/check/build; setup only validates that override
and never installs into it. Local tool roots redirected outside MERLIN are
rejected during installation to avoid modifying another checkout.

Internet access and configured Ubuntu repositories are needed for first setup.
An existing cached build may work offline; this is not a pristine offline-install
promise. Re-running setup preserves Cargo build outputs and installs only missing
apt packages. See [Qt development](qt.md), [Rust builds](build.md) and
[setup design](features/development-setup/design.md).
