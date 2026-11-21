# MERLIN — Unified DOCSIS OFDM/OFDMA Backend Integration

## Master Codex Implementation Prompt

### Role

Act as a principal Rust software architect, DSP engineer, DOCSIS 3.1/4.0 PHY specialist, and API design engineer.

Your responsibility is to complete the integration of DOCSIS upstream OFDMA functionality into MERLIN's existing downstream OFDM simulation toolkit.

The final result must be one scientifically validated MERLIN backend with:

- A downstream OFDM PHY engine.
- An upstream OFDMA PHY engine.
- Shared DSP infrastructure.
- Shared FEC infrastructure.
- Unified PNM capture handling.
- Explicitly separated downstream/upstream public Rust APIs.
- A single native Python distribution mirroring those API namespaces.
- Complete requirements-to-implementation-to-validation traceability.

**This is a backend-only integration. Do not implement, restructure, or redesign the Qt Workbench.**

---

# 1. Repository and Git requirements

Target repository:

https://github.com/mgarcia01752/merlin

Required working branch:

`merge-merlin-morgana`

Continue the existing branch. Do not recreate, reset, rename, or rebase it without approval.

Before continuing:

1. Verify the current branch.
2. Inspect working-tree and index status.
3. Preserve all existing changes.
4. Identify the latest completed integration tasks.
5. Read the current architecture, requirements, checklist, feature designs, and validation records.
6. Review current Cargo manifests, public APIs, and integration dependencies.
7. Continue from the first eligible unfinished backend task.

Do not modify `master`.

Do not modify the original upstream development repository.

Do not introduce the former project's product identity into MERLIN's final public API, documentation, packaging, or executable naming.

Preserve all legally required third-party attribution and notices.

Do not merge into `master`, publish releases, or push without explicit authorization.

## Existing completed work

The integration has already established:

- A unified architecture.
- A canonical requirements baseline.
- Downstream requirements backfill.
- Consolidated feature documentation.
- Unified engineering Agent Skills.
- Upstream LDPC foundation integration.
- Source/reuse/licensing review records.
- Existing downstream preservation and traceability records.

Verify the current checklist states. Do not rerun or reopen completed tasks without a documented technical reason.

---

# 2. Mandatory additive-first policy

MERLIN remains the authoritative target.

Preserve all existing downstream functionality, interfaces, numerical results, documented behaviors, and validation history.

Before modifying an existing downstream implementation:

1. Identify the affected source files.
2. Identify public consumers.
3. Identify the associated requirements.
4. Identify applicable regression tests.
5. Explain why the modification is necessary.
6. Establish a before/after validation baseline.
7. Make the smallest compatible change.
8. Verify downstream behavior remains intact.

Prefer additive changes before consolidating duplicate implementations.

Do not remove working implementations until their replacements pass independent equivalence and regression tests.

Do not rewrite existing MERLIN source merely to obtain cosmetic directory symmetry.

---

# 3. Scientific crate architecture

Maintain the intended logical architecture:

```text
MERLIN/
├── crates/
│   ├── dsp/
│   │
│   ├── fec-codec/
│   │
│   ├── ofdm/
│   │
│   ├── ofdma/
│   │
│   ├── pnm-input/
│   │
│   └── cli/
│
├── bindings/
│   └── python/
│
├── apps/
│   └── qt/
│
└── docs/
```

Responsibilities:

| Component | Ownership |
|---|---|
| `dsp` | Common numerical DSP algorithms |
| `fec-codec` | Shared FEC mechanisms and normative code families |
| `ofdm` | Downstream OFDM PHY |
| `ofdma` | Upstream OFDMA PHY |
| `pnm-input` | Binary capture admission and measurement adapters |
| `cli` | Unified MERLIN command-line interface |
| `bindings/python` | One PyO3 distribution |
| `apps/qt` | Existing desktop application, unchanged |

This remains an evolving architecture.

Do not create `dsp` merely for symmetry if existing common numerical ownership is sufficient.

Keep shared scientific crates free of Qt and PyO3 dependencies.

Neither PHY engine may depend on the other PHY engine.

Avoid circular dependencies.

---

# 4. Public Rust API — mandatory direction separation

## Fundamental API rule

**The public Rust API must explicitly separate downstream and upstream operations.**

The intended logical structure is:

