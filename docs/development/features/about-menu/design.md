# UI-ABOUT-01: Focused About and menu labels

Remove ellipses from dropdown menu labels while preserving shortcuts/actions.
About menu retains About MERLIN, FAQ and Terminology; move manual Check For
Updates into the About dialog. Keep the green automatic Update chip unchanged.
Simplify About to icon, product/version, concise description and equal-size
Project on GitHub, Report Issue and Check For Updates buttons. Retain Close/X.
Remove the standalone sentence and license/acknowledgement section from About;
keep bundled license/NOTICE files and distribution obligations intact.

About emits action signals; Main routes update checks through the existing
Rust-backed UpdateNotification and opens fixed project/issue URLs externally.
Report Issue opens GitHub's issue form, never submits a report itself. Tests
intercept signals without opening browsers or sending reports. Update the UI
and coding-style skills, current user guide, architecture/checklists and native
dark About/menu screenshots. Verify QML actions, keyboard/theme/minimum-window
layout, adapter checks and native build; don't claim hosted validation.

## Validation

Full Qt check passes formatting, warnings-denied Clippy, 45 adapter tests,
native build and 47 QML checks. Tests verify plain menu labels, no old update
menu item/license links, exact project/report URLs via intercepted signals,
update routing, equal button sizes, Space-key activation and minimum-window
bounds in dark/light themes. The spectrum placement assertion now waits for
asynchronous layout after viewport/size changes.

Actual native XCB captures show About/menu cleanup in dark mode and balanced
actions in light mode. Clicking Check For Updates opens the existing update
dialog and completes a manual check. The native log is empty. Current dark
guide images are replaced; local light/routing evidence remains ignored under
docs/development/local/about-menu. No issue was submitted, no license files
were removed, and no commit/push or system installation was executed.
