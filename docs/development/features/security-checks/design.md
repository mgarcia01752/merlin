# SEC-01: source and dependency checks

Add two independent, reusable Ubuntu 24.04 workflows. CodeQL owns source analysis
of Rust using build-mode none and security-extended queries. RustSec cargo-audit
owns known-vulnerability checks against Cargo.lock and a freshly fetched advisory
database. Both run on main/master pushes, pull requests, weekly and manually.
Release publication also waits for both reusable checks. Neither extends the
local pre-push gate or repeats the simulation matrix.

`scripts/security-audit.sh` downloads the official cargo-audit 0.22.2 Linux x64
release into ignored local/security-tools, verifies its pinned SHA256 before
extraction, and writes JSON to ignored local/security-audit. It installs nothing
system-wide and does not edit dependencies. Vulnerabilities, download errors and
invalid checksums fail the job. Unsoundness advisories also fail via --deny unsound; other informational
advisories retain RustSec defaults;
there are no ignored advisory IDs. Reports are retained seven days even on audit
failure. The audit token has read-only repository access and creates no issues.

CodeQL has only the additional security-events write permission needed to upload
its scan. Workflows use ordinary pull_request, never pull_request_target. README
badges link to actual repository workflows and do not claim findings are absent
before remote execution. Validate YAML, trigger/permission/publication wiring,
Bash syntax/ShellCheck and an actual local dependency audit; CodeQL itself needs
GitHub execution.

Initial local validation: workflow triggers, permissions and release dependencies pass
structural checks; Bash syntax, ShellCheck and documentation links pass. The
static musl tool works on Ubuntu 22.04 and cached archives are reverified.
RustSec reports zero vulnerability-class advisories and one unsoundness advisory,
RUSTSEC-2026-0097 in rand 0.9.2. The audit intentionally fails on that finding;
no suppression or dependency change is included. CodeQL execution is pending CI.


SECURITY-DEPS-02 extends the audit to the desktop lockfile. The original rand
finding is patched with 0.9.5; the additional cxx unsoundness finding
RUSTSEC-2026-0202 is patched with cxx 1.0.195/cxx-gen 0.7.195. Current local
core and desktop audits report zero vulnerability/unsoundness findings with
no suppressions. Both JSON reports are retained by the security workflow.
See the [desktop delivery record](../ubuntu-desktop/design.md).

## SECURITY-DEPS-03: patched URL dependency

