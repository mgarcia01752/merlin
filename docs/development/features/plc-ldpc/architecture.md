> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/plc-ldpc/`
> Logs, captures and specification images are not published.

# PLC LDPC coding stage

Implement NEXT-08 separately from payload and NCP. Local original PHY3.1 I20
PDF96 Table21, §7.4.15.2.3 gives the (480,288) mother matrix with L48;
§7.4.3.2/PDF56 specifies right-shifted identities. §7.5.13.6/PDF153 explicitly
punctures a48..95 and b384..431. No shortening. Retained ascending coordinates
0..47,96..383,432..479; punctures are zero LLR erasures on reception.

Original PDF96/153/154 inspected. Source discrepancies: downstream prose names
Table20, which is actually ranging CP/roll-off; use the expressly delegated
fine-ranging mother matrix Table21. Figures64/65 print final b469, inconsistent
with (480,288), the explicit puncture ranges and384-bit output; final retained
coordinate must be479. Record this as an implementation interpretation justified
by the complete explicit text, not a silently corrected normative extract.

fec-codec/plc owns parameters, matrix, standalone puncturing/LLR restoration,
encoder/decoder setup, separate encode/decode operations and typed results.
Use existing safe Rust ldpc-toolbox primitives; no new dependencies or FFI.
ofdm/transmission/plc/coding is a thin typed bridge, not another codec. Provide
one-block operations and bounded complete-frame noiseless integration through
actual timestamp/MC bytes. No block interleaving, randomization, waveform or
blind acquisition in this step. Probe producers/export remain separate TODO.

Validation: independent sequential parity solution, all288 information basis
vectors, explicit puncture boundary vectors, zero erasure placement, syndromes,
noiseless recovery including erased information bits, actual iterations, one
specified correction and failure demonstration, invalid sizes/LLRs/work limits.
Frame test recovers360/720 bytes at both spacings and verifies TS CRC, without
transmitter IDs supplied to decoding. No external interoperability claim.

## Completion and validation

NEXT-08 implemented and checked in the official architecture/checklist. Seven
primitive tests cover independent mother parity across all288 basis vectors,
exact coordinate/erasure behavior, noiseless recovery, specified correction and
failure cases, invalid inputs and distinct truth-versus-parity outcomes. Two
adapter/integration tests recover full frames at both spacings and preserve
structured errors/failure decisions. All nine new tests pass.

A preliminary test incorrectly assumed all-zero LLRs converge to a zero word;
the actual decoder does not converge in that case. The final truth test instead
uses likelihoods for a different valid codeword; no production algorithm was
changed to force a tie result. This preserves the intended scientific distinction.

Full pre-push gate passed, including workspace and analysis-only tests, warnings-
denied Clippy, helper/shell checks, release CLI build and smoke run. Separate
analysis-only all-target Clippy passed. Example has zero errors over5760/2880
information bits and all30 frame blocks report one actual decoder iteration.
Markdown local links/fences, skill validation through all three paths and diff
whitespace checks pass. No new dependencies or system packages.

Remaining work: NEXT-09 block interleaving, followed by independent PLC
randomization/preamble/mapping and physical assembly. OCD/DPD MAC acquisition
packets and common observer registration remain explicitly unchecked.
