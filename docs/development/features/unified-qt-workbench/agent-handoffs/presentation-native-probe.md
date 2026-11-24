# C5 native presentation probe source handoff

This source-only unit supports the existing UIU-SESSION-01 C5 native acceptance
gate in [the primary design](../design.md#uiu-session-01-c5-existing-persistent-presentation-owners).
It does not close C5, C6 or SESSION. Its base is accepted commit
`6b32dcfc401929f087c9871933c088a77ac94ec8`; the lead supplies the current C5
PresentationOwner/native adapter and registration changes independently.

The exact source scope is this handoff and
`apps/qt/qml/validation/SessionPresentationProbe.qml`. The lead owns Main/build
registration and must create this fixed component only for the opt-in
`--session-presentation-probe` mode, after normal startup. No external QML loader,
scientific API, scientific execution, package/install action, policy change or
ordinary-startup probe construction is introduced here.

The component requires the actual `theme`, `workbench`, application `window` and
an absolute, fresh `screenshotPath`. The lead must reject an existing screenshot
destination before instantiation; the component has no filesystem-query API.
The existing dark theme is asserted and never changed. The only file publication
is the explicitly supplied fresh screenshot. No user capture or project is read
or written by the probe.

The explicit Timer advances finite phases at 25 ms, with 400 attempts per phase
and a separate 120-second overall watchdog. It opens the existing receiver
configuration dialog and waits for its real native text/data admission. One
PresentationOwner supplies all pressure. Each binary-search trial clears its
prior filler first; native limits and LogicalFootprint's measured single code
unit derive the search range without duplicating the UTF-16 conversion constant.
The search has a derived logarithmic trial bound. It independently rejects the
next code-unit size, retains the largest accepted string and checks replacement
growth preserves that owner.

Under held pressure it invokes the actual receiver footer `applied()` signal,
checks exactly one `configurationApplied` callback and dialog closure, verifies
native `accepted_options`, and requires optional receiver details plus an actual
workbench poll to reject. It calls the real FAQ method, requires rejection and
the rendered global admission notice, then grabs the real window before clearing
pressure. Help must open and close after release. `finished(true, evidenceJson)`
reports those observations; failure, completion and destruction clear the filler
and disconnect the probe's sole callback. Production handlers stay connected.

Source verification and execution boundaries:

- QML parser/format syntax check: passed with the existing Qt 6.8.3
  `qmlformat -n` and its SDK library path, writing only to
  `/tmp/merlin-session-presentation-probe-qa-source.qml`. The tool does not
  support `-o`; that initial invocation failed before parsing and was corrected
  to its documented standard-output interface. This is not a native load or
  import-resolution test.
- Python integration guide `--check`: current, without a generated-file change.
- Source inspection and the final allowlist contain exactly the two new paths
  named above. No Main/build registration or existing source was changed.
- Staged `git diff --cached --check`: passed.
- Cargo/build, native launch, screenshot generation and runtime assertions:
  **unexecuted** in this isolated source worktree; the lead owns these checks.
- Existing architecture/checklist state remains partial. Main registration and
  fresh artifact evidence are required before any native acceptance claim.
