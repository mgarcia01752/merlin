> **Planning reference — saved 2026-11-20.**
> The full user-supplied prompt is preserved below. Its phases and final
> acceptance criteria describe intended work, not current MERLIN capability.
> Source-backend completeness and the pinned source revision still require
> baseline verification. The `merge-merlin-morgana` branch is already prepared;
> inspect and reuse it rather than recreating it. Saving this guide does not
> start implementation. The current architecture and official checklist remain
> the implementation/status owners until reviewed integration plans update them.

# MERLIN — Unified DOCSIS OFDM/OFDMA PHY Integration

## MASTER CODEX IMPLEMENTATION PROMPT

### Role and objective

Act as a principal Rust software architect, DSP engineer, and DOCSIS 3.1/4.0 PHY specialist.

Your assignment is to integrate a completed upstream OFDMA simulation backend into the existing MERLIN downstream OFDM simulation toolkit.

The final product must be **MERLIN**, a unified, modular, scientifically validated DOCSIS PHY simulation toolkit supporting:

- **Downstream OFDM:** CMTS transmitter → channel → cable modem receiver.
- **Upstream OFDMA:** Cable modem transmitter → channel → CMTS receiver.

The implementation must maximize legitimate code reuse, minimize duplication, preserve normative DOCSIS PHY behavior, and maintain existing downstream compatibility.

This is a backend integration. Do not implement Qt Workbench functionality during this task.

---

# 1. Source repositories and baseline

## Target repository

MERLIN:

https://github.com/mgarcia01752/merlin

MERLIN is the authoritative target repository.

## Source repository

Upstream OFDMA implementation:

https://github.com/PyPNMApps/morgana

Use source commit:

`69b38071cd3ebebc4eb96849f82b7871ab8da5ec`

This is the baseline containing the completed backend delivery checklist.

Verify that this commit exists and inspect its actual contents before integration.

Do not assume later commits are automatically included.

## Baseline requirements

Before modifying code:

1. Inspect both repositories.
2. Record the exact source commit SHAs.
3. Check working-tree cleanliness.
4. Review all relevant Cargo manifests and dependency versions.
5. Review existing Rust and Python public interfaces.
6. Review the current architecture, requirements, checklists, and validation records.
7. Identify existing downstream regression tests.
8. Identify upstream reference fixtures and independent validation tests.
9. Verify licenses and applicable third-party obligations.
10. Establish a documented reproducible baseline.

Do not begin by copying the entire source repository.

---

# 2. Mandatory Git branch

All integration work must occur in a dedicated MERLIN branch.

**Required branch name:**

`merge-merlin-morgana`

Follow this sequence:

```bash
git status --short
git fetch origin
git switch master
git pull --ff-only origin master
git switch -c merge-merlin-morgana
git branch --show-current
git status --short
```

These are illustrative commands; first inspect the environment and adapt safely.

If the branch already exists, inspect its history and working state instead of overwriting or recreating it.

Do not discard uncommitted work.

Do not modify MERLIN's `master`.

Do not modify the upstream source repository.

Do not automatically merge the completed branch into `master`.

Do not push, publish, release, or create a pull request unless explicitly authorized.

---

# 3. Final product identity

The final integrated toolkit is **MERLIN**.

The upstream source project was a temporary development repository, not a separately published product that needs to survive the integration.

The final MERLIN implementation must not retain that former project as a distinct product.

### Remove the former project identity

The final codebase must not contain:

- A `morgana` Cargo crate.
- A `morgana-phy` Cargo crate.
- A `morgana-ldpc` Cargo crate.
- A `Morgana` public Rust controller.
- A separate upstream Python distribution.
- Python imports under the former project's name.
- Separate upstream CLI executables retaining that name.
- Former-project logos or desktop assets.
- Former-project branding in the README.
- Former-project references in user-facing documentation.
- Former-project references in installer metadata.
- Unnecessary source comments describing the historical project migration.

Use functional naming instead:

- `ofdma`
- `upstream`
- `upstream_ldpc`
- `upstream_transmitter`
- `upstream_receiver`
- `upstream_configuration`

