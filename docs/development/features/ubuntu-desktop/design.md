# Ubuntu desktop delivery and update notification

Local release iteration: [DEV-04 explicit test omission](local-validation.md).

## UI-DEB-01: Ubuntu 22.04 amd64 DEB

Build the native Qt desktop on Ubuntu 22.04 with the pinned Rust/Qt SDKs.
Provide a manually runnable GitHub workflow and reusable tag-release job.
Package the executable, private relocatable Qt libraries/plugins/QML, desktop
entry, rounded icon, public user guides, all workbench examples and full notices.
Use a distinct `merlin` desktop package alongside the existing `merlin-ofdm` CLI.
Produce a DEB and SHA256SUMS; validate resource hashes, dependency resolution,
offline startup and workbench restore. Disposable workflow runners install,
upgrade/reinstall and remove the package; local preparation never installs it.
Only Ubuntu 22.04 x64 desktop delivery is in scope for this phase. Other desktop
platforms remain deferred; existing CLI validation is separate.

## UI-UPDATE-01: Stable release notifications

Follow OSCAR-UI ReleaseUpdateService/UpgradeChip behavior: one quiet, bounded
background startup query, a green Update indicator only for a newer stable
release with a matching Ubuntu 22.04 amd64 desktop DEB, and explicit retry in
About. No automatic download or installation. The dialog opens the validated
project release page in the OS browser; the user installs its DEB normally.
Rust owns HTTP, version comparison and metadata validation; QML only presents
bounded status. Reject drafts/prereleases, foreign URLs, mismatched filenames,
missing SHA-256 digests and unreasonable asset sizes. Network failures must not
interrupt editing/simulation. Offline launch and simulation remain available.
The preference to check at startup is application-global, separate from WB JSON;
notification status itself is transient. No capture/device network traffic.

Owning modules: `platform/updates` plus one QObject under `src/bridge`, separate
QML notification/dialog components, and `tools/packaging` desktop helpers.
Tests cover version/security/error cases without live HTTP, async lifecycle,
QML states, resource integrity and DEB layout. Keep installed-run/hosted evidence
explicitly pending until measured. Initial GitHub artifact builds do not constitute
a stable published release; notification requires a published versioned release.


## SECURITY-DEPS-02: Release dependency audit

The baseline audit confirms RUSTSEC-2026-0097 on rand 0.9.2. The advisory's
patched 0.9 series starts at 0.9.3. Pin 0.9.5 in the owning simulation crate and
both lockfiles, preserve the same public simulation/RNG APIs, and validate
seeded channel/source behavior. Audit both core and Qt graphs with `--deny unsound`
and retain both reports; never ignore the advisory to make packaging pass.

Reference inspections (read-only): OSCAR-UI commit
`778575183a385197d315e7bd4cce83254c1ce843`, ReleaseUpdateService, UpgradeChip,
and UbuntuUpdateService; PNM Viewer Toolkit commit
`5fbd5561f3e06b73486546b5ffa520279080a5eb`, scripts/release/linux.py and licenses.py.
The Qt deployment/notice tooling is adapted Apache-2.0 code, retaining its
copyrights. Notification UI/backend code is MERLIN GPL-3.0-only. HTTP lookup uses
pinned ureq 2.12.1 with TLS; dependency resolution retains Rust 1.85 compatibility.

Auditing the previously unaudited desktop graph also identifies
RUSTSEC-2026-0202 in cxx 1.0.158 (exception safety in let_cxx_string!). Pin
cxx 1.0.195 and its matching cxx-gen 0.7.195, preserving CXX-Qt 0.7.3 and
Rust 1.85.1, then rebuild/validate the adapter. This is a patched dependency,
not an ignored advisory or a toolchain upgrade.


## Validation and remaining acceptance

### UI-DEB-03: Fresh-runner bootstrap recovery

The first v0.1.0 hosted run (42955022923) passed version and RustSec checks
but failed before native validation: packaging tests assumed ignored local
storage already existed, and the desktop workflow supplied an explicit SDK path
before provisioning it. Create test scratch storage explicitly and unset the
SDK override only during provisioning, retaining the pinned path for later
checks/builds. Preserve the SDK helper's rejection of incomplete external SDKs.
Validate packaging tests in an isolated fresh tracked-file checkout and check
workflow setup ordering without installing packages or downloading SDKs.
Hosted recovery and installed lifecycle acceptance remain pending a new tag;
do not move the existing v0.1.0 tag.

Local recovery checks pass: all nine packaging tests in both the working tree
and an isolated tracked-file checkout with no preexisting ignored local storage;
all three development setup tests and diff whitespace checks. No compilation
or system installation was needed. The workflow provisioning command clears
only the SDK override for that process; subsequent steps retain the job's path.

UI-DEB-01 and UI-UPDATE-01 are implemented. Native Ubuntu 22.04 / Qt 6.8.3
renders the actual update dialog correctly, including the live no-release-yet
response, startup checkbox, retry, X and Close. The guide includes the actual
dark VM capture; successful future-release notification and URL handoff are
covered by deterministic QML/metadata tests rather than invented live release data.

All 26 adapter tests and 32 QML checks pass, including three new Rust update
checks and the update dialog interaction. All 19 scoped simulation tests pass
following the rand patch. Both RustSec audits report zero findings/unsoundness
warnings without ignores. Formatting, warnings-denied Qt Clippy, package-tool
checks, skill validation/discovery, YAML parsing and documentation links pass.
Cargo license preflight retains 157 dependency notices, including reviewed
workspace license texts matched to exact Cargo VCS commits and REUSE attribution.
Qt source notices and the separate ICU 73.2 license are included. Private staged
ELFs are stripped; SDK/build sources remain unchanged.

The local DEB is built from the current development executable for package review;
the workflow builds an optimized release executable. Extracted-package checks
verify version, desktop entry/icon, every bundled capture hash, all three WB
imports/full requests, private ELF resolution and offline QML startup with no
external SDK paths. This does not establish a hosted build or system installation.
The workflow performs install/reinstall/remove only on its disposable runner.
UI-DEB-02 and installed-OS INSTALL-EXAMPLES-01 acceptance remain pending hosted
and interactive installed evidence. Nothing was committed, pushed, published
or installed system-wide by this work. Local logs/artifacts are ignored under
`docs/development/local/ubuntu-desktop/`.


Final local review artifact: `ready/merlin_0.1.0_ubuntu22.04_amd64.deb` under
that ignored task directory, 61,168,248 bytes. The declared staged file footprint
is 165,202,944 bytes; Ubuntu prerequisites are external and listed in its control
metadata/build log. SHA256SUMS matches. The final stripped package passes all
three original/WB/full-request checks, private Qt/ICU resolution and offline UI
startup. The native executable is built and current. Hosted and installed-VM
acceptance remain explicitly unchecked.
