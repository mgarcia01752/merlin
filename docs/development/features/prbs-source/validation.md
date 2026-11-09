> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/prbs-source/`
> Logs, captures and specification images are not published.

# SRC-02 / SRC-03 completed

Linux, pinned Rust1.85.1. Full scripts/check-before-push.sh passed, including
workspace and analysis-only tests, warnings-denied all-target Clippy, formatting,
shell checks, 32 helper tests, release CLI build/simulation and version agreement.
Five new PRBS integration tests, one internal counter-overflow test and one invalid
source waveform test; existing full waveform recovery test expanded to seeded and
PRBS trials at both spacings. Literal recurrence expectations are locally derived
mathematical vectors, not external interoperability vectors.
Actual payload_waveform example JSON asserted source provenance, 56928 PRBS bits
with zero errors, four real recovered words, decoder success and zero PLC errors
at both25/50kHz. No whole-duration runner claim. Windows/macOS unverified.
342 local document link paths passed before final cross-links; final links are
checked separately. Mermaid source inspected, not rendered. git diff --check passed.