The branch name is an administrative exception.

### Licensing

Do not introduce unnecessary branding or product-history notices.

However, preserve legally required copyright notices, third-party attribution, license texts, and redistribution obligations.

Preserve MERLIN's existing Apache/MIT scientific-code obligations and its GPL-3.0-only Qt application boundary.

Never remove required attribution merely to eliminate historical branding.

---

# 4. Architectural direction

The final implementation should use two PHY crates with shared scientific infrastructure.

## Proposed Cargo organization

```text
MERLIN/
├── crates/
│   ├── dsp/
│   │   ├── src/
│   │   │   ├── complex/
│   │   │   ├── fft/
│   │   │   ├── modulation/
│   │   │   ├── filtering/
│   │   │   ├── noise/
│   │   │   ├── channel/
│   │   │   └── measurements/
│   │   └── Cargo.toml
│   │
│   ├── fec-codec/
│   │   ├── src/
│   │   │   ├── ldpc/
│   │   │   ├── shortening/
│   │   │   ├── bch/
│   │   │   ├── plc/
│   │   │   └── ncp/
│   │   └── Cargo.toml
│   │
│   ├── ofdm/
│   │   ├── src/
│   │   │   ├── configuration/
│   │   │   ├── transmission/
│   │   │   ├── signal_processing/
│   │   │   ├── receiver/
│   │   │   ├── simulation/
│   │   │   └── api/
│   │   └── Cargo.toml
│   │
│   ├── ofdma/
│   │   ├── src/
│   │   │   ├── configuration/
│   │   │   ├── source/
│   │   │   ├── grants/
│   │   │   ├── transmission/
│   │   │   ├── resources/
│   │   │   ├── pilots/
│   │   │   ├── pre_equalization/
│   │   │   ├── receiver/
│   │   │   ├── simulation/
│   │   │   └── api/
│   │   └── Cargo.toml
│   │
│   ├── pnm-input/
│   └── cli/
│
├── bindings/
│   └── python/
│
├── apps/
│   └── qt/
│
├── docs/
├── examples/
├── tests/
└── Cargo.toml
```

This is a conceptual target.

Do not blindly restructure existing code to match this tree.

Prefer retaining established module paths whenever doing so avoids unnecessary changes.

A crate must have a meaningful dependency boundary. Do not introduce additional crates solely for organizational symmetry.

---

# 5. Crate responsibilities

## `dsp` — Shared scientific algorithms

Create a reusable DSP crate if the detailed reuse analysis confirms it provides meaningful value.

It should contain implementation-independent numerical operations such as:

- Complex IQ arithmetic.
- FFT/IFFT primitives.
- Numerical QAM utilities.
- Constellation distance calculations.
- Soft metric arithmetic.
- Filtering.
- AWGN generation.
- Frequency-response calculations.
- Generic channel arithmetic.
- Numerical MER calculations.
- Shared observation primitives.

It must not contain DOCSIS-specific pilots, grants, PLC/NCP signaling, or upstream/downstream processing state machines.

Do not create redundant abstractions around small functions that are already appropriately shared.

## `fec-codec` — Common FEC infrastructure

Retain MERLIN's existing `fec-codec` crate.

Consolidate compatible mechanisms for:

- Sparse parity-check matrices.
- LDPC encoder construction.
- LDPC decoding.
- Shortening.
- Syndrome verification.
- Decoder convergence reporting.
- Iteration accounting.
- Error metrics.

Preserve direction-specific normative code definitions.

### Downstream FEC

Retain:

- Existing downstream LDPC definitions.
- BCH outer coding.
- PLC/NCP coding.
- Existing downstream shortening behavior.

### Upstream FEC

Integrate:

- Long QC-LDPC: N=16,200, K=14,400.
- Medium QC-LDPC: N=5,940, K=5,040.
- Short QC-LDPC: N=1,120, K=840.
- Validated upstream parity-check matrices.
- Upstream shortening and codeword selection.

Preserve independent normative matrix tests.

Do not substitute downstream LDPC matrices based on matching code dimensions.

