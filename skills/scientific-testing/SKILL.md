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
  For PLC block interleaving, verify MSB-first nibbles and independent full read
  orders for both FFT geometries, plus exact hard/soft inverse. Keep local PLC
  carrier/time coordinates distinct from physical FFT bins and data interleavers.
  Selected probe windows must retain full grid metadata and original offsets.
  For PLC randomization, independently verify field polynomial arithmetic and
  raw-bit versus MSB-first label masks (first PLC label mask0xE). Test state
  continuity across codewords/symbols, independent data/NCP state, no preamble
  clocks, frame reseeding, rejected-call invariance and exact soft sign inverse.
  Keep known-phase sequencing distinct from acquisition; recover multiple full
  frames through FEC/permutation/randomization without TX truth in RX inputs.
- For full PLC signals, independently transcribe both preamble tables by columns,
  test every label/sign/energy, and connect actual FEC/QAM to received soft bits.
  Verify acquisition MAC HCS/FCS with independent check strings and corruption
  tests, mandatory OCD edges/protected roles, DPD profile identity, integer repeat
  deadlines and MC continuation/pointers across real frame boundaries. Keep MC
  segmentation separate from multipart MMMs; reject unsupported formats explicitly.
  Physical assembly must preserve missing history, require empty scattered-pilot
  placeholders and protect PLC/pilot/excluded bins. Test actual cell-interleaver
  outputs and all128 phases. Observation windows retain full grid/frequency
  metadata; disabled/exhausted/failing callbacks preserve completed results.
- For receiver configuration, recover settings from received messages rather than
  transmitter objects. Test all descriptor arrival orders, independent OCD/DPD
  counters, default/specific precedence, protected roles, coverage and grid bounds.
  Test source/channel/bootstrap consistency, readiness gating and rejection
  atomicity with the last valid snapshot retained. Keep fixed-session admission
  policy distinct from dynamic profile activation. Include a real FEC/QAM/MC
  integration with pointer-free middle continuations and downstream components
  built only from received settings; never insert idle inside an unfinished packet.
- For waveform TX/RX, verify centered physical-bin ordering and unitary scaling
  using independent impulse/DC/tone/direct-sum vectors, not only a round trip.
  Cover both FFT sizes and every permitted CP/RP pair, including evaluation-only
  zeroRP, the half-sample taper, cyclic suffix and independent overlap/energy
  identities. Verify contiguous sample identity, arbitrary RX block splits,
  incomplete/drain reporting, overflow and rejection-state invariance. Probe
  metadata must retain direction, full geometry and original sample/bin offsets.
  Compare actual pilot/PLC signals through time samples and decode real PLC bits;
  deterministic reference data cells do not establish encoded payload/NCP transport.
  Keep known-reference timing/equalization distinct from blind acquisition and
  future full-channel production-runner validation.
- For production payload/NCP transport, acquire receiver settings through the
  live PLC gate rather than TX plans. Exercise both spacings, all supported NCP
  orders, maximum depth, reset-crossing words and words completed during drain.
  Verify fixed-profile rejection, valid-FEC wrong-CRC, orphan continuation,
  premature start/filler, queue bounds and retained pending/complete outcomes.
  Transport samples and pilot roles only; TX codeword IDs, reservation lengths
  and source bits must not become hidden RX input. Document explicit startup
  filler and source/channel origins. Prove failed/exhausted probes preserve
  actual future recovery, and retain received soft-cell rank metadata. Long-run
  duration must not introduce per-second resets or unbounded waveform buffers.
- For duration jobs, classify whole words by received source-symbol/sample spans,
  independent of late drain arrival. Keep crossing/outside words in global PRBS
  alignment but out of contained-word BER/rate. Test window edges, announced vs
  unrecovered starts, partial/cancelled results and unavailable rates before completion.
  Test runtime expiry with a deterministic clock rather than sleeps. Bound retained
  word details separately from scalar totals and preserve science after sink errors.
- For observation/export, test source/noise RNG continuity with enabled, exhausted
  and failed sinks; observing a seeded payload downstream is not sufficient. Keep
  the existing coding AWGN producer distinct from future waveform impairments.
  Test selected original offsets/metadata, versioned JSONL, record/stream byte
  limits before writer mutation, I/O failure, and non-overwriting CLI destinations.
- For exposed parity/twist/demultiplexing stages, retain independent index anchors
  and compose the same shared mappings behind the legacy facade. Test each reverse
  soft boundary without adding unsupported coding/QAM modes.
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
