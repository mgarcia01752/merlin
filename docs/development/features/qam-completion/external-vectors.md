# QAM-04 deferred: required external interoperability evidence

Status: **deferred at the user's request**. QAM-04 remains unchecked. Existing
frozen mixed-profile transport and specification-derived unit tests remain
implemented. Do not relabel our own encoder/decoder output, imported ideal QAM
point tables or independently transcribed specification arithmetic as external
end-to-end interoperability evidence.

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

## Later implementation task

Keep private originals under ignored `docs/development/local/qam-completion/`.
After permission/provenance review, add sanitized distributable vectors under a
focused test-fixture directory with a manifest, then write Rust tests comparing
stage outputs against their independently produced expected values. Set tolerance
before inspecting discrepancies; do not round scientific values to conceal them.
Run the full gate and record exact vector hashes/tool versions/results in a linked
validation record before marking QAM-04 complete. No tool/license purchase or
external file acquisition is assumed by this plan.
