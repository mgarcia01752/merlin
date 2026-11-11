# UI-WB-CONFIG-01: Main workbench run configuration

Status: implemented and locally validated. Hosted validation of the workflow
consolidation/APT recovery remains pending (CI-DESKTOP-QA-01).

Put Configure WB before Start on the main lower control row. File New creates
an explicit blank WB; successful Open/recovery or first Save activates one. Failed/canceled
Open preserves state. Disable configuration before New/Open and during a run.
Receiver keeps correction mode/iteration limit; WB owns duration, total budget,
noise seed and profile checkboxes (Select All/Clear). Configuration can be saved
before all input blocks are green. Start still requires full validated inputs,
receiver options, comparison and at least one selected compatible profile.

Rust owns the accepted profile-ID list, catalog, validation and frozen sweep.
Each selected profile runs separately in ID order with the existing shared budget.
New WBs default to the applied profile; preserve legacy all/applied behavior when
opening old files. Store explicit selections in version-2 WB JSON and accept
version-1 legacy files through an explicit migration. Reject unknown/duplicate IDs,
invalid types and unsupported versions without partial adoption. Empty selection
is editable/savable but cannot Start. Results remain transient.

Owners: workbench file/session lifecycle, closed_loop Options/admission, a separate
WB Configuration QML dialog and Receiver Configuration QML. Existing core DSP is
unchanged. Test points: New/Open gating, partial setup, profile subsets/rejection,
legacy migration, Save/Save As/Open/recovery, Start admission, running lock, native
dark screenshots. Maintain user guides and leave the compiled Qt app ready.

## CI-DESKTOP-APT-01

Run 43061645839 failed during desktop setup on Ubuntu 22.04: APT could not resolve
azure.archive.ubuntu.com and failed to fetch python3-pil (exit 100). Native tests,
CodeQL and dependency audit passed; publish was skipped. Preserve all gates.
Use bounded retries around strict index refresh and package installation. Prefer
Ubuntu HTTPS mirrors in the disposable runner's mirror list, without changing
local user mirrors. Validate retry behavior using fake commands, not local apt.
Hosted recovery remains unverified until a subsequent run.

## CI-CONSOLIDATE-01: Two workflow entry points

Master (`ci.yml`) owns shared preflight/tooling/Clippy, CodeQL, both-lock RustSec,
three Ubuntu native test jobs and Ubuntu 22.04 Qt checks. It also runs for PR,
manual and weekly events. Release verifies an exact version tag, calls Master
with the release input enabled, then waits for all jobs before collecting and
publishing the same existing CLI/desktop artifacts. Release-only steps build
optimized binaries, package and validate installed lifecycle on disposable CI.
Master builds/smokes the debug CLI; both paths keep the same test/security gates.
Native tests reuse the pinned four-process nextest runner, retaining doctests,
analysis-only and example coverage. Remove the three redundant workflow files.
No force-push, tag mutation or automatic rerun is part of this change.

## Validation and current limits

`qt-desktop.sh check` passes35 adapter tests and36 QML checks, including subset
ordering/rejection, persistence, schema1/2 compatibility, New/Open cancellation
and activation, first Save creation, partial setup, running lock and Cancel
preservation. Formatting and warnings-denied Clippy pass. The unchanged full
four-profile worker test remains enabled and passed; selecting a subset does not
change core modulation/FEC algorithms. A final targeted subset check and native
build are retained after preserving the legacy minimum runtime bound.

Native Ubuntu/Qt dark captures show the main Configure WB control, Run/Profiles
and the reduced Receiver Configuration. Through the real GTK picker, Save As
wrote schema2 with selected IDs1/3 and unchanged applied DPD0. Open restored that
selection visibly. Startup/New captures cover the inactive/active WB states;
the native logs contain no Qt/QML diagnostics. The guide contains current images.
Two preliminary automation attempts did not find the exact requested GTK save
filename; the successful check follows the actual chosen file, validates its JSON
and reopens it. They are not claimed as passing exact-name checks. User input
files and curated examples were not overwritten. The compiled executable is ready. Profile catalog polling borrows only IDs and
does not copy PNM/assignment arrays. Save validates the complete document before
writing, so changed-DPD stale selections cannot create an unreadable saved WB.
Three final persistence tests pass, including actual disk round trip and an
invalid-selection write that preserves the previous file bytes.

Local tooling: six setup tests (including bounded APT refresh/install retry),
nine packaging tests, five release-helper tests and41 Git-helper tests pass.
ShellCheck and workflow shell syntax pass. YAML review verifies exactly two
entry points, shared security/Qt/native gates, three Ubuntu baselines and release
publication dependencies. The installed older actionlint needs one local known
label exception for ubuntu-26.04; other lint checks pass. The cited failed run's
Ubuntu26 native job itself passed, so this exception does not remove that target.
Skill validation and both discovery links pass for the edited canonical skills.

The workflow changes are local: no hosted rerun, tag/push/publication, local APT
operation or system-wide application installation was performed. New hosted
execution is required to establish successful network recovery, parallel CI
execution and the consolidated package/publication lifecycle. Existing master
and tag pushes can each validate the same commit; shared code/caches avoid
configuration drift, not mandatory tests. Logs and scratch WBs remain ignored.

## UI-DIALOG-ACTIONS-01: Consistent editor actions

Use one shared ApplyCancelFooter for all nine Apply editors: WB, receiver,
OCD, DPD, source, measurement input, additional loss, amplifier and attenuator.
Place Apply then Cancel in a single lower-right row with equal button sizes.
Keep Reset defaults, Remove input and profile selection to the left. Preserve
validation, enabled states, acceptance and draft cleanup; Cancel rejects without
applying. Avoid platform-dependent button-role ordering. No DSP or persistence
contract changes. Validate the QML suite, native dark views and rebuilt Qt app.

The main button is labeled Configure WorkBench.

QML validation passes37 checks, including all nine editor footers: exact labels,
left-to-right order, equal width/height, shared baseline,10px edge margins and
spacing, and dismissal through Cancel. Existing accepted-draft tests pass.
The check opens nested channel editors through their parent workbenches.
Both ui-design and coding-style skill validation and discovery links pass.

The rebuilt Qt executable was checked on the native VM desktop. Updated dark
guide captures show the main button, WB Run/Profiles, receiver, amplifier,
attenuator and loss editors. Native logs contain no Qt/QML diagnostics.
No non-Qt simulation rebuild or full core test run was needed for this UI change.
