> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/plc-randomization/`
> Logs, captures and specification images are not published.

# Validation

- Local original PHY3.1 I20 PDF157/Figure68 rendered and inspected; register
  orientation, seeds, mask-bit ordering, once-per-cell clock and reset confirmed.
- Four randomizer API tests pass: independent polynomial oracle over three full
  frames at both spacings, published first label mask, exact soft magnitudes,
  hard inverse, rejection invariance and data/NCP independence.
- One real-codec integration test passes: two complete frames per spacing through
  message FEC, nibble grouping, permutation, randomization, soft inverse, inverse
  permutation, LDPC and timestamp CRC. Receiver receives no transmitter truth.
- Full pre-push gate passes (checks.log). Analysis-only all-target Clippy passes.
- Snapshot example generates six bounded blocks at frame boundaries. JSON shape,
  register reset and first mask checked; Markdown relative targets/fences checked.
- Scientific-testing skill validated through canonical and both discovery paths.
- No external interoperability, waveform, acquisition, Windows or macOS claims.
- NEXT-10 complete. NEXT-11 is PLC preamble; common randomizer observer pending.
