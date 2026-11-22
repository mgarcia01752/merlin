# Qt migration and ownership plan

Status: proposed. No files have moved. The [178-file inventory](source-inventory.json)
pins every tracked `apps/qt` file to the accepted backend checkpoint and records
interfaces, dependency references, consumers, test evidence and a destination.
Reference scans identify candidate relationships, not compiler-resolved call
graphs or line coverage. Missing direct evidence is recorded rather than inferred.
No current file is upstream UI. A candidate for shared scientific presentation
remains downstream until both real consumers and tests exist.

## Destination layout

```text
apps/qt/
  qml/Main.qml
  qml/shared/{shell,components,instruments,inputs,theme,about}/
  qml/downstream/DownstreamWorkbench.qml
  qml/downstream/{configuration,transmitter,channel,receiver,measurements,instruments}/
  qml/upstream/UpstreamWorkbench.qml
  qml/upstream/{configuration,transmitter,channel,receiver,measurements,instruments}/
  src/main.rs
  src/bridge/                  all registered CXX-Qt QObject entry files
  src/shared/{instruments,inputs,workbench,presentation}/
  src/downstream/{inputs,instruments,simulation,workbench}/
  src/upstream/{inputs,instruments,simulation,workbench}/
  src/platform/                native lifecycle, I/O, update and style adapters
  tests/                       retained and new QtTest fixtures
```

These are planned owners, not empty trees to create in advance. Existing file
names are retained unless a documented split needs a new name. `Workbench.qml`
becomes `qml/downstream/DownstreamWorkbench.qml`; a new shared shell owns tabs.
Existing workbenches map by responsibility: workbench configuration →
`downstream/configuration`, TX → `downstream/transmitter`, channel →
`downstream/channel`, receiver and verification → `downstream/receiver`.
Inputs map to direction configuration/measurements; instrument dialogs remain
under the direction until their presentation extraction passes shared acceptance.
Inventory destinations are the first safe move, not a promise that every existing
file becomes shared. Common capture metadata/viewer extraction follows actual
semantic review, not the occurrence of “measurement” in a filename.

`src/bridge/` stays intact for CXX-Qt 0.7.3. Direction-specific QObject files added
there delegate to their respective direction adapters. `build.rs` explicitly
registers each source exactly once. Retain `Merlin.Workbench`, the current Main
resource entry URL, icons, resource bytes, target names and native package launch.
Do not upgrade CXX-Qt as part of this reorganization.

## Dependency order

The official checklist owns dependency edges and state. The implementation order
is: approved plan → fresh baseline → architectural guard → public API gaps →
matching Qt consumers → direction isolation → tabbed shell → upstream windows
and runs → proven shared instruments → persistence/recovery → final acceptance.
Generic theme/dialog moves may run after the guard without waiting for every
API extension. The resource-grid API and upstream observation API are independent
units; grid presentation can follow configuration before full run instrumentation.
Do not move a scientific consumer onto an unvalidated new API just to follow a
directory phase. Each API addition is committed and validated before Qt adopts it.

Broad source movement requires the approved full inventory and migration design.
Use focused `git mv` units, update Rust module paths, QML relative imports,
build registrations, tests, maintained documentation and package/resource includes
together. Preserve historical evidence paths and source-origin records as history.
Run the matching Qt checks and build the executable after source moves. Expand to
core/native Python checks only where scientific/API/dependency contracts changed.

## Shared presentation contracts

| Candidate | Shared presentation | Direction-owned API semantics and required evidence |
| --- | --- | --- |
| Scatter/QAM | Canvas, axes, selection, zoom/pan, legends, bounded point display | Coordinate pairing, actual stage/carrier/symbol identity, modulation and pilot/control labels; real DS and US snapshots and tests. |
| Spectrum | Viewport, ticks, navigation, marker and band display | API-computed PSD, units/window/normalization, sample rate and frequency mapping; DS-only PLC overlays never become upstream metadata. |
| Response/echo | Curve renderer and selection | H, C, C×H, H-hat reference planes, API-computed impulse/echo values and labels; missing views stay unavailable. |
| FEC | Charts, tables, row selection and bounded paging | Actual decoder outcomes/iterations, family-specific counters; upstream data has no invented BCH panel. |
| PRBS/verification | Status, progress and comparison presentation | Actual source recurrence/seed/order and post-decode verification, BER denominator and partial-completion policy. |
| Measurement | Provenance tree, navigation, compatible plotting scaffolding | Original grids/nulls/exclusions, capture association/admission, timestamp/hash, direction and input semantics remain API-owned. |

Pure screen transforms, pixel coordinates, clipping and label layout may remain
in Qt. Scientific FFT, PSD, MER/EVM, reference points, resource allocation,
waveform timing, response/echo, SNR thresholds, bit ordering and FEC derivation
must come through validated backend public operations. Renderer sharing cannot
justify a common scientific configuration/report when meanings differ.

## Inventory review and evidence

Each inventory record gives a first destination, decision, rationale and relevant
tests. `declared_tests` lists source-declared cases; `test_references` lists exact
references or related group fixtures with their scope. Neither means those cases
were rerun in this planning unit. Where a file has no direct tests, its acceptance
requires registration/import/native smoke plus the relevant feature regression;
scientific data paths also require independent backend/API assertions.

License and NOTICE files stay with their consumers. Preserve PNM Viewer and other
copied notices/attribution and GPL Qt versus Apache reusable owner boundaries.
No new reuse from other repositories is required by this plan. Never delete
source material or old imports before replacement behavior passes its gate.
