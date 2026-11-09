> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/lte-ingress/`
> Logs, captures and specification images are not published.

# Validation

Linux local toolchain. Workspace tests, analysis-only tests, fixture-example
unit tests, all-target workspace Clippy with warnings denied, subsequent targeted
Clippy after identifier/constant cleanup, rustfmt and whitespace checks passed.
The LTE integration tests cover independent equal-power/shoulder anchors, symmetry,
nulls, physical-grid origin invariance, invalid powers and coverage rejection.
Fixture tests cover exact header mismatches, profile coverage and separate epochs.

Actual generator output compared byte-for-byte with all four committed snapshots.
Original decoder snapshot hashes and new model/variant hashes match provenance.
Re-running the generator refuses existing destinations without changing outputs.
An altered channel spacing rejects before any output is created. New document
relative links were checked. No LTE waveform or measurement-driven channel/FEC
experiment was run; those remain open. No Windows/macOS validation performed.
