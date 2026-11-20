# Evidence, freshness and selective adoption

[Workflow](../SKILL.md) · [Development workspace](../../../docs/development/README.md)

## Durable evidence

Search current feature records before repeating an investigation. Record one
coherent question with a direct answer, repository-relative paths/symbols, revision
or dirty-candidate identity, review date, evidence and refresh triggers. Keep it
with its existing feature owner and index it from the development workspace.
Do not create a parallel architecture or copy large logs/private payloads.

Distinguish source-confirmed behavior, observed execution, inference and unknowns.
Proposals belong in design, with planned status. A useful fact says which owner
publishes accepted state, what failure test was run and which native path remains
untested; “Cancel works” does not establish those boundaries. Update invalidated
facts with the owning change and preserve historical observations as historical.

Report local numerical tests, source builds, extracted packages, native installed
lifecycle, interactive updater behavior, hardware interoperability and hosted CI
separately. Record the actual command, revision/candidate, environment and result.
A documentation review or configured workflow establishes none of the later gates.

## Changing external contracts

When a task depends on external APIs, SDKs, OS/runner availability, security
advisories or packaging/service rules, verify the relevant official source for
the pinned consumer version. Record URL, version/section, review date and evidence
scope. Search snippets or a recent date alone are insufficient.

Do not silently upgrade source pins or replace the applicable versioned DOCSIS
evidence with a newer issue. Explain conflicts and put required migrations through
architecture/requirements/checklist. Keep specification force and delegated-source
boundaries intact. An offline or unavailable source leaves the claim unverified;
retain the last verified record rather than advancing its date.

Update affected local guidance/examples when the task changes their assumptions.
This is task-triggered review, not a background updater or permission for live API
calls, credentials, package installation, publication or advisory suppression.

## Selective guidance adoption

Compare new guidance against the local owner and record adopt/adapt/retain/defer
decisions. Keep MERLIN-specific domain, licensing, platform, saved-workbench and
Git/release contracts. Record source revision and selected paths; for uncommitted
sources label a working-tree candidate and fingerprint the selected contents.
Never claim that a local checkout is the latest remote revision without checking.

Keep procedures locally usable. Adapt required links to available MERLIN owners,
include necessary examples/resources and use relative discovery links. Do not
copy a skill entrypoint that relies on missing sibling references or helpers.
Write reusable rules without private payloads or workstation-specific dependencies.
Preserve attribution when actual material is copied/adapted, following
[coding-style](../../coding-style/SKILL.md#reused-material-audit).

Changes here do not authorize editing the foundation or another consumer checkout.
When reusable discoveries merit upstream reconciliation, prepare a concise local
proposal with problem, evidence, generic rule, limitations and acceptance if those
external edits are outside scope. Do not claim synchronization from a proposal.

## Development setup review

For setup changes, retain `install.sh --development` and its current ownership.
Define required versus optional prerequisites and validate the entire invocation,
host mapping, tool pins and owned destinations before transactions. Importing or
sourcing a module must not install, download or elevate.

Review `--check` and `--dry-run` for all writes: lazy rustup/SDK downloads, cache or
directory creation, bytecode, logs, configuration changes and package-index refresh
are mutations too. Use already-installed tools for inspection and exercise an
empty-tool-root case with isolated fixtures. Missing prerequisites, invalid input
and a valid dry-run plan need truthful, documented statuses.

Installation must preserve inputs, external SDKs and caches, verify pinned downloads
before execution and stop on failure. A mocked control-flow check is not clean-VM
provisioning evidence. These are review criteria, not a claim that every existing
installer path has been revalidated or authorization to run setup during other work.
