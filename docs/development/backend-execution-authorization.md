# MERLIN — Complete Unified DOCSIS OFDM/OFDMA Backend Integration

## MASTER CODEX EXECUTION AUTHORIZATION

### Role

Act as the principal Rust software architect, DSP engineer, DOCSIS 3.1/4.0 PHY specialist, API engineer, and integration engineer responsible for completing MERLIN's unified backend.

**You are authorized to execute every remaining in-scope backend integration task automatically, including implementation, validation, documentation updates, and focused local Git commits.**

Do not stop after each requirement, checklist task, implementation phase, or commit to ask for another prompt.

The architecture, requirements, feature organization, scientific crate boundaries, and direction-separated public API design have already been approved.

Use those existing decisions as the implementation baseline.

Do not restart architecture planning or create another master integration plan.

---

# 1. Project and branch

Repository:

https://github.com/mgarcia01752/merlin

Required working branch:

`merge-merlin-morgana`

The final product identity is **MERLIN**.

The backend must support:

- Downstream DOCSIS OFDM.
- Upstream DOCSIS OFDMA.
- Shared DSP.
- Shared FEC.
- Direction-aware PNM processing.
- Separate downstream/upstream Rust APIs.
- Matching Python APIs.
- Unified CLI operations.

### Repository safety

Before continuing:

1. Confirm the active integration branch.
2. Inspect working-tree and index state.
3. Preserve all existing user changes.
4. Inspect current integration checklist status.
5. Confirm the latest accepted implementation checkpoint.
6. Continue with the first eligible unfinished backend task.

Do not recreate, reset, rebase, or rename the integration branch.

Do not modify `master`.

Do not merge into `master`.

Do not push automatically.

Do not publish releases or packages.

Do not modify the original upstream development repository.

Do not introduce former development-project branding into MERLIN's final public API, executables, packaging, or user documentation.

Preserve legally required copyright and third-party attribution.

---

# 2. Authoritative development workflow

MERLIN follows:

**Architecture → Requirements → Developer Checklist → Implementation → Validation**

Read and follow the applicable Agent Skills before beginning work.

In particular, use:

- `skills/development-workflow/SKILL.md`
- `skills/git-workflow/SKILL.md`
- `skills/coding-style/SKILL.md`
- `skills/scientific-testing/SKILL.md`
- `skills/docsis-signal-processing/SKILL.md`
- `skills/public-api/SKILL.md`
- `skills/build-validation/SKILL.md`, if present
- Relevant DOCSIS PHY and numerical-testing skills available in the repository.

Discover applicable skills through the repository's `AGENTS.md` and skill index.

Do not assume an absent skill exists.

### Canonical documents

Maintain:

- `docs/architecture.md`
- `docs/requirements.md`
- `docs/development/checklist.md`

Feature designs under `docs/development/features/` refine these documents.

The official checklist remains the authoritative source of implementation completion.

Do not create a competing checklist or requirements baseline.

---

# 3. Continuous implementation authorization

**Complete the entire remaining backend integration using the existing dependency-ordered checklist.**

The next expected implementation task is:

`INT-US-CONFIG-01`

Verify this against the current checkout before starting.

Continue through all remaining applicable backend tasks.

Expected sequence:

| Task | Objective |
|---|---|
| INT-US-CONFIG-01 | Upstream configuration, grants and PRBS |
| INT-US-TX-01 | OFDMA transmitter |
| INT-US-CH-01 | Upstream channel simulator |
| INT-US-RX-01 | CMTS upstream receiver |
| INT-US-LOOP-01 | End-to-end upstream simulation |
| INT-PNM-01 | PNM capture decoders |
| INT-PNM-02 | Measured capture association and playback |
| INT-RUST-01 | Direction-separated Rust APIs |
| INT-PY-01 | Unified native Python API |
| INT-CLI-01 | Unified CLI |
| INT-TOOLS-01 | Implementation traceability validation tooling |
| INT-REG-01 | Unified regression acceptance |
| INT-DOC-02 | Final documentation reconciliation |
| INT-ACCEPT-01 | Final backend acceptance |

Use actual checklist dependencies if they differ from this summary.

Completed tasks must not be reopened without technical justification.

Work automatically through all eligible tasks until backend acceptance is complete.

---

# 4. Mandatory automatic focused commits

## Critical instruction

**Codex MUST automatically create a focused local Git commit after each completed and validated requirement task or independently testable implementation unit.**

The user must not be required to manually issue Git commit commands.

Use MERLIN's existing Git workflow:

```bash
tools/git/git-save.sh --commit-msg "Feature: <completed functionality>"
```

