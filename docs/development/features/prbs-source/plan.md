> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/prbs-source/`
> Logs, captures and specification images are not published.

# SRC-02 / SRC-03

Implement a simulation-owned PRBS15 source, separate from PHY scrambling.
Define the sequence by s[n+15] = s[n+14] XOR s[n], oldest bit first,
with explicit nonzero 15-bit seed and continuous checked bit position.
Expose bounded generation and independent recovered-stream comparison.
Keep the existing seeded waveform API; add an explicit source-selecting API.
Verify literal recurrence vectors, maximal period, chunk continuity, rejection
atomicity, mismatch reporting and actual payload waveform recovery at both spacings.
Update source/component guides, architecture, checklist and public API reference.
PRBS selection is an experiment policy, not a DOCSIS normative traffic pattern.
