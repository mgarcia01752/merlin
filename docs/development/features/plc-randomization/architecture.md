> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/plc-randomization/`
> Logs, captures and specification images are not published.

# PLC randomization

Evidence: PHY3.1 I20 section7.5.13.8, PDF157/Figure68, rendered and inspected.
PHY4.0 FDD delegates PLC to PHY3.1; no FDX implementation claim.

Reuse the GF(2^12) primitive, with fixed internal PLC seeds, never live data/NCP
state. A standalone PlcRandomizer processes one validated interleaved96-label
codeword per call. Its typed output retains labels, first symbol and register
snapshots. A separate receiver instance consumes384 finite time-major LLRs and
returns sign-corrected LLRs. Both endpoints begin at the first post-preamble
symbol; no acquisition/resynchronization or arbitrary phase restore is provided.
State persists across codewords. On completion of120 data symbols, the next
block starts after the eight preamble slots, with reseeding immediately before
its first cell. Preamble samples never enter this API and are never randomized.
Invalid inputs must preserve state. State snapshots are direct test points;
common observer integration remains pending rather than fabricating grid events.

Validate both geometries over multiple complete frames using independent
carryless polynomial reduction, the published first mask, hard and soft inverse,
state separation and rejection invariance. Connect the real PLC FEC/CRC chain
without using transmitter truth in receiver inputs. Update public architecture,
checklist, API reference and user math/diagram. Run full gate and analysis Clippy.
