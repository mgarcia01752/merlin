# QAM-02/03 validation

Status: targeted tests and the full pre-push gate pass.

## Evidence and tests

- Original local PHY I20 pages116–119/122 and ETSI EN302769 V1.2.1 PDF36 were
  rendered and visually checked for row/column, twist and demultiplexer association.
- `qam_slicing.rs`: nearest label/reference/error at every supported order,
  independent BPSK/QPSK exact-tie anchors, outside points and nonfinite/overflow rejection.
- `qam_coding_orders.rs`: independently transcribed full matrices for all ten
  orders, explicit zero positions/cell counts, exact retained soft inverse,
  shortened holes/partial columns and randomized noiseless soft/FEC recovery.
- `uniform_profile_orders.rs`: all eight newly admitted uniform data orders
  through actual live PLC/NCP, cell interleavers, waveform and FEC at both spacings.
- `shortened_receiver.rs`: received-boundary classification of differing short
  lengths and a full word, actual failed decoder decisions/iteration cap,
  malformed inputs, orphan continuation and no fabricated EOF completion.
- `shortened_transport.rs`: independent odd-byte length anchors/admission and
  eighteen shortened words through live acquisition and waveform (three lengths,
  three profiles, two spacings), including cross-QAM,16K and zero-loaded mixed cells.
- Existing16/256 hand-calculated anchors and boundary tests remain; assertions
  that newly verified modes must reject were updated, without removing their
  input-shape/nonfinite/unsupported-order checks.

There are13 new tests across five focused files. Targeted tests report zero
payload errors in noiseless recovery. The full gate passed:32 helper tests, Bash/ShellCheck, formatting,
warnings-denied all-target Clippy, workspace tests, analysis-only tests, release
CLI builds, version agreement and all existing scientific/CLI smokes. The release
example/gate runs `shortened_waveform` and verifies its JSON parses; the example
itself rejects failed noiseless recovery.

The saved release example reports original lengths1032/6136, zero bit errors,
zero actual LDPC iterations and successful LDPC/BCH/known-zero checks. An actual
`ofdm-fec channel` CLI request with a uniform128-QAM physical profile completed
with213480 original bits compared, zero errors and no first-error offset. This
uses the existing JSON request field, not CLI-side calculations or a substitute
coding path. Saved outputs are ignored local artifacts.

A final checked payload accessor rejects caller-corrupted public result vectors
instead of panicking. The shortened receiver/transport and existing observation
invariance tests pass after that change, alongside fresh all-target Clippy. The
observation producer reuses the same original-length payload accessor.

## Limits and deferred evidence

Host validation is Linux with the project-local pinned Rust/Cargo1.85.1. No new
Windows/macOS runtime or external DOCSIS device interoperability claim is made.
The tests' expected matrices are independent transcriptions/calculations, not
externally generated interoperability vectors. QAM-04 closes under the user-defined
[closed-loop acceptance](../closed-loop-validation/design.md); any future
[external comparison](external-vectors.md) remains a separate scope.

Logs/rendered pages belong under ignored `docs/development/local/qam-completion/`.
Private PDFs, generated PNGs, build outputs and transient JSON are not committed.
The original general `simulate` selector, SRC EOF padding and channel request
schema remain unchanged. Explicit shortened traffic is a bounded Rust/reference
experiment; no MAC packet builder or dynamic profile activation is implemented.


## Complete PyPNM source-table verification

`qam_reference_tables.rs` now checks every data label across all ten orders:
32,720 mappings, pinned source SHA-256, independently transcribed PHY Table54
energies and selected inverse-soft-bit signs. The source text files are identical
to PyPNM commit82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9. These tests passed.

This validates row/label import fidelity, not independent coding-chain truth:
production uses the same source files. This source check, together with existing
supported full/shortened/mixed recovery tests, closes QAM-04 within the user-defined
closed loop. Independent external stage vectors are not required for that scope.
See the [source reference review](external-vectors.md#pypnm-text-table-review).
