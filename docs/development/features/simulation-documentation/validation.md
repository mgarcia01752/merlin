> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/simulation-documentation/`
> Logs, captures and specification images are not published.

# Validation

- Five source folders verified against current modules; root config/runner/results
  reviewed separately. QAM256 example confirms seeded-source composition.
- Added seven design pages: index, five component pages and root runner page.
  Each contains a Mermaid diagram; all local links/anchors and fences checked.
- Guides distinguish current coding experiments from future waveform/full-channel
  orchestration and decoded descriptors from applied receiver configuration.
- Coding-style skill requires per-model/module design user docs with diagrams,
  independent of API catalogs. Canonical and both discovery paths validate.
- README/architecture/checklist updated; RX-CONFIG-01 records the receiver-state
  gap discussed with the user. No simulation source changes for this docs task.
- git diff --check passes. Mermaid rendering was not executed (mmdc unavailable).
