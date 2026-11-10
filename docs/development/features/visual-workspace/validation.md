# Initial workbench validation

Validated on Ubuntu 22.04 x64 with Qt 6.8.3 and Rust 1.85.1. UI-01 remains
partial; this evidence covers the shell and presentation, not scientific UI jobs.

- Native Qt build passed. The application opened in the active VM desktop with
  the title MERLIN — Simulation Workbench and WM_CLASS merlin. Its X11 window
  icon property contains the embedded icon. Interactive user approval is pending.
- Qt formatting, warnings-denied Clippy and adapter test harness passed. The
  adapter has no Rust unit tests yet; behavior coverage is in QML tests.
- All 13 QML test cases passed: nine block dialogs, theme/control behavior,
  rendering/minimum-size navigation, and test setup/cleanup. Tests do not start
  simulations; lifecycle controls are disabled.
- Dark/light workbench and minimum-size dialog captures were rendered through
  Qt's offscreen software backend and visually inspected. These are actual QML
  renders, not proof of every desktop compositor or other Ubuntu target.
- Version propagation tests (5) and disposable local release tests (23) passed
  after registering the Qt manifest/lockfile with the existing version helper.
- Shell syntax, Python syntax, document links and diff whitespace checks passed.
- The dark icon is unchanged. The approved matching light icon replaces the old
  light asset. The checkout-local desktop launcher was generated successfully;
  it passed desktop-file-validate and was not installed into an application menu
  or a system directory.

The initial full Rust workspace tests, workspace Clippy with warnings denied,
Rust formatting and analysis-only tests passed. The analysis-only build emits an
existing dead-code warning for SymbolPacker.coded_bits; no scientific source was
modified in this increment. Future UI-only iterations use Qt-focused validation
and preserve compiled artifacts; broader checks run only when necessary.

Logs and rendering outputs are local-only under
`docs/development/local/visual-workspace/` (absent from fresh clones):
`qt-check.log`, `qml-final.log`, `rust-tests.log`, `rust-clippy.log`,
`rust-analysis-only.log`, `version-release-tests.log`, `workbench-dark.png`,
`workbench-light.png` and `workbench-dialog.png`.

Ubuntu 24.04/26.04 runtime validation, installers, settings/file editors,
scientific jobs, pause/resume and analyzers remain unverified or unimplemented.
The known rand advisory remains outside this shell change and is not suppressed.

## OFDM Settings editor increment

Validated on Ubuntu 22.04 x64, Qt 6.8.3 and Rust 1.85.1:

- Qt adapter formatting and Clippy passed; native executable rebuilt and restarted
  on the active VM. The existing core dependency dead-code warning remains.
- Two Rust tests passed: rejected-edit atomicity (invalid bandwidth, CP/RP,
  interleaver depth, malformed/oversized JSON), valid 50 kHz/depth-32 acceptance,
  and bundled registry default/choice agreement with core validation.
- All 14 QML cases passed after adding actual dropdown-keyboard editing,
  Cancel/Escape, Apply and reopen checks with a presentation test adapter. Rust
  tests and native interaction separately verify the production backend.
- Native interaction rejected CP 192 / RP 256, preserving the editor and showing
  the Rust diagnostic. Reset defaults, Apply and reopen succeeded. The OFDM
  popup remains open for user review. The native application log has no QML errors.
- The dark native VM screenshot is tracked in
  [the user guide](../../../user/workbench.md). It shows the actual production
  backend and registry, not the QML test adapter. Private review captures remain
  under `docs/development/local/visual-workspace/`.
- No scientific source changes or full scientific test rerun. The Qt adapter now
  consumes the existing core with default simulation features disabled; accepted
  settings are in memory only. Other editors, file persistence, application
  navigation and run integration remain pending.

The dialog-per-QML-file rule was added to coding-style; skill validation and both
canonical discovery links pass. Ubuntu 24.04/26.04 remain unverified.

## Approved Parameter/Value tree revision

The tree uses full-width Parameter/Value rows with indented children, expandable
Symbol timing, Interleaving and Modulation Profile branches, and descriptions
below. OFDM channel defaults are no longer presented as profile information.
Profile ID, Source file and Subcarrier assignments stay blank with no selected
profile. The core still validates the complete accepted transmission settings;
channel/profile input integration remains pending.

