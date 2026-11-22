# DOCSIS OFDM Channel Sim architecture

This is the maintained architecture for this repository. The
[implementation checklist](development/checklist.md) records completed work,
remaining requirements and the next implementation step.
The [development status](development/status.md) summarizes the current milestone,
recent evidence and remaining product scope.
The [simulation design index](user/simulation/README.md) links a separate
Mermaid-based user guide for each simulation component. Detailed equations and
examples live in the linked stage guides. The [development workspace](development/README.md)
links tracked [feature records](development/features/README.md). Private evidence
and generated artifacts live under ignored `docs/development/local/`.

Status describes the **current source checkout**, including work awaiting commit.
It does not imply a published release or external DOCSIS interoperability.
Review this document and the checklist at feature completion and each commit-message
request. Update affected design/status before supplying the command; partial work
remains unchecked until its implementation and validation are complete.
Checklist IDs use stable component/work-area prefixes, such as PLC, WAVE-TX, WAVE-RX and
RX-CONFIG, rather than temporary NEXT labels. Local numbering identifies work;
dependencies and current priority determine the next item to implement.

## Engineering guidance ownership

The [development-workflow skill](../skills/development-workflow/SKILL.md) owns
the Architecture → Requirements → Developer checklist → Implementation → Validation
procedure and semantic reuse audit. For new or materially revised stages, link
overview labels to their canonical sections, name concrete implementation/test/
registration files as Add (planned), Edit or Reuse, and connect requirements to
dependent tasks and acceptance evidence. This extends navigation and traceability;
it does not replace existing module ownership or renumber historical work.

The [skill catalog](../skills/README.md) routes implementation-specific rules.
[External-change review](../skills/external-change-review/SKILL.md) owns unfamiliar
candidate review while preserving the index and unrelated work. Evidence/freshness
records remain with their feature owner under the
[evidence procedure](../skills/development-workflow/references/evidence.md).
The [adoption design](development/features/engineering-guidance/design.md) records
the selective foundation baseline and overrides. This is documentation ownership;
no simulation, API, UI, dependency or delivery boundary changes.

### Upstream guidance migration

The [source-skill migration record](development/features/upstream-skills/design.md)
maps all pinned source skills to existing MERLIN owners. The
[signal-processing reference](../skills/docsis-signal-processing/references/upstream.md)
owns direction-specific upstream scientific contracts; the
[testing reference](../skills/scientific-testing/references/upstream.md) owns their
acceptance method; the [paired API reference](../skills/public-api/references/paired-api.md)
owns API design/parity review. Existing PHY editions and local discovery links
are reused. This skill migration changed guidance only; current runtime ownership
and MERLIN acceptance are tracked in the unified backend sections below.

## Requirements and historical evidence ownership