```text
merlin
├── downstream/
│   ├── Simulation
│   ├── Configuration
│   ├── ModulationProfile
│   ├── RxMer
│   ├── ChannelEstimate
│   ├── ChannelOptions
│   ├── ReceiverOptions
│   └── Report
│
├── upstream/
│   ├── Simulation
│   ├── Configuration
│   ├── Ucd
│   ├── BurstProfile
│   ├── Grant
│   ├── RxMer
│   ├── PreEqualization
│   ├── ChannelOptions
│   ├── ReceiverOptions
│   └── Report
│
└── common/
    ├── RunControl
    ├── RunLimits
    ├── Progress
    ├── SimulationStatus
    ├── Error
    └── ReportMetadata
```

This is the public design goal, not an instruction to create an unnecessary number of source modules.

Names and exact signatures must be verified against current consumers before implementation.

## 4.1 Downstream API

The downstream namespace owns DOCSIS downstream OFDM simulation.

Preserve the current MERLIN Rust API, including:

- `docsis_ofdm_fec::api::Merlin`
- `docsis_ofdm_fec::api::Simulation`
- Existing configuration types.
- Existing evaluation options.
- Existing reports.
- Existing run controls.
- Existing error types.
- Existing input and profile objects.

Do not rename or remove these existing exports.

Introduce explicit downstream namespace exports or compatibility re-exports where appropriate.

The desired usage is conceptually:

```rust
use merlin::downstream::Simulation;

let mut sim = Simulation::new();

sim.load_profiles(profiles)?;
sim.add_rxmer(rxmer)?;
sim.add_channel_estimate(channel_estimate)?;
sim.set_options(options)?;

let prepared = sim.prepare()?;
let report = sim.run()?;
```

This illustrates the intended API structure. It is not a claim that `merlin` already exists as a Rust crate.

Do not duplicate the downstream simulation engine to create the namespace.

## 4.2 Upstream API

The upstream namespace owns DOCSIS upstream OFDMA simulation.

Expose:

- Upstream simulation/session.
- UCD-based configuration.
- IUC/burst profiles.
- Grant and minislot configuration.
- Upstream RxMER.
- Upstream Pre-Eq.
- Channel impairment configuration.
- CMTS receiver options.
- Simulation execution controls.
- Progress.
- Complete and partial results.
- Scientific reports.

Desired conceptual usage:

```rust
use merlin::upstream::Simulation;

let mut sim = Simulation::new();

sim.load_configuration(configuration)?;
sim.load_rxmer(rxmer)?;
sim.load_pre_eq(pre_eq)?;
sim.set_options(options)?;

let prepared = sim.prepare()?;
let report = sim.run()?;
```

The upstream API must call the validated OFDMA engine.

Do not reproduce PHY algorithms in its public API adapters.

## 4.3 Common API contracts

Common public contracts should include compatible:

- Run control.
- Cancellation.
- Progress.
- Execution limits.
- Result ownership.
- Structured errors.
- Run identity.
- Configuration generation identity.
- Report metadata.
- Provenance.
- Versioned serialization.

Common does not imply identical configuration.

Do not force OCD/DPD and UCD/IUC into one giant configuration type.

Keep physical direction explicit and strongly typed.

## 4.4 Cargo namespace implementation

The current downstream Cargo package is `docsis-ofdm-fec`.

Preserve that package identity and existing import compatibility.

Evaluate the least disruptive way to expose the logical namespaces:

- Existing library modules and re-exports.
- Direction-specific PHY crates.
- A thin public facade crate, only if necessary.

Do not assume a new Cargo package named `merlin` is required.

If the public `merlin::downstream` and `merlin::upstream` syntax requires a facade crate, document its dependency graph, consumer impact, and alternatives.

A facade must contain no duplicate PHY implementation.

Present the final namespace-to-crate mapping for approval before introducing compatibility-sensitive changes.

---

# 5. Rust API lifecycle

Both PHY directions should follow the same conceptual lifecycle:

**Configure → Validate → Prepare → Run → Report**

The contracts must clearly distinguish:

1. Idle session creation.
2. Input loading.
3. Configuration admission.
4. Validation.
5. Preparation/preview.
6. Execution.
7. Progress and status.
8. Cancellation.
9. Pause/resume where supported.
10. Result retrieval.

Loading inputs must never implicitly start simulation.

Preparation must not silently execute PHY work.

A run operates on an accepted, immutable configuration generation.

Preserve correct handling of:

- Busy states.
- Unsupported operations.
- Invalid inputs.
- Missing configuration.
- Resource limits.
- Cancelled work.
- Partial scientific results.
- Decoder failures.
- Report ownership.
- Deterministic seeded execution.

