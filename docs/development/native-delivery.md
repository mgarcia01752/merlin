# Native validation and delivery

## Desktop examples (INSTALL-EXAMPLES-01)

Desktop installers must deliver `examples/workbench/manifest.json` and every
runnable bundle's exact PNM files, self-contained WB, provenance, LICENSE and
NOTICE. Keep `single`, `multi-rxmer` and `multi-rxmer-2` runnable; `multi-rxmer-dschan` is deferred
and excluded from the runnable inventory. Native Open must resolve examples from
the installed executable/resources, not the launch working directory. Test
relocation, offline WB validation, all resource hashes and installed picker
behavior. The Ubuntu 22.04 desktop builder now includes these resources and validates
all three WBs through the production Rust import/request APIs. Installed lifecycle
verification is configured on disposable runners and needs hosted evidence. See the
[design](features/workbench-examples/design.md) and [user guide](../user/workbench-examples.md).

QA-06/07 CLI tooling and the Ubuntu desktop workflow are prepared locally.
Hosted results are separate evidence; this task does not push or execute them. Local Linux results do not establish native
Windows/macOS runtime or installer support. See the [group design](features/completion-group/design.md).

The only entry points are Master (`.github/workflows/ci.yml`) and Release
(`.github/workflows/release.yml`). Master runs a Linux preflight (versions, tooling tests,
ShellCheck, formatting and Clippy), then native tests on Ubuntu 22.04,
24.04 and 26.04 x64. A separate Windows x64 desktop UI job validates the
Qt adapter and QML; tag runs additionally build/test the MSI. macOS is deferred. Matrix failures do not cancel
other platforms. Every platform uses the pinned nextest runner with four concurrent test
processes for workspace, no-default-feature and fixture-preparation tests,
retains doctests, builds the CLI and exercises version/help,
invalid input, JSON export and no-clobber behavior. Rust1.85.1 and locked
Cargo dependencies are shared with local development.

Caches are separated by platform/architecture/toolchain/Cargo.lock and preflight
versus native work. Development/test debug symbols and incremental compilation
are disabled in CI to reduce cache/disk/link work. No Qt or Python runtime is
needed by the delivered CLI; Python here only orchestrates development checks.

`.github/workflows/release.yml` starts on a version tag, verifies exact agreement
with VERSION, and calls the same preflight/native workflow. Tag jobs package
the tested Ubuntu 22.04 binary into a portable archive and DEB using dpkg-deb.
The oldest supported Ubuntu supplies the shared libc baseline; newer Ubuntu jobs
run the same tests and CLI smoke checks without publishing duplicate assets. All
native jobs must succeed before publication. Failed platforms keep the workflow
failed and do not produce a public partial release in this initial DOFT policy.
No tag is created by CI and the local Git helper retains its validation gate.

The Ubuntu 22.04 job uploads one artifact with its archive, DEB and checksums,
retained7days. Publication verifies each uploaded checksum, rejects name collisions
or undeclared files, merges assets and generates one release SHA256SUMS. Release
notes link to tagged per-OS installation instructions using the actual repository
from the workflow environment. The job uses GitHub's scoped token, not stored
credentials. Signing/notarization are not configured.

Local tools under `tools/packaging/` own staging, each OS's package builder,
collection, notes and native smoke checks. Packaging never installs a package or
replaces an output directory. VERSION and the binary's reported version must
agree. Resources are an explicit public allowlist; local specifications, private
captures, source toolchains and build caches are excluded. Full license/notice files accompany every artifact, including pinned Cargo-source
notices, the adapted BCH notice and the PyPNM QAM resource license/attribution. User installation/uninstallation is documented
under [install](../user/install/README.md).

Before closing QA-06/07, push the prepared workflows when authorized, retain the
Ubuntu run evidence and verify installation/upgrade/uninstallation on each target.
The Windows desktop MSI path passes automated native validation in v0.2.7;
Windows CLI builders remain inactive and macOS stays deferred.
A locally generated MSI XML or a passing Linux build does not satisfy that check.

Tagged publication also requires the shared CodeQL and RustSec jobs to
pass. See [security checks](security.md) for schedules, permissions and reports.


## Ubuntu 22.04 desktop DEB (UI-DEB-01)