Release run [45698604799](https://github.com/mgarcia01752/merlin/actions/runs/45698604799)
failed the Qt dependency audit on `url` 2.5.8. Pin 2.5.9 in
`apps/qt/Cargo.toml` and regenerate only its locked resolution.
[RUSTSEC-2026-0592](https://rustsec.org/advisories/RUSTSEC-2026-0592.html)
identifies 2.5.9 as patched. Preserve Rust 1.85.1, dependency notices, all audit
graphs and the existing failure policy. No advisory suppression is permitted.

Owner: [security architecture](../../../architecture.md#security-validation).
Requirement: [REQ-SEC-URL-01](../../../requirements.md#req-sec-url-01).
Task: [SECURITY-DEPS-03](../../checklist.md#task-security-deps-03).
Acceptance: core, Qt, Python and imported-decoder audits pass without vulnerability/unsoundness
findings; existing Qt update URL tests, compilation and Clippy pass on the
patched graph. Hosted acceptance remains pending until the corrected patch run.

## CI-SOURCE-01: repository-owned PNM source closure

The same release failed both wheel builds and all four Python installer jobs
before compilation. They could not authenticate to the private
`PyPNMApps/PNM-Viewer-Toolkit` repository. Its exact pinned commit exists;
successful jobs using warm source caches do not prove clean-runner access.

The user requires MERLIN to build independently of other project repositories.
Import the Apache-2.0 core, original tests/examples, logging test dependency and
licensed MAC-sanitized fixture closure under `third-party/pnm`. Preserve the
original source/resource/test bytes and per-file source hashes. The local test
workspace excludes upstream desktop applications. Existing MERLIN consumers use
local Cargo paths; imported package versions retain their upstream identity.
Only the nine licensed capture files referenced by the original fixture crate
are imported; unrelated demos and Python test-reference data are excluded.
Three readmes have local links/commands adapted, with their original bytes
preserved as hash-verified `README.upstream.txt` records. Executable source and
numerical resources/test vectors remain unchanged.

| Owner | Disposition | Contract |
| --- | --- | --- |
| `third-party/pnm` | Add | Unchanged reusable source/test closure, full notices, source inventory and standalone test workspace. |
| `crates/pnm-input/Cargo.toml`, `apps/qt/Cargo.toml` and all locks | Edit | Same decoder types and version through local paths; no Git dependency sources. |
| `tools/development/check_sources.py` | Adapt | Validate local source hashes and consumer paths without caches or network access. |
| `tools/packaging/notices.py` | Adapt | Include third-party path dependencies in every package's notice inventory. |
| Existing Rust/Python/Qt/CLI consumers | Reuse | Preserve scientific behavior, independent vectors and public contracts. |

No source credential, new authentication flow or repository visibility change is
needed. Normal locked registry packages and local toolchain downloads remain
supported. Preserve imported bytes across Windows checkout conversion. Setup,
skills and development commands must refer to MERLIN-owned sources and SDKs.

Owner: [shared CI architecture](../../../architecture.md#shared-master-and-release-validation-ci-consolidate-01).
Requirement: [REQ-CI-PNM-01](../../../requirements.md#req-ci-pnm-01).
Task: [CI-SOURCE-01](../../checklist.md#task-ci-source-01).
Acceptance: verify imported hashes, original decoder tests, local package notices
and every consumer dependency graph. Extract the complete source into a clean
directory, use an empty Cargo cache and prohibit Git network transports; fetch
and build the actual native wheel, then test its installation. All hosted
installer variants, wheel builds and publication remain required. Fixes use a new authorized patch tag,
preserving existing tags and the separate unmerged Qt branch.

## Local validation before the patch release

- All 138 selected original files are inventoried. Executable Rust, numerical
  resources and nine capture fixtures retain their original bytes; three readme
  adaptations retain independently verified original copies.
- All 102 original decoder tests pass in the standalone imported workspace.
- Source/setup tests: 10 pass. Packaging tests: 26 pass, including collection of
  license/NOTICE material for third-party path dependencies.
- Core, Python, Qt and decoder dependency graphs resolve for both Linux x64 and
  Windows MSVC x64 from extracted source. The Cargo cache was initially empty,
  Git network transports and credentials were disabled, and no Git source cache
  was created. Native wheel construction, actual installed-wheel execution and
  bundled PNM license/NOTICE/provenance checks pass on the local Linux host.
- The first metadata probe requested unfetched platforms after a target-filtered
  fetch. Selecting the matching platform corrected that validation harness; it
  was not an application source-access failure.
- All four RustSec audits pass without vulnerability/unsoundness findings or
  suppressions. The existing `humantime` maintenance warning remains visible in
  application graphs under the unchanged policy. Unrelated Qt dependency edges
  were preserved; `url` and its required `form_urlencoded` update are isolated.
- Documentation links/fragments, integration traceability, skill metadata,
  Bash/ShellCheck and generated Python-guide freshness pass. Source archives,
  detailed commands/timings and reports are retained under ignored
  `docs/development/local/release-ci-repair/`.

These local results do not establish native Windows compilation, Qt consumer
acceptance or publication. The guarded patch helper still runs the complete local
Rust/Clippy/release smoke gate on the bumped candidate. Both task boxes remain
open for actual hosted acceptance on the new patch tag. Existing task states
above retain their historical meaning. No Qt feature branch is merged here.