Do not add downstream BCH processing to upstream scheduled-data decoding.

## `ofdm` — Downstream DOCSIS PHY

Retain MERLIN's existing downstream engine.

Its current Cargo package is named `docsis-ofdm-fec`.

Preserve this package name during initial integration unless an explicit compatibility plan justifies changing it.

Keep downstream-specific:

- OFDM resource allocation.
- PLC/NCP.
- Downstream pilots.
- Downstream interleaving.
- Downstream frame and waveform configuration.
- Downstream channel-estimation behavior.
- Downstream receiver processing.
- Existing simulation APIs and reports.

Avoid a broad downstream rewrite.

## `ofdma` — Upstream DOCSIS PHY

Create a dedicated upstream PHY crate.

Integrate the upstream processing components into this crate without preserving the previous project's crate structure.

It owns:

- UCD/IUC configuration.
- Synthetic MAP/grant interpretation.
- Upstream scheduling and resource allocation.
- Upstream LDPC codeword selection.
- PHY randomization.
- Upstream QAM mapping rules.
- Upstream interleaving.
- OFDMA pilot patterns.
- Burst generation.
- Pre-equalization.
- CMTS receiver acquisition and synchronization.
- Receiver pilot estimation.
- Equalization.
- Soft demapping.
- FEC decoding.
- Independent payload recovery and verification.
- Upstream simulation orchestration.
- Upstream-specific reports.

Use common DSP and FEC implementations wherever compatible.

Preserve the actual normative upstream processing order.

## `pnm-input` — Measurement adapters

Retain MERLIN's existing `pnm-input` crate.

Expand it to provide appropriate downstream and upstream capture admission.

Reuse the already pinned PNM Viewer Toolkit dependency where suitable.

Avoid implementing a second parser for a format already correctly supported by the existing dependency.

Keep binary parsing separate from simulation.

---

# 6. Detailed code reuse audit

Before moving implementations, inspect actual algorithms rather than merely comparing filenames.

For each relevant function, classify it as:

- **SHARE:** A common implementation can serve both PHY directions.
- **REUSE:** An existing component can be used directly.
- **ADAPT:** A common kernel can be reused with PHY-specific configuration.
- **SEPARATE:** Normative or operational differences require distinct implementations.
- **DEFER:** Consolidation is not justified yet.

Produce a source-to-destination map with exact paths.

## 6.1 FFT/IFFT

Inspect MERLIN's waveform processing under:

`crates/ofdm/src/signal_processing/waveform/`

Compare with the upstream implementation's transform and IQ modules.

Verify:

- Complex representation.
- FFT direction.
- Forward/inverse normalization.
- Frequency-bin indexing.
- Phase convention.
- Transform dimensions.
- Numerical tolerance.
- Memory allocation.
- Execution performance.

Use a shared transform implementation only after proving compatibility.

Keep CP, rolloff, windowing, and burst assembly rules in the appropriate PHY crate.

## 6.2 QAM and soft metrics

Inspect MERLIN's modulation and soft-decision modules.

Compare with the upstream QAM mapping, tables, and demapping.

Reuse generic arithmetic:

- Constellation distance.
- Complex point handling.
- Slicing.
- Soft decision calculations.
- LLR numerical utilities.
- MER calculations.

Keep DOCSIS-specific constellation labeling, bit ordering, and normalization correct for each direction.

Do not presume that identical QAM orders imply identical normative mapping behavior.

## 6.3 Channel modeling

Compare both channel simulators.

Share compatible numerical kernels for:

- AWGN.
- FIR filtering.
- Frequency-selective response.
- Phase rotation.
- Carrier-frequency offset.
- Deterministic noise generation.
- Power normalization.

Preserve separate configuration and reference-plane semantics when required.

Upstream must retain the distinction:

`Y[k] = X[k] C[k] H[k] + N[k]`

Where:

- `C[k]` is transmitter Pre-Eq.
- `H[k]` is physical channel response.
- `N[k]` is impairment/noise.
- `Y[k]` is receiver observation.

The CMTS receiver must not have privileged access to simulator truth.

## 6.4 PRBS

