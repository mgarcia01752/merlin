# Security checks

[CodeQL](../../.github/workflows/codeql.yml) scans Rust source with the
security-extended queries and build-mode none. It uploads findings to GitHub's
Security / Code scanning view without repeating the full simulation build.
[Dependency security](../../.github/workflows/dependency-security.yml) checks
Cargo.lock against the current RustSec advisory database with cargo-audit 0.22.2.

Both workflows run on main/master pushes, pull requests, weekly and manually.
Release publication waits for both checks as well as native validation. Findings
and workflow status are separate: a successful CodeQL run may still report alerts
that must be reviewed in GitHub Security. RustSec vulnerabilities fail the audit;
unsoundness advisories also fail via `--deny unsound`. Other informational
advisories retain cargo-audit defaults. No advisory IDs are ignored.
The report artifact is retained seven days. These checks do not replace tests or
prove that the code has no vulnerabilities.

To audit locally on Ubuntu x64:

```bash
scripts/security-audit.sh
```

The script needs Bash, curl, tar, sha256sum and network access. It verifies the
pinned SHA256 of the official static Linux musl binary before extraction and
execution. Downloads stay under ignored `local/security-tools/`; the report is
`local/security-audit/report.json`. No host packages are installed, and Cargo.lock
is unchanged. Cached archives are verified again on every invocation. Updating
the audit tool requires updating its version and checksum together.

Security checks run independently of ordinary CI, including documentation pushes.
They are not added to the local push gate. CodeQL's first execution and upload
permissions must be verified on GitHub after this change is pushed. The
[SEC-01 design](features/security-checks/design.md) records ownership and validation.
