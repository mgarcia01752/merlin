# Native validation and CLI delivery

QA-06/07 tooling is prepared locally. No remote is configured in this checkout;
workflows are not pushed or executed. Local Linux results do not establish native
Windows/macOS runtime or installer support. See the [group design](features/completion-group/design.md).

`.github/workflows/ci.yml` runs a lean Linux preflight (versions, tooling tests,
ShellCheck, formatting and Clippy), then native tests on Ubuntu22.04 x64,
Windows2022 x64, macOS15 ARM64 and macOS15 Intel. Matrix failures do not cancel
other platforms. Every platform runs all workspace tests, the no-default-feature
boundary and fixture-preparation tests, builds the CLI and exercises version/help,
invalid input, JSON export and no-clobber behavior. Rust1.85.1 and locked
Cargo dependencies are shared with local development.

Caches are separated by platform/architecture/toolchain/Cargo.lock and preflight
versus native work. Development/test debug symbols and incremental compilation
are disabled in CI to reduce cache/disk/link work. No Qt or Python runtime is
needed by the delivered CLI; Python here only orchestrates development checks.

`.github/workflows/release.yml` starts on a version tag, verifies exact agreement
with VERSION, and calls the same preflight/native workflow. Tag jobs package
already tested binaries into portable archives and native DEB/PKG/MSI installers.
Linux uses dpkg-deb; macOS uses Apple's pkgbuild; Windows uses WiX3.14.1. All
native jobs must succeed before publication. Failed platforms keep the workflow
failed and do not produce a public partial release in this initial DOFT policy.
No tag is created by CI and the local Git helper retains its validation gate.

Each platform uploads one artifact with its archive, installer and checksums,
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
native run evidence and verify installation/upgrade/uninstallation on each target.
A locally generated MSI XML or a passing Linux build does not satisfy that check.
