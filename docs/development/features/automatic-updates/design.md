# UI-UPDATE-02: Automatic Ubuntu desktop update

Status: implemented and locally validated; installed lifecycle acceptance
(UI-UPDATE-QA-01) remains pending.

A single green Update chip belongs on the right of File, Settings and About.
Match OSCAR-UI UpgradeChip colors, spacing and radius. Clicking the chip starts
the existing trusted stable-release flow directly; the separate dialog shows
progress/errors and retains X/Close and the global startup preference.

Rust owns metadata, bounded TLS download, SHA-256 verification, private staging,
package identity/newer-version checks, polkit installation and normal-user restart.
Use the installed launcher for elevated helper mode before Qt initialization.
Never elevate a source checkout. Updating is unavailable during simulation.
Preserve accepted WB settings in a private recovery file and restore after restart;
transient results and unapplied dialog drafts are not persisted.

References inspected read-only: OSCAR-UI UpgradeChip.java (Apache-2.0),
OSCAR-UI reference revision: 778575183a385197d315e7bd4cce83254c1ce843.
PNM Viewer Toolkit reference revision: 5fbd5561f3e06b73486546b5ffa520279080a5eb.
PNM Viewer Toolkit apps/viewer/src/updates/ubuntu download/install modules
(GPL-3.0-only). Adapt their responsibility boundaries to MERLIN's single DEB.
Keep copied copyrights and Qt GPL license. No Python runtime or sidecar.

```mermaid
flowchart LR
    CHIP[Update chip] --> SAVE[Recovery workbench]
    SAVE --> FETCH[Download and verify]
    FETCH --> AUTH[Ubuntu authentication]
    AUTH --> INSTALL[Private staging and APT]
    INSTALL --> RESTART[Normal-user restart and WB restore]
```

Validation: release metadata/redirect/size/hash rejection; staged DEB identity
and rollback-safe failure reporting; repeated-click guards; QML chip location,
direct action/progress and run lock; adapter Clippy/tests and native dark UI.
No local system installation or real release download is implied by validation.

## Implemented and verified

The menu row contains File/Settings/About and a right-aligned Update chip.
The chip starts installation directly; About retains the separate manual check
and global preference dialog. Main-window close is blocked while installation
is active; the status dialog keeps X/Close. A recovery-save failure or active
simulation prevents the install action. Download progress is one bounded latest
string, independent of the Qt thread. Repeated clicks cannot start extra workers.

Downloads use exact newer-release metadata rechecked on click. Each TLS redirect
is allowlisted; streamed bytes must match the size/hash. Root staging opens
caller-owned regular inputs without following symlinks, using a stable directory
descriptor, and re-verifies bytes and actual DEB control fields. APT requires an
installed package, a strictly newer version and no removals. Authentication and
APT failures leave the original app open; no rollback is claimed. The DEB declares
pkexec. The restarted launcher clears stale Qt runtime paths, runs as the desktop
user and restores accepted WB state. Save may request a destination after restart;
the recovery snapshot does not adopt the original user's Save destination.

`qt-desktop.sh check`: 33 Rust adapter tests and 35 QML checks pass. Final
warnings-denied Clippy and formatting pass. Tests cover corrupt/truncated/oversize
streams, trusted hosts and plan identity, private storage cleanup, checkout refusal,
and actual built DEBs with wrong package/version/architecture. Three package-tool
tests pass. The updated skill passes its validator and both discovery links resolve.

Native Ubuntu/Qt inspection confirmed the current published version is up to date
and the chip stays hidden, with the revised dialog and top menu rendered correctly.
The guide contains the actual refreshed dark screenshot. A second native launch
with an isolated state directory restored the real single-channel example through
--restore-update; all WB blocks became green. Both native logs were empty. The
latest Qt development executable is built. No system package was installed, no
polkit authentication was requested and no commit, push or release was performed.

A real download/elevated replacement/restart cycle, cancellation during polkit,
and partial-APT failure need disposable installed-OS validation (UI-UPDATE-QA-01).
The first upgrade from an older browser-only MERLIN build still requires a manual
DEB install; subsequent upgrades use this new flow. No success is claimed for
those installed paths from pure tests or native source-run screenshots.