Qt-focused validation on Ubuntu 22.04 x64: 15 QML cases pass, including branch
collapse/expand, three blank profile fields, no spectrum editors, and existing
Apply/Cancel/reopen behavior. Two existing Rust adapter tests pass; no scientific
source changed and no full scientific suite was run. PHY I20 §7.5.13.4 and the
local MULPI I25 §§6.4.40–6.4.41 text extraction establish the terminology; this is
not rendered-PDF verification. Other Ubuntu versions remain unverified.

The rebuilt native application was restarted on the VM. Visual inspection of the
production dialog confirmed the approved tree, three editable settings, blank
profile values and descriptions below. The updated dark screenshot is tracked in
the user guide, the native log has no QML errors, and the dialog is left open for
review. Formatting, Qt Clippy/build, document links and diff whitespace pass.

## Heading-only blocks and readiness stripes

Qt presentation checks cover green Complete, yellow Needs attention and red Error
colors with explicit accessible status/reasons, all nine preview blocks initially
yellow, and OFDM still yellow after accepting a settings draft without a profile.
17 QML cases pass. The skill validator and canonical discovery links pass.
No scientific source changes or full scientific suite are needed for this change.
Automatic profile/run-driven state changes remain pending alongside their inputs.

Qt formatting, Clippy, both Rust adapter tests and the native build passed. The
application was restarted on the active Ubuntu 22.04 VM and the heading-only
blocks/yellow stripes were visually checked. Updated native dark overview and
OFDM-dialog screenshots are tracked in the user guide. Native logs contain no
QML errors; the workbench overview is left open. Document links and whitespace
checks pass. Ubuntu 24.04/26.04 runtime validation remains pending.

## All-green Start gate and OCD label

18 QML cases pass after adding the all-block readiness gate. The new check marks
every block green in the presentation fixture, then verifies each individual
block independently closes the gate when yellow or red and restores readiness
when green. Start stays disabled while run integration is unavailable. Fixture
statuses are restored after the check. No non-Qt rebuild/test suite was run.

The block and popup use OFDM Channel Descriptor (OCD), the user-selected full
specification name; stable internal IDs are retained. The architecture records
WB-RULE-03 and distinguishes pre-run readiness from the availability of results.

The Qt-only native build passed and the application was restarted on the VM.
Both the full OCD box label and matching popup title were visually verified;
updated dark overview/dialog screenshots are tracked. The native log is free of
QML errors and the overview is left open. Document links and whitespace pass.

## UI-02: DPD file input and source dependency

Validated on Ubuntu 22.04 x64 with Qt 6.8.3 and Rust 1.85.1:

- 19 QML cases pass, including draft loading, Cancel, Apply, rejected loads and
  staged removal/readiness. Presentation fixtures do not claim decoder validation.
- Four Qt Rust tests pass: the existing OCD tests plus binary/JSON assignment/grid
  parity and draft rejection/removal/compatibility acceptance. Content detection
  succeeds independently of misleading source names.
- Existing targeted core coverage passes: three pnm_json_import cases, two
  pnm_json_contract cases, and one new shared detector case. The existing actual
  CLI import-command test also passes. No full DSP/FEC suite was rerun.
- Qt formatting/Clippy/build and core library Clippy pass. The existing optional
  no-simulation SymbolPacker.coded_bits warning remains; no suppression was added.
- Native Open File loaded both the binary and JSON demo. An ASCII file named
  .bin rejected as unsupported text while preserving the accepted input. Cancel
  preserved accepted input; applied removal returned OCD/DPD to yellow, and
  accepting the compatible demo returned both to green. Start stayed disabled.
- The final executable was rebuilt/restarted and the DPD dialog left open with
  demo profile 0. Native dark screenshots show real production state; logs have
  no QML errors. The original binary fixture hash matches existing provenance.
- No PNM Viewer source is vendored. Cargo fetched pnm 0.3.6 at revision
  5fbd5561f3e06b73486546b5ffa520279080a5eb into ignored local storage. The 73 source,
  build and resource hashes match the reviewed upstream copy. Its notices are
  retained under apps/qt/third-party; runtime needs no upstream checkout.

Decoder-format acceptance remains type 10 / format 1.0 / range schemes. Skip and
unsupported encodings reject; other measurement imports, persistence, profile
editing and simulation integration remain pending. Viewing/selecting four demo
profiles does not broaden the existing measured-runner support contract.