The shared Master workflow checks the native Qt desktop on Ubuntu 22.04 amd64,
retaining pinned SDK/Cargo caches. Release enables optimized compilation and packages
with `tools/packaging/desktop.py`, checks the extracted runtime with
`desktop_smoke.py`, then installs/reinstalls/removes on a disposable runner before
uploading the artifact. The version-tag release calls shared validation and waits for every job
to succeed before collecting both CLI and desktop assets. The desktop package is
`merlin`; it does not overwrite ownership of `merlin-ofdm`.

The desktop runtime owns `/usr/lib/merlin`, its `/usr/bin/merlin` launcher and
desktop icon/entry. Full Qt/Cargo/ICU notices and public resources are included.
Private libraries use launcher-relative paths; system ELF dependencies are mapped
to actual DEB package owners on the Ubuntu 22.04 builder. Runtime startup needs
neither the build checkout nor an external Qt SDK. Installer size, installed file
bytes and declared prerequisites are reported independently.

See the [design/evidence](features/ubuntu-desktop/design.md) and
[Ubuntu desktop user guide](../user/install/ubuntu-desktop.md). Local extraction
is read-only application validation, not permission to install on the user's VM.
The notification checks only stable GitHub releases; manual CI artifacts do not
trigger it. Windows desktop delivery is described below; macOS remains deferred.


CI setup uses complete APT index refreshes and at most three package-setup
attempts, refreshing again after a failed download. Only the disposable desktop
runner prefers archive.ubuntu.com/security.ubuntu.com HTTPS in its mirror list;
local setup does not change user mirrors. Run43061645839 failed before desktop
compilation on Azure mirror DNS/connectivity and python3-pil download (APT100).
The consolidation/retry changes need new hosted evidence; the previous failure
is not reported as resolved remotely. [Design](features/workbench-configuration/design.md).

## Canonical asset filenames

All new deliverables use `merlin-COMPONENT_VERSION_TARGET_ARCH.EXT` through
`tools/packaging/asset_names.py`. Desktop Ubuntu: merlin-desktop; CLI: merlin-cli.
Use ubuntu22.04 for DEBs, linux for portable Linux archives and amd64 for x64
filenames. Installed package identities and paths remain unchanged. A byte-identical
legacy desktop filename is retained for older automatic updaters and checksummed
separately; new clients prefer canonical metadata. CI installs/tests the canonical
DEB once. Existing public releases are immutable. Prepared non-Linux builders
share naming but have no new native verification claim.

## Windows x64 desktop MSI (UI-MSI-01)

Master builds/checks the native Qt adapter and QML on windows-2022 with
Rust1.85.1 and Qt6.8.3 MSVC2022 x64. Tag runs add WiX3.14.1 packaging via
`tools/packaging/windows_desktop.py` and installed lifecycle tests. Exactly one
`merlin-desktop_VERSION_windows_amd64.msi` and SHA256SUMS enter the Windows
artifact; no Windows CLI or macOS assets are required or published. Existing
Linux and security gates remain mandatory before release collection.

The MSI includes private Qt/CRT, the shared desktop public-resource/notice
inventory, an embedded icon, Start menu entry and unchecked optional desktop
shortcut. Installed paths/component IDs and a desktop-only UpgradeCode remain
stable. Windows Installer owns upgrade, repair and removal. The application
uses native Common Item Dialogs in the isolated picker process. Windows
updates link to the full MSI download; automatic install/restart is deferred.

Native installed tests run only on disposable Windows CI: default install,
repair, removal, opted-in shortcut install and final uninstall, plus offline
version/example/GUI probes with SDK paths removed. Keep diagnostic logs and
installer/installed-byte reports separately from public release assets. Native
Windows build/MSI/install/icon/runtime/repair/shortcut/uninstall passes in v0.2.7;
interactive Windows 11 appearance and upgrade/downgrade remain pending.
See the [design](features/windows-desktop/design.md),
[build guide](windows-desktop.md) and [installation guide](../user/install/windows.md).

## Example purpose and packaging (EXAMPLES-01)

Standalone request/settings JSON and the sample message moved unchanged into
`examples/cli/`; its README explains each file and runnable use cases. UI saved
workbenches remain under `examples/workbench/`. Shared measurement collections
remain under `examples/measurements/`. Desktop payloads explicitly exclude the
CLI folder; CLI packages retain the full public inventory. The top-level
[examples index](../../examples/README.md) distinguishes requests, saved settings,
decoded data and provenance. [Design](features/example-layout/design.md).
