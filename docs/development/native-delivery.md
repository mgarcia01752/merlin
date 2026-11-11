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
24.04 and 26.04 x64. Windows/macOS workflow execution is deferred. Matrix failures do not cancel
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
Windows/macOS builders and guides remain prepared for later work.
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
trigger it. Other desktop operating systems remain deferred for this phase.


CI setup uses complete APT index refreshes and at most three package-setup
attempts, refreshing again after a failed download. Only the disposable desktop
runner prefers archive.ubuntu.com/security.ubuntu.com HTTPS in its mirror list;
local setup does not change user mirrors. Run43061645839 failed before desktop
compilation on Azure mirror DNS/connectivity and python3-pil download (APT100).
The consolidation/retry changes need new hosted evidence; the previous failure
is not reported as resolved remotely. [Design](features/workbench-configuration/design.md).
