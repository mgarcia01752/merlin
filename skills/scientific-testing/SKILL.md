---
name: scientific-testing
description: Add or review codec, DSP, profile evaluation and interoperability tests in DOCSIS OFDM Channel Sim. Use for scientific changes and input-format contracts.
---

# Scientific testing

- Keep separate integration tests for codec boundaries, normative parameter
  arithmetic, QAM/LLR conventions, temporal evaluation and document/CLI behavior.
- Use independent expected values and external vectors when available.
  An encoder/decoder round trip alone does not prove DOCSIS compatibility.
- Distinguish independently transcribed specification arithmetic from externally
  produced interoperability vectors. Record exact document issue, page and table.
- Cover BCH correction boundaries and demonstrated failures, LDPC syndromes,
  convergence/failure, bit permutations, QAM labels, energy and LLR signs/scaling.
- Verify constellation labels independently of imported point geometry, including
  BPSK/QPSK control labels. Test published randomizer states, mask-bit ordering,
  pilot/zero-loaded clocking, symbol resets and soft inverse sign conventions.
- Test NCP control FEC separately from payload FEC: use the published CRC-24-D
  vectors, independent mother-code parity expectations and explicit puncture/label
  ordering. Reject the whole field on CRC failure. Test receiver boundary discovery
  from decoded pointers without transmitter diagnostic IDs, alongside packer tests.
- Test PLC LDPC separately from payload/NCP: independently solve the pinned
  mother parity matrix, check explicit puncture boundaries and restore deleted
  information/parity as zero LLR erasures, never shortened known-zero priors.
  Preserve actual iterations and failed decisions. Test CRC, parity convergence
  and truth independently, including a different-valid-codeword example.
- For cell interleavers, verify exact index/delay vectors, all supported depths,
  partial columns and dummy padding, PLC-aligned pilot references, startup/drain
  and source-symbol phase at inverse randomizer resets. Never count None history
  as a zero sample or recovered payload. Keep bit and cell interleavers distinct.
- Verify missing/excluded samples, exact grids, explicit time tolerances,
  protected roles, capacity overheads and intermittent impairments.
- Check invalid schemas, input/work limits, cancellation and partial results.
  Saved results are historical claims; re-evaluate against source measurements.
- Use synthetic sanitized inputs for unit tests. PNM parser fixtures stay in
  the capture adapter; the toolkit consumes decoded typed frames, never services.
- Preserve deterministic seeds. Do not assert elapsed-time equality. Explain
  numerical tolerances and zero-failure confidence bounds.

Run from the repository root:

```bash
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
scripts/cargo-local.sh test -p docsis-ofdm-fec --no-default-features --locked
```

Report tested host/toolchain and unverified targets. Passing Linux tests do not
establish Windows/macOS runtime validation or modem receiver equivalence.
