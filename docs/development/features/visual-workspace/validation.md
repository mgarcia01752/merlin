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