Compare existing PRBS15 implementations.

Verify exact polynomial, sequence phase, seed behavior, bit ordering, chunk continuity, and independent source comparison.

Consolidate only if reference-vector behavior is equivalent.

## 6.5 Channel estimation and equalization

Identify shared mathematical kernels for pilot-based estimation, interpolation, equalization, and confidence handling.

Keep upstream and downstream pilot structures and receiver state machines separate.

Do not produce a universal receiver that depends on extensive direction-specific conditional logic.

## 6.6 Interleaving and pilots

Share only proven generic permutation/sequence primitives.

Preserve direction-specific normative patterns.

Do not reuse downstream pilot or interleaving tables for upstream merely to reduce duplicate code.

---

# 7. Public Rust API convergence

Rust is the authoritative scientific and simulation implementation.

The unified MERLIN Rust API must support both directions consistently.

Do not retain the former upstream product identity as a public Rust API.

Preserve the existing downstream MERLIN controller behavior.

Evaluate a unified entry point similar to:

```rust
// Illustrative API only

let downstream = Merlin::downstream(ds_config)?;
let upstream = Merlin::upstream(us_config)?;

let ds_report = downstream.run()?;
let us_report = upstream.run()?;
```

Alternatively, consider distinct controller types with shared operation conventions:

- `DownstreamSimulation`
- `UpstreamSimulation`

Choose the design that best preserves backwards compatibility and idiomatic Rust usage.

### Common API contracts

Converge:

- Configuration validation.
- Simulation lifecycle.
- Run initiation.
- Progress.
- Pause/resume.
- Cancellation.
- Execution limits.
- Error reporting.
- Report metadata.
- Serialization.
- Source provenance.

Do not force upstream and downstream into one giant configuration structure.

Keep direction-specific parameters typed and independently validated.

### Compatibility

Existing MERLIN Rust API consumers must continue working.

Document any required migration.

Do not silently change established method signatures or report schemas.

---

# 8. Native Python API convergence

Preserve MERLIN's existing Python package.

Do not create or retain a second published Python distribution.

The Python interface must wrap the canonical Rust implementation through PyO3.

Do not implement scientific processing in Python.

Evaluate a consistent interface such as:

```python
from merlin import DownstreamSimulation, UpstreamSimulation

downstream = DownstreamSimulation(ds_config)
upstream = UpstreamSimulation(us_config)

ds_report = downstream.run()
us_report = upstream.run()
```

This is illustrative.

Preserve existing MERLIN Python imports and behavior.

Use direction-specific configuration objects where required.

### Python acceptance

Validate:

- Rust/Python configuration parity.
- Report parity.
- Error-category consistency.
- Progress and lifecycle behavior.
- Cancellation.
- JSON serialization.
- Original input ownership.
- Resource limits.
- Optional wheel construction.

Avoid creating two independent schema systems.

---

# 9. CLI integration

Retain MERLIN's existing CLI package and executable.

Integrate upstream commands into the existing CLI.

Use clear functional command names, such as:

```text
ofdm-fec downstream ...
ofdm-fec upstream ...
```

The exact CLI design should preserve existing command compatibility.

Do not overwrite existing subcommands.

Do not retain a second CLI executable.

The CLI must call the unified Rust APIs and must not contain independent PHY algorithms.

---

# 10. PNM integration

Inspect MERLIN's current PNM adapter and pinned parser dependency.

Determine which upstream formats are already supported and which require additional adapters.

Relevant inputs include:

- Downstream RxMER.
- Downstream channel estimation.
- Downstream modulation profiles.
- Upstream CCAP RxMER.
- Upstream OFDMA Pre-Eq.
- Captured UCD configuration.

Preserve:

- Original input bytes.
- Capture hashes.
- CM/CMTS/channel identities.
- Frequency and subcarrier mappings.
- Scaling and units.
- Timestamps and update order.
- Missing-value semantics.
- Coefficient provenance.
- Explicit association requirements.

Do not silently infer missing metadata.

Do not confuse measured channel estimates with simulated channel truth.

Preserve upstream Pre-Eq set/update semantics and independently validated coefficient handling.