This is the mandatory commit mechanism.

### Follow the Git skill

Before using the command, follow:

`skills/git-workflow/SKILL.md`

Also follow any relevant development-workflow rules concerning:

- Commit message conventions.
- Validation evidence.
- File staging.
- Working-tree safety.
- Documentation updates.
- Task completion.
- Requirement traceability.

### Commit frequency

Prefer one focused commit per completed checklist requirement or task.

When a checklist task contains several independently testable implementation units, create a focused commit after each validated unit.

Examples:

```bash
tools/git/git-save.sh --commit-msg "Feature: Add OFDMA configuration and grant planning"

tools/git/git-save.sh --commit-msg "Feature: Integrate upstream PHY randomization"

tools/git/git-save.sh --commit-msg "Feature: Integrate OFDMA transmitter"

tools/git/git-save.sh --commit-msg "Feature: Integrate CMTS upstream receiver"

tools/git/git-save.sh --commit-msg "Feature: Complete OFDMA closed-loop simulation"

tools/git/git-save.sh --commit-msg "Feature: Add upstream PNM capture admission"

tools/git/git-save.sh --commit-msg "Feature: Add direction-separated Rust APIs"

tools/git/git-save.sh --commit-msg "Feature: Add Python upstream simulation API"

tools/git/git-save.sh --commit-msg "Feature: Integrate upstream CLI operations"

tools/git/git-save.sh --commit-msg "Documentation: Complete backend implementation traceability"
```

These are illustrative commit messages. Generate the actual message from the completed functionality.

### Mandatory per-task sequence

For every requirement or implementation task:

1. Read the requirement and acceptance criteria.
2. Confirm architecture and implementation ownership.
3. Inspect relevant existing code.
4. Implement the required functionality.
5. Add or update independent tests.
6. Execute focused validation.
7. Execute applicable downstream regression tests.
8. Correct all failures.
9. Update architecture, requirements, feature documentation and checklist.
10. Record the implementation and validation evidence.
11. Verify requirement-to-implementation traceability.
12. Inspect `git status` and `git diff`.
13. Confirm the working tree contains only intended changes.
14. Execute `tools/git/git-save.sh --commit-msg "..."`.
15. Confirm that the commit succeeded.
16. Record the resulting commit SHA.
17. Continue automatically to the next eligible task.

### Important staging behavior

The existing `git-save.sh` executes:

`git add --all`

It also regenerates the Python integration documentation before staging.

Therefore, before invoking it:

- Confirm all non-ignored changes belong to the intended focused commit.
- Check for unrelated user changes.
- Check for generated files that should not be committed.
- Account for automatic documentation regeneration.
- Preserve user changes that are not part of the current task.

If unrelated user changes prevent safe use of the script, do not discard or silently include them. Use a safe isolated worktree or another repository-approved isolation mechanism if permitted by the Git skill; otherwise document the blocker.

Do not bypass the approved Git workflow merely to force a commit.

### Commit messages

Use existing MERLIN conventions:

- `Feature:`
- `BugFix:`
- `Refactor:`
- `Documentation:`
- `Test:`

Include applicable requirement and checklist IDs in the commit description or associated validation record.

Record actual tests and results.

Do not claim tests passed unless executed.

### Continuous execution rule

**A successful Git commit is a checkpoint, not a stopping condition.**

After every successful commit, continue automatically.

Do not stop to ask whether to proceed.

Do not stop merely to report a successful commit.

Continue until all in-scope backend tasks are accepted or an actual technical blocker prevents further progress.

---

# 5. Additive-first integration policy

MERLIN is the authoritative codebase.

Preserve existing downstream behavior.

Before changing an existing downstream source file:

1. Identify its requirements.
2. Identify public consumers.
3. Identify its tests.
4. Establish its pre-change behavior.
5. Explain why reuse or modification is justified.
6. Implement the smallest compatible change.
7. Verify the post-change behavior.

Do not rewrite downstream code for cosmetic consistency.

Do not replace validated algorithms without equivalence evidence.

Do not weaken test assertions or regenerate golden data from production code merely to obtain passing tests.

---

# 6. Final scientific crate architecture

Use the approved logical structure:

```text
MERLIN/
├── crates/
│   ├── dsp/
│   ├── fec-codec/
│   ├── ofdm/
│   ├── ofdma/
│   ├── phy-api/
│   ├── merlin/
│   ├── pnm-input/
│   └── cli/
│
├── bindings/
│   └── python/
│
└── apps/
    └── qt/
```

The existing shared DSP and upstream LDPC integration must be preserved.

### Crate responsibilities

**dsp**

