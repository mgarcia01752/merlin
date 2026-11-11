# MERLIN development workspace

This is the working home for ongoing development and its published documentation.
Start with the [status](status.md), [checklist](checklist.md) and
[architecture](../architecture.md), then use the feature records for design history.
Start with [development setup](setup.md) for the Ubuntu package/toolchain installer.

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
├── checklist-review.md       Supporting checklist audit
├── impairment-reference.md   Impairment-stage reference
├── features/<feature>/       Tracked plans, design notes and validation summaries
└── local/<feature>/          Ignored specs, captures, logs and generated outputs
```

The proposed [content-based build/test cache](features/build-cache/design.md)
defines fast development iteration; implementation has not started.

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