---

# 11. Simulation validation

Preserve the existing regression suites from both codebases.

Do not delete original reference fixtures until equivalent integrated tests pass.

## Downstream acceptance

Validate:

- Existing MERLIN API behavior.
- BCH and LDPC.
- PLC/NCP.
- Downstream modulation and interleaving.
- Waveform generation.
- Pilot handling.
- Receiver recovery.
- Channel impairments.
- RxMER and constellation reports.
- Existing examples.
- Python compatibility.

## Upstream acceptance

Validate:

- All three QC-LDPC families.
- Both OFDMA numerologies: 2K/50 kHz and 4K/25 kHz.
- All supported pilot patterns.
- QAM mapping through supported orders.
- Grant and minislot allocation.
- Upstream interleaving.
- PHY randomization.
- Waveform generation.
- Pre-Eq.
- Channel impairments.
- CMTS received-pilot channel estimation.
- Soft demapping.
- LDPC recovery.
- Independent payload verification.
- Measured RxMER ingestion/playback.
- Measured Pre-Eq ingestion/application.
- UCD capture admission.
- Deterministic execution.
- Run cancellation and pause/resume.
- Partial-result reporting.
- Rust/Python API parity.

Preserve the original validation distinction between internally verified simulation behavior and external hardware interoperability.

### Numerical regression policy

Do not silently change golden test vectors.

When numerical behavior changes:

1. Identify the precise cause.
2. Determine whether it is an intentional correction.
3. Validate against an independent reference.
4. Record the numerical difference.
5. Update fixtures only with explicit supporting evidence.

---

# 12. Performance and dependency constraints

The integration must reduce duplication without introducing avoidable runtime overhead.

Avoid:

- Repeated FFT-plan initialization.
- Unnecessary complex-sample copies.
- Per-symbol avoidable heap allocations.
- Repeated LDPC matrix construction.
- Large runtime direction switches inside numerical loops.
- Circular dependencies.
- Duplicate JSON processing.
- Redundant FEC implementations.
- Large monolithic Rust files.

Keep scientific crates independent of PyO3 and Qt.

Preserve the existing Rust toolchain compatibility unless a change is justified.

Do not automatically update pinned dependencies.

Use explicit bounded allocation and execution policies.

Benchmark important numerical kernels when consolidation changes their implementations.

---

# 13. Documentation integration

MERLIN's documentation becomes authoritative for both PHY directions.

Update:

- README.
- Architecture.
- Requirements.
- Development checklist.
- Developer guidance.
- User API documentation.
- Scientific validation records.
- Supported configuration references.
- PNM input documentation.
- Examples and command descriptions.

Preserve scientific source traceability without maintaining a separate former-product identity.

Do not duplicate entire documentation trees unnecessarily.

Avoid references to functionality that is merely planned as though it were implemented.

Retain clear distinctions between:

- Supported runtime capability.
- Internally validated numerical behavior.
- External interoperability.
- Future extensions.

---

# 14. Agent Skills integration

Review existing engineering Agent Skills in both repositories.

Reuse applicable skills for:

- DOCSIS PHY requirements.
- Rust architecture.
- DSP and FEC validation.
- Scientific numerical testing.
- Code reuse.
- API contracts.
- Documentation maintenance.
- Build and regression validation.
- Contribution workflow.

Avoid duplicate or conflicting agent instructions.

Preserve authoritative technical references and specification revision tracking.

Do not distribute proprietary DOCSIS PDFs or private validation data.

---

# 15. Implementation sequence

Follow a controlled, dependency-ordered integration.

## Phase 0 — Baseline and architecture review

Inspect both source revisions, record independent baselines, and produce the final proposed crate architecture.

Deliver:

- Crate inventory.
- Dependency graph.
- Reuse matrix.
- Migration map.
- Compatibility assessment.
- Test plan.

Do not begin source migration until the architecture is reviewed.

## Phase 1 — Branch preparation

Create `merge-merlin-morgana` from MERLIN `master`.

Verify branch isolation.

## Phase 2 — Shared FEC