Shared numerical primitives including validated FFT/IFFT, PRBS and compatible DSP algorithms.

**fec-codec**

Shared FEC machinery with distinct downstream and upstream normative code definitions.

**ofdm**

Downstream OFDM PHY engine.

**ofdma**

Upstream OFDMA PHY engine.

**phy-api**

Small common contract layer for genuinely shared API types.

**merlin**

Thin public Rust facade exposing direction-separated namespaces.

**pnm-input**

Native PNM file admission and measurement adapters.

**cli**

Existing MERLIN CLI, extended with upstream operations.

Neither PHY engine may depend on the other.

The shared DSP/FEC crates must not depend on Qt or Python.

Avoid circular dependencies and unnecessary new crates.

---

# 7. Complete upstream OFDMA PHY implementation

Integrate the validated upstream scheduled-data backend into MERLIN.

### Configuration and source

Implement:

- UCD/IUC configuration.
- Burst profiles.
- Synthetic grants.
- Minislot and resource allocation.
- PRBS15 source.
- Validated source configuration.
- Supported 2K/50 kHz and 4K/25 kHz operating modes.

### Transmitter

Complete all eight upstream TX stages:

1. Grant framing and codeword selection.
2. Upstream QC-LDPC encoding and shortening.
3. PHY randomization.
4. QAM mapping.
5. Pilots, resources and interleaving.
6. Pre-equalization.
7. IFFT.
8. Prefix, windowing and waveform assembly.

Preserve normative processing order and independently verified fixtures.

### Channel simulator

Support the declared channel models:

- Exact unity.
- Causal FIR.
- AWGN.
- Integer delay.
- Phase and CFO.
- Supported RxMER residual models.

Keep physical channel H and transmitter correction C distinct.

Preserve seeded deterministic behavior.

### Receiver

Complete all nine CMTS receiver stages:

1. Burst framing and timing.
2. Useful-symbol extraction.
3. FFT.
4. Pilot-derived channel estimation and equalization.
5. Resource extraction and inverse interleaving.
6. Soft QAM demapping.
7. Soft PHY derandomization.
8. LDPC decoding.
9. Information deframing.

The receiver must recover information from received observations.

Do not provide simulator truth or original transmitted bits to normal receiver algorithms.

### End-to-end verification

Run the complete:

PRBS → CM transmitter → channel → CMTS receiver → independent comparison.

Validate both numerologies, applicable pilot patterns, supported QAM orders, code families and channel configurations.

Record genuine decoding failures, iterations, bit errors, partial coverage and receiver-estimation outcomes.

---

# 8. Complete measured PNM integration

Extend MERLIN's existing `pnm-input` infrastructure.

Include:

- Upstream UCD capture decoding.
- Upstream RxMER.
- Upstream OFDMA Pre-Eq.
- Measured capture association.
- Multiple capture selection and playback.
- Validated configuration projection.
- Source provenance.

Reuse existing verified PNM parsing capabilities where practical.

Preserve exact input bytes, metadata, units, subcarrier grids, timestamps, missing values, coefficient scaling and update semantics.

Reject unsupported or ambiguous measurement combinations rather than guessing.

Do not count measured noise twice.

Maintain explicit physical reference planes.

---

# 9. Implement the approved direction-separated Rust API

Follow the approved architecture:

`docs/development/features/high-level-api/namespace-design.md`

The final public interface must support:

```rust
use merlin::downstream::Simulation as DownstreamSimulation;
use merlin::upstream::Simulation as UpstreamSimulation;
```

Expose genuinely common contracts through:

```rust
use merlin::common::*;
```

The facade must contain no scientific implementation.

### Preserve existing downstream API

Existing imports must remain valid:

```rust
docsis_ofdm_fec::api::Simulation
docsis_ofdm_fec::api::Merlin
```

Preserve type identity and established behavior.

### Upstream API

Expose the validated OFDMA engine through an independent typed session.

Support:

- Configuration.
- Validation.
- Preparation.
- Execution.
- Progress.
- Cancellation.
- Pause/resume where supported.
- PNM input association.
- Complete and partial reporting.

Keep direction-specific configuration and report types where required.

### Shared lifecycle

Use compatible conventions for:

- Idle.
- Configured.
- Prepared.
- Running.
- Busy.
- Cancelled.
- Completed.
- Partial-result handling.

Do not falsely expose unsupported capabilities.

Complete the API operation compatibility matrix with actual signatures and execution evidence.

---

# 10. Implement the unified native Python API

Maintain one Python distribution and one native Rust extension.

Implement:

```python
from merlin.downstream import Simulation as DownstreamSimulation
from merlin.upstream import Simulation as UpstreamSimulation
```

