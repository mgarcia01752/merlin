# Consolidated development workspace

## Scope and outcome

Use `docs/development/` as the sole working home for ongoing development.
Keep reviewable feature plans and validation summaries in tracked `features/`,
and private specifications/captures plus generated outputs in ignored `local/`.
The [workspace guide](../../README.md) and [feature index](../README.md) provide
public navigation; the [architecture](../../../architecture.md) and
[checklist](../../checklist.md) remain authoritative for current implementation.

## Completed

- Move the former root development workspace intact; verify SHA-256 of all
  703 original files before reorganizing project-authored notes.
- Promote 45 authored records with historical-status banners, keeping copied
  specification text, images, captures and migration snapshots private.
- Update runnable examples, local sweep launchers, agent instructions and skills
  to use the new workspace; retain saved simulation result bytes and provenance.
- Link the public development entry point from the main README and maintained
  architecture/checklist. Add the ongoing workspace rules to development skills.

## Validation

Validate public Markdown targets and reject links into ignored local evidence.
Verify the old root directory is absent, local artifacts remain ignored, public
records remain trackable and both skill discovery paths validate. Check the
relocated artifact hashes against the original manifest. No simulation algorithm,
input values, decoder outcomes or coding contracts change in this reorganization.
