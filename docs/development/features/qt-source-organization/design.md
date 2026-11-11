# UI-ORG-01: Organize Qt presentation sources

Status: implemented and validated on Ubuntu22.04 x64 with Qt6.8.3.
Current UI, accepted state and simulation behavior are preserved.

Group the flat QML directory by responsibility, retaining Main.qml as bootstrap:

```text
apps/qt/qml/
  Main.qml
  shell/                       Workbench composition, menu and global settings
  theme/                       Theme and palette
  components/                  Shared dialog, chips, connectors and navigation
  about/                       About and offline licenses
  inputs/{ofdm_settings,profile_input,source_input,measurement_input}/
  workbenches/{transmitter,channel,receiver,verification}/
  instruments/{spectrum,qam,response,slicer,fec,messages,stages}/
```

QML inputs align with the corresponding Rust input-owner names. Shared instrument
windows group by instrument; Rust instrument providers retain cohesive TX/RX
ownership under `src/instruments/{transmitter,receiver,spectrum}`. The transmitter/spectrum root module paths remain available through re-exports
in main.rs, rather than duplicate implementations. CXX-Qt0.7.3 requires all Rust
QObject bridge files for one QML module to share one directory (its build-time
guard references QTBUG-93443). The initial UI-ORG-01 kept receiver_instruments.rs and
transmitter_analyzer.rs in src/ with the other QObject bridges, importing the
grouped providers. UI-ORG-02 below relocates all bridges together. Core algorithms remain unchanged.

Move all existing files once, preserving names and licenses. Update build.rs QML
registration and preserve the same-directory Rust bridge paths. Use explicit relative directory imports for
cross-folder QML references; local QtTest fixtures must work without the compiled
Rust module. Keep Main's existing embedded entry URL. Correct moved Rust resource
include paths without changing embedded bytes. Update current tracked source
references and maintain no duplicate legacy QML tree.

Keep one window per QML file; group related charts/controls with their feature or
instrument. Mirror Rust responsibilities where practical, rather than forcing
one folder per file or duplicating a common data provider for each chart.

This task implements organization only. Shared TP QAM routing and channel
conditioning were separate follow-up tasks; UI-RF-01 has since implemented them.
See the [power implementation record](../rf-power-budget/design.md).

Validation: compare registration against all source files, run existing QML
navigation tests, Qt formatting/Clippy/adapter tests and native build, then launch
on the VM and inspect for import/type warnings. No core workspace retest is
needed for presentation-only file moves. Preserve build caches.

## Ownership and reference map

| QML owner | Rust owner |
| --- | --- |
| shell / theme / components / about | main, branding, workbench_file/session, about_info |
| inputs/ofdm_settings | ofdm_settings |
| inputs/profile_input | profile_input |
| inputs/source_input | source_input |
| inputs/measurement_input | measurement_input |
| workbenches/channel | channel_model and measurement_input |
| workbenches/receiver / verification | closed_loop and receiver instrument providers |
| workbenches/transmitter | transmitter preview and retained-stage providers |
| instruments/spectrum | instruments/spectrum viewport formatting and TX/RX providers |
| instruments/qam / response / slicer / fec | instruments/receiver and instruments/transmitter providers |
| instruments/messages / stages | instruments/transmitter |

Existing local tests resolve fixture logos to absolute file URLs at their caller,
so moving a consuming QML component does not change the logo path. Original
upstream reference paths/hashes in reuse-provenance records remain unchanged.

## Completed validation

- All47 QML files are registered exactly once; Main.qml is the sole root QML file.
  All cross-folder directory imports and moved Rust resource includes resolve.
- Existing28 QML navigation/theme/render checks and23 Qt adapter tests pass.
  Test fixture logos resolve without warnings after the move.
- Qt formatting, warnings-denied Clippy and native executable build pass with
  retained build artifacts. No non-Qt workspace build/test was run for this task.
- A separate temporary native VM instance opened the moved input editors,
  transmitter/channel/receiver workbenches and About dialog without import/type
  or resource warnings. Existing user instance/state was preserved. Actual dark
  rendering was inspected; no visible UI changes or guide-image replacements
  were needed for file organization.
- Skill validation and both discovery links pass. Maintained links, registration,
  resource references and diff whitespace pass. Other OS versions remain untested.


## UI-ORG-02: Organize the remaining Rust Qt adapter

Status: implemented and validated on Ubuntu22.04 x64 with Qt6.8.3.

The reviewed root mixed ten QObject adapters with file access, persistence,
accepted session state and input models. Group those responsibilities without
changing QML type names, accepted state, dialog behavior or saved JSON. Leave
large cohesive models intact; line count alone does not justify splitting them.

- `src/bridge/`: all ten QObject files, registered together in the same QML module.
- `src/inputs/`: source/profile/measurement models, original-byte decoders,
  saved-state restoration and additional-loss editor preview.
- `src/workbench/`: accepted session and self-contained project JSON validation/I/O.
- `src/simulation/`: closed-loop admission, worker lifecycle and retained result.
- `src/platform/`: bounded local file reads, example discovery, native file picker
  and native icon/picker headers.
- `src/instruments/`: existing grouped TX/RX/spectrum providers, unchanged ownership.

Extract profile draft/accepted state from its QObject into a Qt-independent model
and move its existing validation tests with it. Move source and measurement
restoration to their input owners. Project validation must not call QObject
modules. Keep module wiring explicit; no duplicate implementations or empty
forwarding files. Existing instrument aliases remain for established consumers;
other internal uses point to canonical owners.

Update build.rs bridge/native-picker registrations, C++ includes, embedded
resource paths, current source references, Qt guide and coding-style guidance.
Preserve the Main.qml resource URL and all original embedded bytes. Historical
upstream provenance paths/hashes remain historical evidence.

Validate existing adapter tests (including independent binary/JSON parsing,
failed-draft preservation and saved-project validation), formatting, Clippy, QML
checks and native build. Inspect a temporary VM instance for startup/type/import
warnings without closing an existing user session. No core code or input-format
contract changed; no non-Qt workspace test is required. No user-visible layout
changed, so existing user-guide screenshots remain current.


### UI-ORG-02 completed validation

- Formatting and warnings-denied Clippy pass. All23 existing Qt adapter tests
  pass after relocation, including source/profile draft preservation, binary/JSON
  validation, self-contained workbench restoration and actual closed-loop results.
- All28 QML checks pass without warnings; the native executable builds with the
  retained cache/profile. No non-Qt workspace suite was run.
- All ten bridges are registered exactly once in their common directory. Main is
  the only root Rust source. All13 distinct embedded file targets and SHA-256
  hashes match before/after; saved-input/model modules contain no Qt bridge imports.
- A separate temporary VM instance opened input editors, TX/channel/RX workbenches
  and About, with no Qt/QML warnings. The rendered dark About/icon/workbench was
  inspected. Existing user processes and accepted state were preserved; the
  temporary instance was closed after verification.
- Updated documentation links, source references, skill validation/discovery links
  and diff whitespace pass. Local logs and the native review capture live under
  ignored `docs/development/local/qt-source-organization/`.
- Built executable: `apps/qt/target/debug/merlin-qt`,25960656 bytes. Other target
  platforms and installed-package behavior were not retested for this refactor.