Do not expose an operation as supported merely to achieve API symmetry.

---

# 6. Python API — mirror Rust direction separation

MERLIN must have one Python distribution.

Use the existing PyO3 extension.

The final Python API should provide separate downstream and upstream namespaces:

```python
from merlin.downstream import Simulation as DownstreamSimulation
from merlin.upstream import Simulation as UpstreamSimulation
```

Preserve existing downstream compatibility:

```python
import merlin

sim = merlin.Simulation()
```

The existing `merlin.Simulation` must continue representing the existing downstream behavior.

Do not break existing Python imports, classes, method signatures, examples, or saved-workbench compatibility.

## 6.1 Python package architecture

Evaluate the implementation of real Python submodules versus PyO3-registered submodules.

Confirm that the selected design works for:

- Direct imports.
- Package initialization.
- Module discovery.
- Type stub generation.
- Wheel building.
- Installed distributions.
- Python 3.10+ supported environments.

Do not assume dynamically registered attributes automatically support standard Python submodule import semantics.

Use one native extension and one Python distribution unless a justified package boundary requires otherwise.

## 6.2 Python responsibilities

Python is an adapter, not a second scientific engine.

PyO3 handles:

- Native object exposure.
- Input conversion.
- Error translation.
- Rust method dispatch.
- Result ownership.
- Progress and control access.
- Native execution and interpreter-lock release.

All DSP, FEC, simulation and scientific validation remain in Rust.

## 6.3 Rust/Python parity

For every exported operation, validate:

- Same accepted inputs.
- Same configuration defaults.
- Same physical units.
- Same validation behavior.
- Same structured error meaning.
- Same deterministic scientific outcomes.
- Same report values and metadata.
- Same capture provenance.
- Same execution control semantics.
- Correct ownership and object lifetime.

Use actual native Python runtime tests.

Do not treat the existence of PyO3 bindings or type stubs as successful API acceptance.

---

# 7. Unified API compatibility matrix

Create one authoritative API compatibility matrix.

For each operation, record:

| Field | Requirement |
|---|---|
| Direction | Downstream / Upstream / Common |
| Operation | Configure, validate, prepare, run, report, etc. |
| Rust namespace | Actual exported path |
| Rust type/method | Exact signature |
| Python namespace | Actual import path |
| Python type/method | Exact signature |
| Input ownership | Borrowed, copied, owned |
| Units | Explicit physical units |
| Error behavior | Structured result |
| Lifecycle behavior | Idle, busy, running, cancelled |
| Report contract | Actual result type/schema |
| Compatibility | Existing/new/alias |
| Requirement | Stable requirement ID |
| Checklist | Delivery task |
| Tests | Exact test cases |
| Evidence | Executed results |
| Status | Implemented, planned, accepted, deferred |

Do not mark hypothetical symbols as implemented.

Preserve existing downstream exports and their prior acceptance.

---

# 8. Mandatory implementation traceability

Every backend requirement must have a complete and verifiable chain:

**Architecture → Requirement → Checklist → Implementation → Test → Validation**

This applies to:

- Shared DSP.
- Shared FEC.
- Downstream OFDM.
- Upstream OFDMA.
- PNM capture processing.
- Rust API.
- Python API.
- CLI backend integration.

For each requirement, record:

1. Stable ID.
2. Architecture owner.
3. Feature design.
4. Checklist task.
5. Implementing crate.
6. Exact source file.
7. Relevant public symbol or internal stage.
8. Test file.
9. Specific test case.
10. Validation command.
11. Observed result.
12. Commit SHA.
13. Status.
14. Limitations.

The traceability record must distinguish:

- Planned.
- Implemented but unvalidated.
- Integrated and validated.
- Failed or blocked.
- Deferred.
- Historical source evidence.

Do not transfer acceptance from the original source repository directly into MERLIN.

Do not infer a successful test from an existing source path.

---

# 9. Automated traceability validation

Extend the current MERLIN integration validation tooling.

Do not create a competing tracking system.

Automate checks for:

- Missing implementation owners.
- Invalid source paths.
- Missing public API symbols.
- Missing acceptance tests.
- Missing executed test evidence.
- Broken requirement/task links.
- Broken architecture links.
- Duplicate requirement IDs.
- Duplicate checklist task IDs.
- Invalid historical aliases.
- Stale source paths after migration.
- Unmatched Rust/Python exports.
- Incorrect documentation completion status.
- Incompatible requirement status and actual implementation coverage.

