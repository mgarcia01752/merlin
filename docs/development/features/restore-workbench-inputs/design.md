# UI-INPUT-RESTORE-01 — Restore accepted workbench inputs

Replace the old Load demo actions in DPD and DS Channel Estimate with
**Restore Workbench**, and offer the same action in RxMER. Remove the two unused
production demo invokables and their hard-coded embedded fixtures. Examples
remain available through normal file/workbench Open; test fixtures remain tests.

The existing Rust `begin` operations restore draft captures from accepted state,
including state reconstructed from embedded original bytes in a WB JSON file.
QML invokes those operations, resets transient viewers and restores draft
options from accepted metadata.
Do not reread source paths, add a recent-file cache, import another example,
apply a draft automatically, or change WB JSON. After a new Apply, restoration
uses that accepted input even if the workbench has not yet been saved to disk.

DPD restores the accepted capture and selected profile ID and closes its draft
plot. RxMER restores the accepted capture list, selection and playback/tolerance
options, stopping preview animation. Channel Estimate restores the accepted
response and tolerance, including flat/unity when that is the accepted state.
Restore is disabled for DPD/RxMER when no accepted capture exists. The response
always has a valid flat/unity default to restore. Button availability derives
from the accepted metadata returned on opening/restoring, independently of
subsequent draft removals. Apply/Cancel and title X retain their existing roles.
Hover/focus help explains that the action discards pending edits.

## Validation

QML workflow coverage includes disabled empty DPD/RxMER restoration, non-default
accepted profile IDs, draft removal, imported/flat response restoration, capture
list and option restoration, and unchanged accepted state. Existing import,
WB reload, validation and Apply/Cancel tests remain. Run Qt formatting, Clippy,
Rust and QML tests, build the latest executable, inspect actual native workbench
restoration and update current dark guide screenshots. Simulation science,
persistence schema, external source files and runtime decoder dependencies are
unchanged. No full permutation test or installed-package validation is needed
for this editor lifecycle change.

## Completed validation

`scripts/qt-desktop.sh check` passes formatting, warnings-denied Clippy, 62 Rust
adapter tests, 55 QML checks and the latest native build. QML checks exercise
both empty and accepted input states, a non-default accepted DPD profile ID,
removal/restoration, response tolerance and both imported/flat states, and RxMER
capture-list/playback restoration. The existing WB reload tests also pass.

Owned native Ubuntu dark/light runs open an actual self-contained WB, remove
and restore its DPD draft, switch its captured channel response to flat and
restore the original response, then clear and restore its RxMER list. No Apply
is used during these checks. Native logs are free of warnings; guide captures
show the final toolbar and restored inputs. Helpers and intermediate captures
remain ignored under `docs/development/local/restore-workbench-inputs/`.
Other platforms and installed packages were not tested.