The [approved functional documentation organization](development/features/unified-backend/source-review.md#functional-ownership-after-doc-org-01) is delivered by
[DOC-ORG-01](development/checklist.md#doc-org-01). Active feature records own detailed
contracts; the three canonical documents and original historical acceptance keep
their authority. Frozen source references remain evidence, not current support.

The [requirements baseline](requirements.md#detailed-downstream-contract-register)
records existing downstream behavior at stage, API and adapter granularity.
Existing task/feature requirement IDs remain stable. The official checklist owns
overall progress; its subordinate UI checklist owns detailed presentation states.
The [backfill design](development/features/downstream-requirements/design.md)
maps those contracts to current modules, maintained tests and historical records.
It adds traceability without replacing stage definitions or acceptance history.

Historical records describe their original checkout/platform and can contain
superseded pending states. Later named refinements govern current behavior;
neither a test path nor a prior source-repository pass establishes a new run.
Missing execution records stay explicit coverage gaps with targeted acceptance
tasks. Original completed tasks retain their marks while those gaps remain open.
DS-REQ-01 closes documentation backfill only. DS-GAP-01..03 now have
[scoped current or recovered evidence](development/features/unified-backend/baseline.md).
All applicable unified-workspace gates must close before final backend acceptance. Broader
native/installed/hosted checks retain their existing separate task owners.

The [current backend master prompt](development/backend-integration-master-guide.md)
is preserved verbatim alongside the earlier integration guide. Its
[clause review](development/features/unified-backend/master-prompt-review.md)
maps every heading to this architecture, canonical requirements and delivery gates.
[INT-PLAN-01](development/checklist.md#int-plan-01) accepts this preflight only;
it does not accept new runtime capabilities or namespace decisions. New explicit
contracts are [REQ-COM-API-07](requirements.md#req-com-api-07),
[REQ-COM-API-08](requirements.md#req-com-api-08),
[REQ-COM-API-09](requirements.md#req-com-api-09),
[REQ-COM-TRACE-01](requirements.md#req-com-trace-01) and
[REQ-COM-TRACE-02](requirements.md#req-com-trace-02).
Implementation records must carry all fourteen evidence fields described in the
review. Existing historical acceptance is retained with its original evidence
precision; missing candidate/test/command details remain gaps, never invented passes.

## Unified backend integration

Delivery gates: [INT-DOC-01](development/checklist.md#int-doc-01).

This section is the authoritative **integration design** alongside the unchanged
implemented downstream architecture below. The source review is pinned to
`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`; MERLIN baseline is
`fa26de619b5d28a442c42e46ad1ed19262a9d08a`. That initial documentation checkpoint
contained no upstream runtime. The first additive implementation now supplies
[standalone upstream LDPC](development/features/unified-backend/fec-validation.md)
in the existing FEC crate. Shared FFT/PRBS15 kernels now have a numerical leaf in
`crates/dsp`, with passing [consumer acceptance](development/features/signal-processing/validation.md). Upstream PHY
now includes an independent typed engine/session with local
[synthetic backend acceptance](development/features/ofdma/design.md#session-and-core-acceptance).
Capture adapters, Rust/native Python APIs and directional CLI now have local
acceptance. Tooling and full unified regressions, including unchanged downstream Qt
compatibility, now pass locally. Final documentation/backend review is accepted
within the declared scope; the official checklist owns completion. The [regression record](development/features/unified-backend/validation.md#int-reg-01-complete-unified-workspace-regression)
retains exact candidates, executed cases and platform/security boundaries.
Source acceptance
reports are historical evidence, never MERLIN integration passes. The
[requirements baseline](requirements.md), [integration checklist](development/checklist.md#unified-backend-integration)
and [document dispositions](development/features/unified-backend/source-review.md)
provide behavior, delivery status and provenance respectively.

### Unified crate ownership

Delivery gates: [INT-LICENSE-01](development/checklist.md#int-license-01).

| Component | Current owner | Integration design and compatibility |
| --- | --- | --- |
| Unified Rust root | `crates/merlin`, Cargo/library `merlin` | Implemented thin facade; only re-export direction APIs and common contracts. No scientific implementation or core dependency back to this facade. |
| Common API contracts | `crates/phy-api`, Cargo `merlin-api` / library `merlin_api` | Implemented leaf; shared cancellation query/control and existing error categories only. Directional limits/reports/errors remain directional. |
| Downstream PHY | `crates/ofdm`, Cargo `docsis-ofdm-fec` | Retain package, public imports, control channels, interleavers, waveform, received configuration and report schemas. |
| Common FEC | `crates/fec-codec` | `src/upstream_ldpc/` now owns upstream definitions/codec and independent source tests; existing BCH, downstream LDPC, shortening, NCP and PLC remain. |
| Generic DSP | `crates/dsp`, Cargo `merlin-dsp`; existing directional numerical owners retained | Shared centered unitary 2K/4K/8K FFT and one-step PRBS15 recurrence; downstream wrappers preserve geometry/errors/state. Independent/frozen equivalence and hot-path measurements precede consumer rewiring; INT-DSP-01 now has passing consumer acceptance. No generic channel/QAM/measurement state extraction. |
| Upstream PHY | `crates/ofdma`, package `docsis-ofdma` | Independent typed configuration, TX/channel/received-only RX, bounded session, strict versioned document admission and instruments are implemented; Rust/native Python/CLI consumers pass local acceptance. Full unified local regression passes; supported scope and wider platform boundaries remain explicit. |
| Native measurement input | `crates/pnm-input` | Implemented `src/upstream/` owns bounded binary admission and complete measured association; core accepts an immutable typed handoff and retains original provenance. |
| CLI | `crates/cli`, executable `ofdm-fec` | Implemented upstream operations via facade/native input; retain every existing command and output contract. No second executable. |
| Native Python | `bindings/python`, package `merlin` | One private `_native` extension and real downstream/upstream/common modules; existing imports/exports retain identity. No second distribution or Python science. |
| Qt | `apps/qt` | Existing downstream presentation stays unchanged; upstream UI implementation is outside this backend integration. |

**Current dependency direction:** native Python → facade with optional
`native-input` → `pnm-input`; CLI → facade and `pnm-input`. The CLI also retains
its direct downstream dependency for existing handlers. Base facade defaults and
independent direction features are unchanged. Native input enables both PHY
adapters and re-exports existing contracts without scientific implementation.
Both independent PHYs → common API/DSP/FEC leaves;
`pnm-input` → downstream/upstream APIs and exact pinned `pnm`.
The independent Qt adapter retains its current dependencies.
The downstream `pnm-input` Cargo feature is not a dependency back to the adapter.

**Implemented dependency direction:**

```mermaid
flowchart TD
    CLI[Existing CLI] --> ROOT[merlin facade]
    PY[Existing native Python adapter] --> ROOT
    ROOT --> DS[Downstream PHY]
    CLI --> DS
    ROOT --> US[Upstream PHY]
    ROOT --> API[merlin-api contracts]
    DS --> API
    US --> API
    ROOT -->|optional native-input| INPUT[Native PNM input adapter]
    CLI --> INPUT
    INPUT --> DS
    INPUT --> US
    INPUT --> PNM[Exact pinned PNM decoder]
    DS --> FEC[Common FEC mechanisms and separate code definitions]
    US --> FEC
    DS --> DSP[Shared numerical DSP kernels]
    US --> DSP
```

No core PHY/FEC/DSP depends on CLI, Python, Qt, native input adapter or another
checkout. There is no cycle from PHY back to `pnm-input`. The planned API modules
live with their respective PHY ownership; the selected literal public root uses the thin facade described below.
All scientific algorithms/dependencies remain safe Rust. Optional downstream
analysis-only builds and existing feature behavior remain supported.

### Unified shared DSP and FEC

Active feature detail: [FEC mechanisms and distinct code families](development/features/fec/design.md)
and [shared numerical conventions](development/features/signal-processing/design.md).
Their initial documentation acceptance was separate from the executed DSP
consumer acceptance linked below.

Delivery gates: [INT-REUSE-01](development/checklist.md#int-reuse-01), [INT-FEC-01](development/checklist.md#int-fec-01), [INT-DSP-01](development/checklist.md#int-dsp-01).

Shared numerical evidence is recorded in [DSP integration validation](development/features/signal-processing/validation.md).
`merlin_dsp::transform::{TransformPlan,TransformSize}` owns only bounded array math;
downstream `waveform::transform` retains mode selection and exact error mapping.
`merlin_dsp::prbs15::step` owns one recurrence transition; source owners retain
seed/absolute position, resource validation, identity and independent verifier policy.
Both PHY wrappers now use these kernels through their accepted migration gates. The
[frozen former-kernel fixture](../crates/dsp/tests/support/legacy_transform.rs)
is test-only equivalence evidence, separate from independent Fourier/PRBS checks.

FEC mechanisms may share sparse adjacency, systematic encoder plans, bounded
decoder setup/iteration accounting and syndrome operations only where semantics
match. Upstream code definitions remain independently source-checked: long
N=16200/K=14400, medium N=5940/K=5040 and short N=1120/K=840. Their matrices
must not be replaced by downstream definitions of matching dimensions. Ordinary
upstream scheduled data has neither downstream BCH nor ranging-only puncturing.
Preserve shortened known-zero consistency separately from convergence/parity and
original payload truth. `fec-codec` remains usable without modulation/channel models.

The first additive backend implementation is
`crates/fec-codec/src/upstream_ldpc/{definitions,codec,error,mod}.rs`, with
independent `tests.rs` and `tests/{oracle,source_audit,source_fixtures}.rs`.
It reuses the existing private sparse plan through a byte-buffer operation;
the downstream GF2 operation and existing root codec imports remain unchanged.
This describes the first standalone codec checkpoint. Subsequent grant selection,
upstream transport and native Python delivery have their own accepted integration
records in the unified sections. Existing
[downstream codec tests](../crates/fec-codec/tests/codec.rs),
[normative checks](../crates/fec-codec/tests/normative.rs) and
[shortening tests](../crates/fec-codec/tests/shortening_codec.rs) remain.

DSP candidates are FFT/IFFT, complex representation, distance/LLR arithmetic,
noise/FIR/phase kernels, power accumulation and PRBS primitives. Compare exact
normalization, phase/bin/bit ordering, seeds, continuity, allocation and execution
cost before sharing. Equal names, orders or dimensions are insufficient. Retain
PHY-specific pilots, labeling, resource/grant state, CP/window/burst assembly and
receiver state machines. If extraction is justified, use proposed
`crates/dsp/src/{complex,transform,modulation,channel,noise,measurements}.rs` with
`crates/dsp/tests/equivalence.rs`; otherwise document direct local reuse. These
are concrete candidate paths, not installed modules or permission for empty stubs.
INT-REUSE-01 must settle the mapping before changing numerical owners; INT-DSP-01
accepts a justified no-new-crate decision too. Benchmark changed hot kernels and
preserve downstream re-exports/results without regenerating golden vectors.

### Unified downstream processing

The existing [module boundaries](#modules-and-stage-boundaries) and
[transmission/receiver flow](#transmission-and-receiver-flow) remain authoritative
for implemented downstream behavior. Source → BCH/LDPC → downstream bit processing,
packing/NCP, randomization/QAM and time/frequency interleaving → pilots/PLC physical
assembly → IFFT/prefix/window → channel → RX framing/FFT/estimation → inverse
resource/interleaving/demapping → received NCP boundaries → LDPC/BCH → independent
source comparison retains its existing order and startup/drain semantics.

Current owners are `crates/ofdm/src/transmission/`, `signal_processing/`,
`simulation/`, `scenario/`, `profile/` and `api/`. Preserve control/data distinctions,
null/excluded carriers, captured profiles and established receiver modes. Regression
anchors include [payload transport](../crates/ofdm/tests/payload_transport.rs),
[waveform integration](../crates/ofdm/tests/waveform_integration.rs),
[receiver configuration](../crates/ofdm/tests/receiver_configuration.rs) and
[public paths](../crates/ofdm/tests/public_paths.rs); INT-REG-01 requires the full
relevant suites, not only these representative files.

### Unified upstream configuration and source

Active feature detail: [OFDMA configuration, grants and source](development/features/ofdma/configuration.md).

Delivery gates: [INT-US-CONFIG-01](development/checklist.md#int-us-config-01).

Registered `docsis-ofdma` (`docsis_ofdma`) in `crates/ofdma` now owns the typed
foundation. `crates/ofdma/src/configuration/` owns UCD/IUC, represented MAP/grant,
profile/resource/loading and separately validated TX/RX projections. Generic
versioned JSON, captured UCD inspection and actual execution support are distinct.
Retain inspectable optional/reserved controls without claiming ranging/probing
execution. Configuration load/preview is idle and atomic. Source format tags must
be explicitly translated/versioned; renaming product identity must not relabel
raw original bytes or silently accept an incompatible schema.

The independent member is registered in `Cargo.toml` with `crates/ofdma/Cargo.toml` and
`crates/ofdma/src/lib.rs`. Concrete configuration owners are
`crates/ofdma/src/configuration/ucd/{mod,channel,controls,profiles}.rs` and
`crates/ofdma/src/configuration/{grant,profile,waveform}.rs`; shared resource
planning uses `crates/ofdma/src/resources/{grid,permutation,pilot_patterns}.rs`
and `crates/ofdma/src/grant/{mod,plan,fec_selection}.rs`. These owners are now
implemented. Versioned document admission and the complete idle session are now
accepted within INT-US-CONFIG-01 after the typed reference milestone.

`crates/ofdma/src/source/{prbs15,verifier}.rs` owns continuous source and
independent verification. Grant boundaries reset the 23-bit PHY randomizer, not
PRBS15. Source accounting separates meaningful data, information-fill ones,
shortening zeros, parity and zero grant pad; invalid/backpressured work consumes
no extra source. The verifier alone receives original truth, checks positions/
coverage and preserves failed recovered decisions. Reuse the existing PRBS only
after exact recurrence, seed, phase, bit order and chunk equivalence checks.
Existing tests: `crates/ofdma/tests/{configuration,grant_plan,prbs,upstream_pilots,upstream_interleaving}.rs`.
Document admission tests `crates/ofdma/tests/configuration_json.rs` now pass locally.
See [the implementation record](development/features/ofdma/design.md#int-us-config-01-typed-foundation).

### Unified upstream processing order

Active feature detail: [OFDMA transmitter](development/features/ofdma/transmitter.md)
and [OFDMA receiver](development/features/ofdma/receiver.md), with their exact
requirement/task/module/test boundaries.

Delivery gates: [INT-US-TX-01](development/checklist.md#int-us-tx-01), [INT-US-RX-01](development/checklist.md#int-us-rx-01).

Typed upstream TX composition is implemented, with stage tests recorded in
[the implementation record](development/features/ofdma/design.md#int-us-tx-01-transmitter-implementation-unit).
Combined CONFIG/TX acceptance now passes the document/session gates.
Typed channel transport and restricted residual playback are implemented with
[local channel evidence](development/features/ofdma/design.md#int-us-ch-01-channel-implementation-unit).
Normal received-only RX composition now has
[local stage and internal loop evidence](development/features/ofdma/design.md#int-us-rx-01-received-only-receiver-implementation-unit).
Document/session, synthetic stage, adapter and full unified regression acceptance
now pass locally; standalone LDPC
encoding/decoding now has the codec owners and acceptance identified below. The retained
[stage contracts](development/features/unified-backend/references/upstream-stage-contracts.md)
preserve detailed scientific definitions, source equations/tables, reference
planes and limitations at the pinned source revision. That reference is historical;
local ownership and any deliberate adaptations are controlled here.

```mermaid
flowchart LR
    S[Continuous PRBS source] --> T1[Grant plan and upstream LDPC]
    T1 --> T2[Grant randomizer then QAM and joint resource framing]
    T2 --> T3[Pre-Eq C then IFFT and sample assembly]
    T3 --> CH[Response H then residual then timing/phase]
    CH --> R1[Configured framing then useful window and FFT]
    R1 --> R2[Received-pilot effective-channel estimate and equalization]
    R2 --> R3[Inverse resources then soft demap and derandomization]
    R3 --> R4[Restore shortening then LDPC and deframe]
    R4 --> V[Independent original-bit comparison]
    S -. expected truth only .-> V
```

The linked stage index below is the complete eight-TX/six-channel/nine-RX
inventory. Fused numerical implementations must preserve these observable
boundaries, representations and state/identity semantics. A source stage checked
as implemented does not check its MERLIN integration task.

| Stage | Representation boundary |
| --- | --- |
| [US-TX-01: Grant framing and FEC selection](#us-tx-01-grant-framing-and-fec-selection) | Information/source → grant plan |
| [US-TX-02: LDPC encoding and shortening](#us-tx-02-ldpc-encoding-and-shortening) | Grant plan → encoded retained bits |
| [US-TX-03: Grant-seeded PHY randomization](#us-tx-03-grant-seeded-phy-randomization) | Encoded bits → randomized grant bits |
| [US-TX-04: Cell words and upstream QAM](#us-tx-04-cell-words-and-upstream-qam) | Randomized bits → labeled loaded cells |
| [US-TX-05: Pilots, resources and joint interleaving](#us-tx-05-pilots-resources-and-joint-interleaving) | Loaded cells/pilots → role-scaled physical bins |
| [US-TX-06: CM Pre-Eq](#us-tx-06-cm-pre-eq) | Physical bins → Pre-Eq-corrected bins |
| [US-TX-07: Centered unitary IFFT](#us-tx-07-centered-unitary-ifft) | Corrected bins → useful time samples |
| [US-TX-08: Prefix, window and sample assembly](#us-tx-08-prefix-window-and-sample-assembly) | Useful samples → indexed burst/guards |
| [US-CH-01: Indexed sample admission](#us-ch-01-indexed-sample-admission) | Indexed samples → validated admitted blocks |
| [US-CH-02: Exact unity transport](#us-ch-02-exact-unity-transport) | Admitted samples → exact transport |
| [US-CH-03: Physical channel response](#us-ch-03-physical-channel-response) | Transport samples → causal response samples |
| [US-CH-04: AWGN and restricted RxMER residuals](#us-ch-04-awgn-and-restricted-rxmer-residuals) | Response samples → residual-impaired samples |
| [US-CH-05: Integer timing, phase and CFO](#us-ch-05-integer-timing-phase-and-cfo) | Impaired samples → timing/phase/CFO samples |
| [US-CH-06: Output and passive observations](#us-ch-06-output-and-passive-observations) | Indexed output → RX delivery/passive observations |
| [US-RX-01: Configured burst framing and timing](#us-rx-01-configured-burst-framing-and-timing) | Received indexed blocks → framed intervals |
| [US-RX-02: Useful-symbol extraction](#us-rx-02-useful-symbol-extraction) | Frames → complete useful windows |
| [US-RX-03: Centered unitary FFT](#us-rx-03-centered-unitary-fft) | Useful windows → raw physical bins |
| [US-RX-04: Received-pilot estimation and equalization](#us-rx-04-received-pilot-estimation-and-equalization) | Received pilots/bins → effective estimate/corrected cells |
| [US-RX-05: Resource extraction and inverse interleaving](#us-rx-05-resource-extraction-and-inverse-interleaving) | Physical cells → ordered grant cells |
| [US-RX-06: Soft QAM demapping](#us-rx-06-soft-qam-demapping) | Cells → finite MSB-first soft bits |
| [US-RX-07: Soft PHY derandomization](#us-rx-07-soft-phy-derandomization) | Soft bits → derandomized coded metrics |
| [US-RX-08: Shortening restoration and LDPC decoding](#us-rx-08-shortening-restoration-and-ldpc-decoding) | Retained metrics → actual LDPC decisions/status |
| [US-RX-09: Information deframing](#us-rx-09-information-deframing) | Decoded information → meaningful decisions/coverage |

### US-TX-01: Grant framing and FEC selection

Grant framing must derive coded capacity and the upstream codeword sequence from accepted resources/loading while separating meaningful bytes, information-fill ones, shortening zeros, parity and zero grant pad.

Implemented owner: `crates/ofdma/src/grant/plan.rs`; acceptance tests: `crates/ofdma/tests/grant_plan.rs`.
Requirement: [REQ-US-TX-01](requirements.md#req-us-tx-01); delivery: **INT-US-TX-01**.

### US-TX-02: LDPC encoding and shortening

Encoding must use the selected source-verified upstream long/medium/short LDPC family with systematic order and exact shortening semantics. All three code families must pass source audit and codec acceptance before dependent waveform implementation.

Implemented standalone owner: `crates/fec-codec/src/upstream_ldpc/codec.rs`;
acceptance tests: `crates/fec-codec/src/upstream_ldpc/tests.rs` and
`crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs`.
[Codec acceptance](development/features/unified-backend/fec-validation.md)
precedes integration with the planned upstream transport.
Requirement: [REQ-US-TX-02](requirements.md#req-us-tx-02); delivery: **INT-FEC-01**.

### US-TX-03: Grant-seeded PHY randomization

PHY randomization must use the defined 23-bit grant seed, first-bit/tap convention and per-grant reset over exactly the transmitted bit positions.

Implemented owner: `crates/ofdma/src/bit_processing/randomizer.rs`; acceptance tests: `crates/ofdma/tests/randomization.rs`.
Requirement: [REQ-US-TX-03](requirements.md#req-us-tx-03); delivery: **INT-US-TX-01**.

### US-TX-04: Cell words and upstream QAM

QAM mapping must form MSB-first cell words in accepted visit order using exact upstream labels, normalization and role-dependent loading.

Implemented owner: `crates/ofdma/src/signal_processing/qam.rs`; acceptance tests: `crates/ofdma/tests/qam_mapping.rs`.
Requirement: [REQ-US-TX-04](requirements.md#req-us-tx-04); delivery: **INT-US-TX-01**.

### US-TX-05: Pilots, resources and joint interleaving

OFDMA framing must apply the accepted pilot pattern, known signs, role/loading plan, block partitioning and joint interleaving/address order.

Implemented owner: `crates/ofdma/src/transmitter/framing.rs`; acceptance tests: `crates/ofdma/tests/upstream_interleaving.rs`.
Requirement: [REQ-US-TX-05](requirements.md#req-us-tx-05); delivery: **INT-US-TX-01**.

### US-TX-06: CM Pre-Eq

Pre-EQ must apply one accepted complex corrective bank after role scaling and before IFFT, preserving the distinction between C and injected H.

Implemented owner: `crates/ofdma/src/signal_processing/pre_eq.rs`; acceptance tests: `crates/ofdma/tests/pre_eq.rs`.
Requirement: [REQ-US-TX-06](requirements.md#req-us-tx-06); delivery: **INT-US-TX-01**.

### US-TX-07: Centered unitary IFFT

IFFT must transform complete accepted 2K/50 kHz or 4K/25 kHz physical grids at 102.4 Msamples/s using the defined centered unitary convention.

Implemented owner: `crates/ofdma/src/signal_processing/transform.rs`; acceptance tests: `crates/ofdma/tests/transforms.rs`.
Requirement: [REQ-US-TX-07](requirements.md#req-us-tx-07); delivery: **INT-US-TX-01**.

### US-TX-08: Prefix, window and sample assembly

Sample assembly must apply the specified CP/RP, cyclic extension, taper and overlap while retaining exact useful/prefix origins, burst edges, tails and explicit guards.

Implemented owner: `crates/ofdma/src/transmitter/assembly.rs`; acceptance tests: `crates/ofdma/tests/tx_framing.rs`.
Requirement: [REQ-US-TX-08](requirements.md#req-us-tx-08); delivery: **INT-US-TX-01**.

### US-CH-01: Indexed sample admission

Channel admission must validate finite indexed sample blocks, clock, identities, continuity and budgets before changing state.

Implemented owner: `crates/ofdma/src/channel/stream.rs`; acceptance tests: `crates/ofdma/tests/channel_models.rs`.
Requirement: [REQ-US-CH-01](requirements.md#req-us-ch-01); delivery: **INT-US-CH-01**.

### US-CH-02: Exact unity transport

Initial unity transport must preserve sample bit patterns and metadata exactly without impairments or random draws.

Implemented owner: `crates/ofdma/src/channel/stream.rs`; acceptance tests: `crates/ofdma/tests/channel_models.rs`.
Requirement: [REQ-US-CH-02](requirements.md#req-us-ch-02); delivery: **INT-US-CH-01**.

### US-CH-03: Physical channel response

The response extension must apply the declared causal synthetic complex FIR without implicit normalization, circular wrap or per-symbol resets.

Implemented owner: `crates/ofdma/src/channel/stream.rs`; acceptance tests: `crates/ofdma/tests/channel_models.rs`.
Requirement: [REQ-US-CH-03](requirements.md#req-us-ch-03); delivery: **INT-US-CH-01**.

### US-CH-04: AWGN and restricted RxMER residuals

Residual synthesis must offer explicit Off, sample-AWGN and restricted RxMER-derived modes using separate deterministic noise state and declared reference power.

Implemented owner: `crates/ofdma/src/channel/residual.rs`; acceptance tests: `crates/ofdma/tests/rxmer_channel.rs`.
Requirement: [REQ-US-CH-04](requirements.md#req-us-ch-04); delivery: **INT-US-CH-01**.

### US-CH-05: Integer timing, phase and CFO

The timing extension must implement only the designed integer delay, constant phase and CFO with emitted-sample-clock continuity.

Implemented owner: `crates/ofdma/src/channel/stream.rs`; acceptance tests: `crates/ofdma/tests/channel_models.rs`.
Requirement: [REQ-US-CH-05](requirements.md#req-us-ch-05); delivery: **INT-US-CH-01**.

### US-CH-06: Output and passive observations

Channel output must retain indexed samples and bounded passive observations while separating diagnostics from legitimate RX inputs.

Add (planned) owner: `crates/ofdma/src/instruments/observer.rs`; acceptance tests: `crates/ofdma/tests/matched_metrics.rs`.
Requirement: [REQ-US-CH-06](requirements.md#req-us-ch-06); delivery: **INT-US-CH-01**.

### US-RX-01: Configured burst framing and timing

Burst framing must derive timing from accepted RX configuration and preserve actual indexed blocks and explicit missing intervals.

Implemented owner: `crates/ofdma/src/receiver/framing.rs`; acceptance tests: `crates/ofdma/tests/rx_framing.rs`.
Requirement: [REQ-US-RX-01](requirements.md#req-us-rx-01); delivery: **INT-US-RX-01**.

### US-RX-02: Useful-symbol extraction

Useful-symbol extraction must select exactly the scheduled N-sample windows without silently filling incomplete windows.

Implemented owner: `crates/ofdma/src/receiver/framing.rs`; acceptance tests: `crates/ofdma/tests/rx_framing.rs`.
Requirement: [REQ-US-RX-02](requirements.md#req-us-rx-02); delivery: **INT-US-RX-01**.

### US-RX-03: Centered unitary FFT

FFT must produce raw physical bins using the inverse of the accepted centered unitary TX convention.

Implemented owner: `crates/ofdma/src/signal_processing/transform.rs`; acceptance tests: `crates/ofdma/tests/transforms.rs`.
Requirement: [REQ-US-RX-03](requirements.md#req-us-rx-03); delivery: **INT-US-RX-01**.

### US-RX-04: Received-pilot estimation and equalization

Channel estimation/equalization must estimate effective C×H from received known pilots using the declared FrameFlatLs or FramePilotLinear policy and explicit zero-forcing validity.

Implemented owner: `crates/ofdma/src/signal_processing/estimation.rs`; acceptance tests: `crates/ofdma/tests/rx_estimation.rs`.
Requirement: [REQ-US-RX-04](requirements.md#req-us-rx-04); delivery: **INT-US-RX-01**.

### US-RX-05: Resource extraction and inverse interleaving

Resource extraction must independently invert the accepted resource/interleaving order and preserve unavailable scheduled cells and their bit positions.

Implemented owner: `crates/ofdma/src/receiver/mod.rs`; acceptance tests: `crates/ofdma/tests/waveform_loop.rs`.
Requirement: [REQ-US-RX-05](requirements.md#req-us-rx-05); delivery: **INT-US-RX-01**.

### US-RX-06: Soft QAM demapping

Soft demapping must produce MSB-first finite MaxLog natural-log P(0)/P(1) metrics from shared upstream labels/scales and a valid ReferenceVariance.

Implemented owner: `crates/ofdma/src/signal_processing/demapping.rs`; acceptance tests: `crates/ofdma/tests/qam_mapping.rs`.
Requirement: [REQ-US-RX-06](requirements.md#req-us-rx-06); delivery: **INT-US-RX-01**.

### US-RX-07: Soft PHY derandomization

Soft derandomization must independently regenerate the grant mask and reverse LLR signs before LDPC without changing magnitude or position.

Implemented owner: `crates/ofdma/src/bit_processing/randomizer.rs`; acceptance tests: `crates/ofdma/tests/randomization.rs`.
Requirement: [REQ-US-RX-07](requirements.md#req-us-rx-07); delivery: **INT-US-RX-01**.

### US-RX-08: Shortening restoration and LDPC decoding

LDPC integration must restore shortening exactly once through the existing codec contract and retain real decoder outcomes.

Implemented standalone owner: `crates/fec-codec/src/upstream_ldpc/codec.rs`;
acceptance tests: `crates/fec-codec/src/upstream_ldpc/tests.rs` and
`crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs`.
[Codec acceptance](development/features/unified-backend/fec-validation.md)
precedes integration with the planned upstream transport.
Requirement: [REQ-US-RX-08](requirements.md#req-us-rx-08); delivery: **INT-FEC-01**.

### US-RX-09: Information deframing

Information deframing must remove only receiver-known residual fill and retain actual complete/partial/failed meaningful decisions and coverage.

Implemented owner: `crates/ofdma/src/receiver/framing.rs`; acceptance tests: `crates/ofdma/tests/rx_framing.rs`.
Requirement: [REQ-US-RX-09](requirements.md#req-us-rx-09); delivery: **INT-US-RX-01**.

### Unified channel and measurement reference planes

Active feature detail: [OFDMA channel](development/features/ofdma/channel.md)
and [Pre-Eq correction banks](development/features/ofdma/pre-equalization.md).

Delivery gates: [INT-US-CH-01](development/checklist.md#int-us-ch-01).

Preserve `Y[k] = X[k] C[k] H[k] + N[k]`: C is transmitter correction, H physical
response, N impairment and Y the received observation. Received known pilots
estimate effective C×H. Normal RX receives samples and legitimate accepted
configuration, never H/C, original payload/PRBS seed, residual targets or noise RNG.
Truth correction is an explicitly labeled reference experiment, not normal recovery.

Unity transport must preserve signed zeros, indices, rate, guards/tails and sample
bit patterns without RNG. Nonunity order is causal FIR → output-referred noise/
residual → integer delay/phase/CFO, with continuous state across chunks and exact
drain. Initial RxMER residual synthesis requires unity physical H and every
scheduled C bank, with timing disabled; reject incompatible later updates before
source consumption. Do not relax this merely because downstream supports another
combination. Broader joint models remain deferred with their source design.

Implemented upstream owners are `crates/ofdma/src/channel/{stream,residual}.rs`,
`measurements/{rxmer,pre_eq}.rs` and `simulation/{request,run,control,report}.rs`.
Share kernels only through the reuse gate; configuration, reference planes and
missing-value policies stay direction-specific. All retained taps identify run,
configuration, stage, units, original indices, sample/bin geometry and coverage.
Observation/plotting cannot change RNG draws or decode outcomes. MER/BER/status,
convergence/parity/shortened-zero checks, source mismatch and unavailable/partial
coverage remain independent report fields. Job/retention/iteration budgets must
be explicit and validated cumulatively, not guessed from source task completion.

Planned passive metric/tap owners are
`crates/ofdma/src/instruments/{metrics,observer,spectrum}.rs`; the actual loop,
control and independent comparison acceptance belongs in
`crates/ofdma/tests/closed_loop.rs` and `crates/ofdma/tests/matched_metrics.rs`.

### Unified PNM and capture admission

Active feature detail: [upstream capture admission and measured input](development/features/pnm-measurement-inputs/upstream.md).
The [Pre-Eq feature](development/features/ofdma/pre-equalization.md) owns the
correction-bank/application handoff after raw adapter admission.

Delivery gates: [INT-PNM-01](development/checklist.md#int-pnm-01), [INT-PNM-02](development/checklist.md#int-pnm-02).

Implemented raw admission is owned by `crates/pnm-input/src/upstream/{ucd,ucd_projection,rxmer,pre_eq}.rs`;
`association.rs` owns complete measured admission. Raw acceptance tests are
`crates/pnm-input/tests/{upstream_ucd,upstream_ucd_projection,upstream_rxmer,upstream_pre_eq}.rs`;
whole-plan cases are in `upstream_association.rs`.
The Pre-Eq adapter reuses the exact pinned typed decoder for minor zero and its
fixed-point kernel for the approved opaque-minor profile without rewriting bytes.
UCD and CCAP major2 RxMER use bounded source-derived owners because the pin lacks
those contracts. Reuse the exact pinned decoder only for verified formats; any missing upstream
format needs a bounded source-derived Rust adapter, not a second parser for an
already supported format. The core typed models stay Qt/path/runtime independent.

Preserve original bytes/hashes, CM/CMTS/interface/channel identities, exact grids,
subcarrier-zero origins, units/scales, missing values, capture times and update order.
UCD message-envelope inspection is not configuration projection. Structural
multipart admission must preserve all fragments and reject missing/duplicate/
inconsistent sets before semantic projection. CM assignment, occupied bandwidth,
CCAP-interface→UCID mapping and synthetic replay epochs stay explicit.

RxMER single/average/hold/interpolation operates on residual power with stated
weights/coverage, not dB averages or silent frequency resampling. Pre-Eq applied
banks versus last updates have distinct scales and meanings; Set/Adjust requires
external operation/status/corresponding-probe/base context absent from the file.
Missing/ambiguous/stale bases reject atomically. Preserve source timestamps
separately from synthetic symbol offsets and original updates separately from
normalized effective banks. Measured C is neither physical H nor receive noise.
Preflight the entire selected schedule before any source/RNG consumption.

`MeasuredExperiment::admit` in the native adapter returns core-owned
`docsis_ofdma::api::MeasuredConfiguration`. The independent core constructor
validates domain readiness and the bounded versioned provenance envelope; it
does not parse files or certify caller-supplied capture claims. The session
accepts that immutable handoff atomically through `configure_measured`, retaining
original generic context separately from captured projection. Completed/partial
reports retain the entire original measured plan and source hashes.
[Local input acceptance](development/features/pnm-measurement-inputs/upstream.md#measured-association-acceptance)
covers both geometries and rejects incompatible future updates before work.

Existing downstream [input ownership](#pnm-measurement-input-direction),
[binary/API tests](../crates/pnm-input/tests/high_level_api.rs) and
[workbench tests](../crates/pnm-input/tests/workbench.rs) remain compatible.

### Unified Rust Python and CLI APIs

Active feature detail: [paired Rust/Python convergence](development/features/high-level-api/upstream.md).

Delivery gates: [INT-US-LOOP-01](development/checklist.md#int-us-loop-01), [INT-RUST-01](development/checklist.md#int-rust-01), [INT-PY-01](development/checklist.md#int-py-01), [INT-CLI-01](development/checklist.md#int-cli-01).

Retain `crates/ofdm/src/api/` and current downstream imports. The implemented
`merlin::{downstream,upstream}` facade re-exports those independent APIs.
`merlin::common` re-exports the `merlin-api` leaf: Cancellation, RunControl and
the unchanged fifteen ErrorCode variants. Downstream RunControl/ErrorCode are
exact re-exports, while rich upstream pause/progress state remains directional.
An explicit common-token bridge also cancels a paused upstream worker through
a bounded condvar wait; no worker or callback is added to the common leaf.
The upstream ApiError retains original causes and legacy code strings and adds
a shared typed category query. [Namespace implementation validation](development/features/high-level-api/namespace-design.md#namespace-implementation-validation)
records compatibility/build evidence. The [complete operation rows](development/features/high-level-api/operation-matrix.json)
now pass compiler-export and executed Rust case checks. Native namespace execution
also passes the [installed-wheel unit](development/features/high-level-api/python-evidence.json);
complete native operation coverage now passes [INT-PY-01 acceptance](development/features/high-level-api/python-operation-evidence.json).
The implemented
`crates/ofdma/src/api/{session,error,measured,reports}.rs` owns the upstream typed API, sharing
lifecycle/error/provenance conventions where semantics match. The existing
`merlin.Simulation` remains valid; an upstream controller is additive and does
not force one giant configuration or rename downstream exports. Final public
names/signatures retain the accepted INT-RUST-01 compatibility mapping.
The literal unified `merlin` root is now provided by a selected minimal re-export facade; it owns no controller, science, input parsing or lifecycle implementation.

The [selected exact namespace design](development/features/high-level-api/namespace-design.md)
is recorded by [INT-API-MAP-01](development/checklist.md#int-api-map-01), refining
[REQ-COM-API-07](requirements.md#req-com-api-07),
[REQ-COM-API-08](requirements.md#req-com-api-08) and
[REQ-COM-API-09](requirements.md#req-com-api-09). Cargo `merlin` / library
`merlin` in `crates/merlin` re-exports `docsis_ofdm_fec::api::*` under `downstream`,
`docsis_ofdma::api::*` under `upstream`, and genuinely shared contracts from
Cargo `merlin-api` / library `merlin_api` in `crates/phy-api` under `common`.
The existing downstream Cargo package/library and every `api::*` import remain.
Both PHYs depend only on common leaf owners, never on each other or the facade.
Direction-isolation facade features retain independent core/analysis-only builds.
A Cargo alias alone cannot assemble two PHY owners under one literal root.

Initial shared contracts are cancellation/query and unchanged error categories;
directional limits/configuration/engine errors/reports remain directional. Existing
`RunControl` and `ErrorCode` paths survive through re-exports after common extraction
is validated. The design keeps current public AtomicBool engine entrypoints while
adapting their internal cancellation query; it exposes no mutable shared flag.
No namespace placeholder is introduced by documentation. Initial upstream typed
sessions accompany stages; real closed-loop acceptance precedes INT-RUST-01 facade
convergence and INT-PY-01 native exposure. The [operation matrix](development/features/high-level-api/compatibility-matrix.md)
retains exact method/evidence ownership.

Direct Rust contract/parity acceptance lives in `crates/merlin/tests/namespaces.rs`
and `crates/ofdma/tests/experiment.rs`, alongside received-only engine tests.

The existing `bindings/python` builds one optional native distribution. Its
`src/upstream/{configuration,simulation,reports,capture,document,measurements}.rs`
adapts accepted public Rust operations; `tests/test_upstream_*.py` retains the
source-derived parity, capture, ownership and lifecycle cases.
Keep typed/runtime declarations, buffer/result ownership, structured error
categories, interpreter-lock release, Busy handling, bounded progress, cancellation
and pause semantics aligned. No scientific calculations or separate schemas in
Python. Preserve current saved-workbench schema/recipe behavior; introducing an
upstream document needs explicit version/migration and both language consumers.

[REQ-COM-API-08](requirements.md#req-com-api-08) adds standard Python imports
`from merlin.downstream import Simulation` and
`from merlin.upstream import Simulation`; both now execute in the local installed wheel.
Retain root `merlin.Simulation`/`merlin.Merlin` downstream compatibility and the
existing distribution. The real packages pass discovery, typing, wheel and installed
CPython 3.10 checks; other declared Python 3.10+ environments need native checks. Adding module
attributes alone cannot satisfy the import contract. The selected layout is a real
`merlin` package with one private `merlin._native` extension and import-only
`downstream.py`, `upstream.py`, `common.py`, matching stubs and `py.typed`. Preserve
root native class identities, distribution metadata `merlin-ofdm` and old imports.
Installed-wheel imports must run in fresh processes outside the source checkout.
[REQ-COM-API-09](requirements.md#req-com-api-09) owns the single compatibility matrix.

Upstream CLI dispatch now lives in `crates/cli/src/commands/upstream.rs` with
`crates/cli/tests/upstream.rs` acceptance. The existing executable accepts upstream
run/preview, measured plans and raw UCD/RxMER/Pre-Eq inspection through the facade
and native input adapter. The original signal handler requests both the unchanged
downstream AtomicBool and common upstream stop token. Existing commands/output
stay intact; the bounded JSON owner retains create-new output and failed recovery
reports. [CLI acceptance](development/features/high-level-api/cli-evidence.json)
records direct Rust/process parity and the complete downstream CLI regression scope.
Existing [native parity tests](../bindings/python/tests/test_parity.py) and
[workbench parity](../bindings/python/tests/test_workbench.py) remain mandatory.
Loading/preparing stays idle; only explicit run executes a frozen accepted plan.
Completed science survives cancellation. Pause retains PRBS/randomizer/RNG/
interleaver/overlap/update/receiver state at defined safe boundaries.

### Unified normative evidence and exclusions

The [implementation traceability gate](development/features/unified-backend/design.md#int-tools-01-executed-implementation-traceability)
now validates fourteen-field records, exact declarations/executed cases and
historical candidate identities through the existing audit. Explicit acceptance
also requires real compiler/native exports and compiled Cargo test discovery.
The official checklist remains the sole completion authority; retained historical
formats do not transfer standalone source acceptance to new MERLIN functionality.

Delivery gates: [INT-TOOLS-01](development/checklist.md#int-tools-01), [INT-DEFER-01](development/checklist.md#int-defer-01).

[REQ-COM-TRACE-02](requirements.md#req-com-trace-02) extends the existing
`tools/validation/integration_traceability.py` and functional-document validator.
Current prompt coverage/link checks are structural evidence only. The
[automation coverage register](development/features/unified-backend/master-prompt-review.md#automated-validation-gaps)
names the symbol, exact-test, export and executed-evidence checks now implemented
and accepted under INT-TOOLS-01, and repeated at final acceptance. Indexes support
the official checklist and must not assign a second completion state.

Preserve PHY 3.1 I20 §7.4.1 (processing), §7.4.3/Table8 (FEC), §7.4.4–7
(randomization/mapping/interleaving), §7.4.9–10 (transform/prefix/window),
§7.4.15–17 (distinct ranging/probes/pilots/Pre-Eq), and §9.4.8 (upstream RxMER).
PHY 4.0 I07 §7.2.4/§7.3.4 delegates upstream behavior; preserve FDX/FDD and
optional-capability distinctions rather than asserting full 4.0 waveform support.
MULPI 3.1 I25 controls UCD/MAP, CCAP-OSSI 3.1 I18 controls upstream RxMER file
admission, and CM-OSSI 3.1 I27 D.2.13 controls applied/update coefficient files.
The [source review](development/features/unified-backend/source-review.md) retains
exact hashes, page/table provenance, source claims and independently derived
fixtures. This documentation pass does not claim fresh PDF verification.

Source-only validation is not independent external hardware interoperability.
Preserve the independent all-family matrix corrections, row/edge fingerprints,
frozen impulse parity supports and public source-vector files; never regenerate
expected values from migrated production constants to make tests pass.

Deferred designs retain broader C/H/RxMER composition, inferred plant response,
fractional clock/stochastic phase noise, blind/wider acquisition and confidence,
finite-file transport, ranging/preamble/probe execution and upstream GUI. Their
source design gates do not accept executable MERLIN support. No device control,
SNMP/TFTP, multi-CM scheduler/MAC service or Python sidecar is introduced. Existing
Ubuntu/Windows delivery, two CI entrypoints and all release/security gates remain.

### Unified acceptance and final documentation gate

Delivery gates: [INT-BASE-01](development/checklist.md#int-base-01), [INT-REG-01](development/checklist.md#int-reg-01), [INT-DOC-02](development/checklist.md#int-doc-02), [INT-ACCEPT-01](development/checklist.md#int-accept-01).

INT-DOC-01 accepts only the consolidated design/requirement/task records.
INT-DOC-02 accepts agreement between implemented paths, skills, API/user docs and
observed MERLIN integration outcomes. INT-ACCEPT-01 requires all relevant downstream
and upstream Rust/native Python/CLI/PNM regressions inside the unified workspace,
limits/cancellation/determinism and notice/dependency review. Do not import source
checkmarks, source package/wheel/VM claims or stale historical status into current
MERLIN acceptance. Keep original technical documentation and vectors until their
requirements, decisions and evidence have been preserved and replacements pass.

Current backend acceptance: [INT-ACCEPT-01 final review](development/features/unified-backend/validation.md#int-accept-01-final-backend-acceptance)
records all 22 mandatory tasks, complete local regression and final documentation
correspondence. INT-DEFER-01 remains outside this backend delivery. Existing
native/hosted/installer and external interoperability boundaries remain explicit.

## Minimum acceptance: MERLIN closed loop

The user clarified on2026-11-09 that this project validates its own single-channel
transmitter, channel simulator and receiver. Compare decoded PRBS15/file data
against the original source, retain actual LDPC/BCH outcomes and iterations, and
verify control/configuration recovery, bounded work and partial results.
Another DOCSIS implementation or external reference dataset is not required.

WAVE-RX-05, QAM-04 and QA-05 are complete within their documented supported
closed-loop configurations. Existing normative/source-table checks remain in
place. These results do not claim external modem/CMTS interoperability. The
[closed-loop design](development/features/closed-loop-validation/design.md) and
[evidence](development/features/closed-loop-validation/validation.md) define the
minimum for that historical downstream checkpoint. Subsequent Qt, native Python
and OFDMA backend delivery is tracked in the unified sections above; multiple
channels and wider native/hosted acceptance remain separate.

## Terminology: cell and symbol

**Cell = QAM symbol** in the data path: one complex I/Q point carried by one
subcarrier during one OFDM symbol period. For example, a256-QAM cell represents
eight coded bits. The term **OFDM symbol** means the complete set of subcarrier
points transmitted together during that period, rather than one QAM symbol.

Use **QAM symbol (cell)** when introducing the term. Existing cell-interleaving,
cell-word and cell-index names retain the DOCSIS PHY terminology. Pilot/control
cells may use BPSK or QPSK instead of a data-QAM constellation. Bit interleaving
reorders coded bits; cell interleaving reorders modulation symbols across
frequency and time.

## Purpose and scope

MERLIN (Modulation, Encoding, Reception, Link Impairments & Noise) is the
DOCSIS OFDM Simulation Toolkit. Its transmitter simulates the downstream PHY
transmit functions of a CMTS for one OFDM channel, paired with a simulated channel
and receiver. Actual waveform generation and payload recovery exercise the
supported PHY path. Full CMTS MAC
services and multi-channel operation are outside the current scope.

Build a standalone, DOCSIS-focused Rust simulation core with two modes:

- **Ideal/reference:** known payload, fixed single downstream OFDM channel up to
  192 MHz, initially uniform 256-QAM, noiseless recovery first, then controlled
  impairments. The initial observation default is one second, with configurable longer
  durations and separate acquisition, completion and drain in the channel stepper.
- **Measurement-driven:** condition a simulation on supplied RxMER, downstream
  channel estimates and an applied modulation profile. Preserve exact grids,
  missing readings, timestamps and provenance. Establish reference planes before
  combining impairments so measured residual noise is not counted twice.

The configurable-duration waveform stepper supports noiseless, seeded sample
AWGN and per-carrier RxMER residuals with optional supplied diagonal response H.
The measurement path admits captured data profiles ID0..15 as independent
bootstrap ProfileA (wire ID0) experiments, with uniform data orders through16384-QAM or verified
non-shortened mixed loading, and explicit same-grid temporal measurement epochs.
Pilot-derived receiver correction is selectable alongside supplied-H reception. The channel job and CLI expose
cancellation, runtime limits, partial results and versioned probe export.
The existing CLI also retains the earlier16-QAM coding experiment;
Uniform/mixed packing/interleaving is exercised by the shared library, channel
CLI, examples and tests. Separate seeded, continuous PRBS15 and finite exact-byte file sources are implemented.
File traffic supports explicit PRBS15 fill and separate meaningful-data verification;
see the [PRBS design](user/simulation/prbs.md) and [file-source guide](user/simulation/file-source/README.md).

Downstream mapping and reference soft demapping cover constellations through
16384-QAM. Complete coding support must be tracked separately for each order;
current short8/9 bit processing supports all ten downstream data orders. See
[modulation coverage](reference/ofdm-support.md). DOCSIS4.0 FDD delegation does
not establish FDX behavior. OFDMA is a later scope.

All FEC, DSP, channel models and numerical calculations are Rust, including
algorithm dependencies. Simulation crates forbid unsafe code. The Qt
presentation shell and optional native Python bindings use the public Rust APIs; neither
owns numerical calculations. The Qt shell integrates bounded closed-loop jobs
and receiver instruments; optional native Python bindings are implemented. There is no
device capture/control or network-service dependency.

## Sparse control-channel encoding (FEC-SPARSE-01)

`fec-codec/src/sparse_encoder.rs` owns a private immutable substitution plan for
NCP/PLC systematic parity. It uses the existing sparse row/column adjacency and
solves checks with one unknown parity bit. Setup, storage and encoding are
O(rows + columns + nonzeros); the supported fixed matrices admit this order.
Non-peelable systems return an error rather than falling back to dense Gaussian
elimination. Payload LDPC retains its existing upstream staircase fast path.
Decoders, normative matrices, public APIs and puncture/shorten ordering retain
existing behavior. See the [design and measured validation](development/features/sparse-encoder/design.md).

## Downstream FEC shortening (FEC-03)

`fec-codec/src/shortening/` implements variable original-length coding around the
existing BCH/LDPC mother stages. `parameters` owns validated K, `encoding` the
six-stage TX coordinates, `decoding` known-zero RX restoration and consistency,
and `codec` their reusable composition. Each stage returns direct bounded test
points, with independently measured K-bit payload comparison and actual decoder
flags/iterations. Existing full-block constructors remain unchanged.

Local PHY3.1 I20 §7.5.4, PDF112–115 specifies only the16200-bit rate8/9 mother
code and shortening with all168 BCH/1800 LDPC parity retained. Normal64800-bit
frames are not used. Original PDF Figures46–47 were visually verified; PHY4.0
I07 FDX/FDD §7.2.5.4/§7.3.5.4 delegates FEC to these3.1 requirements. No additional
mother family or broader FDX simulation follows from that delegation.

For original K bits, prepend S=14232-K zeros before BCH, remove that prefix,
append S zeros before LDPC and omit those systematic zeros from the transmitted
word. Its length is K+1968. RX reconstructs finite positive known-zero evidence,
uses the existing mother decoder, restores the BCH prefix and checks omitted
coordinate consistency. Numerical LLR saturation is an explicit receiver policy;
failed decisions and actual statuses remain available for independent truth BER.

The [task design](development/features/fec-shortening/design.md) and
[component guides](user/fec-shortening/README.md) document diagrams, equations,
module contracts and validation. QAM-03 now connects explicit shortened words through
bit interleaving, cell packing, received NCP length discovery and waveform samples.
The file source still uses its full-word padding policy; no automatic EOF shortening
or measurement-JSON contract change is introduced.

## QAM coding and diagnostic decisions (QAM-02/03)

`signal_processing/modulation/slicing/` owns the standalone nearest-symbol decision.
It returns label/reference/error/squared distance, chooses the lowest label on exact
computed ties and never replaces soft LDPC input. Results are direct API values;
registered slicer JSONL producers remain separate instrumentation work.

Full uniform bit processing uses PHY I20 Tables24–37 and ETSI EN302769 V1.2.1
Table10(a), including explicit5/3/11/12-bit extensions for128/2048/8192/16384-QAM.
Permutation indices >=16200 expose these known padding positions. RX removes them
after inverse routing, retaining all actual mother-code likelihoods. Full uniform
profile packing/collection now uses the selected padded count rather than16200
for every constellation.

`modulation/interleaving/shortened.rs` owns24-column shortened ordering and skipped
unused holes; parity ordering is shared with the full mother path. The profile-aware
TX explicitly accepts valid odd-byte shortened words. `packing/shortening.rs` owns
transport admission and the received-span length rule from PHY I20 §8.3.2 PDF178–179.
The opt-in `receiver/payload/variable.rs` waits for CRC-accepted received start/filler
boundaries and classifies full/shortened words without TX sizes or identities.
It retains at most16213 soft values; incomplete history/EOF is not completion.
Original-length decisions, mother stages, actual statuses and consistency are exposed.

`simulation/payload/shortened/` composes these owners through actual PLC acquisition,
NCP, interleavers and waveform samples in a bounded reference experiment. It accepts
explicit original words, supplies a real following filler boundary, then drains.
It adds no noise, MAC framing, automatic file shortening or new general channel schema.
See [component guides](user/qam-processing/README.md) and the
[task design](development/features/qam-completion/design.md). QAM-04 validates this closed loop alongside the pinned source-table checks.
[External interoperability](development/features/qam-completion/external-vectors.md)
is optional future scope.

## Unified Qt Workbench

This is the accepted architecture for a separate UI delivery after accepted
backend checkpoint `23f63c9`. The user's [continuous execution authorization](development/features/unified-qt-workbench/execution-authorization.md)
accepts planning and authorizes task commits. [REQ-UIU-01 through REQ-UIU-27](requirements.md#unified-qt-workbench-requirements)
and the [UIU delivery tasks](development/checklist.md#unified-qt-workbench-delivery)
define its obligations and acceptance; no new UI functionality is accepted here.
See the [design and master-clause map](development/features/unified-qt-workbench/design.md),
[complete source inventory](development/features/unified-qt-workbench/source-inventory.json)
and [API gaps](development/features/unified-qt-workbench/api-gap-matrix.md).
This new UI scope supersedes the backend phase's upstream-UI deferral only;
completed backend/FEC gates and wider PHY deferrals are unchanged.

Requirements: [REQ-UIU-01](requirements.md#req-uiu-01).
Delivery: [UIU-PLAN-01](development/checklist.md#task-uiu-plan-01), [UIU-BASE-01](development/checklist.md#task-uiu-base-01).

### Unified Qt public API boundary

```mermaid
flowchart TB
    SHELL["One Qt application: shared menu and persistent direction tabs"]
    SHELL --> DS["Downstream QML and session adapter"]
    SHELL --> US["Upstream QML and session adapter"]
    DS --> BRIDGE["Thin QObject entry files together in src/bridge"]
    US --> BRIDGE
    BRIDGE --> FACADE["merlin::downstream / upstream / common"]
    FACADE --> DAPI["docsis-ofdm-fec::api"]
    FACADE --> UAPI["docsis-ofdma::api"]
    FACADE --> INPUT["Optional native-input re-exports: pnm-input"]
    INPUT --> DAPI
    INPUT --> UAPI
    DAPI --> LEAF["Common API / DSP / FEC leaves"]
    UAPI --> LEAF
    SHELL --> PLATFORM["Platform I/O, updates, ownership and UI state"]
```

All new Qt scientific calls use the accepted high-level facade API. Neither QML
nor QObject/direction adapters invoke internal PHY engines, DSP, FEC, native PNM
decoders or private science directly. QObject code translates requests, owns UI
session references, dispatches bounded asynchronous jobs and maps public errors,
progress and results. Configuration admission, previews, waveform/resource
construction, scientific measurements and results remain in backend owners.
The facade adds re-exports/optional feature wiring, never scientific code.
The implemented optional `native-input` feature exposes existing directional
admission under `downstream::inputs` and `upstream::inputs`. Passive downstream
`TransmissionConfiguration` and `SourceInput` validate timing/profile and exact-byte
source choices without executing or mutating sessions; see the
[typed contracts and parity](user/api/input-admission.md). Remaining editor,
Designer and idle-preview gaps keep UIU-API-01 open; Qt adoption is separate.
Original-capture `RxMer::preview` and `ChannelEstimate::preview` reuse the existing
bounded measurement-view/projection owner, retaining exact full grids, provenance,
nulls and explicit original H/RxMER units without source or session advancement.
`MeasurementSequence` adds immutable bounded admission of ordered unique original
RxMER inputs through the existing residual-power planner. All actual contributor
weights/epochs remain explicit for static averages and synthetic hold/interpolation.
Equivalent views use the numerical residual-power owner; `check_response` admits
explicit unity or captured H with the existing spatial/epoch/coverage contracts.
Optional native `inputs::RxMerDesign` wraps the existing version-1 saved Designer
evaluator with immutable typed flat/imported baselines and noncompounding recipe
replacement. Exact envelopes, original roles/grid/epochs and backend source-kind
labels remain explicit; Python is bounded transport/ownership only.
Read-only `ProfileCapture` retains the complete encoded source profile catalog,
compact schemes and original header/order without widening legacy simulation
`ProfileId`. Its bounded raw metrics and `Dpd::summary`/`view` reuse the existing
assignment, ideal Shannon and useful-symbol-rate owners. The shared parser omits
inspection arrays for legacy scientific callers; see
[profile contracts and parity](user/api/profile-inspection.md).
`ChannelConfiguration` freezes the existing loss/power/filter parameters and uses
the same spatial/epoch/coverage and channel-composition owners as execution.
Its [configured transfer preview](user/api/channel-preview.md) retains full original
grids and inputs, actual filter diagnostics and bounded before-receiver views;
it creates no waveform/filter stream or simulation state.
Independent PHYs do not depend on Qt, Python, the facade or each other.

The [operation/parity matrix](development/features/unified-qt-workbench/api-gap-matrix.md#operation-and-parity-matrix)
records exact existing lower-level call owners and missing public observations.
Each gap gets a typed public operation, Rust/Python parity, independent scientific
and API tests and a focused validated commit before Qt adopts it. Do not solve a
gap by exposing whole internal modules. Existing downstream API imports and
Python `merlin.Simulation` retain behavior. An automated Cargo/source boundary
check initially permits only reviewed pre-existing migration sites with sunset
tasks; final acceptance requires zero scientific migration entries. The implemented
[Cargo/source guard](../tools/validation/qt_api_boundary.py) uses exact dependency
records and whole-file hashes from the [legacy record](development/features/unified-qt-workbench/legacy-qt-boundary.json).
It rejects new/unreviewed direct package identities, including aliases, and is
called by Qt checks and Master preflight. Infrastructure packages carry explicit
non-simulation roles; this lexical gate does not replace scientific-owner review.

Requirements: [REQ-UIU-02](requirements.md#req-uiu-02), [REQ-UIU-03](requirements.md#req-uiu-03), [REQ-UIU-09](requirements.md#req-uiu-09), [REQ-UIU-24](requirements.md#req-uiu-24).
Delivery: [UIU-BOUND-01](development/checklist.md#task-uiu-bound-01), [UIU-API-01](development/checklist.md#task-uiu-api-01), [UIU-API-02](development/checklist.md#task-uiu-api-02), [UIU-API-03](development/checklist.md#task-uiu-api-03), [UIU-API-04](development/checklist.md#task-uiu-api-04), [UIU-API-05](development/checklist.md#task-uiu-api-05).

### Unified Qt source ownership

One native executable, `Merlin.Workbench` URI and current Main resource entry URL
are retained. QML has shared shell/theme/components and separate downstream and
upstream subtrees. Registered QObject entry files all remain in `src/bridge/`
for pinned CXX-Qt 0.7.3; direction adapter implementation lives in separate Rust
subtrees. Platform/update/I/O code remains application infrastructure. Exact
first destinations and consumers/tests are in the inventory and
[migration plan](development/features/unified-qt-workbench/migration-plan.md#destination-layout).
Preserve UI-ORG-01/02 acceptance, copyrights, notices and working presentation;
no unused symmetric placeholder trees or blanket scientific shared modules.

Requirements: [REQ-UIU-04](requirements.md#req-uiu-04), [REQ-UIU-08](requirements.md#req-uiu-08), [REQ-UIU-25](requirements.md#req-uiu-25).
Delivery: [UIU-ORG-01](development/checklist.md#task-uiu-org-01), [UIU-DS-01](development/checklist.md#task-uiu-ds-01).

### Unified Qt session and file lifecycle

Accepted user choices: **one active run across both tabs initially** and
**Save the active direction's project**. Each direction owns configuration,
inputs, accepted generation, session, run identity, worker control, results,
instrument selections, path and dirty state. Replace current implicit latest
result/input singletons with explicit session-owned state. Callbacks carry
direction/session/generation/run identity; stale results cannot overwrite another
session. Tab switching preserves work and does not stop execution. A paused run
retains the shared execution slot. Controls target the selected tab's own session;
another active direction is indicated and a second Start is rejected visibly.
Future concurrent dispatch requires aggregate resource admission and isolation
tests, not shared scientific configuration or a different API architecture.

Normal project files remain direction-specific. Open validates/detects direction
and guards that target tab's dirty state before mutation. Schema 1/2 retain DS
interpretation; a proposed schema 3 explicitly tags direction. Backend/native
input owners admit and serialize scientific project contents through the public
API with Rust/Python parity. Qt retains chooser, safe filesystem replacement,
dirty prompts and presentation state. Update recovery privately bundles both
independently valid projects plus active-tab state. All file/capture, viewport,
worker, retained-result and aggregate-memory budgets remain bounded; errors or
cancellation preserve accepted inputs/completed trials. See the
[compatibility matrix](development/features/unified-qt-workbench/compatibility-matrix.md).

Requirements: [REQ-UIU-05](requirements.md#req-uiu-05), [REQ-UIU-06](requirements.md#req-uiu-06), [REQ-UIU-07](requirements.md#req-uiu-07), [REQ-UIU-22](requirements.md#req-uiu-22), [REQ-UIU-23](requirements.md#req-uiu-23).
Delivery: [UIU-SESSION-01](development/checklist.md#task-uiu-session-01), [UIU-SHELL-01](development/checklist.md#task-uiu-shell-01), [UIU-FILE-01](development/checklist.md#task-uiu-file-01), [UIU-RECOVERY-01](development/checklist.md#task-uiu-recovery-01).

### Unified Qt upstream scientific presentation

The upstream data path is **CM transmitter → physical channel → CMTS receiver**.
CMTS UCD/numerology/IUC/MAP/grant/resource settings are a distinct control path.
CM pre-equalization C acts at TX; H, C and C×H remain separately labelled at
their actual reference planes. CMTS RxMER is a measured observation admitted to
the supported residual model, not a physical response or arbitrary correction.
Synthetic and measured modes expose the backend's real combination restrictions.
The configuration/TX/channel/RX/PNM/verification drill-ins use direction-specific
models and authoritative stage observations. The user's upstream diagram remains
an input to final visual placement, not permission to reconstruct scientific flow.

The OFDMA resource-grid viewer has symbol index on X and physical subcarrier
index on Y. A new bounded idle upstream API observation must expose authoritative
allocation/pilot coordinates and actual roles, grant/IUC/pattern metadata and
counts. Qt only renders/inspects these results with bounded batched drawing,
zoom/pan and selection. No pilot placement/allocation or one-object-per-cell grid
belongs in QML. Preserve accepted operating modes and normative evidence in the
backend; no wider PHY capability is implied by a visible control.

Requirements: [REQ-UIU-10](requirements.md#req-uiu-10), [REQ-UIU-11](requirements.md#req-uiu-11), [REQ-UIU-12](requirements.md#req-uiu-12), [REQ-UIU-13](requirements.md#req-uiu-13), [REQ-UIU-14](requirements.md#req-uiu-14).
Delivery: [UIU-US-01](development/checklist.md#task-uiu-us-01), [UIU-US-02](development/checklist.md#task-uiu-us-02), [UIU-US-03](development/checklist.md#task-uiu-us-03), [UIU-US-04](development/checklist.md#task-uiu-us-04).

### Unified Qt shared instruments

Shared scatter/axes/zoom/navigation, spectrum/response plots, FEC tables/charts,
verification and provenance widgets consume typed public API data. Numerical
FFT/PSD, MER/EVM, reference points, response/echo, LLR, bit ordering, resource
mapping, FEC outcomes and physical metadata remain backend-owned. Pure screen
transforms, clipping and layout remain presentation. Extract a scientific
instrument only after two real direction consumers and their tests demonstrate
compatible semantics. Keep DS PLC/NCP/profile labels, DS H and US H/C/C×H,
upstream LDPC versus downstream BCH/control chains, missing data and partial
outcomes explicit. Detailed candidate decisions are in the migration plan.

Requirements: [REQ-UIU-15](requirements.md#req-uiu-15), [REQ-UIU-16](requirements.md#req-uiu-16), [REQ-UIU-17](requirements.md#req-uiu-17), [REQ-UIU-18](requirements.md#req-uiu-18), [REQ-UIU-19](requirements.md#req-uiu-19), [REQ-UIU-20](requirements.md#req-uiu-20), [REQ-UIU-21](requirements.md#req-uiu-21).
Delivery: [UIU-SHARED-01](development/checklist.md#task-uiu-shared-01), [UIU-SHARED-02](development/checklist.md#task-uiu-shared-02).

### Unified Qt acceptance

All new delivery tasks remain unchecked until matching tests pass in MERLIN.
The accepted backend evidence is historical context, not a new UI baseline run.
Establish fresh Qt baseline evidence before code moves. Validate each API extension
before its Qt consumer; preserve downstream tests/imports/project behavior and
scientific vectors. Final acceptance requires both direction runs, installed-wheel
parity, a zero-exception scientific boundary, two-consumer instrument evidence,
bounded lifecycle/persistence/recovery, Qt Rust/QML checks, native build/launch,
resolved links/registration and an accurate owner/evidence report. Record actual
platform and installed-lifecycle limits. Preserve build caches; this phase does
not authorize pushes, releases, system installs or unsupported wider PHY work.

Requirements: [REQ-UIU-26](requirements.md#req-uiu-26), [REQ-UIU-27](requirements.md#req-uiu-27).
Delivery: [UIU-QA-01](development/checklist.md#task-uiu-qa-01).

## UI: MERLIN Simulation Workbench

The opening scheduling statements below describe the historical downstream UI
milestone. Python/backend delivery has since completed within its accepted scope;
the proposed unified UI phase above does not reset this downstream history.

The CLI functional MVP is complete for its documented closed-loop configurations.
The next major phase is the native Qt Quick/QML workbench, initially targeting
Ubuntu 22.04, 24.04 and 26.04 x64. Python bindings follow UI-01 through UI-04.
The initial UI-01 shell now exists under apps/qt: the clickable diagram, section
preview dialogs, theme switching, embedded icons and a tree-based OFDM editor.
The DPD capture importer, PRBS-TX editor and reusable SA with TX sample generation
also exist. Channel Estimate/RxMER share native capture viewers and Rust-backed
configuration/playback previews. Bounded full-loop Rust jobs now drive run
controls and actual receiver QAM, spectrum, response/echo, slicer/LLR and FEC
instruments. Receiver selectors query one immutable result; unsupported transport
modes remain explicit. See the [workbench guide](user/workbench.md) and
[Qt build/VM instructions](development/qt.md). Other target validation is pending.

The user's high-level diagram defines a clickable workbench, with focused pop-up
dialogs for editing each section, selecting an applicable file, and inspecting
purpose, status and available stage observations. Keep the signal path visible
behind the dialogs. Develop left to right, beginning with **OFDM Channel Descriptor (OCD)**:
then PRBS-TX / Transmitter and modulation-profile input, Channel Simulator and
its measurement inputs, and finally Receiver / PRBS-RX and results.

```mermaid
flowchart LR
    subgraph TXCONFIG["Transmitter Configuration"]
        SETTINGS["OFDM Channel Descriptor (OCD)"]
        PROFILE["Downstream Profile Descriptor (DPD)"]
    end
    SETTINGS --> TX["Transmitter"]
    SOURCE["PRBS-TX / file source"] --> TX
    PROFILE --> TX
    TX --> CHANNEL["Channel Simulator"] --> RX["Receiver"]
    CE["DS Channel Estimation PNM input(s)"] --> CHANNEL
    CE --> RX
    MER["RxMER PNM input(s)"] --> CHANNEL
    RX --> VERIFY["PRBS-RX / recovered-data verification"]
```

PRBS-TX and PRBS-RX represent source generation and independent recovered-data
verification. The workbench omits the original/recovered comparison connector;
verification remains independent and does not feed back or reseed the source.
RxMER sits above Channel Simulator with a downward arrow. DS Channel Estimate
sits below the gap between Channel Simulator and Receiver, splitting its input
connection to both blocks. File sources retain their own exact-byte verification. Measurement
inputs use supported decoded PNM JSON. The DPD input additionally accepts binary
modulation-profile PNM through a pinned Rust decoder dependency.
The detailed diagram below retains analyzer and receiver-reference ownership.

OCD and DPD are stacked inside the left-hand Transmitter Configuration panel,
with a merged visual connector into Transmitter. This is a presentation group;
each block retains its own dialog, readiness and validation. The top-row source/
measurement/verification blocks align above their processing stages. Rounded
cards and a branded header with the existing theme-specific logo follow the
approved reference graphic (UI-SHELL-09); light-mode support remains available.

UI-SHELL-10 centers the OCD/DPD merge on their midpoint and centers the supplied
channel-estimate split between Channel Simulator and Receiver. The external
supplied-H caption is removed. In-card PNM labels use the CM-OSSI I27 object names
`CmDsOfdmModulationProfile`, `CmDsOfdmRxMer` and `CmDsOfdmChanEstimateCoef`;
these document capture families and do not change optional flat/unity behavior.
The header pulse continues beside the toolkit title toward the logo, with a
separate fine underline below. Source evidence is recorded in the
[design](development/features/visual-workspace/design.md#ui-shell-10-centered-connections-and-in-card-pnm-types).

### Workbench block readiness rules

UI-COPY-01 keeps normal application wording focused on user tasks: no language,
framework or adapter ownership commentary in About, accessibility text or routine
help. Persistent notes are concise and limited to actionable errors, missing/
partial data, import identity/compatibility, display limits and essential
measurement assumptions. Detailed explanations stay in guides/field help;
original legal notices and capture names remain intact. See the
[design](development/features/ui-copy/design.md).

The left icon/WB gutter is removed (UI-SHELL-11). The diagram and header use
equal side margins; application branding stays in the top header and window icon.

PRBS-RX (UI-VERIFY-02) presents comparison acceptance, distinct run status and
summary cards plus payload recovery and retained per-payload error charts.
The existing Rust FEC query supplies at most 64 normalized word bars and explicit
omitted/uncompared counts. No-comparison BER remains unavailable; running/paused
states hide old final totals. These are aggregate recovery and sequence-index
charts, not BER history. Opening the panel never starts a job or changes DSP.
See the [design](development/features/verification-panel/design.md).

Each diagram block displays its heading, with a small in-card `CM PNM File:`
type footnote on DPD, RxMER and DS Channel Estimate. No generic status/subtitle
text is added inside cards. The left stripe communicates
readiness; the tooltip and accessible description provide a status and reason.
These are MERLIN workflow rules, not DOCSIS specification requirements.

| Status | Stripe | Meaning |
| --- | --- | --- |
| Complete | Green | The block's applicable requirements are satisfied by accepted, validated state. Before a run this means configured/ready, including receiver/comparison setup; results are not a prerequisite to starting. |
| Needs attention | Yellow | Required input, configuration, integration or results are missing or pending. |
| Error | Red | An actual selected-input, accepted-configuration or processing failure is present, with a concrete diagnostic. |

**WB-RULE-01:** OFDM Channel Descriptor (OCD) remains yellow until its accepted settings are valid
and a compatible modulation-profile PNM input has been successfully accepted by
Rust. Selecting a filename, opening a dialog or applying default settings does
not satisfy the profile requirement. Committing removal/replacement recalculates
readiness from the new accepted state. An accepted profile that
becomes incompatible is red; a missing profile is yellow. Rejected draft files
show their diagnostic in the dialog and preserve accepted readiness. Error takes precedence over missing
prerequisites; otherwise complete requires every applicable prerequisite.

**WB-RULE-02:** Keep each rule with its owning block and dependencies. Rust owns
simulation input/configuration validation and processing outcomes; Qt maps their
status and reason to presentation. A rejected draft shows its diagnostic inside
the editor and preserves the previously accepted block state. Do not invent red
errors for missing inputs, cancelled dialogs or unimplemented UI. Expected noisy
FEC outcomes belong to simulation results and are not automatically UI failures.
Optional inputs do not become mandatory through this display policy; future
explicit bypass/default modes must be evaluated by their owning Rust request.

**WB-RULE-03:** Start requires all nine workbench blocks to be green. Any yellow
or red block disables Start. Green means readiness to participate in the next run,
not the presence of results from that run; PRBS-RX must be ready to compare before
Start. Optional measurement blocks can be green through an explicit validated
bypass/default mode. Rust must still validate/freeze the complete request before
dispatch. Run availability and lifecycle are additional gates. The current shell
checks all block statuses and dispatches a validated frozen request through the
Rust worker only when idle. Receiver/PRBS-RX acceptance is configuration, not a
requirement to have prior results.

Current implementation: accepted OCD-compatible DPD turns DPD and OCD green;
accepted PRBS-TX configuration also makes Transmitter ready. Default unity response
becomes ready on the valid DPD/OCD grid. Accepted compatible RxMER makes RxMER
and Channel Simulator green. Removing/replacing inputs revalidates dependencies;
Cancel preserves accepted state. Applying supported receiver options validates
the complete accepted input set; enabling independent source comparison configures
PRBS-RX. Both become green before the first run. Start/Stop and Pause/Resume use
the same bounded Rust closed-loop worker; missing or incompatible inputs block it.
Optional measurements are not required for every future run.

### OFDM editor, registry and application navigation

Clicking OFDM Channel Descriptor (OCD) opens an editable pop-up with an expandable
Parameter/Value tree, dropdowns for supported values, and a description panel
below the selected field. Symbol timing contains cyclic prefix/roll-off, and
Interleaving contains time-interleaver depth. A Modulation Profile branch shows
blank Profile ID, Source file and Subcarrier assignments values until a profile
is selected; accepted DPD input supplies that summary. There is no Channel context branch.
The OCD describes common channel parameters; the DPD describes downstream
profiles (MULPI I25 §§6.4.40–6.4.41; PHY I20 §7.5.13.4). Bandwidth and spacing
remain in the Rust configuration, outside this editor; future input integration
must reconcile them separately rather than treating DPD as channel numerology.
Apply validates a staged configuration through the core Rust API and retains it in memory; Cancel
discards staged edits. Accepted settings feed the closed-loop worker and
self-contained workbench persistence.
The diagram block and popup use the full name OFDM Channel Descriptor (OCD).
This editor label does not imply complete OCD message import/export support.
Each dialog has its own QML file and focused Rust adapter where needed.
Draft editors share `components/ApplyCancelFooter.qml`: equally sized Apply then
Cancel buttons in one lower-right row with matching margins. Explicit layout
preserves this order across platforms. Secondary actions stay left; validation
and draft ownership remain with each editor. Cancel rejects without applying.
See [UI design skill](../skills/ui-design/SKILL.md).

All MERLIN popup dialogs inherit a shared title header with an upper-right
circular X, while retaining footer Close/Cancel controls. X rejects the current
popup without applying drafts or closing its parent; native file pickers retain
OS controls. See [UI-SHELL-06](development/features/popup-close-controls/design.md).
The Qt adapter reuses core simulation APIs for TX snapshots and measurement
configuration previews; QML implements no DSP.

Use an OSCAR-style metadata registry concept without importing its TLV protocol
model. Canonical location: `resources/registry/ofdm-settings.json`,
bundled with the application and loaded through Rust. Each entry records a stable
field ID, group, label, supported choices with display units, description,
application default where applicable, and pinned specification/version/section/
table references. Label paraphrases and any short exact quotations explicitly;
do not represent MERLIN defaults as normative requirements. PHY is authoritative
for these waveform fields; cite MULPI only for fields with actual pinned MULPI
evidence. Rust remains authoritative for numerical validation and derivation;
registry consistency checks must detect divergence from supported Rust values.

Workbench persistence uses a separate versioned `*.merlin.json` document
containing all accepted block settings and all associated PNM inputs, not a copy
of the registry. Input paths alone do not satisfy the restoration contract.
The registry describes fields; the workbench file stores accepted user choices.
Schema version 1 embeds original inputs as bounded base64 with byte counts and
SHA-256, including PNM and payload files. Load fully validates before replacing
accepted state; cross-input incompatibilities remain editable readiness diagnostics.
Save uses a temporary sibling and atomic replacement; imported original paths are
protected. Results, drafts and transient instrument viewports are not persisted.

The application has a standard top **File | Settings | About** menu bar above
workbench branding, rather than content-area navigation buttons. Settings →
Global opens appearance preferences; About → About MERLIN opens project details.
The instruction banner and standalone theme button are removed.
**File → Open**, **Save** and **Save As** use standard labels and shortcuts
and operate through the OS-native file chooser and Rust persistence adapter; individual capture inputs remain in
their corresponding block dialogs. **Settings → Global → Light/Dark mode**
applies the shared theme immediately to all windows; explicit workbench saves
also retain that choice.
Settings is separate from OFDM transmission configuration. About presents
MERLIN's expanded name, concise purpose, active icon and compiled version.
Matching project, issue-report and manual-update buttons route through Main.
About omits the standalone sentence and license/acknowledgement lists; all
required license and NOTICE texts remain bundled with distributions.
Each dialog owns its own QML file. See UI-SHELL-02 in the
[workbench design](development/features/visual-workspace/design.md).

### Workbench JSON configuration contract (version 1)

File → Open, Save and Save As use a versioned JSON workbench document. It contains every block's accepted settings
and all PNM inputs associated with those blocks, including selected profiles,
channel-estimation and RxMER inputs. Do not reduce the saved document to OCD
settings or filenames alone. Preserve input identity/provenance and enough input
content to restore the same configuration. Version 1 embeds exact originals as
base64 with byte counts/SHA-256 and caps the document at 144 MiB; original input
limits remain in force. Unsupported schema versions are rejected.
Rust owns serialization, file I/O and validation. OS-native pickers choose paths;
opening validates the whole document before replacing accepted state, invalidates
old results, and never starts a simulation automatically. Rejected loads preserve
the current workbench. Saving does not overwrite original PNM captures. Generated
results, caches and worker state are not configuration.

Channel Simulator power controls extend this same contract:
Save/Save As/Open restore accepted launch gain, receiver-side attenuation,
tilt/cable mode and parameters, and any exposed level/calibration/reference-band
choices with explicit units. New workbenches and supported legacy files missing
the new controls default to0 dB gain,0 dB attenuation and Tilt Off, while preserving
existing accepted additional-loss settings. The frozen run request must use those
restored values, rather than QML defaults. Version and migrate the affected
section explicitly; validate before atomic adoption and preserve state on failure.
Computed Channel Power and transient TP/SA/QAM navigation are not configuration.
### Channel composition preview (UI-CH-02 / WAVE-CH-COMPOSE-01)

Channel Model contains graphic Tilt / Cable Loss and Echo / Delay controls with
an enlargable Combined Channel Response graph. Duplicate measurement actions
are removed from this window; RxMER and DS Channel Estimate keep their existing
workbench blocks. Accepted previews do not start a run or reuse old results.
`signal_processing/channel/composition` owns the shared immutable prepared plan
for validation, waveform execution and Rust-rendered response projections.
Synthetic echoes follow diagonal magnitude loss/conditioning; their complex
responses multiply, with each effect applied once. The echo filter alone owns
continuous memory and tail. Imported H remains exclusive with synthetic paths;
captured FIR includes imported H/loss in its checked reconstruction. Existing
sample-AWGN, fit, grid, active-null and static-filter bounds remain in force.
See the [design](development/features/channel-workbench/design.md#ui-ch-02--wave-ch-compose-01-graphic-model-controls-and-composition).

UI-RF-01 implements this extension in `sections.channel_power` schema1 while
preserving `sections.channel_model` schema1. Legacy missing power sections select
unity gain/attenuation and relative units. Restored settings feed the run request.

### Curated workbench examples and Open location

`examples/workbench/manifest.json` inventories three runnable bundles: `single`
(one DPD, one RxMER, one supplied DS channel estimate) and `multi-rxmer` (one DPD,
five compatible RxMER inputs, one estimate), plus `multi-rxmer-2` (channel 193,
132 pre-collected RxMER files, companion DPD, flat/unity response because no
matching DS channel estimate is available). Each contains original files and a
self-contained WB with hashes, provenance and licenses. Each bundle matches modem,
channel, frequency grid and DPD coverage internally; production Rust project/request
validation checks compatibility. Original epochs remain unchanged; multi-RxMER
uses average residual power by default. The channel 193 source has irregular
acquisition intervals and a long gap; fixed-symbol hold/interpolation playback
is an explicit simulation clock. Selection/restore/playback permit 256 files,
while retaining 64 MiB original bytes and 4,000,000 decoded carrier values.

All native Open dialogs start in this examples root. Rust locates it relative
to installed resources or the marked source checkout; the isolated native picker
receives that directory only for Open. Save destinations keep OS behavior.
Linux shared-resource, portable and macOS resource layouts are recognized;
installed desktop delivery and per-OS runtime validation remain pending.

INSTALL-EXAMPLES-01 requires future desktop installers to include the runnable
manifest/bundles and verify hashes, offline restore and default Open from an
unrelated working directory. Existing CLI staging copies the examples already;
that is not desktop installer acceptance. `multi-rxmer-dschan` is a reserved,
non-runnable scenario: channel-estimate collection ordering, interpolation,
cadence, receiver interaction and WB schema remain CH-EST-PLAYBACK-01 work.
See the [design](development/features/workbench-examples/design.md) and
[user guide](user/workbench-examples.md).

### RxMER capture actions (UI-RXMER-04)

The RxMER capture toolbar uses native Open file(s) and collection controls;
demo/notch actions are removed (UI-RXMER-04). The shared Channel Estimate dialog
retains its own demo action. Existing Rust fixture/import APIs and WB restores
are unchanged. See the [design](development/features/rxmer-workbench/design.md#ui-rxmer-04-remove-demo-actions-from-rxmer).

### Chart zoom interaction (UI-ZOOM-01)

Workbench-wide zoom policy (UI-ZOOM-01): existing zoomable charts use wheel/
trackpad zoom anchored at the pointer and horizontal left-drag range selection.
Keep Full Span to reset the retained view; omit Zoom In/Out and +/- buttons.
Shared `ChartNavigation.qml` owns transient index selection only; existing Rust
viewport APIs retain sample ownership, bounds, units and decimation. Gestures
never rerun simulation or edit inputs. Keyboard +/- and Home remain available.
The shared TX/RX SA and RxMER/Channel Estimate viewer use this policy; diagnostic
plots without zoom remain unchanged. See the
[design](development/features/mouse-zoom/design.md).

### SA symbol-timing and cable panel (UI-SA-04)

The SA lower panel shows a CP-start zoom of actual TransmitWindow coefficients,
with symbol/FFT timing, prefix and roll-off boundaries from the completed
snapshot's frozen settings. It is explicitly a dimensionless window view, not
signal amplitude. Full time-domain IQ trace inspection remains future work.

Cable selection offers Belden 1694WB (nominal VF 0.82), CommScope P3 500 (0.87)
and custom VF. Rust derives the ideal flat guard budget `(Ncp-Nrp)/Fs` as extra
path length and, for the explicitly labeled two-way reflection assumption, half
that length as reflector spacing. Zero reserved timing margin is explicit.
These are timing estimates, not guaranteed echo immunity or maximum plant reach.
Unknown custom VF gives no distance; invalid VF is rejected. Cable changes only
query bounded display data and never rerun DSP or alter results. Applied input
changes clear old timing with the old spectrum. Simulation crates own calculations
and window coefficients; Qt owns layout/drawing. See the
[design and evidence](development/features/transmitter-spectrum/design.md#ui-sa-04-symbol-timing-and-cable-echo-budget).

### Shared Spectrum Analyzer (SA) and transmitter readiness

UI-SA-06 uses mutually exclusive Complex baseband / Capture frequency mapping
radio buttons in the shared TX/RX SA. The Frequency axis caption and raw Tap ID
line are removed; named test-point selection stays. Fixed FFT-bin grids hide the
axis choices. Changing axis preserves the current bin window and never reruns
the simulation. See the [control design](development/features/transmitter-spectrum/design.md#ui-sa-06-concise-frequency-axis-controls).

UI-SA-07 places FFT/window/sample-rate metadata in a compact plot overlay and
Run simulation at the lower left. Cancel preview appears only during local TX
preview generation; it never controls the main run. The cable panel uses a labeled
grid for flat guard time, extra cable path and distance to a reflector, explaining
VF and the out-and-back assumption. Uniform-cable/zero-margin limitations remain;
no signal calculations change. See the
[layout design](development/features/transmitter-spectrum/design.md#ui-sa-07-plot-metadata-footer-actions-and-cable-context).

UI-CH-01 implements Channel Input → Channel Model → Channel Output in its own
Qt dialog. RxMER and DS Channel Estimate open existing input frames directly.
Input/output open a Spectrum/QAM chooser; no separate SA block exists. Input uses
retained TX samples/physical bins; output uses RX-input samples/raw received FFT
cells before equalization. Missing observations remain unavailable. Input QAM
retains one physical symbol with bin filters; output retains existing received
role/profile/bin/symbol selectors. Main execution/readiness rules remain unchanged.
See the [staging design](development/features/channel-workbench/design.md).

WAVE-CH-TILT-01 adds optional Additional Loss under Channel Model, default Off.
The shared Rust impairment planner composes signed manual slope or full additional
cable attenuation with H. The initial versioned Belden 1694A preset uses pinned
manufacturer attenuation, explicit additional length and ft/m, without extrapolation.
VF-only echo presets remain separate. Measurements and provenance stay unchanged.
The existing Y = H_effective (X + E) model preserves post-EQ RxMER; reference RX
uses effective H, while pilot RX estimates received pilots. Cable distance does
not introduce a receiver noise floor or automatically worsen BER. No physical
cable delay/convolution or calibrated RF power is implied. The versioned optional
WB `channel_model` section defaults to Off when absent. See the
[model, limits and evidence](development/features/channel-tilt/design.md) and
[user guide](user/impairments/additional-loss.md).

UI-RF-01 implements the Channel Simulator amplifier/attenuator and inline TP
layout. Click the right-pointing amplifier triangle or variable-resistor/diagonal-
arrow attenuator to edit a draft. Apply accepts; X/Close cancel. Defaults are0 dB
gain,0 dB attenuation and Tilt Off. TP circles before/after amplification open the
shared SA/QAM instrument chooser; output instruments follow Common Path Attenuator (CPA) attenuation.
UI-RF-02 expands this diagram with the available application client area, using
24 px dialog edge insets and32 px balanced inner drawing margins. Matching blocks
retain readable dimensions; remaining width is distributed evenly among the six
connector gaps. Matching RxMER/DS-estimate branches stay centered over/below the
model, with equal vertical clearances. Larger/maximized windows provide more
spacing rather than centering a fixed narrow canvas.
No dedicated SA block or branch exists. All three channel taps use one QAM window;
output selectors expose actual raw RX cells, while input taps use retained TX
physical bins. Post-amplifier data uses the exact core scalar gain at that boundary.

The Rust PowerConditioning model owns scalar gain/attenuation, optional TX DATA
reference in dBmV/6 MHz, alternative amplifier output target and optional independent
receiver noise density after attenuation. Calibration is opt-in; legacy normalized
runs remain relative. The reference maps unit-power DATA FFT cells through the
unitary204.8 MS/s waveform convention to rms mV², without a hidden factor of two.
Imported H remains a relative multiplier. RxMER remains post-EQ residual; an
explicit independent receiver floor is additional, never automatically enabled.
Noise/response playback preserves both settings and correct LLR variances. Gain,
cable loss and attenuation affect actual synthesized RX samples and decoding.

Time-domain channel and completed-run TX/RX SA routes calculate Channel Power by
integrating full-resolution linear PSD over a fixed occupied channel band. Actual
pilot energy, internal gaps and noise remain included. Total dBmV, band-average
6 MHz equivalent level and bandwidth are separate; zoom never changes integration.
Relative runs report normalized power. Calibrated channel output warns outside
the user's±12 dBmV/6 MHz band-average operating target without fabricating FEC
failure. See the [implementation and model assumptions](development/features/rf-power-budget/design.md#ui-rf-01-implemented-scalar-conditioning-and-channel-instruments).

RF-POWER-01 remains partial: detailed RF-port loading/capability, per-subband
limits, ADC overload/clipping, receiver AGC and wideband composite input are not
modeled. The proposed60 dBmV is not treated as a universal normative CMTS ceiling.

UI layouts and diagrams follow visual symmetry, centerline alignment and a shared
spacing scale. Adjacent visible block/icon edges have equal clearance; matching
branch blocks have equal dimensions and connector lengths. Center labels and the
complete group, preserve balanced margins and prefer straight connections. These
rules are maintained in the coding-style skill and apply to future UI work.

UI-ORG-01 groups QML under `shell`, `theme`, `components`, `about`, `inputs`,
`workbenches` and `instruments`; Main.qml remains the embedded bootstrap.
Instrument windows/charts are grouped by spectrum, QAM, response, slicer, FEC,
messages and stages. Input directories use the corresponding Rust owner names.
Qt-facing instrument providers now live under `apps/qt/src/instruments/`, with
cohesive transmitter/receiver and shared spectrum modules. Existing root Rust
paths are retained through re-exports or the existing QObject bridges. Pinned
CXX-Qt requires those bridge files to stay together for this QML module.
UI-ORG-02 moves all ten into `src/bridge/`, with independent input models and
restoration under `src/inputs/`, accepted state/persistence under `src/workbench/`,
run admission/worker under `src/simulation/`, and filesystem/desktop interop under
`src/platform/`. Saved-project validation calls the input models directly, without
a QObject dependency. Main remains bootstrap/module wiring; native interop headers
live beside their platform adapter. This changes ownership paths, not UI behavior
or the workbench JSON contract.
File moves preserve registered QML type names, imports, embedded resources and
visible behavior. See the [organization design](development/features/qt-source-organization/design.md).

UI-INSTR-01 now supplies one channel TP QAM window and provider routing for all
three channel boundaries. The SA is shared. Dedicated TX and paired receiver QAM
workbench tools retain their existing specialized wrappers. Core
numerical algorithms stay in their existing Rust modules. RF power extensions
remain in the [power-budget plan](development/features/rf-power-budget/design.md).

UI-DOC-02 adds About → Terminology linking the maintained
`docs/user/terminology.md` on the repository's GitHub master branch. The menu emits
an explicit URL and the desktop shell opens the system browser; this optional
online documentation does not affect offline simulation or bundled licenses.
The online page requires the documentation to be pushed before it is available.
UI-TX-06 labels Source / PRBS-TX and Source Bits · PRBS-TX to identify the owning
input block without implying that file-based sources are always PRBS.

UI-TX-05 identifies mixed TX QAM roles with green Data squares, red NCP circles
and a visible legend; filler and unavailable classifications remain separate.
The mapped TX probe retains a bounded source-role vector from actual PackedCell
variants, never from I/Q values. The Qt Rust adapter transports the first aligned
interleaved snapshot's roles through the existing SymbolInterleaver; unavailable
history remains unknown. Role-stratified reduction preserves rare controls within
the shared2048-point cap. These TX-only diagnostics never enter receiver inputs.
Theme colors and marker shapes apply to future mixed-role instruments. See the
[role-color design](development/features/transmitter-workbench/design.md#ui-tx-05-qam-symbol-role-colors-and-legend).

UI-TX-04 presents OCD/DPD messages through a shared folding TLV tree grid with
Field/Value/Type/Length/Offset columns, keyboard expansion and raw-byte inspection.
Rust interprets only the existing fixed descriptor subset; the bundled
`resources/registry/plc-descriptor-tlvs.json` supplies display metadata traced to
that implementation. QML owns visible-row projection, folding and selection,
never binary interpretation. Child fields retain absolute packet-byte ranges;
unknown/reserved values stay raw and marked unsupported. This is a bounded JSON
tree grid, not a native QAbstractItemModel TreeView. Reuse the component for
compatible structured inspectors. See the
[tree-grid design](development/features/transmitter-workbench/design.md#ui-tx-04-folding-descriptor-tree-grid).

UI-TX-03 opens a Transmitter workbench matching Receiver stage/test-point
navigation, with bounded source/LDPC/bit/cell pages, QAM and waveform inspection.
The shared SA is attached directly below TX Output, before channel impairments;
IFFT, prefix and window taps are selectable inside it. Without a completed run,
TX Output retains the existing transmit-only preview; other inspectors read main
Start results. Opening the workbench does not automatically run a preview.
Separate PLC/NCP/pilot paths feed frame assembly. The message inspector
reassembles observed PLC bytes through existing decoders and shows actual OCD,
Profile A DPD and NCP DPD headers, expandable TLVs and original packet bytes.
Completed TX PLC frames are distinguished from incomplete scheduled frames using
actual transmitted-sample totals; capture limits and incomplete packets remain
visible. Imported modulation-profile PNM bytes are never presented as MAC packets.
The adapter retains64 PLC frames, at most32 displayed packets, one snapshot per
bit/cell/bin boundary and32768 contiguous output samples. Per-stage phase filters
admit acquisition PLC while keeping other TX/RX snapshots observation-only.
The coding view exposes LDPC, not a separately retained BCH word; mixed mapped
Data/NCP cells are explicitly grouped. NCP/pilot physical-bin inspection is shared.
The redundant small MERLIN eyebrow is removed from the main header.
See the [design](development/features/transmitter-workbench/design.md).

**WB-RULE-04:** accepted OCD, DPD and PRBS-TX inputs jointly configure Transmitter.
When they are green and form a supported Rust transmitter request, Transmitter
is green without a separate acceptance dialog. Missing inputs are yellow; actual
unsupported/inconsistent accepted input or generation failure is red with a reason.
Applied input changes invalidate the old spectrum and cancel any stale preview.
Rejected drafts/Cancel preserve accepted state. Captured data profile IDs0..15
can drive independent previews/runs; their assignments become the fresh run's
bootstrap Profile A (wire ID0), preserving captured identity in provenance. Full-loop Start still requires all
blocks plus dispatch readiness; SA generation is independent of that gate.

SpectrumAnalyzerDialog and SpectrumPlot are shared presentation components, not
TX-specific analysis implementations. A Rust adapter supplies the selected tap,
sample identity, progress and bounded viewport. The first connected tap is
`tx.overlap_add.samples`, before channel impairments. Future channel/RX sample
taps reuse the same estimator and dialog while preserving their reference plane.
The analyzer accepts actual contiguous complex I/Q samples, never bit arrays or
already-transformed bins mislabeled as a time waveform.

The signal remains **complex baseband**: no RF upconversion occurs. Default axes
show frequency offset from DC (−102.4 to +102.375 MHz for the 8192-point case).
Optional capture-frequency mapping adds the captured physical-bin origin to
axis labels only; samples and PSD remain identical. Power is normalized PSD in
dB re one sample unit squared/Hz, with no dBm/dBmV calibration assumed.

The Rust transmit-only preview reuses accepted source configuration, captured
carrier layout/loading and existing BCH/LDPC/NCP/PLC/pilot/IFFT/CP/window/overlap
APIs. Idle priming fills interleaver history before consuming source bits. The
bounded estimator averages 32 periodic-Hann FFT windows with 50% overlap in
linear power, retaining full PSD/samples in Rust and returning at most 2048
min/max display buckets. Zero power stays explicit; a plotting floor does not
change retained density. Hover, keyboard selection, zoom and full-span rendering
are presentation operations. The SA lower panel shows frozen symbol timing, actual transmit-window
coefficients and a nominal cable echo budget. Sample bookkeeping and selection
text remain omitted from the panel. The **Run simulation** button, when enabled
for the transmit-only preview fallback, reruns that preview and its FFT;
it does not dispatch the full channel/receiver loop. Diagnostics use its tooltip.
At most one cancellable worker runs; stale results
cannot replace results for newer accepted input.

See the [SA design](development/features/transmitter-spectrum/design.md) and
[user guide](user/spectrum-analyzer.md). Main Start owns the full loop and RX
instruments; this separate TX preview does not establish RF compliance or file recovery.

### PRBS-TX source configuration

PRBS-TX now has its own workbench editor. Its block name is retained, but the dialog
title is **Transmit source (PRBS-TX)** because the existing Rust source API also
accepts files. Use a separate Parameter/Value tree dialog with field descriptions,
mode-dependent controls and Apply/Cancel, consistent with OCD and DPD.

| User option | Configuration and validation |
| --- | --- |
| Source mode | Continuous PRBS15 (initial draft default), or File (text/binary). The user must Apply before the block becomes ready. |
| PRBS15 seed | Decimal integer 1–32767; initial value 32767 (`0x7fff`), matching the runner. Show the fixed polynomial and period as read-only descriptions. |
| File | Open through the OS-native picker; retain exact original bytes in Rust. No text conversion or extension-based rejection; text and binary use the same source path. |
| File summary | Read-only filename, original byte/bit count and original-byte SHA-256, supplied by Rust. Limit original bytes to 16 MiB. |
| Append PRBS15 | File mode only, off initially. Disabled means send the original file once, then idle/filler. |
| Total meaningful bits | Enabled only with append; includes original file plus fill. Require original bits ≤ target ≤ 134217728. Do not auto-size from duration/profile or silently add fill. |
| Fill seed | Enabled only with append; independent seed 1–32767, initially 32767. Fill starts after the original file and continues without codeword reseeding. |
| Fill summary | Rust-derived original bits, target bits and additional PRBS bits. Keep final-codeword padding separate. |

Allow empty files as the core does: no-fill sends no payload; a positive target
produces finite fill. Equal target/original length appends nothing; partial-byte
fill is supported. Duration belongs to run configuration, not a source bit-count
estimate. Finishing a run does not guarantee all file bits were generated/recovered.
Other PRBS polynomials, repeat-file, editable payload text and automatic targets
are outside this first editor; do not expose unimplemented choices.

Rust owns bounded file reads, immutable bytes, provenance, validation and the
accepted source snapshot. Reuse `Prbs15::new`, `FilePayload::new`, `PrbsFill` and
`PayloadSourceConfig`; do not duplicate generators or fill math in QML. Loading
and editing stage a draft; Apply validates and commits, Cancel preserves the
accepted source. Rejected drafts report locally without changing accepted readiness.
PRBS-TX is yellow until a valid source is explicitly accepted, then green; an
accepted source that becomes invalid is red. Clearing a file requires replacing
it with a valid source before Apply. Independent PRBS-RX truth uses the accepted
source/seed, with its own state; source acceptance alone does not complete PRBS-RX
or enable Start. Edits affect the next frozen job, never an active stream.

Accepted source settings feed the frozen request and explicit WB persistence. The
optional `source-input` feature exposes existing source validation without FEC or
runner dependencies. `FilePayload::with_fill` shares immutable bytes while
revalidating a policy, avoiding payload copies during edits. See the
[source configuration plan](development/features/visual-workspace/design.md#ui-02-prbs-tx-source-editor)
and [source UI guide](user/source-input.md).

### Downstream profile input dialog

File and folder selection must use the operating system's native picker, without
a Qt-rendered fallback or custom file explorer. Qt platform integration may invoke
the OS picker. PRBS-TX and DPD now use a shared GTK native chooser on Ubuntu,
in an isolated presentation-only process of the same executable, before Qt
initialization. It exchanges only a URI/cancellation/error and prevents GTK and
Qt from consuming each other's UI events. An explicit error is returned if
GTK/display initialization is unavailable. No
Qt file browser is instantiated; application-owned editors remain Qt dialogs.

The Downstream Profile Descriptor (DPD) input block opens its own Qt tree dialog
with Open File, Load demo, detected representation, File/Channel information,
Profiles with numbered Range rows, profile selection and staged removal.
These are downstream OFDM modulation-profile PNM captures or supported decoded
JSON, not raw DPD MAC messages. Supported captures are type 10, format 1.0 with
range schemes; skip/unsupported encodings reject explicitly. Tree queries retain
compact schemes in Rust and show at most 256 scheme rows for the selected profile.

`input_format` is a separate safe Rust content detector. Extensions are picker
hints; JSON text (including optional UTF-8 BOM), recognized binary PNM, unsupported
text/binary and empty content are distinguished before decoding. Files are bounded
to 16 MiB. The Qt Rust adapter retains original bytes/hashes and draft/accepted
input; QML presents bounded rows. Binary conversion uses pnm 0.3.6 fetched from
PNM Viewer Toolkit at Git revision 5fbd5561f3e06b73486546b5ffa520279080a5eb.
Sources live in ignored Cargo cache, not a vendored viewer copy. Runtime needs no
viewer checkout, Python, network or decoder subprocess. Preserve dependency notices.

The core's `scenario::pnm_json::import_profiles` shares validation with the CLI's
three-capture importer. The optional `pnm-input` feature enables it without FEC or
simulation in Qt. Apply checks spacing and carrier span against accepted OCD
settings before committing a selection; Cancel and rejected loads preserve input.
WB persistence and simulation-job integration are implemented; bandwidth/spacing
editing and skip-scheme import remain outside this editor. See the [profile input guide](user/measurement-inputs/modulation-profile.md)
and [development setup](development/setup.md).

### Reuse of the PNM Viewer Qt framework

Normal local Qt `build`/`run` helpers use optimized release binaries, as do
packaging and Python wheels. Explicit `build-debug`/`run-debug` preserve fast
unoptimized development iteration. Diagnostic logging remains opt-in and
independent of compiler profile; normal logging retains warnings/errors.
The `check` helper keeps its development-profile validation workflow. See the
[desktop build guide](development/qt.md).

UI-WB-OPEN-01 gives main File → Open / Ctrl+O a distinct OS-native chooser mode
with an active `*.merlin.json` filter. The examples-root resolver and directory
navigation remain; individual measurement/source input pickers are unaffected.
Filename filtering is only presentation: Rust still validates complete WB
contents before adoption and preserves the current session on Cancel/error.
See the [workbench-file design](development/features/qt-closed-loop/design.md#ui-wb-open-01-main-open-file-filter).

UI-WB-OPEN-02 runs the main native chooser, bounded read, decoding, embedded
original validation and editor/run preparation in one background worker. Qt
polls a bounded completion channel and shows the current phase, an animated
activity bar, Time since Open and Cancel without waiting. The timer includes
file selection; the activity bar does not claim measured completion or time
remaining (UI-WB-OPEN-03). Success publishes the complete accepted snapshot and
transfers already prepared editor/request models on the GUI thread; it does not repeat capture
imports or start a run. Errors and cooperative cancellation preserve the current
session and Save destination; one worker stays tracked until completion.
Update recovery uses the same loader. See the
[responsive Open design](development/features/responsive-workbench-open/design.md).

UI-RXMER-SELECT-01 retains all imported originals while optional SHA-256
identities select Single Shot, Average or ordered Replay inputs. Missing
selection retains legacy all-capture behavior; empty, unknown and duplicate
explicit selections are rejected. Rust applies selection consistently to
published inputs, effective preview and accepted run requests. The bounded
selection query uses accepted CWB duration per profile and core OCD timing to
report replay capacity `1 + floor((complete_symbols - 1) / interval_symbols)`.
The measurement option/viewport IPC limit is 24 KiB for up to 256 identities;
scalar OCD/power/run documents retain 4096-byte limits. Selected identities
persist in the measurement options in WB JSON, with original bytes unchanged.
See the [selection design](development/features/rxmer-selection/design.md).

UI-RXMER-ANIM-01 adapts PNM Viewer's capture-preview transport with preserved
GPL attribution. QML owns Play/Pause/step/slider and 1/2/4/8 display frames/s;
Rust resolves the selected originals and returns at most 1024 chart buckets
for one frame, with source index/time and selected position/count. Each tick
avoids capture decoding and full playback-plan preparation. This original-frame
preview preserves the frequency viewport and never applies configuration or
starts a simulation. Its wall-clock speed is separate from synthetic OFDM
playback timing. Editing, closing or reaching the last frame pauses animation.

UI-DPD-VIEW-02 replaces the initial single-trace profile plot with the PNM
Viewer modulation layout: independent multi-profile overlays, dB/bit-loading
selection, carrier filters, channel/MAC chips, gross Profile BW and role lanes.
Rust owns checked coordinates, shared ideal Shannon math, data-only gross-rate
arithmetic and bounded extrema; QML owns transient filters and rendering.
Missing/filtered carriers break trace joins. Special roles retain separate lanes.
Viewing is independent of Apply/readiness and WB persistence; originals retain
filenames and provenance while the editor hides filenames. Each popup keeps
X/Close and the plot supports mouse navigation and Full Span. See the
[design](development/features/pnm-measurement-inputs/profile-view.md) and
[user guide](user/measurement-inputs/modulation-profile.md).

Reuse suitable Qt shell, theme, dialog, navigation, Rust/Qt adapter and build
patterns from `/home/dev01/Projects/PNM-Viewer-Toolkit`. The user authorized
read-only reference inspection; all modifications stay in MERLIN. Select and
adapt components to this simulator rather than copying the viewer's product
workflows. MERLIN remains standalone, with no runtime dependency on the viewer
checkout. All simulation, DSP, modulation and FEC remain in MERLIN's Rust engine.

The user accepted GPL reuse: the Qt application incorporating viewer desktop/UI
code uses GPL-3.0-only, preserving source copyrights, licenses and applicable
NOTICE content. Independent signal-processing libraries remain Apache-2.0. Check the
actual license of each selected file and retain third-party terms. Record source
paths, revision and adaptations in the UI task design when copying occurs.
The shell now adapts the viewer bootstrap, palette and icon wrapper; exact source
hashes and adaptations are recorded in the UI design. The coding-style and
public-API skills carry these reuse and ownership rules into implementation.

### UI iteration and VM handoff

The VM is the rendering and interactive review target. Preserve compiled Qt and
Rust dependency artifacts between iterations and leave the current Qt executable
ready to run. Rebuild changed modules and relink as needed; do not routinely clean
build outputs. UI-only work uses Qt-focused checks, with non-Qt compilation/tests
only when affected shared code or integration makes them necessary. Required
release/push gates remain intact.

The proposed [build/test cache](development/features/build-cache/design.md)
adds SHA-256 task fingerprints and affected-test selection above Cargo. It is
planning-only; see BUILD-01 through BUILD-04 in the main checklist.

Every workbench UI or graphic change includes a corresponding user-guide update.
Use dark-theme screenshots and graphics in those guides, capturing implemented
UI changes on the VM and clearly labeling planned mockups.

### Workbench run controls

Persistent **Start (▶), Stop (■), and Pause/Resume (⏸/▶)** icons sit in the
lower-right corner, with accessible names, tooltips and state-dependent availability.
Start validates and freezes settings before beginning a bounded Rust job. Stop
requests cooperative cancellation and retains completed results with explicit
partial status. Pause must suspend advancement while retaining source, RNG,
interleaver, receiver and observation state; Resume continues the same run.
Opening or editing a dialog must not change active simulation state. Settings
changes apply to the next run.

Pause is a new Rust lifecycle requirement, not an existing `run_channel` feature.
Before implementing it, define safe pause boundaries, whether paused time counts
toward the host-runtime budget, Stop/close behavior while paused, and worker cleanup.
Do not implement Pause by restarting a run or merely freezing the display.
Jobs run away from the UI thread. Rust owns all DSP, modulation, FEC, validation,
numerical calculations and bounded results; Qt owns presentation and transient
interaction. Existing Rust APIs and test points remain the source of truth.

The [UI checklist](development/ui-checklist.md) tracks the detailed work beneath
UI-01 through UI-04 in the [main checklist](development/checklist.md).
The [task design](development/features/visual-workspace/design.md) and
[earlier workspace plan](development/features/visual-workspace/plan.md) record
interaction, ownership and validation. UI-01 remains partially implemented; this shell does not complete the milestone.

### Updated workspace diagram guide

The updated user sketch adds a separate **OFDM transmission settings** panel.
Use this as the workspace navigation guide, not a final screen layout. Each
processing block and analyzer opens its own focused panel; the settings panel
configures the transmitter and the matching receiver bootstrap for the next run.

```mermaid
flowchart LR
    SOURCE["Source: PRBS15 / text or binary file"] --> TX["Transmitter"]
    SETTINGS["OFDM transmission settings<br/>Cyclic prefix · roll-off/windowing<br/>Time-interleaver depth · pilot configuration/scaling"] --> TX
    PROFILE["Modulation-profile PNM input"] --> TX
    TX --> CH["Channel simulator"] --> RX["Receiver"]
    MER["RxMER PNM input(s)"] --> CH
    CE["DS channel-estimation PNM input"] -->|"Channel response H"| CH
    CE -. "Supplied-H mode: reference correction" .-> RX
    SETTINGS -. "Matching RX bootstrap settings" .-> RX
    RX --> LDPC["LDPC statistics"]
    RX --> BER["BER / recovered-data comparison"]
    SOURCE -. "Independent expected-data reference" .-> BER
    RX --> POST["QAM analyzer: post-CH-EST-RX correction"]
    RX --> PRE["QAM analyzer: pre-CH-EST-RX correction"]
    RX --> PILOTS["QAM analyzer: continuous / scattered pilots"]
    RX --> PLC["QAM analyzer: PLC data / preamble"]
```

The settings panel explains the purpose and trade-offs of each control:

| Setting | Purpose and ownership |
| --- | --- |
| Cyclic prefix | Protection against channel delay spread, with a capacity cost; applied by WAVE-TX and removed by matching WAVE-RX. |
| Roll-off/windowing | Symbol-edge shaping and spectral behavior; matched waveform geometry owns taper and overlap processing. |
| Time-interleaver depth | Spreads coded cells across symbols, affecting burst robustness, latency and startup/drain state. |
| Pilot configuration/scaling | Applicable continuous/scattered pilot references and boost; TX generation and CH-EST-RX must use consistent PHY settings. |

Use supported PHY choices and existing Rust validation; do not expose arbitrary
pilot patterns or a second Qt implementation of the settings math. Settings are
fixed during a run. A change is validated before starting a new run, preserving
active source, interleaver and receiver state. The settings panel links to the
[transmission guide](user/transmission.md), [waveform components](user/waveform/README.md)
and [pilot design](user/pilot-signals.md).

Receiver analyzer arrows represent read-only test points, including the raw FFT
before correction and matched points after correction. Supplied H and independently
pilot-estimated H-hat remain separate receiver modes. The source comparison path
is verification, not receiver feedback or PRBS reseeding. PNM labels describe input
roles; current files enter through decoded JSON or validated typed scenarios.

The Rust API and CLI implement **PRBS15 or user-supplied text/binary file bytes**.
The planned Qt Source block will open editable settings and an OS file picker.
The source preserves exact bytes, MSB-first file bit order, original length and
independent recovered-data comparison; text needs no encoding conversion.

Before a run, Rust validates the16MiB input bound and optional explicit total-bit
target/PRBS seed. Appending PRBS15 is opt-in and finite; never silently append,
repeat or truncate input. The future source panel shows original size, target
and remaining fill count from the same Rust contract. A minimum/alignment or
automatic target based on coding/profile/duration remains **TODO for later
definition**; fixed-size FEC blocks do not require fixed-size user files.

Keep original-file bits, appended PRBS15 bits and final-codeword padding distinct.
Recovered-file output contains only the original bytes; compare file and PRBS15
segments separately, excluding coding padding from BER. Without optional fill,
send once and then use idle/filler. Duration completion does not imply full file
completion: explicit generated/compared flags and recovered-prefix metadata
retain partial results. Source/CLI integration now precedes the planned Qt
control; continuous PRBS behavior is unchanged. SRC-04..06 track this extension.
See the [file source design](user/simulation/file-source/README.md) and
[implementation record](development/features/file-payload-source/implementation.md).

The file panel associates inputs with their processing stages. Measurement files
continue through the current decoded-PNM JSON importer; listing multiple RxMER
files does not itself activate playback; explicit validated epoch schedules
control temporal transitions. Supplied CE provides H to the
channel and1/H to reference RX correction. The expected-data connection to BER is
a verification path, not receiver feedback into the source.

## PNM measurement input direction

Use the current `pnm-decode` JSON output from PNM Viewer Toolkit as the starting
external measurement format for RxMER, downstream channel estimates and applied
modulation profiles. A future Rust import adapter will translate those decoded
documents into this repository's typed scenario inputs. Rust applications may
also supply the same validated scenario types directly through the public API.
Binary PNM decoding remains outside the simulation engine; the engine does not
launch `pnm-decode` or depend on the viewer application.

The `import-pnm` CLI and shared `scenario::pnm_json` API implement this direction
for the current range-scheme decoder output. The waveform command accepts the
converted scenario with explicit source/profile/epoch selection. Raw decoder
documents are inputs to import-pnm, not direct channel requests. Ideal/reference
runs require no PNM captures. Explicit temporal epochs and frozen mixed ProfileA
loading now use this same importer/engine; dynamic profile activation remains NCP-03.

Start the importer against representative current decoder output. Formal schema
versioning, a supported-toolkit-version table and compatibility enforcement are
deferred under INPUT-02 in the [checklist](development/checklist.md). Existing
scenario validation for grids, missing readings, timestamps and provenance
continues to apply.

The [PNM input guide](user/measurement-inputs/README.md) links current decoder
fixtures and Rust-generated RxMER alternatives. The sanitized originals are kept
locally under ignored development storage. The three demo captures share a spatial
grid but have different timestamps; they are not contemporaneous observations.
The importer and current fixed-profile waveform integration now consume these
fixtures; their different timestamps still require explicit tolerance.

The [LTE ingress approximation](user/measurement-inputs/lte-ingress.md) is a shared
Rust frequency-domain residual-power helper under `signal_processing::noise`.
Fixture preparation checks exact spatial alignment and emits a synthetic RxMER
document plus settings/alignment sidecar. It does not synthesize an LTE waveform.
The implemented measurement-driven mode starts noiseless and uses RxMER as its total
residual-error budget; no extra thermal/system floor is enabled. Derived LTE MER
already includes baseline plus added interference and must be applied once, not
combined with a second baseline or LTE injection in the waveform model.

See the [impairment component guides](user/impairments/README.md). Sample AWGN owns
an independent contiguous seeded stream. The measured channel synthesizes Gaussian
bin residuals plus supplied H through its own IFFT/CP/window/overlap stream. It is
a synchronized symbol-diagonal model, not arbitrary continuous-time convolution.
The receiver uses explicit known H; blind estimation remains open. Variances follow
the same inverse cell interleaver as received I/Q before per-cell LLR calculation.
Existing channel input/output and RX FFT/equalized probes expose the actual stages.
The protected LTE sweep starts narrow and rejects overlap with the PLC6MHz neighborhood.
The same Rust fixture generator offers a separate700–727MHz data-only RxMER
stress cap to observe actual post-FEC errors. It preserves pilots, PLC, nulls and
source identity; it is not modeled as a single LTE carrier. See the
[experiment commands and results](user/impairments/experiments.md).
An offline Rust fixture exporter preserves the original RxMER PNM header and
encodes exactly representable scenario readings for external viewing. It does
not change the decoded-JSON production input contract.

## Paired pre/post equalization constellation

**CONST-01..05** add bounded paired observations using the supplied-response
path before independent pilot estimation. Standard DS channel-estimation PNM
supplies H; the channel applies H and RX applies1/H. No correction-coefficient
input mode is inferred or added.

`simulation/channel/constellation/` separates selection, typed results and bounded
collection. The collector pairs `ReceivedSymbol.bins` and `equalized` at the same
physical bin/time, then carries diagnostic coordinates through an independent
instance of the existing receive interleaver. Actual received NCP payload ranks
select eligible points and the existing source window excludes acquisition,
filler and outside-observation cells. Failed payload codewords remain eligible.
No transmitter truth is passed into classification, equalization or decoding.

The default target is8192 total QAM pairs for a supported profile/carrier interval.
A bounded NI*M reorder map delays emission until receive-time order is known;
neither inverse history nor result storage grows with run duration. Collection
stops at its requested cap, with explicit partial/unavailable status when too few
points exist. Confirmed pairs survive cancellation and terminal stage/probe
failure. FEC results and RNG streams remain independent of capture settings.

`ChannelRun` exposes borrowed and owned partial capture access. `run_channel`
returns the same typed result through optional `constellation` JSON, reusing the
existing channel command and16MiB output policy. Default requests omit the new
result/settings field when collection is disabled. Existing raw/equalized generic
probe stages remain separate and available. No new noise/QAM/equalizer math or
Qt UI is introduced. See the [accepted plan](development/features/paired-constellation/plan.md),
[implementation](development/features/paired-constellation/implementation.md) and
[user guide](user/impairments/paired-constellation.md). MODEL-05 now adds the supported pilot-estimated receiver below; bounded
bootstrap acquisition is implemented by WAVE-RX-05. Arbitrary blind RF acquisition
is outside the minimum model.

## Receiver Channel Estimation (CH-EST-RX) and pilot/PLC analyzers

**MCE (CH-EST-RX-07..12)** extends this foundation with received-only reliability,
a static continuous sample-filter channel, integer-delay echoes and explicit
bounded reconstruction of CM response coefficients. The receiver derives correction and reliability from received
samples and declared bootstrap inputs, without applied H, configured tap delays or
exact simulator noise powers. Channel truth remains a separate diagnostic input.
Existing diagonal/calibrated modes remain explicit reference baselines.
MERLIN Channel Estimation (MCE) is disabled by default and configured in
Receiver Configuration through one three-choice Receiver mode dropdown:
Supplied CM channel estimate, Pilot-based channel estimation, and Pilot-based
channel and noise estimation (MCE). Both estimated modes use the existing pilot
channel estimator; MCE adds received-only noise reliability. The UI exposes MCE
averaging only for MCE, with contextual explanations and tooltips. See the
[mode comparison](user/receiver-estimation/modes.md). Its setting is independent of channel impairments;
accepted changes apply to the next run.
In the default reference workflow, supplied H impairs the channel in both
MCE states: Off uses reference 1/H correction; On uses independently estimated
1/H-hat. MCE changes receiver correction, not the applied response or seeded noise.
Existing saved receiver-mode choices remain compatible. `merlin_estimated` uses
receiver-generated pilots and temporal continuous-pilot differences, never exact
channel/noise inputs. A small MCE chip indicates enablement without moving the
centered Receiver heading. When MCE is enabled, the diagram hides the direct
supplied-H branch to Receiver, retaining the channel impairment and signal paths.
The response Display selector names MCE or legacy pilot estimates from retained
result metadata rather than relabeling old results with next-run settings.
Training/Tracking describes 128-symbol history readiness,
not certified accuracy. Apply is blocked during a run; draft Cancel/X preserves
accepted settings. Rust settings/results are shared by CLI and Qt; the optional Python adapter consumes the same Basic Rust API.

`signal_processing/channel/filter` owns static sparse delay lines or dense 8K FFT
overlap-add (4096 input samples per chunk, at most2049 coefficients/2048 delay samples).
CM reconstruction explicitly uses unity extension of excluded bins, causal shift,
truncation and active-bin fit limits; original captures remain unchanged. Channel
memory tail accounting is separate from TX overlap and interleaver drain. Response
epoch transitions reject for continuous filters; fractional delays remain deferred.
The response instrument compares realized H with received H-hat only after reception.
See the [design and validation record](development/features/independent-channel-estimation/design.md),
[MCE guide](user/receiver-estimation/mce.md) and [filter guide](user/impairments/waveform-filter.md).

Channel Estimation (CH-EST) names the processing function; the RX or TX suffix
identifies its direction. Receiver Channel Estimation (CH-EST-RX) estimates H
from received pilots; CH-EST-TX is reserved for transmitter-side work.

**CH-EST-RX-01..06 are implemented for the documented bootstrap scope.**
`signal_processing/receiver/channel_estimation/` owns receiver-generated pilot
references, per-pilot LS estimates, bounded128-symbol history, delay-aware complex
interpolation and bounded cyclic-prefix timing search. It accepts received samples
and independent references, never the simulated H. `receiver_mode=pilot_estimated`
uses1/H-hat; omission preserves the supplied-H baseline.

The CLI keeps known sample timing/PLC phase. A separate public acquisition operation
composes an observed CP boundary with pilot reception while retaining known
numerology and PLC phase. The newer WAVE-RX-05 search selects bootstrap numerology and total CFO;
absolute PLC-frame acquisition and arbitrary RF discovery remain separate.
Neither API claims a production CM receiver.

The first bounded raw PLC frame is corrected using the completed pilot pattern
before FEC/admission. Independent source/receiver owners continue beyond startup.
Raw noise calibration is converted through the actual applied inverse; it does
not add noise or claim a complete posterior model of estimator uncertainty.
History aging, edge/gap interpolation, near-zero rejection and prior pilot
predictions are explicit. A ready flag is not a guarantee of estimate accuracy.

`simulation/channel/protected_constellation/` owns opt-in separate continuous
pilot, scattered pilot, PLC16-QAM and BPSK-preamble pairs, including acquisition
and failed-frame observations. Targets/windows and combined retention are bounded;
current-fit pilots are labeled separately from pre-update predictions. Public
Rust access and the CLI expose identical simulation results. Existing payload
capture reports each selected cell's actual modulation order and correction mode.
`rx.channel_estimation.response` adds a dimensionless-gain test point; the final
job carries the latest bounded receiver estimate, not unlimited FFT history.

See the [subsystem user guides](user/receiver-estimation/README.md) and
[implementation/validation record](development/features/pilot-channel-estimation/implementation.md).

### Channel-input readiness (WB-RULE-05 / UI-CHANNEL-01)

Channel Simulator requires valid RxMER configuration and valid channel-response
configuration, plus supported channel settings. The response defaults to an
explicit **Flat / unity** configuration: linear H=1+0i per carrier on the accepted
DPD/OCD grid, equivalent to 0 dB gain and 0° phase. Linear H=0+0i is not a flat
channel; it suppresses the signal. This synthetic default is not a measured PNM
estimate or receiver H-hat. Unity counts as configured once the grid is valid;
a response capture is optional in this mode. Imported responses must pass the
existing exact compatibility and active-gain checks. RxMER needs accepted
compatible inputs and a valid combination/schedule before Channel Simulator is
green. Dependency changes revalidate and invalidate old results. This rule is
implemented for configuration and reused for frozen closed-loop requests. See
[the design](development/features/rxmer-workbench/design.md#ui-channel-01-response-configuration-and-channel-readiness).

### Shared Channel Estimate/RxMER viewer

Separate Channel Estimate and RxMER dialogs compose one shared tree/chart viewer
and one focused Rust input adapter. The exact pinned PNM decoder parses binary
captures; standalone MERLIN JSON import APIs preserve the existing format contracts.
Original bytes, hashes, epochs and complete arrays remain in Rust. Local OS-native
selection supports one response file or multiple RxMER files, with no Qt browser
fallback. Inputs are staged and Apply validates before replacing accepted state.

RxMER modes are single, equal-capture-weight average of linear error power,
explicit symbol-scheduled hold, and interpolation of residual power. The effective
preview uses immutable core MeasurementPlayback queries; original capture epochs
are never rewritten and frequency grids are never interpolated. Preview symbol,
explicit ordering and a configurable capture-time tolerance are visible controls.
Response charts show gain, wrapped phase, real or imaginary H; synthetic unity is
explicitly labeled. Bounded min/max charts preserve missing-data breaks.

UI-RXMER-01/02 and the editor/scheduling-preview part of UI-RXMER-03 are implemented.
The production runner now consumes this power playback on one continuous RNG/
waveform stream. Whole schedules must fit the observation window. Acquisition
uses the initial powers and completion/drain retain the terminal modeled state.
These channel controls do not change the TX-only SA; the RX SA includes them. See the
[design](development/features/rxmer-workbench/design.md),
[validation](development/features/rxmer-workbench/validation.md) and
[user guide](user/channel-measurements.md).

### Receiver QAM view with RxMER

The shared SA remains tap-specific: TX output precedes impairments and is
unchanged by RxMER; channel/RX input spectra may show added residual power even
when the occupied signal shape changes little. The primary QAM analyzer samples
RX after equalization and before decisions, paired with raw RX FFT samples using
the existing bounded Rust constellation collector. Display must preserve matching
carrier/time, correction mode and modulation; pilots/PLC/mixed QAM orders are not
silently pooled. Symbol-window selection will show RxMER playback changes over
time. The bounded retained UI queries now implement these connections. See
[UI-QAM-01](development/features/rxmer-workbench/design.md#ui-qam-01-receiver-constellation-and-shared-sa-reference-planes).

### Receiver Workbench

Clicking Receiver opens a dedicated workbench with processing stages,
selectable test points and shared diagnostic instruments. The QAM analyzer has
independent **Signal: Data Profile / NCP / PLC / Continuous Pilots / Scattered
Pilots** and **View: Raw / Equalized / Paired** selectors. Available received
profiles and compatible modulation groups can be selected with inclusive
physical-bin and receive-symbol bounds over the retained capture. Empty ranges
clear the plot; unretained history is not synthesized.
PLC data and preamble remain separate. Optional hard slicing is a diagnostic
branch; soft demapping consumes received I/Q before hard decisions. Control and
payload decode paths are distinct; PRBS comparison belongs to payload recovery.

The shared SA has an independent **Test Point** selector, initially RX Input
(`channel.output`), then eligible framed/useful time-sample taps. An FFT-bin power
view is explicitly frequency-domain with appropriate normalization, not another
FFT of bins or an automatically calibrated PSD. QAM signal selection does not
silently filter the SA's full-channel sample tap. Channel-response H/echo remains
a separate simulation view. Instrument selection is independent of simulation state. Charts query
immutable bounded Rust results from one complete or partial closed-loop run.

The navigation and selectors are implemented in separate receiver/QAM dialogs.
Stage/instrument chips use shared rounded styling, equal widths within each row
and aligned test-point/annotation slots; pin accents are decorative, not status.
The Rust presentation adapter maps taps to registered IDs and reports unavailable
observations. Receiver SA generation stays disabled locally: main Start runs
the full loop and the instrument queries retained results; no TX fallback is used.
Existing Rust payload/PLC collectors have explicit support limits. Matched NCP classification now uses CRC-accepted received consumed-cell counts
and the shared inverse coordinate transport. Payload/NCP/PLC/pilot QAM, RX time
spectra and FFT-bin power consume real retained observations. Receiver becomes green after full-request
validation; PRBS-RX requires explicit independent comparison acceptance. Unsupported
views show a reason; no generated clouds, mislabeled TX data or TX-truth profile
assignment. See [receiver workbench design](development/features/receiver-workbench/design.md)
and [user guide](user/receiver-workbench.md).

### Qt closed-loop execution and workbench files

The main diagram places RxMER to the left of DS Channel Estimate. Both feed the
channel; a dashed DS Channel Estimate → Receiver arrow labels supplied-H reference
correction. The channel consumes H, while reference RX uses 1/H. Pilot-estimated
H-hat remains independent and does not consume the supplied-response arrow.

UI-RUN-01 freezes accepted source, DPD loading, timing, original measurements,
playback and receiver options in Rust. The worker reuses ChannelRunSettings and
the channel job API; QML performs no DSP or FEC. Default UI observation is 3 ms,
limited to 1–1000 ms, 1–100 decoder iterations and a finite active host budget.
These are application limits, not DOCSIS defaults. Supplied-H and pilot-estimated
correction use the existing known-boundary bootstrap; no RF acquisition is claimed.

Pause waits between completed OFDM steps, retaining source, RNG, overlap,
interleaver and receiver state; paused time is excluded from the host budget. Stop
also works while paused. Complete/cancelled/runtime-exceeded/failed results preserve
actual completed comparisons. Changing accepted simulation settings invalidates old
results and requests cancellation; busy Open/Save are disabled. Worker shutdown
requests cancellation without blocking the UI thread.

Observation-only phase filtering prevents acquisition from exhausting the sample
retention budget. Retain finite contiguous RX input/framed snapshots, one useful
symbol and one raw FFT; never concatenate useful samples across prefix gaps.
Payload/PLC charts use existing matched collectors and shared raw/corrected axes.
NCP I/Q collection and available profile/modulation/bin/receive-symbol selectors
are implemented over bounded retained windows. Payload captures retain at most
65,536 pairs, NCP 8,192 pairs and each protected stream 8,192 pairs; at most 2,048
selected pairs are rendered. Per-stage probe event caps reserve the global budget
for four H-hat snapshots and one actual payload LDPC-input window.

UI-CH-EST-RX-01 separates supplied H from actual receiver H-hat and shows gain,
phase, real/imaginary response and a masked band-limited impulse diagnostic.
UI-SLICER-01 uses the existing Rust slicer on selected cells and exposes actual
post-descrambling NCP/payload LLRs plus coding-order LDPC input. Protected streams
show physical decisions without inventing a per-cell LLR boundary. UI-FEC-01 adds
a prominent FEC Statistics shortcut and actual branch counters, iteration
distribution and bounded completed-word records. See the
[receiver instrument design](development/features/receiver-instruments/design.md).
FEC/PRBS-RX show actual job totals, and FFT-bin power has separate units from PSD.

See [design](development/features/qt-closed-loop/design.md),
[validation](development/features/qt-closed-loop/validation.md) and
[user guide](user/closed-loop-workbench.md).

## Temporal measurements and applied profiles

`simulation/channel/timeline.rs` owns explicit piecewise measurement epochs.
Import resolves strictly increasing original timestamps to physical observation
symbol boundaries with ceiling division; direct Rust requests can specify an
explicit symbol schedule. All frames retain identity/grid/time/provenance and
pass coverage checks before work. The channel replaces response/residual plans
without resetting source, noise, interleaver or overlap state. Supplied-H RX updates
its reference; estimated RX continues to observe pilots without receiving H.
No layout or profile-activation change is allowed mid-run.

`transmission/layout/bit_loading.rs` implements the normative Ci=P^-1(Bi) frequency
mapping. `modulation/interleaving/mixed.rs` owns non-shortened24-column mixed bit
processing; the packer groups coded bits using per-source-rank loading and explicit
last-cell padding. Actual PLC DPDs deliver the frozen applied ProfileA to RX.
The receiver demaps variable-width soft cells and removes only final-cell padding
before actual LDPC/BCH recovery. All ten uniform data orders and mixed0,4,6..14-bit
profiles are supported. Shortened words use the separate explicit API/reference
path described above. This fixed-duration path rejects dynamic activation; the opt-in NCP-03 stream
path is separate. Uniform modes always select their own rules rather than substituting mixed processing.

The temporal model is piecewise-constant variance with conditionally independent
Gaussian residuals. RxMER alone supplies no burst correlation or event timing.
Power/lag tests and an identical-epoch RNG/FEC invariance test validate this model;
a different future burst process requires separate explicit inputs and evidence.
See [measurement playback](user/impairments/measurement-timeline.md) and
[applied-profile design](user/payload-transport/applied-profiles.md).

## Modules and stage boundaries

**Each distinct algorithm stage has a focused module and a directly testable typed
input/output boundary.** Stateful stages own explicit structs; stateless stages
can use functions. Do not create empty object wrappers or files solely to mimic
classes. Orchestrators connect components without hiding their intermediate data
or implementing a second copy of their mathematics.

The [unchecked-item review](development/checklist-review.md) distinguishes
completed component foundations from unfinished adapter, job/CLI lifecycle and
expanded PHY requirements. It predates the duration-stepper completion below.
Checked substeps do not close their parent requirement.

Parity interleaving, column twisting and cell demultiplexing now expose separate
forward/soft-inverse APIs and direct intermediate arrays. The existing facade
composes their shared mapping owners. QAM-03 extends this foundation to all ten
uniform short8/9 data orders, including explicit padding and its reverse boundary.

Current source ownership:

```text
crates/
├── fec-codec/src/
│   ├── bch.rs                 Payload BCH encoder/decoder
│   ├── ldpc.rs                Payload LDPC encoder/soft decoder
│   ├── concatenated.rs        Full-word composition and payload comparison
│   ├── shortening/            Variable-length TX/RX boundaries and mother-code reuse
│   ├── crc.rs                 CRC-24-D
│   ├── ncp/                   NCP mother matrix, shortening/puncturing and decoder
│   ├── plc/                   Separate PLC matrix, encoding, puncturing and decoding
│   └── parameters.rs          Verified coding dimensions
├── ofdm/src/
│   ├── types/                 Shared identities, units and modulation registry
│   ├── transmission/
│   │   ├── settings.rs        Fixed numerology
│   │   ├── timing.rs          Integer sample-clock schedules
│   │   ├── layout/            Physical roles and inverse pilot reference
│   │   ├── frame/             Protected physical frequency-bin assembly
│   │   ├── packing/           Bounded codeword queue and symbol plans
│   │   ├── symbols/           Actual NCP/data/filler assembly and typed cell values
│   │   ├── ncp/               Messages, control mapping and field CRC acceptance
│   │   └── plc/               Acquisition/messages, coding, interleaving, masks and signals
│   │       └── receiver_configuration/  Received channel/profiles, collection and readiness
│   ├── signal_processing/
│   │   ├── modulation/        QAM, LLRs, bit processing and cell randomization
│   │   ├── interleaving/      Separate time, frequency and composition modules
│   │   ├── pilots/            Shared sequence, power and separate CP/SP signals
│   │   ├── waveform/          TX IFFT/prefix/window/overlap and RX FFT/prefix primitives
│   │   ├── receiver/          Known timing, reference equalization and extraction
│   │   │   └── payload/       Acquired configuration, NCP hypotheses and bounded word recovery
│   │   ├── channel/           Unity time-domain reference transfer
│   │   ├── noise/             AWGN and measured residual-noise processing
│   │   └── ...                Analytical bit-loading/capacity helpers
│   ├── simulation/
│   │   ├── source/            Seeded / PRBS15 / file sources; independent verification
│   │   ├── pilots/            Typed stage composition and optional observation
│   │   ├── plc/               PLC composition, acquisition and optional observation
│   │   ├── waveform/          Separate TX/RX, bounded reference experiment and probes
│   │   ├── payload/           Continuous payload TX/RX, probes and bounded waveform proof
│   │   ├── channel/           Settings, shared link, stepper, window accounting and bounded jobs
│   │   ├── test_points/       Borrowed events, finite selection and versioned JSONL writer export
│   │   ├── coding_loop/       Independent 256-QAM codeword trials
│   │   ├── runner.rs          Existing 16-QAM trial orchestration
│   │   ├── config.rs          Work limits and receiver policy
│   │   └── results.rs         Counters, truth comparisons and confidence bounds
│   ├── scenario/              Decoded observations, grids and association
│   └── profile/               Hypotheses, role protection and screening
└── cli/src/
    ├── main.rs                Startup and exit
    ├── arguments.rs           CLI syntax
    ├── commands/              Thin operation adapters
    ├── logging/               CLI-only stderr backend and RUST_LOG setup
    └── json_io.rs             Bounded input and safe new-file exports
```

Dependencies flow from CLI/future adapters to `docsis-ofdm-fec`, then to the
optional `fec-codec`. Analysis-only builds exclude codec-backed simulation.
Preserve established imports through re-exports when reorganizing modules;
each calculation retains one canonical implementation.

## Transmission and receiver flow

The diagram describes the target channel path. Dashed-border boxes are planned
components. The fixed256-QAM noiseless channel stepper now connects the solid path;
nonideal operation remains separate work. The diagnostic hard slicer is now a standalone API.

```mermaid
flowchart TB
    SOURCE["Source payload · seeded or continuous PRBS15"] --> BCH["BCH encoding"]
    BCH --> LDPC["LDPC encoding"] --> BITS["Parity → twist → demultiplexing"]
    BITS --> PACK["Codeword packing"]
    PACK --> NCP["NCP messages → CRC → control FEC"]
    PACK --> CELLS["Typed data/NCP/filler assembly"]
    NCP --> CELLS
    CELLS --> RAND["Cell randomization"] --> QAM["QAM mapping"]
    QAM --> TIME["Time interleaving"] --> FREQ["Frequency interleaving"]
    PILOTS["Continuous/scattered pilot signals"] --> FRAME["Physical-bin assembly"]
    PLC["PLC preamble, messages and signal"] --> FRAME
    FREQ --> FRAME --> IFFT["IFFT"] --> CP["Cyclic prefix"]
    CP --> WINDOW["Windowing and overlap-add"] --> CHANNEL["Reference or impaired channel"]
    CHANNEL --> TIMING["RX timing and block framing"] --> FFT["RX prefix removal and FFT"]
    FFT --> EQ["RX known-reference equalization and extraction"]
    EQ --> INVF["Frequency de-interleaving"] --> INVT["Time de-interleaving"]
    INVT --> DEMAP["Soft demapping and inverse randomization"]
    DEMAP --> NCPRX["NCP FEC/CRC → received boundaries"]
    NCPRX --> BITRX["Data collection and inverse bit processing"]
    BITRX --> LDPCDEC["LDPC decoding · iterations/status"]
    LDPCDEC --> BCHDEC["BCH decoding"] --> TRUTH["Compare original and recovered payload"]
    SOURCE -->|"Independent truth"| TRUTH
    NCPRX -.-> SLICER["Optional hard slicer · profile-selected data I/Q"]
    classDef planned stroke-dasharray: 5 5;
    class SLICER planned;
```

The NCP and payload receiver paths use the appropriate separate demapping
hypotheses. Inverse randomization uses the recovered source-symbol phase, not the
later channel-output phase. Actual pilot signals are inserted after interleaving;
PLC, continuous pilots and exclusions never enter the reduced interleaver domain.
Initially timing/equalization can use known reference settings; blind acquisition
and estimated-channel operation are separate milestones, not implied by that mode.

Current integration tests already connect real payload/NCP coding, randomization,
QAM, cell interleaving, inverse pilot references and received pointer discovery.
The reusable payload TX/RX owners now feed a duration-planned channel stepper;
signal-processing algorithms remain in their existing component modules.

## Test points and data access

Existing APIs expose useful intermediate arrays and results. The common typed,
bounded observer covers pilot/PLC, physical bins, waveform TX/RX, payload/NCP,
source output and the existing coding experiment's noise output. A versioned
bounded JSONL writer adapter and the channel CLI export selected events. Other
stage IDs below remain proposed where direct outputs are not wired to producers. “Direct” means a caller can
obtain data through an existing stage API.

| Test point | Owner / data | Access now |
| --- | --- | --- |
| `source.payload` | `simulation::source`; source bits and seed/source identity | Direct output plus bounded source producer in coding/channel jobs |
| `tx.bch.word` | `fec_codec::bch`; BCH data plus parity | Direct |
| `tx.ldpc.word` | `fec_codec::ldpc`; systematic word plus parity | Direct |
| `tx.bits.parity`, `tx.bits.twist`, `tx.bits.demux` | Separate bit-processing stages; bit positions and cell labels | Separate forward/soft-inverse APIs and direct intermediate arrays; composed mapping preserved |
| `tx.packing.symbol` | `transmission::packing`; starts, roles, filler and offsets | Direct plan; IDs are transmitter truth only |
| `tx.ncp.messages`, `tx.ncp.crc`, `tx.ncp.encoded` | Message bytes, CRC, control-code bits/labels | Direct field/codec outputs |
| `tx.randomizer.cells` | Randomized labels and diagnostic LFSR state | Direct per-cell output/state |
| `tx.qam.iq` | Normalized I/Q with modulation and reference plane | Direct mapper output |
| `tx.time.cells`, `tx.frequency.cells` | Each cell-interleaver output with startup markers | Direct separate APIs |
| `tx.pilots.sequence`, `tx.pilots.continuous`, `tx.pilots.scattered`, `tx.pilots.power` | Separate sequence, signals, boost and physical locations | Direct values and bounded observer |
| `tx.plc.message` | `transmission::plc`; TS+MC information bytes and288-bit inputs | Fixed OCD/ProfileA/NCP packets, scheduling/reassembly and bounded message-byte observer |
| `tx.plc.coded` | `fec_codec::plc` and thin `transmission::plc::coding`; mother/transmitted bits | Mother bits direct; transmitted bits through bounded observer |
| `rx.plc.llrs`, `rx.plc.decoded` | Restored mother LLRs and soft decisions with iterations/parity | Direct APIs; common observer TODO |
| `tx.plc.coded_nibbles`, `tx.plc.interleaved` | Grouped labels and time-major PLC grid | Direct typed results and bounded observer; explicit grid metadata |
| `rx.plc.deinterleaved` | Hard-label or soft-bit inverse PLC permutation | Direct APIs; common observer TODO |
| `tx.plc.randomized`, `rx.plc.descrambled` | Independent PLC masks and sign-only soft inverse; frame-index/register snapshots | Direct block APIs; TX labels through bounded observer, RX observer TODO |
| `tx.plc.preamble`, `tx.plc.iq` | Verified BPSK preamble and shared16-QAM mapping | Direct typed arrays and bounded observer |
| `tx.frame.bins` | Full physical frequency grid, roles and zero/excluded bins | Direct assembler and bounded observer; data None preserved |
| `tx.ifft.samples`, `tx.cp.samples`, `tx.window.samples`, `tx.overlap_add.samples` | Separate TX waveform stages, frozen geometry and sample indices | Direct stage results and bounded observer |
| `channel.input`, `channel.output` | TX-output/RX-input I/Q with preserved sample indices | Unity, sample AWGN or symbol-diagonal H/RxMER synthesis with bounded observation |
| `rx.framed.samples`, `rx.cp.samples`, `rx.fft.bins`, `rx.equalized.iq` | RX known-boundary periods, useful samples, raw bins and reference-compensated bins | Direct results and bounded observer |
| `rx.plc.iq`, `rx.pilots.iq`, `rx.reduced.iq` | RX separated protected roles and data/scattered domain | Direct results and bounded observer; sparse pilots retain physical bins |
| `rx.timing`, `rx.channel.estimate` | Estimated timing/channel state, distinct from supplied reference | Direct typed acquisition/CH-EST outputs exist; named streamed acquisition event is not registered |
| `rx.frequency.cells`, `rx.time.cells` | Separate inverse cell-interleaver outputs and source phase | Direct |
| `rx.ncp.llrs`, `rx.ncp.accepted` | Actual soft records, CRC-accepted messages and direct block iterations | Production source decoder and bounded observer |
| `rx.data.llrs`, `rx.descrambler.llrs` | Data likelihoods after/before sign inversion | Post-inverse soft records have production observer; raw demapper remains direct |
| `rx.slicer.decisions` | Nearest label/reference point and distance/error vector | Direct API and bounded Qt decisions implemented; no streamed producer |
| `rx.bits.inverse` | Complete word LLRs restored to codeword order | Production collector and bounded observer; independent inverse bit-stage APIs available directly |
| `rx.ldpc.bits` | Decisions, parity success and actual iteration count | Direct |
| `rx.bch.payload` | Recovered bits, status and corrected-bit count | Direct |
| `tx.payload.ldpc`, `tx.payload.labels` | Actual enqueue-stage coding/label outputs | Bounded TX observer; queue ID is diagnostic, placement time unknown |
| `tx.cells.mapped`, `tx.cells.interleaved`, `rx.cells.restored` | Typed cells preserving pilots and unavailable history | Production stream owners and bounded directional observer |
| `rx.payload.bits` | Actual recovered BCH payload, independent of source truth | Production collector and bounded RX observer |
| `result.truth` | Actual bit/codeword comparisons and trial counters | Bounded payload-waveform proof plus duration-step results, PRBS totals and fixed iteration histogram; whole-word source-window metrics and explicit complete/partial job results implemented |

The implemented contract provides borrowed bit/label arrays, sparse pilot records
and a power record, with explicit metadata and finite selection budgets. See the
[pilot guide](user/pilot-signals.md) for the executable example and exact behavior.
The broader contract, including future producers, must provide:

- Stage ID, direction and run/configuration identity; codeword/profile identity
  where known. Receiver identities must come from decoded signaling/session state,
  never transmitter diagnostic IDs supplied as hidden receiver input.
- Source-symbol and channel-symbol indices separately; sample-clock position when
  applicable, PLC phase, physical/reduced grid mapping, units and normalization.
- Explicit value type and shape: bits, labels, I/Q, roles, LLRs or typed metadata.
  Preserve missing/startup/drain markers and success/failure states.
- Immutable borrowed data during a callback or explicitly owned snapshots with
  documented lifetime. Do not expose mutable internal buffers or RNG state.
- Opt-in stage selection, symbol/codeword/sample ranges and explicit data limits.
  Stream large sample blocks; do not retain an entire second of waveform by default.
- A disabled-observer path and tests proving observation does not alter RNG draws,
  stage order, simulation outputs or decoder decisions. Export/sampling belongs
  to adapters; numerical calculations remain in Rust stage modules.

Native typed stage calls remain usable without observers. JSON is an export
adapter, not the only way to get data. Probe failures and cancellation need an
explicit runner policy that preserves already completed simulation results.

The fixed-setting PLC path now generates OCD, ProfileA and NCP descriptors as
single-part management MAC packets, with HCS/FCS, mandatory edge/CP/PLC assignments,
MC segmentation and independent reassembly. A frozen scheduler repeats the set
within the200ms default using integer sample timing; packets too large for the
supported single-part contract or repeat budget fail explicitly. The source
settings and scheduler cadence must match. Received OCD geometry reuses the
existing physical-layout validator; DPD TLVs retain explicit profile identity.

Separate PLC stages own message coding, nibble grouping, block interleaving,
randomization, shared16-QAM mapping and the eight-symbol BPSK preamble. Full-frame
composition is in `simulation/plc/{transmitter,receiver}.rs`, bounded to one128-symbol
frame per operation. The receiver takes received equalized local PLC I/Q with
known spacing/frame alignment, retains decisions/iterations and independently
checks message and packet acceptance. This is not blind acquisition.
`transmission/plc/receiver_configuration/` now applies received OCD/ProfileA/NCP
messages through encapsulated channel, profile, collection, configuration and
readiness owners. Channel reconstruction and packet validation share one OCD
physical validator. Profiles apply specific-over-default assignments independent
of TLV order, with OCD protected roles taking precedence. Source/channel identity
is pinned per fixed session; OCD and the two profile counts remain independent.
A complete immutable receiver snapshot derives physical/reduced grids, prefix,
roll-off, interleaving and NCP modulation without transmitter objects.

`simulation/plc/acquisition.rs` composes PLC I/Q decoding, bounded MC reassembly
and configuration admission. Only supplied PLC bootstrap spacing/alignment stays
outside received signaling. The readiness gate is Waiting/Ready/Rejected and
exposes data/NCP settings only when Ready. Rejection preserves accepted descriptors
and historical configuration; a valid repeat can reopen the gate. Changed packet
content requires an explicit reset in this fixed-session implementation. Live DPD routing is now opt-in through NCP-03. Descriptor timeouts and automatic
loss recovery remain separate work; payload receive orchestration is described below. See the per-subsystem
[receiver-configuration design guides](user/receiver-configuration/README.md).

`transmission/frame/assembly.rs` combines already cell-interleaved data/NCP,
scattered placeholders, PLC and actual pilots in increasing physical-bin order.
Protected bins cannot be overwritten; excluded bins are zero and unavailable data
history stays None. Normalization is an explicit RMS reference. Completed PLC and
physical outputs expose bounded optional windows through
`simulation/plc/observation.rs`; callbacks cannot modify signal-processing stage state.
The [PLC channel guide](user/plc-channel.md) includes the complete flow, math,
reproducible example, supported fixed subset and verification scope. Individual
[coding](user/plc-coding.md), [message](user/plc-messages.md),
[interleaving](user/plc-interleaving.md) and [randomization](user/plc-randomization.md)
guides remain focused stage references. External interoperability remains pending;
production fixed-profile data/NCP transport is implemented separately below. The separate waveform path
now carries these PLC/pilot signals through actual time-domain samples with
known-reference reception; see [waveform design](user/waveform/README.md).

## Waveform direction and configuration

Waveform means the complete **TX → channel → RX** path, not just reception.
The [checklist](development/checklist.md#waveform-direction-and-completion-scope)
uses direction-specific groups:

| Group | Direction and responsibility | Processing boundary |
| --- | --- | --- |
| `WAVE-TX` | Transmitter: IFFT, prefix insertion, windowing and overlap-add | Physical frequency bins → time-domain samples |
| `WAVE-CH` | Connecting channel: noiseless reference first | TX time samples → RX time samples |
| `WAVE-RX` | Receiver: framing, prefix removal, FFT, extraction and recovery | RX time samples → recovered frequency bins and decoded signals |
| `WAVE-QA` | Validation across both directions and the channel | Bounded observations and independent end-to-end comparisons |

Each subsystem owns its configuration/state and exposes testable intermediate
outputs. Shared transform/prefix primitives keep one numerical owner; TX/RX
composition remains separate. Test truth is used only for comparison, never as
an input to the receive decoder.

The first noiseless waveform milestone is implemented: WAVE-TX-01..03,
WAVE-CH-01, WAVE-RX-01..04 and WAVE-QA-01..02. Centered unitary transforms,
separate prefix operations, a half-sample Tukey taper, stateful overlap-add,
known-boundary stream framing, reference equalization and protected-role
extraction have independent public boundaries. TX/RX each own configuration
and history; finite sample blocks retain absolute ticks. Completed outputs are
observed after computation through thirteen direction-tagged probe producers.

`simulation/waveform/reference.rs` streams at most two PLC frames through real
samples and compares recovered bins and actual decoded information bits.
Its data plane contains deterministic256-QAM reference cells, not encoded
payload/NCP. Tests cover both spacings with a192MHz configured cap, a wide
active envelope and internal exclusions; the example uses a24MHz envelope.
Neither establishes external interoperability or a one-second payload experiment.
CELL-02/WAVE-RX-06 now add the separate actual payload waveform path described
below. The duration stepper is implemented separately; nonideal acquisition
(WAVE-RX-05) now has a bounded bootstrap search; temporal expansion has its
separate MODEL-06 implementation. Window accounting
and bounded job/CLI lifecycle are implemented separately.

See the [per-subsystem guides](user/waveform/README.md) and
[pinned waveform evidence](reference/waveform-evidence.md).

Prefix length and roll-off are **user-configurable before a run and constant
within that run**. TX validates and retains the chosen configuration. OCD
advertises the applicable settings; RX derives its channel configuration from
received signaling after the explicit known-phase PLC bootstrap. A setting
change starts a new configured run/session in this fixed-session model. See
[the prefix explanation](user/transmission.md#configurable-prefix-constant-per-run).

## Production fixed-profile payload transport

`transmission/symbols/assembly.rs` encodes the existing packer plan's real NCP
messages/CRC/FEC and maps continuously randomized data, NCP and BPSK filler.
Typed cells carry samples/pilot placeholders only; outer None retains unavailable
interleaver history. TX diagnostic IDs and reservation offsets never enter RX.
`simulation/payload/transmitter.rs` owns separate coding, packing, assembly and
cell-interleaving state, with a bounded queue and explicit M-1 drain.

`signal_processing/receiver/payload/` separates acquired-configuration support
checks, NCP hypotheses, the bounded pending-word collector and source decoder.
`simulation/payload/receiver.rs` checks the live PLC gate and immutable snapshot
on every call, inverse-interleaves actual received samples and reconstructs the
source index from known matched-pair delay. The source decoder uses CRC-accepted
NCP spans, independent soft hypotheses and actual LDPC/BCH results. Bad control
closes recovery without clearing pending history or caller-owned completed words.

Support is frozen ProfileA ID0 with all ten uniform orders or verified mixed loading,
including zero-loaded carriers, and uniform positive QPSK/16-QAM/64-QAM NCP.
The fixed constructors still reject odd selection, activation updates and tagging.
NCP-03 adds a separate opt-in dynamic full-word path described below. See the applied-profile design for non-shortened coding bounds. Descriptor counts remain independent.
The synchronized payload origin is a128-symbol boundary after PLC acquisition.
TX deliberately emits recorded BPSK replacements for unavailable startup/drain
history; RX suppresses only the configured M-1 warm-up, using no TX validity list.

Ten bounded producers expose coding, mapped/interleaved/restored cells, accepted
control, soft records and recovered payload. Source/channel coordinates and
original data-position ranks stay explicit. The bounded
`simulation/payload/reference.rs` proof acquires PLC through samples, configures
independent payload RX, streams actual encoded payload/NCP through the waveform,
and compares recovered words only after decoding. All three NCP orders have cell
transport tests; both spacings additionally have actual sample-domain proof at
maximum depth with reset-crossing words and drain completion.

See the [per-subsystem design](user/payload-transport/README.md) and
[evidence](reference/payload-transport-evidence.md). This completes CELL-02 and
WAVE-RX-06 for the supported subset. SRC-02/SRC-03 now add an explicit PRBS15
source and independent recovered-stream verifier. The waveform example uses
continuous PRBS across codewords, the sequence period and PHY reset; no source
truth enters receive decoding. Literal recurrence vectors and a complete-period
check are mathematical evidence, not external DOCSIS interoperability vectors.

## Configurable-duration channel stepping

`simulation/channel/` now owns frozen settings, checked64-bit duration plans,
shared actual waveform/PLC link composition, a one-symbol-at-a-time runner and
bounded cumulative/per-step results. It reuses existing payload/waveform TX/RX
owners, rather than duplicating them in channel wrapper modules. The old bounded
payload verifier shares the same link and preserves its public import paths.

The initial observation default is one second, configurable up to an application
limit of one hour. Only complete source-symbol periods are scheduled; unused
trailing ticks stay explicit. Acquisition, source observation, queued-word
completion, cell-interleaver drain and the final window tail are separate. PRBS,
partial codewords, PLC phase, randomizers and waveform overlap persist across calls
and second boundaries. Existing bounded test points remain available.

The runner offers PRBS payloads to an eight-word queue, stops generating at the
observation boundary, completes queued words, then drains. Per-step decoded words
carry actual iteration counts/BCH flags and independent PRBS comparisons. Summary
storage does not grow with duration: PLC iteration records are consumed per frame,
payload iterations use a101-bin histogram, and samples/words are not retained as
whole-run history. Failed sessions close explicitly; zero BER is unavailable
until actual bits have been compared. A1.01-second50kHz release experiment recovered
8950 words/127376400 payload bits with zero payload or PLC errors.

The RUN group is now implemented for the fixed-profile noiseless scope. A shared
window classifier uses received inclusive source-symbol spans and a half-open
observation window; whole-contained words contribute window BER/information-bit
rate, crossing/outside words remain explicitly excluded while global PRBS truth
stays continuous. Late receive completion during drain does not change source
membership. Partial jobs distinguish received announced incomplete starts from
admitted-but-unrecovered words. Final window rate stays unavailable until completion.

The job wrapper checks a caller cancellation token and finite host-runtime budget
between bounded steps, retains finite word details and all completed scalar totals,
and reports Complete/Cancelled/RuntimeExceeded/Failed independently of decoder
success. The CLI writes the same typed result and never replaces existing files.
Versioned JSONL export preserves original selected offsets, units and grid metadata,
with independent record/stream limits and explicit terminal I/O/limit errors.

ARCH-05/06/07 and QA-04 close this infrastructure milestone: maintained export,
source/noise draw-invariance tests, separate bit-processing boundaries, and a
release channel/probe smoke check in the pre-push gate. The subsequent impairment
group adds controlled waveform noise, PNM JSON import, fixed-profile measured
operation and supplied H compensation. The supported MODEL-05/06/07 extension
now adds pilot estimation, explicit epochs and mixed applied profiles; full blind
acquisition and dynamic profile switching remain separate. The design follows the
[user diagram reference](development/impairment-reference.md).
See the [component guides](user/channel-runner/README.md),
[channel CLI](user/channel-runner/cli.md), [export](user/simulation/probe-export.md)
and [bit-processing design](user/bit-processing/README.md).

## Planned module ownership

These paths are the accepted direction for upcoming work, not existing files or
promises that the algorithms have been implemented. Refine a path in this document
before implementation if source analysis shows a better existing owner.

| Planned path under `crates/ofdm/src/` | Responsibility |
| --- | --- |
| `transmission/layout/additional_pilots.rs` | Specification-based additional continuous-pilot placement |
| `signal_processing/receiver/estimation.rs` | WAVE-RX: independently verified nonideal synchronization/channel estimation beyond the existing known-reference receiver |

If PLC needs a new coding algorithm, its reusable primitive belongs in
`fec-codec`, selected only after local PHY verification. Do not assume payload or
NCP coding is interchangeable with PLC coding. Shared sequence/math definitions
have one owner; do not copy an LFSR into every consumer.

The hard slicer is diagnostic: nearest constellation point with an explicit tie
policy, symbol-error counts and data for EVM inspection. It must not silently
replace soft LLR inputs to LDPC. Its standalone implementation is complete under QAM-02;
observer registration remains distinct from the direct decision API.

## Diagnostic logging

Simulation crates use the shared `log` facade; only the CLI installs `env_logger`.
The backend defaults to info on stderr and accepts `RUST_LOG` filters. Libraries
keep logger ownership with the caller. Error records belong at the terminal
application boundary, with separately suppressed failures recorded by their owner.
Expected decoder failures/input rejection are debug diagnostics; simulation
outcomes remain typed results. No sample/capture arrays or MACs are logged.

The current retrofit covers command/JSON failure handling, simulation lifecycle,
BCH and payload/NCP/PLC LDPC outcomes, timestamp CRC rejection and observer
failures/budgets. Logs are separate from typed test points and cannot control
simulation decisions or RNG draws. See [logging](development/logging.md).

The simulation core exposes `SimulationError` for domain failures, with stable
`code` and readable `message` fields. CLI and future adapters translate this
shared type; renaming it does not change serialized error categories.

## Lifecycle, validation and future interfaces

Freeze numerology, layout and profile before each run. Carry partial codewords,
interleaver buffers and sequence phase across calls. Keep PLC and data/NCP
randomizer states separate. The cell-interleaver pair adds M-1 symbols of delay;
None history is not transmitted payload; the physical reference uses explicit
recorded startup/drain replacements. Warm-up, configurable observation duration
(initially one second) and drain need separate accounting, including complete
versus unfinished observation-window words in the shared source-window classifier. The existing
`one_second_timing` helper now delegates to general duration/sample planning,
while preserving its original32-bit result shape. New plans use64-bit counts.

Bound files, buffers, observations, work and decoder iterations. Preserve
completed results on cancellation. Keep decoder success, truth errors, measured
observations and analytical screening distinct. No completed comparisons means
no fabricated zero BER. The seeded baseline is reproducible on the tested build;
external interoperability and untested platforms remain separate validation work.

Python/Qt adapters call the same simulation APIs and test-point contracts.
They do not implement DSP, codec math or simulation JSON post-processing.
Native Python bindings and Qt presentation reuse the existing Rust engine.
Native CLI packaging and
multi-OS validation/publication workflows are now prepared locally; only Linux can
be verified on this host. Their remote execution is deferred, and the optional Qt
application is not part of these CLI installers.

## Stage references

- [FEC codec](user/fec-codec.md), [coding evidence](reference/coding-mode.md)
- [QAM mapping](user/qam-mapping.md), [256-QAM bit/coding loop](user/qam256-coding-loop.md)
- [Randomizer](user/cell-randomizer.md), [symbol packing](user/symbol-packing.md), [NCP](user/ncp.md)
- [PLC LDPC coding](user/plc-coding.md)
- [PLC message blocks](user/plc-messages.md), [PLC evidence](reference/plc-evidence.md)
- [Pilot signals and test points](user/pilot-signals.md)
- [Physical layout](user/carrier-layout.md), [cell interleaving](user/cell-interleaving.md)
- [Transmission setup and timing](user/transmission.md), [public APIs](reference/public-api.md)
- [JSON contracts](reference/documents.md), [build/test](development/build.md), [Git/releases](development/git.md)

Pinned simulation sources and section/page evidence are recorded in those guides.
Use local versioned PHY sources before adding pilot/PLC details. Update this
architecture and the [checklist](development/checklist.md) together as stages change.

The [pilot/PLC reference demonstration](user/impairments/pilot-plc-constellation.md)
uses an example-owned observer over existing RX FFT/equalized probes. It keeps
the four signal classes separate during acquisition. The newer CH-EST-RX shared
collector and pilot-derived correction are documented in the receiver guides above.

## Completion group: dynamic control, layout, acquisition and delivery

NCP-03 adds a bounded received-DPD generation bank under
`transmission/plc/receiver_configuration/generations.rs`, the temporal N/U
recognizer/schedule under `transmission/ncp/activation.rs`, and opt-in streaming
payload transport. Each data start selects profile0..15 plus C parity from the
actual received NCP. The collector changes bit processing at word boundaries;
continuations retain their selected generation. Positive uniform NCP changes use
the exact preceding128-symbol U pattern to choose N before demapping. The
fixed-session constructors remain strict. Further same-parity replacement requires
reacquisition; dynamic shortened words/OPT tagging are not implemented.
See [dynamic control design](user/ncp-profiles/README.md).

LAYOUT-03 adds a deterministic additional-continuous-pilot generator in
`transmission/layout/continuous.rs`, independent of simulation features. It shares
the physical validator, computes density from actual active frequency span, merges
eligible spectrum, protects PLC/poor bins/edges, covers isolated regions and
perturbs placement without touching simulation RNG state. Generated settings feed
the same descriptor/OCD builder as explicit settings.
See [placement and equations](user/carrier-placement.md).

WAVE-RX-05 now adds a finite hypothesis search in
`signal_processing/receiver/channel_estimation/search.rs`, alongside the original
known-numerology fractional-only operation. Candidate25/50kHz bootstrap geometries
are selected using received samples; integer-carrier shifts and two-symbol
held-out pilot scoring resolve CFO aliases and narrow-channel pilot counts.
The selected total derotator retains one absolute sample origin across blocks.
Search/selected hypothesis/timing/CFO/phase/error and normal receiver outputs are
public test points. Tests cover24/90/192MHz and reflected-path/sample-noise cases,
including actual zero-error coded-block recovery. CP/RP and layouts remain
candidate inputs, not discovered RF metadata; absolute PLC frame acquisition is
separate. The duration CLI retains its synchronized reference mode.
See [RX guide](user/waveform/rx-acquisition.md) and
[design and validation](development/features/receiver-acquisition/design.md).

INPUT-02 adds an explicit producer allowlist and contract revision under
`scenario/pnm_json/contract.rs`. CLI request schema2 requires the pnm-decode0.3.6
export declaration; legacy schema1 is explicit and unchanged. Shared Rust imports
retain actual JSON hashes and simulation checks. Ancillary additive display fields
are allowed, while required field/unit/assignment drift rejects. Preserve the
request with the scenario because ordinary upstream JSON has no producer stamp.
See [decoder contract](user/impairments/pnm-import.md#explicit-decoder-contract-input-02).

QA-05 now records completed closed-loop source/control/payload validation,
following the user's scope clarification. External-vector tooling only checks
provenance/files/hashes and stays optional; it is not a minimum acceptance gate. QA-06/07 add locally prepared native tests and CLI package
builders under `tools/packaging/`, with tag-triggered workflows under
`.github/workflows/`. Package builders install nothing, preserve licenses/examples,
refuse clobber and consume matching tested binaries. Initial delivery targets Ubuntu 22.04, 24.04 and 26.04 x64, with tests on each
and one shared archive/DEB built on 22.04. Windows/macOS execution is deferred.
Native execution/publication awaits the first authorized workflow push. QAM-04 also closes within the documented closed-loop scope.
See [delivery design](development/native-delivery.md) and
[group record](development/features/completion-group/design.md).

### Synthetic width validation

The [channel-width fixture design](development/features/channel-widths/design.md)
uses shared Rust pilot placement and per-bin loading for24/90MHz occupied widths.
Decoder-shaped synthetic inputs retain four profile IDs; fixed-runner tests
exercise each loading as internal ProfileA without expanding measured-selector
support. No new production stage or input schema is introduced.

### Local documentation-only push validation

The Git push helper classifies each outgoing commit before ordinary branch
pushes. Only maintained documentation paths skip the local build/test gate.
Initial pushes and release/tag modes retain full validation; clean-tree, ancestry
and unchanged-commit guards remain mandatory. Remote CI is unchanged.
See [DOC-PUSH-01 design](development/features/docs-only-push/design.md).

### Security validation

Separate CodeQL and RustSec workflows own source-security analysis and dependency
advisory checks. They run without the simulation matrix on Ubuntu 24.04, and
tag publication depends on both succeeding alongside native tests. The local
audit launcher verifies a pinned static tool in ignored local storage; it does
not install system packages or change the simulation core or dependency graph.
See [security checks](development/security.md).

### Optional native Python API

`bindings/python` is a separate PyO3/maturin workspace using the canonical
`docsis_ofdm_fec::api` and `pnm-input` crates. It converts typed requests,
loads supported files, dispatches synchronous Rust jobs without the interpreter
lock and retains independently owned reports. Another thread or Ctrl-C requests
cooperative cancellation; concurrent controller operations receive Busy.
No scientific calculation is implemented in Python and no CLI subprocess is
invoked. Mutable requests are copied at acceptance; scientific results are
read-only, and JSON dictionaries are independent transport snapshots.

Local development wheels target Ubuntu 22.04 x86_64 / CPython 3.10. Other targets
remain unverified. Package distribution is `merlin-ofdm`, imported as `merlin`;
these are local development artifacts, not a publication or release claim.
See the [Python guide](user/python/README.md) and
[current design](development/features/high-level-api/design.md).

## Ubuntu 22.04 desktop delivery and release notification

UI-DEB-01 adds a native amd64 `merlin` DEB separately from `merlin-ofdm` CLI
ownership. `tools/packaging/desktop.py` stages a private Qt 6.8.3 runtime under
`/usr/lib/merlin`, a `/usr/bin/merlin` launcher, desktop entry/icon, public guides,
all runnable example bundles and complete Cargo/Qt notices. Qt's scanner and
recursive ELF dependency inspection select runtime resources; observed system
libraries are mapped to explicit DEB prerequisites. Build on Ubuntu 22.04 with
Rust 1.85.1 and locked sources. No external Python/Qt SDK is needed after setup.

The desktop job in shared `.github/workflows/ci.yml` supports validation and a
reusable tag-release job. It validates, packages and exercises offline startup,
all WB imports, hashes and runtime dependencies; disposable CI runners then
install/reinstall/remove before uploading the DEB and SHA256SUMS. Existing tag
publication waits for desktop, CLI, CodeQL and audits. Only Ubuntu 22.04 x64
desktop delivery is targeted now; configured jobs are not hosted-run evidence.
Provision the pinned SDK before requiring its explicit path, and initialize
ignored test scratch storage on fresh checkouts. The first v0.1.0 hosted run
failed at those two bootstrap boundaries; local fixes pass focused checks,
while successful hosted packaging and installation remain pending.
For local-only iteration, explicit `--skip-test` retains static/tooling/build
checks and all Git guards while omitting Rust tests and simulation smokes.
Hosted release validation is unchanged; see
[DEV-04](development/features/ubuntu-desktop/local-validation.md).
Default local Rust validation now uses a bounded process-per-test runner,
four workers on this PC, with independent QAM/width cases separately scheduled.
Doc/example/analysis phases and timing-sensitive serialization preserve coverage;
Qt checks remain separate. This changes local scheduling, not hosted commands
or simulation state. See [DEV-05](development/features/parallel-local-tests/design.md).

UI-UPDATE-01 keeps public stable-release discovery in the GPL application adapter
under `platform/updates`, outside DSP/domain crates. One bounded worker queries
GitHub; QML polls plain status without blocking simulation. A single green Update chip sits at the right of the File/Settings/About row.
Its click starts the update directly; About offers manual checks in a separate
progress/preferences dialog. Numeric versions, stable flags,
exact project/tag URLs, baseline asset name/size and SHA-256 metadata are checked.
Only a newer compatible stable desktop release produces a notification.
Manual workflow artifacts do not. Startup failures are quiet; manual retries
show concise status. The application-global startup preference is persisted
outside WB JSON. UI-UPDATE-02 adds verified streaming download (300 MB bound, 15-minute copy
deadline, allowlisted HTTPS redirects) and installed Ubuntu 22.04-only polkit
installation. Before Qt starts, the elevated installed executable copies the
caller-owned regular DEB into private root staging, rechecks size/hash and
package/version/architecture, rejects non-upgrades and calls APT with no removals.
The GUI stays unprivileged and open on failure. On success it launches the new
installed app as the desktop user and exits. Accepted WB settings are first
saved privately under the application state directory and restored on restart;
results and unapplied drafts are not recovered. Updates cannot begin during a
simulation. Checkouts cannot install. Global update state remains outside WB
JSON. Installed lifecycle validation remains pending; see the
[automatic update design](development/features/automatic-updates/design.md).

Audits cover both Cargo lockfiles without suppression. SECURITY-DEPS-02 patches
rand to 0.9.5 and cxx/cxx-gen to 1.0.195/0.7.195 while retaining Rust 1.85.1 and
CXX-Qt 0.7.3. See the [design and validation](development/features/ubuntu-desktop/design.md)
and [desktop user guide](user/install/ubuntu-desktop.md).

### Receiver profile performance

The profile-comparison mode runs each accepted DPD profile as a separate
closed-loop experiment, resetting source/receiver/noise/playback state under
identical configured conditions and seeds. It does not switch profiles mid-run.
Receiver Profile Performance combines measured post-equalization MER,
ideal profile minimum MER and per-profile TotalCW/CCW/UFEC counters, with bounded
instrument retention and independent source verification. Retained assignment
roles define a first-to-last PLC band; the Qt Rust adapter clips it to the
requested viewport and QML paints it behind both curves. The reusable FecCounters type owns private counters and exposes getters and
copyable snapshots. CCW includes actual LDPC hard-decision changes and BCH
corrections, while source mismatches remain independently reported. See the
[profile performance design](development/features/profile-performance/design.md).

The sweep accepts at most32 profiles/262144 assignments and shares the configured
active runtime budget across jobs. All imported data profiles must validate
before admission. Each captured loading becomes the independent experiment's
bootstrap ProfileA (wire ID0); this does not imply live on-wire switching.
Completed/partial performance curves and counters remain selectable per captured
ID. QAM and slicer retain bounded per-run snapshots; spectrum and channel-response
instruments identify the latest completed profile’s detailed snapshot. Stop/budget expiry preserves completed entries and marks unstarted
profiles explicitly. WB receiver options persist all-profiles/applied-only mode;
legacy missing fields preserve all-profile behavior. New WBs initially use the
applied profile, with explicit captured-ID checkbox subsets in the main WB
configuration. Results remain transient.
`CarrierMer` stores per-physical-bin observation signal/error powers in the core;
`ChannelJobResult` exposes them through the same Rust API as other results.
The optional Python bindings expose these Rust reports; neither FEC accounting
nor MER depends on Qt.

### Main WB configuration and explicit profile selection (UI-WB-CONFIG-01)

Configure WB sits before Start on the main lower control row. File New creates
an explicit blank WB; successful Open/recovery or first Save activates one. Cancellation or
failed Open does not change activation or accepted state. New resets accepted
inputs, run/comparison state and Save destination after confirmation for an
active WB. Configure WorkBench stays enabled while idle. Without an active WB it
opens a Create New/Open Existing chooser, continuing to configuration only after
successful activation. Native Open cancellation/failure never opens configuration.
Rounded shared action buttons use equal run-control widths and heights, balanced
spacing and theme-aware focus/hover/disabled states (UI-WB-CONTROLS-01).
UI-BUTTONS-01 extends `WorkbenchActionButton` to ordinary actions across inputs,
instruments and shell frames. `WorkbenchDialog` supplies standard button-box
delegates, preserving automatic role forwarding; `ApplyCancelFooter` receives
the caller's theme. `ReceiverChip` composes the shared action style, while
`StageChip` owns diagram-only stage decoration. Compact controls and the green
Update chip retain constrained sizes; native pickers and specialized tabs,
tree disclosures, diagram blocks and TP shapes keep their interaction semantics.

Run/Profiles tabs edit duration per profile, shared host budget, noise seed and
captured profile-ID checkboxes. Receiver owns correction mode and iteration
limit. Both dialogs merge through the same Rust Options snapshot, preserving
other fields. Configuration validation accepts partial setup; frozen-request
admission still requires compatible inputs and a nonempty selection. The main
Start additionally requires an active WB and every block green.

Schema2 writes selected IDs under receiver.selected_profile_ids; global run and
receiver settings share this persisted model despite their separate UI dialogs.
None means the documented applied/all legacy policy; an empty list is accepted
configuration but cannot run. New defaults use applied-only; schema1 files keep
previous all/applied semantics. Unknown/duplicate IDs, unsupported versions and
invalid limits reject before atomic adoption. Only selected profile layouts are
admitted for the bounded independent sweep; the applied DPD capture stays intact.
See [design](development/features/workbench-configuration/design.md) and
[user guide](user/workbench-configuration.md).

### Shared Master and Release validation (CI-CONSOLIDATE-01)

Only ci.yml (Master) and release.yml (Release) remain. Master owns preflight,
CodeQL, both-lock RustSec, Qt checks and native Ubuntu22/24/26 tests with the
four-process pinned nextest runner and retained doctests/example/analysis phases.
Release verifies its tag and calls that same workflow with release=true, adding
optimized builds, packages and installed lifecycle checks. Publication waits for
all shared jobs and exact artifacts/checksums. The initial desktop failure in
run43061645839 was Azure mirror DNS/connectivity during APT setup, before Qt
compilation. Strict refresh plus bounded setup retries and CI-only HTTPS mirror
preference address that path; successful hosted recovery remains unverified.

### FEC Monitor presentation (UI-FEC-02)

The receiver FEC provider constructs bounded per-profile snapshots during the
existing result-retention step. Observation-window FecCounters remain separate
from whole-run iteration, recovery and verification totals. Rust supplies outcome
fractions, exact histogram bins with trimmed unused range and recovery partitions
that distinguish rejection, source mismatch and unfinished payloads. QML presents
Overview/Control Channels/Codewords tabs and a profile selector independently of
other instruments. Retain at most64 word records and32 profiles; never clone
whole signal snapshots for this instrument. Zero/unavailable and partial results
remain explicit; no source, decoder, run admission or WB persistence changes.
See [design](development/features/fec-monitor/design.md) and
[user guide](user/fec-monitor.md).

### Run wall timing (UI-RUN-TIME-01)

A shared monotonic Rust RunTiming object records sweep/profile wall durations,
last completed-step age, phase and decoded-word count. The worker marks actual
profile boundaries/completion; late UI polling cannot extend a completed time.
The main WB displays formatted wall times via its existing poll, independently
of simulated duration and the core active-time budget. Pause keeps wall time
advancing. Completion/Stop/failure freeze it; new Start or accepted input changes
reset it. Retention is bounded to the admitted32 profiles and remains transient.
Progress callbacks occur after steps; observation count excludes interleaver
drain. Never interpret an advancing UI timer as proof of decoder progress or
CPU contention. No core processing, cancellation boundaries or WB schema change.
See [design](development/features/run-elapsed-time/design.md) and
[user guide](user/closed-loop-workbench.md#elapsed-time-and-slow-profiles).

Technical UI/status terms are defined in the canonical
[terminology guide](user/terminology.md), reached by About → Terminology.
Interleaver depth/drain definitions distinguish implemented symbol delay from
CPU/decoder-dependent elapsed time; new terms follow skills/ui-design.

### Indexed QAM soft demapping (QAM-PERF-01)

`signal_processing/modulation/soft_bits` derives bounded lazy search indexes from
the unchanged embedded label-ordered reference. Complete Cartesian geometry and
coordinate-separable bit labels are verified before using two axis binary searches
and constant-time predecessor/successor lookups per output bit: O(log M) per cell.
Irregular/coupled geometry uses exact balanced per-hypothesis spatial searches,
with worst-case O(M log M) work. Small control/16-QAM/cross-128 scans remain direct.
Preserve computed Euclidean minima, I-squared plus Q-squared order, variance, MSB
ordering, clipping and error policy. Inconclusive extreme-coordinate finite bounds
use the old exhaustive scan. Cache admission depends only on12 supported immutable
references, never external capture sizes. No public API, mapping tables, FEC rules,
RNG, run budgets, persistence or Qt numerical ownership changes.
See [design](development/features/qam-demapper-performance/design.md) and
[user guide](user/qam-mapping.md).

### Received QAM markers (UI-QAM-DOTS-01)

Channel-output and receiver raw/equalized I/Q use circular scatter markers.
The shared TX scatter keeps ideal role shapes and accepts a presentation-only
received-sample flag at the output tap. Rust owns coordinates and bounded sample
selection; dots do not represent density or hard decisions. The output tap shows
the retained/displayed count limit.

### Contextual help and spectrum details

Contextual run help uses stable core error codes and the bundled
`resources/help/diagnostics.json` catalog. Hover or focus a known workbench
message for its suggested fix; click or press Enter/Space/F1 for the offline
FAQ. About → FAQ opens the index. Rust derives timing advice from the actual
playback plan; unknown errors retain their original message without advice.
Spectrum acquisition details appear below the axes and can be hidden with
**Details**, independently of warnings and power readings.

### Interrupted release candidate recovery

The release helper owns explicit `--resume` admission. It reconstructs an exact
version-only candidate from HEAD in temporary storage, preserves checkout bytes
and reruns selected validation before commit/tag/atomic push. All normal Git
guards remain; committed-tag recovery still uses `--tag`.

### Explicit quick local validation

`--test-quick` flows through branch/release helpers to the shared local gate and
nextest launcher. `tools/testing/quick-tests.json` owns 21 exact sweep omissions;
the Python selector creates a scoped package/binary/test filter. Unlisted tests
run by default. Representative waveform and CLI cases, docs, example, analysis,
smokes and Git guards remain. Hosted CI/full default are unchanged. Checkout
failures reuse candidate recognition to print a safe resume command; unrelated
edits receive read-only inspection commands.

### Release asset identity

The Python asset-name helper owns component/version/target/architecture filenames.
Package IDs stay merlin and merlin-ofdm. Desktop releases provide an identical
legacy alias for installed pre-migration updaters; new Rust adapters prefer exact
canonical metadata and permit only the exact trusted legacy fallback. Signature,
URL, size, digest, duplicate and installed-package checks remain. Workflow
installation selects the canonical file rather than every DEB.

### Focused About and dropdown labels (UI-ABOUT-01)

Menu labels use plain text without ellipses, retaining standard shortcuts.
About menu contains About MERLIN, FAQ and Terminology. Manual checking lives
inside About MERLIN and routes through the existing Rust-backed update adapter;
the green automatic Update chip is unchanged. About emits fixed project/issue
URLs for Main to open externally, without automatically submitting reports.

### Graphical transmitter test points

UI-TX-GRAPH-01 preserves the seven upper processing blocks and replaces the lower
text chips with equal clickable graph cards. Rust `instruments/transmitter/graphs`
projects retained source bits, coded-bit rasters, interleaved bits/labels,
constellations, ordered I/Q and physical bin power/layout roles. QML shares one
graph renderer between previews and the full inspector; exact values remain
under Details. Source/interleaved bits show 128-bit pages, LDPC rasters 2048-bit
pages, labels/ordered cells 64-element pages. Physical graphs show the full
retained single-symbol span with at most 2048 power extrema and 1024 role bands;
Details still page through exact bins. Missing history remains absent.

Ordered-cell comparisons use separate captured source/channel symbol identities
and common amplitude limits, not an inferred complete permutation. The physical
map uses `CarrierLayout::symbol_roles` with the actual snapshot symbol and frozen
run settings; Data locations may carry transported NCP. Relative bin power is
not calibrated channel power or PSD. Mapped Data/NCP retain green/red roles.
Spectrum Analyzer, captioned TX Output, opens the existing shared analyzer; the redundant lower
Spectrum Analyzer chip is removed. Preview queries run only on dialog open or
result-token changes. Existing retained limits, simulation inputs and WB JSON
are unchanged. See the [design](development/features/transmitter-graphs/design.md)
and [user guide](user/transmitter-workbench.md).

Channel readiness preparation uses the pure
`inputs::channel_model::preview_inputs` helper with the supplied input snapshot.
Background workbench loading must not consult GUI thread-local sessions; the
GUI-only preview wrapper delegates to the same implementation. A restored valid
workbench can therefore enable Start immediately after model adoption.

Instrument selection at all channel boundaries shares the graphical
`instruments/selection/InstrumentChoice.qml` tiles: a spectrum navigation glyph
and a constellation glyph in matching rounded graph cards, with the analyzer
name above each icon and no signal-boundary caption. Focus and routes are
unchanged. These
icons are navigation symbols rather than live measurements. FEC control-channel
statistics use a compact, width-bounded Channel / Metric / Value table; the
underlying whole-run counters are unchanged.

### Command-line workbench startup and diagnostics

UI-WB-OPEN-04 parses `--workbench FILE|-` before Qt starts and consumes the
request once after the window is ready. File and standard-input reads share the
bounded background loader and atomic adoption; file inputs retain their Save
destination, stdin inputs clear it. Update recovery cannot be combined with an
explicit startup workbench. Opening never starts a simulation.

Normal loading shows only phase/activity/Cancel. `--debug-ui` enables the elapsed
label and desktop debug records; RUST_LOG independently overrides stderr logging.
The desktop owns its logger, with a quiet warn default. Lifecycle records contain
events and scalar timing, not capture contents or paths. The maintained inventory
is [desktop debug controls](development/debugging.md). WB JSON is unchanged.

The dark workbench header uses a separate transparent logo asset to blend into
the themed surface while retaining the original source. Workbench-level QAM
legends are removed; symbol-role legends remain inside their instrument views.
The capture editor retains its functions while generic instruction text is
removed; Preview symbol supplies hover and keyboard-focus help instead.

### Expanded symbol timing (UI-SA-TIMING-02)

The shared SA timing panel opens a resizable Symbol Timing popup. A bounded
Rust projection in `instruments/spectrum/symbol_timing.rs` reads the actual core
TransmitWindow coefficients for two adjacent symbols and their overlap; QML
only renders coordinates, labels and shading. The overview domain is fixed per
FFT size. Detail separates overlap from usable flat guard and labels CP end /
FFT start; its fixed sample span includes the maximum CP relative to the second
symbol start. Tapers retain exact half-sample weights and zero outside support.

The transmitter analyzer retains accepted OCD geometry independently of sample
generation and exposes a configuration revision. Timing uses completed-run
settings, then transmitter-preview settings, or accepted current configuration
when no result remains; every view labels its source. Configuration revision
changes refresh timing without serializing plots on every polling tick. VF
affects only distance estimates. Core windowing, simulation results and WB JSON
are unchanged. See the [design](development/features/symbol-timing-popup/design.md).

### Nearest-point MER estimator (UI-QAM-MER-01)

The Apache core `signal_processing::modulation::mer` adapts PNM Viewer's power
accumulator and nearest-reference row search. Its typed `MerCalculator` and
paired `calculate_mer` operation are independent of Qt and future bindings.
QAM retains this as a secondary estimate; UI-QAM-RUNS-02 supplies the primary
matched TX-reference reading.
Receiver and channel-output QAM adapters calculate over all filtered retained
cells before the scatter cap. Raw and equalized estimates remain separate;
known pilot amplitude is explicit, and received gain/phase are never fitted.
This decision-directed estimate can overstate MER under severe noise; it is
neither imported RxMER nor the existing TX-truth per-carrier diagnostic.
QML displays labels/status and smaller markers without changing simulation or
WB persistence. Mixed unclassified TX-bin snapshots do not imply a valid MER.
See the [design](development/features/qam-mer/design.md).

### Restore workbench inputs (UI-INPUT-RESTORE-01)

DPD, RxMER and Channel Estimate use the existing Rust draft `begin` operations
for Restore Workbench. They recover accepted captures/options from the current
workbench, including WB-file restoration, without rereading source paths or
applying a pending draft. QML resets only transient views. DPD/RxMER disable the
action without accepted captures; channel response can restore flat/unity.
Production hard-coded demo loaders are removed. File Open and bundled example
workbenches remain available. See the
[design](development/features/restore-workbench-inputs/design.md).

### Run-specific QAM and reference MER (UI-QAM-RUNS-02)

Payload/NCP paired diagnostic transport now retains optional matched physical
TX references; protected pairs attach references after reception. Neither path
supplies TX truth to a receiver stage. The core streaming paired MER operation
measures original received-reference error; the nearest-point estimate remains
separate and may be optimistic under dense/noisy QAM.

`receiver/qam_history` retains bounded compact per-run constellations and decoder
LLRs and the frozen receiver mode, keyed by original captured profile ID,
separately from wire ID0. QAM labels its actual CMCE/PE/MCE correction from that
selected run, never from current editable settings. No full
job/waveform archive is retained. Exact run selection serves QAM/channel-output/
slicer; unavailable choices never fall back. Latest keeps its full original
capture, archives advertise their actual retained population, and accepted input
changes/Start clear both histories. Spectrum and channel-response instruments retain latest-run scope; FEC and
Profile Performance keep their existing per-profile selection.
See the [design](development/features/qam-run-results/design.md) for resource caps.

### RXMER-DESIGN-01: designed RxMER and profile PLC overlays

`signal_processing::rxmer_designer` owns the bounded, deterministic dB editing
recipe. It preserves exact frequency grids and absent values and does not model
RF phase, infer VSWR-to-MER depth, consume RNG draws or replace playback rules.
The Qt Rust measurement adapter retains immutable imported baselines, or an
explicit flat template derived from the accepted profile. A version1 MERLIN
JSON envelope carries baseline provenance, recipe and collection position. It is
not a PNM binary schema. Existing bounded measurement sections embed these records
in WB files and the existing playback path consumes the derived frames normally.

The GUI holds only recipe controls, scalar metadata and bounded before/after
curves on a common axis. Rust generates batches with capture progress and
cancellation; only complete batches with unchanged draft/profile identities are
staged. Parent Apply remains the scientific admission boundary. Designer and
parent Cancel retain their separate accepted-state guarantees. Reopening a
coherent collection restores its original baseline and recipe without compounding.

`pnm-input::rxmer_export` owns guarded PNM 1.0 encoding and full-precision
decoded-PNM JSON serialization, summaries and create-new filesystem publication.
Designer Save materializes an immutable complete-grid recipe snapshot on a Rust
worker and displays separate binary/JSON admission. Both the Designer preflight
and crate write boundary validate. PNM alone has explicit quarter-dB rounding and
reject/clamp conversion; JSON preserves finite values/nulls. Native pickers and
QML never encode data. Single files get provenance sidecars; collections get one
manifest. Export does not Apply or run simulation. Optional Python wraps the same
crate operations. See the [save design](development/features/rxmer-designer/pnm-export-plan.md)
for publication/cancellation/platform boundaries.

Rust derives one continuous PLC overlay from the first and last PLC assignments
in the accepted modulation profile. It is clipped to the source viewport and
omitted for incompatible identity/grid. QML shades beneath the curves; no carrier
positions are inferred from measurement magnitudes. Main RxMER, animation and
Designer use the same overlay helper. Designer requests carry bounded source
start/count coordinates through the Qt adapter, with both curves decimated and
scaled together over that window. Zoom to PLC derives a clipped context window
from the accepted profile; Full Span and shared pointer/keyboard navigation
change only the view, and capture/recipe edits retain zoom. See the
[design](development/features/rxmer-designer/design.md) and
[user guide](user/measurement-inputs/rxmer-designer.md).

### UI-MSI-01: Windows desktop delivery

Windows x64 adds a Qt desktop MSI alongside the Ubuntu desktop and Linux CLI.
No Windows CLI package is part of the release inventory; macOS remains deferred.
`tools/packaging/desktop_payload.py` shares the existing public examples/docs
and full Cargo/Qt notices between native desktop builders. Windows staging
uses windeployqt, app-local MSVC CRT and WiX3 with stable desktop-only upgrade
identity, executable/installer icons and optional unchecked desktop shortcut.
The native Qt boundary conditionally links GTK on Linux and Windows Common Item
Dialogs on Windows. Scientific libraries and capture ownership are unchanged.

Linux APT update/recovery code is compiled only for Linux. Windows preferences
and WB update recovery use LOCALAPPDATA. Update now downloads the exact verified
stable MSI through the shared bounded HTTPS transport. A fixed normal-user
PowerShell helper checks registered MSI/component ownership and package identity,
waits for the original app to exit after a ready/commit handshake, elevates only
msiexec through UAC and relaunches MERLIN normally with accepted WB recovery.
The Internet-zone marker is retained; execution policies and OS security are
unchanged. Installer codes/logs survive outside staging; cancellation/failure
reopens the previous app when available. A 3010 result explicitly requests a
later Windows restart, never an automatic reboot. Native Windows updater
upgrade/UAC/cancel/recovery validation remains pending under UI-WIN-UPDATE-01.
The shared workflow adds native Windows adapter/QML checks; tag publication
requires Windows MSI installed lifecycle checks alongside every existing
Ubuntu/security gate. The GPL platform adapter selects palette-aware Fusion
controls through QQuickStyle before loading QML, independently of launch-shell
style hints; native OS file pickers keep their native controls.

v0.2.7 passes hosted Windows adapter/QML/build/MSI and
installed icon/runtime/repair/shortcut/uninstall checks. Interactive Windows 11
and cross-version upgrade/downgrade review remain pending under UI-MSI-09. [Design](development/features/windows-desktop/design.md).

### EXAMPLES-01: separate CLI example requests

Standalone CLI requests and their sample file payload live under `examples/cli/`
with a file/use-case/command README. UI-ready saved state stays under
`examples/workbench/`; measurement collections remain shared data under
`examples/measurements/`. The top-level example index explains these distinctions.
Desktop staging uses the explicit workbench/measurement allowlist and excludes
CLI requests. CLI archives retain all public examples. No input schema or
scientific bytes change. [Design](development/features/example-layout/design.md).

### API-RUST-01: Basic Rust API and optional native adapters

`crates/ofdm/src/api` provides a high-level API around existing scenario/profile validation,
`MeasurementPlayback` residual-power preparation and `run_channel` jobs. It
owns immutable inputs, OCD resolution, controller-scoped collections, static
selection/preflight and owned scoped reports. Scientific processing remains in
existing engine modules. Only `run` dispatches experiments. Existing CLI and Qt
paths remain unchanged; neither is a dependency of this high-level API.

`pnm-input` independently calls pinned Apache `pnm` 0.3.6 for supported
binary/JSON profile, RxMER and channel-response conversion, with bounded I/O
and separate original/intermediate provenance. Core `Dpd::profile_ids_from_bytes`
validates decoded catalogs and returns sorted `ProfileId` values;
`CaptureLoader::profile_ids` adds native binary decoding. Python discovery
delegates to this Rust path and never probes or parses profile inventories.
Loaded DPDs retain the validated sorted catalog; Rust `profile_ids()` and Python
`profile_ids` read that member without I/O or decoding, separately from the
selected profile. Created profiles have a singleton catalog and edited profiles
preserve their source catalog.
No GPL Qt orchestration is copied.
The separate optional Python adapter calls this same Rust API after its Rust
validation gate. It releases the interpreter lock, preserves cooperative
cancellation and owns independent result snapshots.
[Design](development/features/high-level-api/design.md),
[user guide](user/api/README.md),
[runnable Rust/Python examples](../examples/api/README.md), and
[downstream Python installation](user/python/integration.md). The separate
`tools/python/update_integration_doc.py` generator refreshes installation pins
on saves/pushes; bump releases include the generated guide in their validated
commit and exact recovery contract. Existing-tag retries remain read-only.
Local native validation and artifact evidence
are recorded in the [feature validation report](development/features/high-level-api/validation.md).

The public Rust `EvaluationOptions.receiver_mode` and matching PyO3 option pass
through the existing receiver enum, independently of applied H selection. The
supplied-channel default remains unchanged. PilotEstimated and MerlinChannelEstimated
use the same existing Rust receiver operations as Qt; no Python estimation or Qt
orchestration enters the library. Reports retain the selected mode and accurate
assumptions. The collection example now selects MCE while retaining the curated
WB's other settings; the saved WB uses CmChannelEstimated. The regression comparison
uses the separate run_multi_rxmer_cm_chan_est.py entry point with supplied-channel mode to preserve
its UI reference. Matching Rust collection entry points share the documented
`examples/api/rust/support/multi_rxmer.rs` workflow, with identical profile
discovery, capture inputs, settings and actual FEC/BER reporting.
Both Python entry points share one workflow through an explicit
receiver-mode argument; no mutable global override or duplicate engine is used. See [receiver parity](development/features/high-level-api/design.md#api-receiver-01-match-receiver-correction-across-rust-python-and-qt).

The MCE typed variant is `MerlinChannelEstimated` (Python uppercase alias
`MERLIN_CHANNEL_ESTIMATED`). Serde retains `merlin_estimated` as the canonical
saved value and accepts `merlin_channel_estimated` as an input alias, preserving
Qt and existing workbench/CLI compatibility without duplicate receiver modes.

The matching `run_designed_rxmer` Rust/Python examples use the custom synthetic
RxMER binary under `examples/measurements/designed-rxmer/` directly against
every discovered compatible single-bundle profile.
They retain original bytes and keep the adjacent designer recipe as provenance;
binary loading does not apply the recipe or automatically ingest the sidecar.

The development-only generated API reference uses rustdoc for the high-level
Rust modules and Sphinx autodoc against the actual native Python extension.
Class/member coverage is validated without constructing controllers or running
simulation. Artifacts remain under ignored `local/api-reference/`; the build
adds no runtime or Qt dependencies. See the
[reference build guide](reference/api-reference.md).

### Saved workbench API input

The Apache `pnm-input::Workbench` loader independently decodes the saved WB
schema 1/2 contract, validates embedded originals and preflights one existing
high-level controller per selected profile. It depends on the Rust engine and
PNM decoder only. No GPL Qt orchestration or Qt types enter the library.
Saved RxMER Designer version-1 envelopes are decoded by the Apache workbench
input adapter using the existing Rust `RxmerRecipe::apply`. Both flat and
imported baselines preserve exact grids, gaps, timestamps and original envelope
bytes. Typed `RxMer::from_frame` admission retains a separate validated frame
intermediate without fabricating a PNM capture header. The
Python `Workbench` adapter reuses those controllers and the existing GIL-releasing
execution/cancellation path; it performs no scientific interpretation.

Core `ProcessingOptions` carries existing MCE, channel-filter, additional-loss
and power policies into the same engine jobs. Owned options/reports preserve
those policies; Python exposes a copied inspection dictionary. WB bytes and
capture provenance remain immutable. Loading never runs simulation. Results are
one owned batch per profile, with identifiers scoped to each controller.

This milestone imports accepted PRBS15 configurations with static single/average
RxMER. File sources, timeline playback and incomplete editors fail explicitly;
there is no silent fallback or arbitrary WB-setting approximation. See the
[WB user/API contract](user/python/workbench.md) and
[design](development/features/high-level-api/design.md#api-wb-01-native-saved-workbench-import).

### API-WHEEL-01: native Python release validation

Release-only Ubuntu 22.04/Windows 2022 x64 jobs build CPython 3.10 wheels through
the shared notice-preserving native helper. The wheel matrix selects Bash on
Ubuntu and PowerShell on Windows to preserve the MSVC linker path. A development
test harness installs one wheel into a clean temporary venv and executes the real single-capture API
example outside the checkout, checking native identity and actual FEC/BER. This
adds no simulation logic to Python or Qt dependencies. Only tested wheel/checksum
artifacts enter release collection; both platforms and existing security/native
gates are mandatory. Windows remains unverified until its hosted job passes.
See the [separate wheel release guide](user/python/releases.md).

### Font-aware Designer notch controls (RXMER-DESIGN-12)

The Qt Designer notch grid compares its available width with the measured label,
field and arrow widths. It uses two groups per row when they fit, otherwise one
group per row, keeping mouse targets within the clipped effects editor. This is
presentation only; existing Rust validation, frequency units and recipe semantics
are unchanged. [Design](development/features/rxmer-designer/design.md#rxmer-design-12-font-aware-notch-field-layout).


### Ubuntu 22.04+ released-package compatibility

Keep Ubuntu 22.04 as the release build baseline and preserve artifact names.
The release-only `ubuntu-compatibility` job depends on the desktop and Python
wheel builders and tests their checksummed artifacts on Ubuntu 24.04 and 26.04.
It runs the real Python example and desktop installed lifecycle checks without
building replacement artifacts. Reusable-workflow completion gates publication;
existing three-version core tests and Windows/security checks remain mandatory.
New hosted compatibility execution is pending; no local installation is required.


### API-OCD-01: captured-profile bandwidth resolution

Basic Rust API OCD construction derives bandwidth only when exactly one
supported width contains the complete captured grid span. The Python binding
uses the same resolution. Ambiguous smaller spans still require an override;
manual OCD configuration remains explicit. Derived, explicit and defaulted
origins remain distinct; loading and resolution never run the simulation.


### API-PROFILES-01: retained profile inventories

The core controller owns compatible DPD collections and all-profile preflight/
dispatch; pnm-input owns file/decoder conveniences. Python forwards to those
Rust operations through the existing GIL-releasing worker. Loading selects the
first profile without simulation. All-profile execution preserves current
selection and adds profile identity to every result, including not-started
entries. Existing single-profile workflows remain valid. Bounds are 16 profiles,
256 combined experiments and the existing 64 MiB retained capture budget.

### API-PROTOCOL-01 — Default profile-capture loading

Rust owns the DOCSIS 3.1 protocol default. The capture convenience loader and
optional Python protocol argument share it; explicit protocol and OCD overrides
remain supported. Loading never starts simulation.

### API-SIMULATION-01 — Idle session entry point

Simulation owns an optional existing Merlin controller; it never constructs dummy
channel geometry. Decoded profile loading initializes or atomically replaces
configuration while retaining compatible measurements and their scoped IDs.
Python adapts file loading and delegates to this Rust session with existing worker
and cancellation semantics. Legacy Rust Merlin and the Python Merlin alias remain.
Basic examples use create/load/configure/run; advanced OCD and Workbench remain.
Example timing is emitted by shared Rust checks and native wheel validation.

### API-CMCE-EXAMPLE-01 — Explicit reference correction examples

The paired Rust/Python single-capture CM channel-estimate examples explicitly
select the captured channel response and CmChannelEstimated receiver correction.
They use the existing engine without additional calculations or API changes.
Shared timed Rust checks and native wheel release checks exercise this workflow.

### API-CMCE-NAME-01 — CM receiver enum names

CmChannelEstimated / CM_CHANNEL_ESTIMATED are the preferred Rust/Python API
names. Compatibility aliases retain existing source callers; serialized
supplied_channel remains stable, with cm_channel_estimated accepted on input.
Receiver algorithms, defaults and saved workbench behavior are unchanged.

### API-CPA-01 — Common Path Attenuator controls

Core PowerConditioning owns CPA loss (`cpa_attenuation_db`). Basic Rust controller
and Simulation setters validate candidate power snapshots before acceptance;
Python dispatches directly to those methods. No DSP is duplicated, and existing
JSON/Qt transport retains `attenuation_db`. Getter values are effective gain and
accepted CPA loss; configuration never runs the engine.

### Python-only installation

`install.sh --install-pymerlin` delegates to `scripts/python-api/`; source builds
reuse `tools/python/build_wheel.py`. Release installation verifies SHA256SUMS.
Both routes install into a virtual environment and remain independent of Qt and
the CLI. See the [installer guide](../scripts/python-api/README.md).

The source installer accepts host CPython 3.10+ without an Ubuntu release
allowlist. Shared CI/CD validates source installation and a version-only import
on Ubuntu 22.04/24.04/26.04 and Windows. This packaging path retains the Python
adapter → Basic Rust API → engine boundary; no simulation is used by its smoke.

### RXMER-DESIGN-13: synthetic starting variation

The Rust RxMER recipe optionally applies independent seeded Gaussian dB offsets
before tilt, ripple, notch and value replacements. Qt admits this option only
for flat synthetic baselines and rejects imported-baseline requests/envelopes
that enable it. Capture-indexed ChaCha8 streams preserve repeatability and gaps;
the designer never consumes simulation RNG state. The existing saved envelope
retains deviation and seed, with absent options preserving legacy behavior.
See [Designer workflow](user/measurement-inputs/rxmer-designer.md).

### Update failure diagnostics

The Qt update adapter builds native Windows path components and normalizes
installer arguments before MSI handoff. Rust owns bounded installer-log reads,
UTF-16 decoding, user-profile-path redaction and issue report construction. QML
handles the explicit clipboard action and opens the fixed GitHub form; the user
reviews and submits it. Logs are never embedded in URLs or workbench JSON.

### Receiver soft-decision presentation

The Qt Rust adapter projects retained LLRs into bounded histogram bars and
min/max decoder-index groups, and computes selected-symbol Voronoi regions
from existing constellation references. QML renders these in Symbol Decisions,
Bit Confidence and Decoder Input tabs. Endpoint counts preserve the distinction
between demapper limits and the display range. No hard decisions feed the FEC
engine, no LLRs are synthesized, and no saved-workbench/API schema changes.
