# MERLIN development workspace

The [unified Qt Workbench plan](features/unified-qt-workbench/design.md) defines
the next, separate UI scope after accepted backend checkpoint `23f63c9`. Start
with its [API gap matrix](features/unified-qt-workbench/api-gap-matrix.md),
[complete Qt inventory](features/unified-qt-workbench/source-inventory.json) and
[official delivery tasks](checklist.md#unified-qt-workbench-delivery).
The [UI master guide](features/unified-qt-workbench/master-guide.md) and
[additional public API contract](features/unified-qt-workbench/high-level-api-contract.md)
are retained instruction sources. [Continuous execution is authorized](features/unified-qt-workbench/execution-authorization.md);
old scheduling statements below describe their own historical checkpoints.
Current execution uses the [three-level testing strategy](features/unified-qt-workbench/testing-strategy.md)
for focused commits, official task acceptance and integration milestones.

The [selected namespace mapping](features/high-level-api/namespace-design.md) fixes the literal Rust root, common contract leaf and real Python package layout; namespace runtime acceptance remains separate.

This is the working home for ongoing development and its published documentation.
Start with the [status](status.md), [checklist](checklist.md) and
[architecture](../architecture.md), then use the feature records for design history.
Start with [development setup](setup.md) for the Ubuntu package/toolchain installer.

The [requirements baseline](../requirements.md) is authoritative for behavior and
acceptance in both directions. The [unified backend consolidation](features/unified-backend/design.md)
links the [complete source review](features/unified-backend/source-review.md),
[integration gates](checklist.md#unified-backend-integration) and
[documentation validation](features/unified-backend/validation.md).
The [shared DSP design](features/signal-processing/design.md#int-dsp-01-implementation-slice) and
[validation record](features/signal-processing/validation.md) track the next numerical integration step.

The [pre-import baseline](features/unified-backend/baseline.md),
[reuse review](features/unified-backend/reuse-review.md) and
[standalone upstream LDPC acceptance](features/unified-backend/fec-validation.md)
record the first implementation step. Final implementation/documentation acceptance
remains open until the remaining unified-workspace gates pass.

The [downstream requirements backfill](features/downstream-requirements/design.md)
links historical tasks, detailed stage/API contracts, current owners and located
evidence. Its [validation](features/downstream-requirements/validation.md) separates
the original documentation checkpoint from later acceptance. The three targeted
gaps now have [current or recovered evidence](features/unified-backend/baseline.md);
final integration tests remain separate.

The [unified OFDM/OFDMA integration guide](ofdm-ofdma-integration-guide.md)
preserves the full user-supplied master prompt for backend integration planning.
It defines intended scope, source pin, reuse audit, compatibility requirements,
phases and acceptance targets; it does not claim completed integration.

The [current backend master prompt](backend-integration-master-guide.md) is also
preserved in full. Its [preflight and clause map](features/unified-backend/master-prompt-review.md)
link every heading to architecture, requirements and official tasks. The
[API compatibility matrix](features/high-level-api/compatibility-matrix.md) separates
inspected downstream exports from proposed namespaces and pending runtime evidence.
Both guides are instruction sources; the canonical documents retain design,
requirements and status authority. No upstream Qt work is part of this backend gate.

The [upstream skill migration](features/upstream-skills/design.md) records the
pinned source-skill inventory, local destinations and retained overrides, with
[guidance validation](features/upstream-skills/validation.md).

Use the [engineering skill catalog](../../skills/README.md) and
[development workflow](../../skills/development-workflow/SKILL.md) for
Architecture → Requirements → Developer checklist → Implementation → Validation.
New or materially revised tasks link requirements, explicit dependencies,
architecture/file owners and acceptance evidence. Existing IDs and status owners
remain stable. The [selective guidance adoption](features/engineering-guidance/design.md)
records the reviewed foundation candidate, local overrides and validation scope.

The [Qt closed-loop and workbench-file design](features/qt-closed-loop/design.md)
tracks native execution/persistence and its validation.

The [Transmitter workbench design](features/transmitter-workbench/design.md)
tracks staged TX instruments and actual OCD/DPD packet inspection.

The [Channel Simulator staging](features/channel-workbench/design.md) and
[additional-loss model](features/channel-tilt/design.md) record channel instruments,
cable evidence and response/noise policy.

The [Qt source organization](features/qt-source-organization/design.md) records
QML feature folders, instrument providers and the CXX-Qt bridge constraint.

The [workbench example design](features/workbench-examples/design.md) covers
the channel 193 collection in `multi-rxmer-2`, portable WBs and input bounds.

The [Basic Rust API and Basic Python API](features/high-level-api/design.md) has
[fixture-backed validation](features/high-level-api/validation.md) and separate
[Rust](../user/api/README.md) and [Python](../user/python/README.md) guides.

The separate [UI checklist](ui-checklist.md) tracks the Qt workbench.
[Qt setup and VM validation](qt.md) covers the initial native shell.

```text
docs/development/
├── README.md                 Development entry point
├── status.md                 Current milestone and remaining scope
├── checklist.md              Official completion/TODO list
├── ui-checklist.md           Detailed Qt workbench tasks
├── build.md                  Toolchain, validation and generated files
├── git.md                    Commit, push and release workflow
├── logging.md                Logging conventions and usage
├── debugging.md              Maintained desktop debug controls
├── checklist-review.md       Supporting checklist audit
├── impairment-reference.md   Impairment-stage reference
├── features/<feature>/       Tracked plans, design notes and validation summaries
└── local/<feature>/          Ignored specs, captures, logs and generated outputs
```

The proposed [content-based build/test cache](features/build-cache/design.md)
defines fast development iteration; implementation has not started.
Local full tests use the [parallel runner](features/parallel-local-tests/design.md)
with four process-isolated tests at a time. This is scheduling, not result caching.

The [sparse NCP/PLC encoder record](features/sparse-encoder/design.md) explains
which dense setup was replaced, its supported matrix structure and isolated timings.

## Working on a feature

1. Create a focused directory under `features/` for its plan and reviewable notes.
2. Keep private specifications, copied source snapshots and generated evidence
   under the corresponding `local/` directory. From the repository root,
   `mkdir -p docs/development/local/<feature>` prepares local output storage.
3. Implement and validate the shared Rust stages. Maintain user guides alongside
   behavior changes; record validation summaries in the feature directory.
4. Update the canonical architecture and checklist when scope or status changes,
   and review both before preparing a commit message.

The [feature index](features/README.md) links the migrated project records.
[Build and test](build.md), [Git and releases](git.md) and [logging](logging.md)
remain the maintained development instructions.

## Workspace migration

The former root `development/` directory was consolidated here. Project-authored
feature records are tracked under `features/`. Private/generated material was
moved intact to ignored `local/`; no captures, specification PDFs or simulation
results were deleted. Archived migration prototypes and copied specification
extracts remain private. Local saved results may contain their original execution
paths; they are historical evidence, not current runnable path instructions.

Public Markdown links point to tracked documentation. Commands use local output
paths explicitly; a fresh clone does not include the ignored evidence directory.
There is no longer a separate root development workspace.

[Security checks](security.md) describe CodeQL, RustSec and local dependency audits.

Receiver instrument completion: [design](features/receiver-instruments/design.md) · [user guide](../user/receiver-workbench.md).


Channel conditioning and shared TapPoint instruments (UI-RF-01):
[design and validation](features/rf-power-budget/design.md#ui-rf-01-implemented-scalar-conditioning-and-channel-instruments) ·
[user guide](../user/channel-measurements.md#launch-amplifier-attenuator-and-tappoints).

[Responsive channel workspace (UI-RF-02)](features/rf-power-budget/design.md#ui-rf-02-responsive-channel-workbench-spacing) uses available window space with balanced margins and equal gaps.

[Ubuntu 22.04 desktop DEB and update notification](features/ubuntu-desktop/design.md) tracks packaging, application-global release checks and hosted acceptance.

Use the [PNM RxMER/FEC skill](../../skills/docsis-pnm-rxmer-fec/SKILL.md) for
reviewed CableLabs guidance, searchable V05/V02 text and original PDF links.

Latest [backend execution authorization](backend-execution-authorization.md) permits
focused local commits and continuous integration, with no push or release.
