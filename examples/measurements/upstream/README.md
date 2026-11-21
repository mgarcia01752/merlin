# Upstream synthetic measurement inputs

[Measurement index](../README.md) · [Upstream inputs](../../../docs/user/upstream/measurements.md)

These authored synthetic fixtures exercise bounded upstream admission. They are
not field captures or external interoperability evidence.

| Folder | Use |
| --- | --- |
| `ucd/` | Original management envelopes, starting at DA and excluding CRC. |
| `rxmer/` | CCAP major2 RxMER headers, exact subgrids and missing readings. |
| `pre-eq/` | Applied s2.13 and last-update s1.14 complex coefficients. |
| `measured/` | Complete synthetic capture cohorts and explicit association plans. |

Origin: the approved source revision
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`, Apache-2.0, copyright
2026 MORGANA contributors. The [per-file inventory](../../../docs/development/features/ofdma/source-files.json)
records original hashes. Hex fixtures are byte-identical; JSON plans change only
the configuration/plan format identifiers to the explicit MERLIN schema names.
Decode tests read hex as octets rather than generating golden data from production
encoders. Complete measured playback has its separate INT-PNM-02 acceptance gate.
