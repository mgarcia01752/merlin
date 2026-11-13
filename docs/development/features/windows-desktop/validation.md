# Windows desktop validation

UI-MSI-01 implementation is complete; UI-MSI-02 native execution is pending.
No Windows MSI, installed Windows result or package-size measurement was produced
on this Linux host. No commit, push, signing or publication was performed.

Local checks:

- 18 packaging tests pass, including stable path identities, upgrade separation,
  unchecked/secure shortcut property, icon binding, no-clobber/case-collision
  rejection and the mocked native deployment/authoring pipeline.
- 7 development-tool checks pass, including native Windows SDK executable names.
- Qt formatting and Clippy pass. All 75 Qt adapter Rust tests pass, including
  Windows MSI metadata selection and the existing Linux updater tests.
- All 68 QML checks pass, including Download MSI skipping recovery/install.
- The latest Linux Qt executable is rebuilt; its offline `--smoke-test` passes.
- Full public desktop resources and complete cached notices were staged locally
  (1282 entries) and authored into MSI XML without case collisions.
- WiX3 structural XSD validation passes for the generated product/full payload
  and adapted UI. libxml2 cannot parse WiX's Microsoft regex extensions, so this
  local structural probe omits pattern facets. Native candle/light retain all
  validation and are required before producing a Windows artifact.
- ICO/BMP dimensions/provenance checks pass. Regeneration produces identical
  hashes; the actual generated dialog/banner artwork was visually inspected.
- VERSION metadata and canonical release-skill discovery checks pass.
- Workflow lint passes other rules. The installed actionlint does not recognize
  the pre-existing ubuntu-26.04 runner label; only that known label warning is
  excluded during the local lint check. Existing Ubuntu jobs remain unchanged.

Native Windows CI must still compile/link resources and COM pickers, deploy the
actual runtime, create the MSI, measure bytes and pass install/repair/shortcut/
uninstall/offline probes. Interactive Windows 11 review must cover the native
wizard, shortcuts/icons, Open/Save/multiple pickers, upgrade/downgrade and normal
launch/security approval behavior. Linux checks do not establish those outcomes.

See [design](design.md), [build guide](../../windows-desktop.md) and
[installation guide](../../../user/install/windows.md).

## EXAMPLES-01 follow-up

Desktop staging now excludes `examples/cli/` while retaining workbench and
shared measurement collections. The standalone request JSON files and sample
message moved with identical hashes; all tracked consumers use their new paths.
The real public-resource/notice payload was re-authored successfully with this
allowlist and its product XML passes the same structural WiX check. Packaging
checks now total 20, including desktop exclusion and retained CLI archive inputs.
Native Windows build/installed acceptance is still pending; it is not replaced
by these local authoring checks. CI also checks the actual WiX compiler version.

The follow-up Windows smoke regression verifies uppercase environment keys after
copying `os.environ`, removal of developer SDK paths and rejection of QML startup
errors. This fixes the native SYSTEMROOT lookup before the first Windows run.

## UI-MSI-03: first hosted Windows FAQ failure

[Release run 43365042088](https://github.com/mgarcia01752/merlin/actions/runs/43365042088)
passed version, preflight, CodeQL, RustSec, all three Ubuntu native jobs and
Ubuntu desktop packaging/lifecycle. Windows compiled the adapter and ran 71
tests: 69 passed and two FAQ tests failed on LF-only heading matching. MSI
packaging/lifecycle did not run; publication was skipped.

The repair normalizes bounded embedded FAQ text before section matching and
adds regression coverage for every catalog entry and the index under LF/CRLF.
Unknown codes and oversized input remain rejected. Native Windows validation
and MSI lifecycle still require a successful new hosted run.

Focused local verification: all six `help::tests::` tests pass (70 other adapter
tests filtered out), including explicit LF/CRLF regression coverage. Rust
formatting and diff whitespace checks pass. The full Qt check was stopped at
the user's request; no full Rust/QML suite or application rebuild was completed.
The focused command compiled its test binary in 3m44s; tests took 0.01s.