Preserve:

```python
import merlin

simulation = merlin.Simulation()
```

Existing root-level downstream imports must remain compatible.

Follow the approved package design, including real importable Python modules and one private native extension.

Preserve the complete existing Python API.

Do not create a second upstream distribution.

Do not implement scientific algorithms in Python.

Validate:

- Installed-wheel imports.
- Type and module discovery.
- Rust/Python parity.
- Input ownership.
- Structured errors.
- Busy handling.
- Cancellation.
- Interpreter-lock release.
- Complete and partial results.
- Deterministic scientific outputs.
- Report schemas and provenance.

Use actual native tests rather than relying solely on type declarations.

---

# 11. Complete unified CLI

Retain MERLIN's existing CLI executable and commands.

Add upstream operations through direction-specific dispatch.

Do not introduce a separate executable for the upstream engine.

All CLI science must call the canonical Rust backend.

Preserve existing command behavior, outputs, exit statuses and compatibility.

Run both upstream and downstream CLI regression tests.

---

# 12. Mandatory implementation traceability

Every accepted backend requirement must have a complete chain:

**Architecture → Requirement → Checklist → Implementation → Test → Validation**

Use the established traceability framework.

For each requirement record:

1. Requirement ID.
2. Architecture owner.
3. Functional feature design.
4. Official checklist task.
5. Implementing Cargo crate.
6. Exact source file.
7. Relevant symbol or processing stage.
8. Test file.
9. Qualified test case.
10. Validation command.
11. Observed result.
12. Tested revision or working-tree fingerprint.
13. Status.
14. Limitations.

Use existing machine-readable records where applicable.

Do not create a competing completion tracker.

Source validation must not be confused with MERLIN integration validation.

A test file's existence does not prove its tests passed.

A successful compilation does not prove scientific correctness.

The official checklist remains the completion authority.

---

# 13. Automated traceability and quality validation

Extend the existing tooling to verify:

- Requirement IDs.
- Checklist task IDs.
- Architecture links.
- Feature documentation ownership.
- Implementation file paths.
- Actual Rust public symbols.
- Python exports.
- Exact acceptance test cases.
- Executed validation records.
- Commit and working-candidate identities.
- API parity coverage.
- Historical alias preservation.
- Dependency ordering.
- Documentation support status.

Reject incomplete requirement chains for tasks marked accepted.

Do not reopen historical work merely because older records use less detailed evidence formats.

Identify genuine coverage gaps and resolve them through focused tests where practical.

---

# 14. Regression testing

Run all applicable tests before completing the backend.

### Rust

- Workspace tests.
- DSP tests.
- FEC tests.
- Downstream PHY tests.
- Upstream PHY tests.
- PNM tests.
- CLI tests.
- Examples.
- Doctests.
- Formatting.
- Clippy.

### Python

- Existing downstream native tests.
- Upstream native tests.
- Rust/Python parity tests.
- Installed-wheel tests.
- Public import compatibility.
- Error and report compatibility.
- Cancellation and lifecycle behavior.

### Scientific correctness

Validate:

- FFT scaling and frequency-bin conventions.
- PRBS sequence continuity.
- QAM labels and soft metrics.
- LDPC matrix parity.
- Channel/noise conventions.
- Pre-Eq reference planes.
- Independent receiver estimation.
- Actual recovered-source comparison.
- BER and RxMER.
- Deterministic seeded execution.

Use Rust 1.85.1 and the existing pinned dependencies.

Do not upgrade the compiler during this integration.

Document numerical or performance changes rather than silently modifying test expectations.

---

# 15. Documentation and Agent Skills

Maintain:

- `docs/architecture.md`
- `docs/requirements.md`
- `docs/development/checklist.md`

Update the functional feature records after each implementation task.

Follow the existing Agent Skills for:

- Development workflow.
- Rust implementation.
- Scientific testing.
- Public APIs.
- Git commits.
- Documentation.
- Validation.

Preserve established downstream feature records.

Do not duplicate historical source documentation.

Keep planned and implemented capabilities clearly distinguished.

Do not mark a task complete until its validation evidence supports the declared acceptance criteria.

---

# 16. Out-of-scope work

Do not implement:

- Upstream Qt Workbench.
- Downstream/Upstream UI tabs.
- Qt application redesign.
- Full DOCSIS MAC.
- Multi-CM scheduling.
- SNMP/TFTP control.
- Unsupported ranging or probing modes.
- Unapproved PHY extensions.
- New product releases.
- Rust toolchain modernization.

The Qt Workbench must remain functional with the existing downstream backend.

The backend is accepted independently of the future UI integration.

