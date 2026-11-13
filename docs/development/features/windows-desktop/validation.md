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

## UI-MSI-04: Windows QML failure needs startup/test logs

[Run 43370737351](https://github.com/mgarcia01752/merlin/actions/runs/43370737351)
passes all 72 Windows adapter tests, confirming the FAQ repair. QML exits
without console output; the log records only `QML tests failed`. Ubuntu and
security jobs pass, while MSI packaging/lifecycle and publication are skipped.
No specific QML assertion or startup cause is established from that evidence.

The Windows check now explicitly waits for the runner and captures text/JUnit,
stdout/stderr and decimal/hex exit status under `dist/windows-qml-logs/`. CI
uploads that directory even on failure for Master and Release. Checks still fail
on a nonzero runner exit. No assertions or hosted validation gates are removed.

A single existing QML layout test passes locally with both file loggers; the
JUnit file parses as XML. No Rust rebuild/full suite was run. Windows PowerShell
and GUI-process behavior remain pending native execution; this is a diagnostic
change, not a claimed repair of the unidentified QML failure.

## UI-MSI-05: confirmed QML layout/path repair

Run 43373385198 fails the About action text-plus-padding width assertion and
screenshot output to `/D:/...`; QML reports 66 passed and two failed. All 72
Windows adapter tests, Ubuntu native/desktop jobs and security checks pass;
MSI packaging/lifecycle and publication are skipped.

About now rounds the largest implicit action width up, uses equal minimum
widths and stacks the actions when three columns cannot fit. The existing
layout check additionally exercises 24-pixel action fonts in both themes and
keeps text/padding, screen-boundary and keyboard assertions. Its popup palette
matches the application window; the refreshed dark user-guide image is rendered
from the Qt test fixture (example version).

Both native helpers prepare and select the screenshot output directory. QML
saves relative filenames, avoiding file-URL conversion and Windows drive-prefix
errors. Both focused About and render tests pass on Linux Qt6.8.3 (four passes
including setup/cleanup); shell syntax/ShellCheck and diff checks pass. Full
suites were not run. Windows execution and MSI/lifecycle remain pending CI.

The Linux Qt desktop executable was rebuilt successfully in 1m58s and its
offscreen `--smoke-test` exited successfully without output. This confirms local
startup only; it is not Windows runtime or installer evidence.

## UI-MSI-06: Windows notice checksum failure

[Run 43470708630](https://github.com/mgarcia01752/merlin/actions/runs/43470708630)
passes all 72 Windows adapter tests, all 68 QML checks and optimized Windows
compilation. Ubuntu/security checks pass. MSI staging fails in `collect_notices`
with `Reviewed dependency license checksum changed`; installed lifecycle and
publication are skipped. The previous About/screenshot fixes are confirmed.

A disposable Git index reproduces the mismatch with `core.autocrlf=true`. The
scoped `.gitattributes` rule preserves original packaging-resource bytes without
modifying their provenance or weakening exact-byte checks. Error messages now
identify the resource and expected/actual SHA-256 values. Two focused tests pass:
actual Git checkout/complete collection with and without attributes, and rejection
of altered license bytes. No Rust/Qt rebuild/full suite is needed for this change.
Windows MSI packaging and installed lifecycle still require hosted evidence.

## UI-MSI-07: installed icon registration

[v0.2.5 run 43476334867](https://github.com/mgarcia01752/merlin/actions/runs/43476334867)
passes notice verification, native MSI authoring and relocated smoke. The MSI
is 65,450,080 bytes; staged installed files total 133,732,485 bytes. Initial
native MSI installation succeeds, but installed verification finds no DisplayIcon.
The retained installation log shows ARPPRODUCTICON and IconCreate referring to
extensionless ApplicationIcon. Publication is skipped.

The `.ico` identifier now follows WiX authoring guidance across Icon,
ARPPRODUCTICON and both shortcuts. Missing properties still fail validation,
with safe strict-mode access and a retained registration JSON record. All five
focused Windows packaging tests pass. Native registration and the remaining
repair/shortcut/uninstall checks require the next hosted release run.

## UI-MSI-08: use MSI product metadata for icon verification

[v0.2.6 run 43480254153](https://github.com/mgarcia01752/merlin/actions/runs/43480254153)
passes native validation/package generation and installs successfully. The log
shows ProductInfo(ProductIcon=ApplicationIcon.ico) and IconCreate, but uninstall
registration lacks DisplayIcon. The prior filename fix was insufficient to
resolve this verifier assumption. Publication is skipped.

The lifecycle verifier now queries Windows Installer's ProductInfo ProductIcon
for the registered product code, requires the cached icon and compares its hash
with packaging/windows/merlin.ico. It retains registry uniqueness/uninstall
metadata, smoke, repair, optional shortcut and uninstall checks. Evidence includes
the queried product code and cached icon path. No native assertions are skipped.
PowerShell/COM execution remains pending CI; no local system installation or
full simulation/Qt suite is run.

## UI-MSI-02: successful native release validation

[v0.2.7 run 43482850185](https://github.com/mgarcia01752/merlin/actions/runs/43482850185)
passes every required job and publishes the complete stable release. Windows
passes 72 Rust adapter tests, 68 QML checks, native optimized compilation/MSI
authoring, relocated smoke and default install, registered icon/hash checks,
repair, opted-in shortcut installation and final uninstall while preserving a
user workbench. The lifecycle log ends with
`MERLIN_MSI_INSTALL_REPAIR_SHORTCUT_UNINSTALL_OK`.

The published MSI is 65,445,984 bytes; staged installed files total 133,733,453
bytes. Qt and MSVC CRT are bundled; Windows 10/11 x64 system libraries remain
external prerequisites. The MSI is unsigned. The same run passes Ubuntu
22.04/24.04/26.04 native checks, Ubuntu desktop lifecycle, CodeQL and RustSec.
The release workflow completed in 16m57s; the local skip-test push took 1m32s.

This is Windows Server 2022 CI evidence. Interactive Windows 11 icon/wizard/
picker/security-prompt review and cross-version upgrade/downgrade remain open
under UI-MSI-09. No interactive approval, signing or macOS support is claimed.

## UI-MSI-09: remaining interactive and upgrade review

Retain native Windows 11 review evidence for branded wizard/icons, native
Open/Save/multiple pickers and normal security prompts. Exercise upgrade from
an older installed release and reject downgrade without losing user files.
These checks remain pending; successful CI repair is not upgrade evidence.
