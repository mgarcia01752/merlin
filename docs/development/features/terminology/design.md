# UI-DOC-02: Online terminology guide

Add docs/user/terminology.md as the maintained glossary for workbench, waveform,
QAM, coding, measurement and run terms. Define cell/QAM symbol/OFDM symbol without
confusing their units; distinguish a label, hard decision and LLR, and explain
that Source uses the accepted PRBS-TX configuration (PRBS15 or exact file bytes
with optional fill). Link current detailed guides and pinned PHY terminology.

About → Terminology… emits the canonical GitHub master Markdown URL through the
shared menu. Main opens that URL with the system browser; tests intercept the
signal so they do not launch a browser. Existing About and offline licenses stay
available. Link the glossary from the documentation index and workbench guide.
The remote Markdown is only available after the user pushes it; do not publish
or claim remote availability during local implementation.

Validate QML menu text/target dispatch, local documentation links, glossary
consistency and native dark menu capture. Rebuild Qt only with retained artifacts;
no non-Qt compilation is required for these UI/documentation changes.

Implemented and validated: the glossary covers signal-path, modulation/cell,
configuration/measurement, coding/result and analyzer terms. The About menu
emits the exact master-branch GitHub Markdown URL; Main delegates to the default
browser. All27 QML checks pass, including interception of the menu target without
opening a browser. Native dark capture shows the new menu action. The Qt build
succeeded with retained artifacts, and the executable is running in the VM with
no QML warnings. Local links, image integrity and diff whitespace pass. Remote
publication/browser rendering of the new page remains pending the user's push;
no commit, push, remote mutation or browser launch was performed.
