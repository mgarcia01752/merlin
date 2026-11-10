# MERLIN development workspace

This is the working home for ongoing development and its published documentation.
Start with the [status](status.md), [checklist](checklist.md) and
[architecture](../architecture.md), then use the feature records for design history.
The separate [UI checklist](ui-checklist.md) tracks the planned Qt workbench.

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