Preserve legacy requirement IDs and checklist states.

The official developer checklist remains the completion authority.

Machine-readable indexes are supporting evidence, not competing status systems.

---

# 10. Shared DSP integration

Continue `INT-DSP-01` according to its established dependencies.

Inspect actual upstream and downstream algorithms.

Identify genuinely common kernels:

- Complex IQ arithmetic.
- FFT/IFFT.
- QAM distance calculations.
- LLR numerical primitives.
- FIR filtering.
- AWGN.
- Phase/CFO calculations.
- Channel-response mathematics.
- PRBS.
- Generic MER calculations.

For each candidate:

1. Compare implementations.
2. Compare numerical conventions.
3. Identify existing consumers.
4. Classify SHARE/REUSE/ADAPT/SEPARATE/DEFER.
5. Propose a common owner.
6. Preserve independent reference vectors.
7. Validate numerical equivalence.
8. Benchmark hot paths.
9. Preserve downstream compatibility.
10. Record requirement and acceptance evidence.

Do not eliminate validated implementations without equivalent replacement acceptance.

Do not create the `dsp` crate unless its dependency boundary is justified.

---

# 11. FEC integration

Preserve the already integrated upstream LDPC foundation.

Do not repeat completed FEC migration without a technical reason.

Retain:

### Downstream

- BCH.
- Data LDPC.
- Shortening.
- PLC/NCP coding.

### Upstream

- Long QC-LDPC.
- Medium QC-LDPC.
- Short QC-LDPC.
- Normative matrices.
- Upstream shortening.
- Independent parity and codeword tests.

Reuse compatible decoder mechanisms without mixing normative code definitions.

All three upstream code families must remain traceable to independent source-derived fixtures and executed MERLIN validation.

---

# 12. Upstream OFDMA backend

Integrate the upstream PHY into `crates/ofdma` according to the approved functional architecture.

Preserve:

- UCD/IUC configuration.
- Synthetic grants and minislots.
- PRBS and source verification.
- Eight upstream transmitter stages.
- Upstream pilot structures.
- Interleaving.
- Pre-Eq.
- OFDMA waveform generation.
- Channel simulation.
- Nine CMTS receiver stages.
- Independent receiver channel estimation.
- Soft demapping and LDPC decoding.
- Independent recovered-source comparison.
- RxMER and BER measurements.
- Actual QAM/Spectrum observations.
- Deterministic execution.
- Cancellation and partial results.

Preserve the distinction between transmitter correction C, physical channel H and receiver-estimated effective channel.

The receiver must not access simulator truth in normal operation.

Do not introduce a full CMTS MAC, multi-CM scheduler, SNMP/TFTP control, or unapproved ranging implementation.

---

# 13. PNM integration

Retain the existing `pnm-input` crate.

Consolidate verified upstream and downstream PNM admission.

Support the declared formats within their validated scope, including:

- Downstream RxMER.
- Downstream channel estimation.
- Downstream modulation profiles.
- Upstream RxMER.
- Upstream OFDMA Pre-Eq.
- UCD configuration capture handling.

Preserve:

- Exact source bytes and hashes.
- Identity metadata.
- Frequency and grid information.
- Units.
- Missing values.
- Capture timestamps.
- Pre-Eq update semantics.
- Source association rules.
- Original versus derived measurements.

Use the existing verified parser dependencies wherever practical.

Do not duplicate a parser without technical justification.

Maintain Rust/Python API parity for capture operations.

---

# 14. Unified CLI

Retain MERLIN's existing CLI executable and its existing command behavior.

Add upstream commands using direction-specific dispatch.

Do not introduce a second executable under the former project's name.

The CLI should use the same public Rust backend APIs.

Preserve existing commands, JSON behavior, exit statuses and compatibility.

Validate both directions independently.

---

# 15. Backend validation requirements

Before final acceptance, run the complete relevant regression suites.

## Downstream

Validate:

- Existing public Rust API.
- Existing Python API.
- BCH/LDPC.
- PLC/NCP.
- Modulation and interleaving.
- Waveform processing.
- Receiver estimation.
- Channel impairments.
- PNM.
- RxMER and constellation reports.
- Deterministic execution and errors.

## Upstream

Validate:

- Both supported OFDMA numerologies.
- All declared pilot patterns.
- Supported QAM orders.
- Upstream LDPC code families.
- Grant/source processing.
- Transmitter stages.
- Channel models.
- CMTS receiver stages.
- Pre-Eq.
- Measured capture association.
- End-to-end payload recovery.
- Deterministic scientific outputs.
- Runtime control and partial results.

