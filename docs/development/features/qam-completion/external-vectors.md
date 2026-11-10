# Optional external QAM interoperability references

External interoperability is outside MERLIN's minimum closed-loop scope, as
clarified by the user on2026-11-09. QAM-04 is complete for its documented source
checks and internal coding/recovery tests. No external-vector result is claimed.
See [minimum acceptance](../closed-loop-validation/design.md).

The following list is retained only for a future explicitly requested external
comparison; it is not a blocker for QAM-04 or QA-05.

## What to obtain

A reproducible vector bundle generated independently of MERLIN, from a trusted
DOCSIS PHY reference implementation or a suitable CMTS/test instrument. A binary
capture alone is insufficient without known input bits and configuration. Start
with a noiseless digital reference, rather than an RF capture needing separate
synchronization/calibration, so bit-order defects can be localized.

| Required item | Content |
| --- | --- |
| Provenance | Producer/tool or instrument, version, export procedure, applicable specification issues and redistribution permission |
| Source truth | Original information bits, bit/byte order, exact K, header/stuffing policy; no seed-only promise without the defined generator |
| FEC output | Expected BCH and LDPC coordinates before/after shortening; retained parity order |
| Bit processing | Expected parity/twist/demux stage bits, column geometry and explicit pad/unused positions |
| Profile | Actual per-carrier bit loading, protected/zero-loaded roles, profile ID, physical/source ordering and applied frequency permutation |
| Cell randomization | Initial state, symbol/reset phase, cell clocks and expected pre/post-randomized labels |
| QAM output | Expected label-indexed I/Q, normalization/scaling and numerical tolerance; square and non-square data |
| Transport evidence | For waveform tests: NCP messages/CRC, exact start boundaries, timing, spacing, CP/RP, depth, startup/drain and optional ideal samples |
| Integrity | SHA-256 per file, documented dimensions, sanitized identities and a manifest linking all files |

## Initial acceptance set

1. Full uniform64/128/512/1024/2048/4096/8192/16384-QAM words with expected outputs;
   retain16/256 controls. Include zero-extension positions for128/2048/8192/16384.
2. Mixed profiles combining square/non-square/high-order loading and zero-loaded
   carriers; include a word ending in a partially filled cell and one crossing
   symbols. Include16384-QAM rather than testing only low-order cells.
3. Shortened square/non-square/mixed words with an incomplete final interleaver
   column, valid odd-byte transport sizes, expected terminal pointers and padding.
4. At least one externally impaired soft/sample vector with known truth to check
   actual error decisions/statuses. Iteration counts are decoder-algorithm-specific;
   do not demand equality between unrelated decoder implementations.
5. For full transport interoperability, both25/50kHz spacing, defined reset phases
   and source/channel coordinate metadata. A mapper-only vector cannot validate
   NCP, cell interleaving or a complete receiver.

## Optional later implementation task

Keep private originals under ignored `docs/development/local/qam-completion/`.
After permission/provenance review, add sanitized distributable vectors under a
focused test-fixture directory with a manifest, then write Rust tests comparing
stage outputs against their independently produced expected values. Set tolerance
before inspecting discrepancies; do not round simulation values to conceal them.
Run the full gate and record exact vector hashes/tool versions/results in a linked
validation record before reporting an external interoperability result. No tool/license purchase or
external file acquisition is assumed by this plan.

## PyPNM text-table review

Reviewed the local PyPNM checkout at commit
`82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9`. All thirteen copied QAM/scaling
text files match byte-for-byte. For data orders, the imported convention uses
row index as the MSB-first label. A new Rust test checks all 32,720 data labels
across ten orders, source SHA-256 hashes, independent Table54 scaling constants
and selected inverse-soft-bit signs. No additional table copy or Python DSP is
introduced; the production resources retain their existing license/provenance.

This closes the **complete source-table preservation check**, not independent
coding-chain interoperability. The production mapper already uses those files.
Comparing them against that mapper can catch import/order/build defects but
cannot independently certify the source labels or FEC/interleaver behavior.
PyPNM's separate `CodeWordLutGenerator` constructs generic axis Gray labels and
uses a sequential-Gray fallback for nonrectangular constellations. Its generated
`qam_lut.py` must not silently replace the preserved DOCSIS table row convention.

For an optional external comparison, the evidence would be a file produced by another
DOCSIS implementation containing known input bits plus expected encoded/interleaved
bits and QAM output. Original PNM constellation samples and ideal point tables do
not contain the complete known transmitted-bit chain. This optional comparison must not be reported as executed unless the actual
external stage comparisons pass; it is not required to close the internal task.

The pinned PHY specification explicitly names **PHYv3.1 QAM** sample code for
bit-to-point mapping (AnnexA; reference list section2.2). This would be a useful
independent mapper reference if obtained with its provenance. The legacy official
`CM-PHYv3.1_QAM-Mapping140610.zip` HTTPS download returned404 during this review;
no such archive was found in the inspected local specification/resource trees.
Do not silently substitute generic Gray-code output. A mapper-only sample still
would not supply the full encoded/interleaved/control truth listed above.
