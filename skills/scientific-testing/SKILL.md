---
name: scientific-testing
description: Add or review codec, DSP, profile evaluation and interoperability tests in DOCSIS OFDM FEC Toolkit. Use for scientific changes and input-format contracts.
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