Integrate upstream LDPC support into MERLIN's existing FEC crate.

Consolidate proven common mechanisms.

Run both FEC regression suites.

## Phase 3 — Shared DSP

Introduce the shared DSP crate where justified.

Migrate validated numerical primitives incrementally.

Run numerical equivalence tests after each extraction.

## Phase 4 — OFDMA PHY

Create `crates/ofdma`.

Integrate upstream configuration, transmitter, channel, receiver, measurements, and simulation logic.

Do not retain former upstream crate identities.

Run the complete upstream reference suite.

## Phase 5 — PNM admission

Consolidate binary capture parsing and measured input association through MERLIN's PNM infrastructure.

Validate both downstream and upstream inputs.

## Phase 6 — Unified APIs

Converge Rust lifecycle and reporting contracts.

Integrate upstream into MERLIN's Python and CLI interfaces.

Preserve downstream compatibility.

## Phase 7 — Full validation

Run complete backend regression suites.

Verify:

- Downstream functionality.
- Upstream functionality.
- API compatibility.
- Determinism.
- Error handling.
- Resource bounds.
- Documentation traceability.

## Phase 8 — Cleanup

Remove redundant implementations and old package identities.

Consolidate dependencies.

Reconcile documentation, examples, skills, tests, and licenses.

Do not remove code merely because it is unused until its replacement is validated.

## Phase 9 — Completion review

Produce a final engineering report and identify any remaining limitations.

Do not merge into `master` without explicit approval.

---

# 16. Required development workflow

Use:

**Architecture → Requirements → Developer Checklist → Implementation → Validation**

For each phase:

1. Define the architectural change.
2. Identify exact files to add, edit, reuse, or remove.
3. State mandatory requirements.
4. Identify dependencies.
5. Establish acceptance criteria.
6. Implement the smallest coherent change.
7. Execute relevant tests.
8. Record observed results.
9. Update the checklist only after acceptance passes.
10. Continue to the next dependency-ordered task.

Do not mark a phase complete solely because code compiles.

Do not replace working functionality with stubs.

Do not weaken tests to obtain passing status.

---

# 17. Final acceptance criteria

The integration is complete when:

- MERLIN retains a fully operational downstream OFDM engine.
- MERLIN contains a fully operational upstream OFDMA engine.
- Common DSP/FEC implementations have been consolidated where valid.
- The two PHY engines remain independently executable.
- Existing downstream APIs remain compatible.
- The unified Rust API exposes both directions.
- MERLIN's native Python package exposes both directions.
- The CLI supports upstream and downstream operations.
- PNM input handling supports the declared measurement formats.
- All existing relevant regression tests pass.
- All upstream reference validation is retained.
- No unwanted former-project runtime or branding identity remains.
- Required source notices remain legally compliant.
- No Qt implementation was introduced.
- No scientific crate depends on Qt or Python.
- All changes remain isolated in `merge-merlin-morgana`.
- No changes have been merged into `master`.

---

# 18. Final deliverables

Provide:

1. Final Cargo workspace and crate dependency diagram.
2. Function-level DSP/FEC reuse report.
3. Source-to-destination migration summary.
4. Rust API documentation.
5. Python API documentation.
6. CLI integration documentation.
7. PNM capture compatibility matrix.
8. Scientific regression results.
9. Performance comparison where applicable.
10. Updated architecture, requirements, and development checklist.
11. List of remaining limitations or deferred capabilities.
12. Final integration acceptance report.

## Final instruction

**The objective is to integrate upstream OFDMA natively into MERLIN, not to package two independent projects together.**

Maintain shared numerical infrastructure, distinct PHY processing pipelines, one coherent public API family, and one MERLIN product identity.

Create and use `merge-merlin-morgana` for all integration changes.

Do not begin Qt UI implementation.

Do not commit or merge into `master`.

The architecture may evolve as actual code reuse and validation reveal better boundaries. Document and justify those changes rather than adhering rigidly to a speculative directory layout.

Deliver a robust, maintainable, and scientifically defensible unified DOCSIS 3.1/4.0 PHY simulation backend.