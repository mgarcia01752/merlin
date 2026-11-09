> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/channel-runner/`
> Logs, captures and specification images are not published.

# RUN-01 / RUN-02 validation

Linux pinned Rust1.85.1. Full scripts/check-before-push.sh passed: workspace and
analysis-only tests, all-target warnings-denied Clippy, formatting, shell checks,
32 helper tests, release CLI build/smoke and version metadata.
Final constant-only naming cleanup was followed by all-target Clippy, rustfmt
and six focused runner/timing tests on the final code. No failures.

Six new tests cover independent duration arithmetic, overflow and sample rounding,
both spacings at maximum depth, fixed snapshots, bounded one-hour construction,
sample-block invariance, contiguous symbols/PRBS comparisons, completion/drain,
invalid input and terminal observer errors. Existing payload waveform proof was
also rerun after sharing its waveform link with the channel owner.

Release actual waveform run with 1.01-second50kHz source observation completed:
44888 source symbols,8950 recovered payloads,127376400 compared PRBS bits,
zero payload/PLC errors,352 received PLC frames, zero pending bits. No whole-run
waveform or per-word history retained. Source crosses second/reset/PRBS-period
boundaries continuously. This is noiseless known-reference validation, not external
DOCSIS interoperability or nonideal synchronization evidence.
Final release example3ms JSON independently checked for Finished, interval samples,
complete generated/recovered counts and zero errors. Window-specific throughput
classification, cancellation and the CLI facade remain open.

379 local document link paths and balanced code fences passed; diagrams inspected
as source, not rendered. git diff --check passed. Windows/macOS unverified.
Architecture/checklist reviewed: RUN-01/02/03/05 complete for the current subset;
RUN-04 is next. Remaining lifecycle/delivery items retain unchecked parents.
