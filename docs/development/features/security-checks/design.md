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