## API

Validate:

- Downstream backward compatibility.
- Direction-separated Rust imports.
- Direction-separated Python imports.
- Rust/Python parity.
- Error categories.
- Run control.
- Serialization.
- Owned results.
- Native wheel behavior within declared supported platforms.

Do not change golden vectors merely to obtain passing results.

---

# 16. Backend-only completion boundary

The integration is accepted when:

1. Shared DSP/FEC work has passed its required regression gates.
2. Downstream OFDM remains fully operational within its previously supported scope.
3. Upstream OFDMA is operational within its declared supported scope.
4. The Rust API exposes explicit downstream and upstream namespaces.
5. Existing Rust downstream imports remain compatible.
6. The Python package exposes matching downstream/upstream namespaces.
7. Existing `merlin.Simulation` remains compatible.
8. Rust and Python expose validated direction-specific operations.
9. PNM processing supports both directions.
10. CLI functionality supports both directions.
11. Requirements are linked to actual integrated implementations.
12. Accepted requirements have executed validation evidence.
13. Scientific outcomes remain reproducible.
14. Documentation matches actual runtime capability.
15. No former upstream product identity remains in the final product interface.
16. No Qt implementation changes are required to accept the backend.

**The backend is allowed to reach full acceptance before the Qt Workbench has an upstream tab.**

The Qt Workbench will be addressed in a subsequent independent development phase.

---

# 17. Documentation updates

Maintain the existing authoritative documents:

- `docs/architecture.md`
- `docs/requirements.md`
- `docs/development/checklist.md`

Update the functional feature documents:

- `docs/development/features/fec/`
- `docs/development/features/signal-processing/`
- `docs/development/features/ofdma/`
- `docs/development/features/pnm-measurement-inputs/`
- `docs/development/features/high-level-api/`

Keep requirement IDs stable.

Preserve original downstream history and validation records.

Update documentation after each accepted implementation stage.

Document current support accurately.

Do not claim external hardware interoperability from internal simulation testing.

---

# 18. Required workflow

Use the established:

**Architecture → Requirements → Checklist → Implementation → Validation**

For every remaining integration task:

1. Review its existing requirements and dependencies.
2. Identify the exact implementation files and symbols.
3. Identify affected downstream consumers.
4. Establish relevant regression baselines.
5. Implement the smallest coherent change.
6. Run independent numerical and integration tests.
7. Run applicable Rust/Python parity tests.
8. Record exact commands and results.
9. Update implementation traceability.
10. Update the official checklist after acceptance.
11. Update related documentation and Agent Skills.
12. Continue to the next eligible task.

Do not bypass dependency gates merely to implement the public API sooner.

Develop the initial upstream Rust API alongside upstream stages, then perform final public convergence after the real upstream closed loop is validated.

Do not create an early FEC-only Python facade and describe it as the complete upstream API.

---

# 19. Required initial Codex response

Before modifying compatibility-sensitive public API exports, provide:

1. Existing downstream Rust API inventory.
2. Existing downstream Python API inventory.
3. Implemented upstream functionality inventory.
4. Proposed public Rust module tree.
5. Proposed public Python package tree.
6. Exact Rust namespace-to-Cargo-crate mapping.
7. Backward compatibility strategy.
8. Common lifecycle/error/report contracts.
9. API operation and parity matrix.
10. Implementation-level traceability plan.
11. Required checklist and requirements updates.
12. Exact files to add, edit, reuse or remove.
13. Regression and acceptance plan.

Clearly distinguish existing code from proposed implementation.

Present the final namespace and compatibility decisions for review before modifying established API symbols.

After approval, continue the dependency-ordered backend implementation and record test evidence at each stage.

---

# Final objective

MERLIN must become one unified DOCSIS PHY simulation backend with two clearly separated public interfaces:

**`downstream` — OFDM PHY**

**`upstream` — OFDMA PHY**

Both share proven DSP/FEC infrastructure and compatible lifecycle/reporting contracts without mixing direction-specific PHY configuration.

The native Python distribution mirrors this separation and delegates all scientific computation to Rust.

The integration must maintain complete traceability from architecture and requirements through actual Rust/Python implementations and executed acceptance evidence.

Preserve the existing MERLIN downstream implementation, public interfaces, feature history and validated behavior.

**Finish and validate the complete Rust/Python backend before beginning Qt Workbench integration.**