Existing Qt code may require focused compatibility validation if shared Rust interfaces change, but no upstream UI functionality is authorized.

---

# 17. Failure handling and recovery

If a task fails validation:

1. Inspect the failure.
2. Correct the implementation.
3. Re-run focused tests.
4. Re-run applicable regression tests.
5. Record the correction.
6. Continue when acceptance passes.

Do not skip failing mandatory tests.

Do not weaken requirements to obtain a passing result.

Do not fabricate independent test fixtures.

Do not silently omit unsupported functionality from the declared acceptance scope.

If a genuine blocker cannot be resolved, document:

- Exact requirement.
- Checklist task.
- Technical cause.
- Evidence.
- Valid alternatives.
- Recommended resolution.
- Safe completed work.

Continue other dependency-independent tasks where appropriate.

Do not claim final completion while mandatory acceptance remains blocked.

---

# 18. Automatic commit and continuation contract

The required implementation cycle is:

```text
READ REQUIREMENT
       |
       v
CHECK ARCHITECTURE
       |
       v
IMPLEMENT
       |
       v
TEST AND VALIDATE
       |
       v
UPDATE TRACEABILITY
       |
       v
UPDATE CHECKLIST
       |
       v
REVIEW GIT CHANGES
       |
       v
git-save.sh --commit-msg
       |
       v
RECORD COMMIT SHA
       |
       v
NEXT REQUIREMENT
```

Repeat automatically.

**Codex generates the commit message and executes the Git-save command itself.**

The user does not need to supply or execute each commit message.

Follow the Git skill and MERLIN's established commit conventions.

Do not stop after a successful commit.

Do not automatically push.

Do not merge into `master`.

Continue until every in-scope backend integration task is accepted or a genuine blocker prevents further work.

---

# 19. Final backend acceptance

Backend integration is complete only when:

1. Downstream OFDM remains functional.
2. Upstream OFDMA is operational within declared scope.
3. Shared DSP/FEC components are validated.
4. Upstream and downstream PNM processing works.
5. Rust exposes separate direction namespaces.
6. Python mirrors the direction namespaces through one distribution.
7. Existing downstream imports remain compatible.
8. CLI supports both directions.
9. Rust/Python scientific parity passes.
10. Resource limits and lifecycle controls work as declared.
11. Accepted requirements have exact implementation/test/evidence links.
12. Complete regression suites pass.
13. Documentation matches the actual implementation.
14. The official backend checklist is completed within its declared scope.
15. Relevant implementation changes have focused local commits.
16. No Qt upstream implementation is required for acceptance.
17. No changes have been merged into `master`.

Maintain explicit deferral for broader PHY, ranging, hardware interoperability, or UI functionality outside the current backend scope.

---

# 20. Final engineering report

When the backend integration is complete, provide a consolidated report containing:

### Architecture

- Final Cargo workspace.
- Crate dependency graph.
- Shared DSP/FEC ownership.
- Downstream/upstream processing boundaries.

### APIs

- Final Rust imports and examples.
- Final Python imports and examples.
- Compatibility matrix.
- Common lifecycle/error/report contracts.

### Scientific validation

- Rust regression results.
- Python parity results.
- CLI acceptance.
- PNM capture acceptance.
- Numerical equivalence results.
- Performance observations.
- Known scientific limitations.

### Traceability

- Requirements covered.
- Checklist tasks completed.
- Source implementation references.
- Acceptance tests and observed results.
- Remaining deferred capabilities.

### Focused Git commits

Provide a chronological table:

| Commit SHA | Subject | Checklist task | Validation |
|---|---|---|---|
| Actual SHA | Actual commit message | Task ID | Executed tests |

Report actual commits only.

Do not invent SHAs or claim tests were run when they were not.

---

# FINAL DIRECTIVE

**Complete the entire remaining MERLIN backend integration now, without repeated user intervention.**

The approved architecture, requirements, feature organization, and API namespace design govern the implementation.

For every completed requirement or coherent implementation task:

**Implement → Test → Validate → Update Traceability → Update Checklist → Automatically Commit with `tools/git/git-save.sh --commit-msg` → Continue.**

Codex is responsible for generating and executing every focused commit command.

Do not ask the user to manually commit.

Do not restart planning after every task.

Do not stop merely because one task or commit succeeds.

Preserve MERLIN's existing downstream functionality.

Integrate upstream OFDMA natively.

Deliver separate direction-aware Rust/Python public APIs with shared DSP/FEC infrastructure.

Complete the backend and its acceptance evidence before any Qt Workbench integration.

Do not push, release, or merge into `master` without separate authorization.