---
name: scientific-testing
description: Add or review codec, DSP, profile evaluation and interoperability tests in DOCSIS OFDM Channel Sim. Use for signal-processing changes and input-format contracts.
---

# Simulation testing

For integration acceptance, use the [canonical requirements](../../docs/requirements.md)
and [integration gates](../../docs/development/checklist.md#unified-backend-integration).
Retained source test reports/vectors inform independent cases, but every new
accepted capability needs passing tests in unified MERLIN. Source design checks
and documentation migration are not executable PHY acceptance.

For upstream codec/OFDMA work, also read the
[upstream validation procedure](references/upstream.md). Keep its direction-specific
source/decoder checks alongside all applicable downstream regressions below.

Use [development-workflow](../development-workflow/SKILL.md) to connect new test
work to requirement IDs, owning stages, prerequisite tasks and acceptance evidence.

For unified Qt integration, apply the [three-level testing strategy](../../docs/development/features/unified-qt-workbench/testing-strategy.md).
Focused commits run affected functionality, numerical/boundary vectors,
formatting, relevant Clippy and applicable API/dependency checks. Official UIU
completion requires complete relevant feature/crate, affected Qt/QML,
compatibility and traceability gates; major milestones/final acceptance retain
full applicable regressions. Expand across all affected consumers for shared
science, API compatibility, CXX-Qt registration, serialization and cross-direction
lifecycle changes. Record executed, not-required and deferred checks separately.
Keep compiled artifacts/profiles and avoid unrelated wheel rebuilds or suites;
new linked native code still requires an artifact containing that code.

Local full validation uses `scripts/test-local.sh all` with checksum-pinned
nextest and four process-isolated workers; `MERLIN_TEST_THREADS` overrides the
1..32 budget. Split independent parameter matrices into separately named tests
instead of hiding expensive cases in one loop. Preserve assertions, source
seeds, private temporary outputs and all combinations. Keep genuinely dependent
steps inside their case; serialize shared resources and timing-sensitive cases
with explicit profile overrides. Preserve doctests, examples and analysis-only
coverage. Qt/QML runs separately. Measure full-suite wall time and failures;
parallelism is not a successful-test cache or permission to omit coverage.

- Minimum acceptance is MERLIN-only closed-loop verification: original source →
  MERLIN TX → configured channel → MERLIN RX → original-bit comparison. The user
  explicitly excluded external interoperability as a prerequisite. Do not block
  minimum task closure on another implementation or external dataset. Retain
  normative/source checks and accurate evidence labels; a passed closed loop is
  not a claim of external compatibility.
- Keep separate integration tests for codec boundaries, normative parameter
  arithmetic, QAM/LLR conventions, temporal evaluation and document/CLI behavior.
- Use independent expected values and external vectors when available.
  Source-table preservation checks may use pinned PyPNM row labels and hashes,
  but must not be reported as independent coding-chain interoperability when
  production uses those same tables. Keep generic Gray-LUT conventions separate
  from the DOCSIS row/label convention and preserve explicit control permutations.
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
- Dynamic profile tests must admit actual CRC-protected DPD packets and select
  C/N generations from received fields. Verify the exact MSB-first128-symbol U
  pattern, a word crossing the NCP change and continuous receive sequence/RNG/
  interleaver history. Keep fixed-path rejection and parity-slot replacement
  limits explicit; packet/hash admission alone does not verify interoperability.
- For nonideal acquisition, fit the channel from continuous pilots and score
  independent held-out scattered pilots. Test unknown timing/phase and nonzero
  fractional CFO with noise, derotation across arbitrary block splits and weak/
  ambiguous rejection. Distinguish the original fractional-only operation from
  the bounded bootstrap-hypothesis search. For the latter, cover both spacings,
  signed integer offsets, two-symbol narrow-channel scoring, indistinguishable
  hypotheses, raw-block continuation and recovered coded bits without TX H in RX.
  State candidate CP/RP/maps and search bounds; these synthetic checks do not
  establish arbitrary RF acquisition or external full-channel interoperability.
- Pilot placement tests must use actual active-frequency endpoints, include
  predefined-pilot coverage of isolated PLC regions, and verify edge/poor-bin
  avoidance, distinct count, density and reproducible nonperiodic placement.
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
- For paired pre/post-equalization constellation capture, classify payload from
  received NCP spans rather than TX labels. Preserve same-cell physical/source
  coordinates through the shared inverse interleaver, retain FEC-failed payloads,
  exclude control/pilots/filler and verify receive-time ordering, finite pending
  storage and honest partial counts. Collection/removal/exhaustion must preserve
  noise/source draws and decoder outcomes. Compare direct Rust and CLI data through
  the same JSON parse boundary; do not round simulation samples to hide differences.
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

Explicit `--test-quick` selects the reviewed local subset via
`tools/testing/quick-tests.json`, skipping 21 named expensive sweep cases while
retaining representative recoveries and other tests. Keep doctests, preparation
example, analysis-only checks, release smokes, static/tooling checks and Git
guards. Full remains default; hosted CI is unchanged. Reject combining it with
`--skip-test`; preserve the selected mode through `--resume` and printed retry
commands. Report omissions and do not equate subset success with full coverage.

## Parser boundaries and fixture transformations

Choose expectations independently of production conversion helpers/constants.
For signed fixed-point input, test zero, extrema, one representable step, negative
and fractional values, byte/component order and scaling through the public parser
or serializer as well as the helper. For example, a synthetic four-fractional-bit
representation maps raw 1 to 0.0625 and raw -1 to -0.0625. This arithmetic example
does not define a DOCSIS sentinel. Missing-value markers stay format-specific;
the same bit pattern can be valid in another format.

Map existing assertions, parameter combinations and oracle origins when moving or
splitting tests; confirm actual test discovery rather than accepting case counts
as equivalent coverage. Cover truncation, declared lengths, trailing bytes,
unsupported versions, overflow and allocation/work bounds where the contract applies.
Keep convergence/status and independent recovered truth as separate assertions.

Sanitization is an explicit, format-aware preparation step producing a new approved
fixture, never an incidental packaging mutation. Preserve original evidence in its
authorized ignored location. Record source/output hashes, transformation, permission
and provenance without exposing private identities. Keep aliases consistent across
related times/channels and distinct for distinct sources. Reject unknown layouts
rather than guessing offsets or replacing arbitrary byte patterns.

Decode original and prepared copies through the format owner to compare unaffected
semantics. Review filenames, archive members, metadata, screenshots and logs
separately; changing a fixture does not sanitize earlier Git history. Curated bundles
need an approved relative-path/size/hash inventory and bounded archive-member checks.
Builds read that inventory without rewriting fixtures; repeatability and installed
resource discovery require their own evidence. Follow the existing example-folder
and packaging owners rather than adding a second bundler.
