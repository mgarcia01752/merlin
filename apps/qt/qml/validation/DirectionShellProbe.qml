/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed, opt-in native Main probe. Main supplies the real adapters and image paths.
// Start with examples/workbench/downstream/single/workbench.merlin.json in the dark theme.
// No scientific values are invented: preparation reapplies its accepted options.
QtObject {
    id: probe
    required property var rootWindow
    required property var workbench
    required property var tabs
    required property var host
    required property var controls
    required property var captureItem
    required property string downstreamImagePath
    required property string upstreamImagePath
    signal finished(bool success, string message)

    // GUI scheduling/traversal bounds, independent of all scientific limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 800
    readonly property int overallTimeoutMs: 120000
    readonly property int itemTraversalLimit: 8192
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    property QtObject state: QtObject {
        property string phase: "loading"
        property int phaseRetries: 0
        property double startedAt: 0
        property bool done: false
        property bool startedRun: false
        property bool downstreamSaved: false
        property bool upstreamSaved: false
        property bool settingsWorked: false
        property bool aboutWorked: false
        property bool fileRejected: false
        property bool foreignRejected: false
        property bool pausedAcrossSwitch: false
        property bool configureWorked: false
        property bool terminalObserved: false
        property bool idleSwitchObserved: false
        property bool runningSwitchObserved: false
        property var downstreamItem: null
        property var upstreamItem: null
        property var settingsPopup: null
        property var aboutPopup: null
        property var downstreamIdentity: null
        property var upstreamIdentity: null
        property var pausedRevision: null
        property string acceptedOptions: ""
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /**
     * Stop this fixed acceptance operation on the first missing invariant.
     * @param condition Actual native or visual condition under test.
     * @param message Failure explanation retained in the terminal JSON record.
     */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message)
        }
    }

    /**
     * Advance a bounded asynchronous phase without blocking the GUI thread.
     * @param phase Next compiled phase name.
     */
    function next(phase) {
        state.phase = phase
        state.phaseRetries = 0
    }

    /**
     * Inspect public QML object lists within one finite fixture traversal.
     * @param root Actual Main item or menu bar to inspect.
     * @param name Existing object's stable objectName.
     * @returns The real object, or null when it is absent.
     */
    function findItem(root, name) {
        const pending = [root]
        const visited = []
        while (pending.length) {
            const item = pending.pop()
            if (!item || visited.indexOf(item) >= 0) {
                continue
            }
            require(visited.length < itemTraversalLimit,
                    "Native shell object traversal exceeded its fixture bound.")
            visited.push(item)
            if (item.objectName === name) {
                return item
            }
            for (const key of ["children", "contentData", "menus"]) {
                const children = item[key]
                if (!children || typeof children.length !== "number") {
                    continue
                }
                require(pending.length + children.length <= itemTraversalLimit,
                        "Native shell pending traversal exceeded its fixture bound.")
                for (const child of children) {
                    pending.push(child)
                }
            }
        }
        return null
    }

    /**
     * Compare visible routing with fresh native identities without parsing tokens.
     * @param direction Expected compiled direction tag.
     * @param requireControls False only during a same-turn transient switch;
     * deferred footer text is checked after returning to the settled context.
     * @returns Fresh native shell metadata.
     */
    function checkCursor(direction, requireControls) {
        const current = JSON.parse(host.backend.state())
        require(current.selectedDirection === direction
                && rootWindow.selectedDirection === direction
                && tabs.selectedDirection === direction
                && host.presentation.selectedDirection === direction,
                "Visible direction cursor differs from the actual native cursor.")
        require(typeof current.downstreamIdentity === "string"
                && current.downstreamIdentity.length > 0
                && typeof current.upstreamIdentity === "string"
                && current.upstreamIdentity.length > 0,
                "Native direction identity is unavailable or lossy.")
        require(tabs.identityFor(downstream) === current.downstreamIdentity
                && tabs.identityFor(upstream) === current.upstreamIdentity,
                "Direction tabs replaced an opaque native identity.")
        require(host.backend.accepts(direction, tabs.identityFor(direction))
                && host.accepts(direction, tabs.identityFor(direction)),
                "Selected direction does not own its actual native identity.")
        require(!host.backend.accepts(direction,
                    tabs.identityFor(direction === downstream ? upstream : downstream)),
                "Native routing accepted the other direction's identity.")
        if (requireControls !== false) {
            require(controls.direction === direction
                    && controls.identity === tabs.identityFor(direction)
                    && controls.contextCurrent,
                    "Shared controls retain a stale direction context.")
        }
        require(tabs.downstreamItem === state.downstreamItem
                && tabs.upstreamItem === state.upstreamItem
                && rootWindow.workbench === state.downstreamItem,
                "Switching direction recreated a persistent Main child.")
        require(state.downstreamItem.visible === (direction === downstream)
                && state.upstreamItem.visible === (direction === upstream)
                && state.downstreamItem.enabled === (direction === downstream)
                && state.upstreamItem.enabled === (direction === upstream),
                "Hidden direction remained visible or enabled.")
        return current
    }

    /**
     * Activate an existing tab button through its real Main signal connection.
     * @param direction Compiled target direction, with no execution redirection.
     */
    function switchDirection(direction) {
        const button = findItem(tabs, direction + "DirectionTab")
        require(button && button.enabled, "Actual direction tab is unavailable.")
        button.clicked()
    }

    /**
     * Activate one actual shared run button with its admitted opaque origin.
     * @param name Existing direction control objectName.
     */
    function clickControl(name) {
        const button = findItem(controls, name)
        require(button && button.enabled, "Actual shared control is disabled: " + name)
        button.clicked()
    }

    /**
     * Verify real acknowledged pause and unchanged accepted science across switches.
     * @returns Fresh native downstream status while the genuine lease is active.
     */
    function checkPaused() {
        const current = JSON.parse(workbench.runBackend.poll())
        const shell = JSON.parse(host.backend.state())
        require(current.busy && current.paused,
                "Downstream execution did not retain its acknowledged pause.")
        require(shell.applicationBusy && shell.executionDirection === downstream
                && shell.executionStatus.indexOf("paused") >= 0,
                "The actual paused downstream execution no longer owns the global gate.")
        require(shell.downstreamIdentity === state.downstreamIdentity
                && shell.upstreamIdentity === state.upstreamIdentity,
                "Tab or foreign control changed an accepted native identity.")
        require(current.result_token === state.pausedRevision
                && JSON.stringify(current.accepted_options) === state.acceptedOptions,
                "Tab or foreign control changed paused downstream state.")
        // result_token is the legacy presentation revision, never a RunToken.
        return current
    }

    /**
     * Capture the actual dark QML frame after checking its live tab invariants.
     * @param direction Tab shown by this evidence image.
     * @param path Fresh destination supplied by the native runner.
     * @param nextPhase Phase resumed only after the real image was saved.
     */
    function capture(direction, path, nextPhase) {
        checkCursor(direction)
        checkPaused()
        require(workbench.theme.dark === true, "Evidence capture requires the actual dark theme.")
        require(captureItem && captureItem.width > 0 && captureItem.height > 0,
                "Main did not supply its rendered QML frame.")
        require(typeof path === "string" && path.length > 0,
                "A fresh native image destination is required for each direction.")
        next("capturing-" + direction)
        const started = captureItem.grabToImage(function(result) {
            if (state.done) {
                return
            }
            try {
                checkCursor(direction)
                checkPaused()
                require(result.saveToFile(path), "Could not save native " + direction + " image.")
                if (direction === downstream) {
                    state.downstreamSaved = true
                } else {
                    state.upstreamSaved = true
                }
                next(nextPhase)
            } catch (error) {
                complete(false, String(error))
            }
        })
        require(started, "Main QML frame rejected native image capture.")
    }

    /**
     * Finish once; a failing probe requests cancellation only for its own run.
     * @param success True only after every native, routing and image invariant.
     * @param message Explicit failure reason, empty on full success.
     */
    function complete(success, message) {
        if (state.done) {
            return
        }
        state.done = true
        phaseTimer.stop()
        if (!success && state.startedRun && workbench && workbench.runBackend) {
            workbench.runBackend.stop()
        }
        finished(success, JSON.stringify({probe: "direction-shell", success: success,
            phase: state.phase, error: message,
            loadingProject: !!workbench.loadingProject, hasProject: !!workbench.hasProject,
            controlsContextCurrent: !!controls.contextCurrent,
            controlDirection: controls.direction, controlIdentity: controls.identity || null,
            shellMetadata: JSON.parse(host.backend.state()),
            shellPresentationRejected: host.presentationRejected,
            controlsPresentationRejected: controls.presentationCoverage.rejected,
            fileMessage: workbench.fileMessage,
            persistentChildren: !!state.downstreamItem && !!state.upstreamItem,
            nativeCursor: success, sharedSettingsOnUpstream: state.settingsWorked,
            sharedAboutOnUpstream: state.aboutWorked, fileRejectedOnUpstream: state.fileRejected,
            configureRouted: state.configureWorked, pausedLeaseAcrossSwitch: state.pausedAcrossSwitch,
            foreignControlsRejected: state.foreignRejected, genuineTerminalObserved: state.terminalObserved,
            idleSwitchObserved: state.idleSwitchObserved,
            runningSwitchObserved: state.runningSwitchObserved,
            dark: !!workbench && !!workbench.theme && workbench.theme.dark === true,
            downstreamImage: state.downstreamSaved ? downstreamImagePath : null,
            upstreamImage: state.upstreamSaved ? upstreamImagePath : null}))
    }

    /**
     * Execute the fixed Main lifecycle with per-phase and overall watchdogs.
     * All scientific readiness and control acknowledgements come from Rust.
     */
    function advance() {
        if (state.done) {
            return
        }
        try {
            state.phaseRetries += 1
            require(state.phaseRetries <= phaseRetryLimit,
                    "Native direction probe timed out in " + state.phase + ".")
            require(Date.now() - state.startedAt <= overallTimeoutMs,
                    "Native direction probe exceeded its overall watchdog.")
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem) {
                return
            }
            workbench.pollRun()
            host.poll()
            if (state.phase === "loading") {
                if (workbench.loadingProject || !workbench.hasProject || !controls.contextCurrent) {
                    return
                }
                require(rootWindow.visible && workbench.theme.dark === true,
                        "Probe requires the visible native Main with a dark loaded workbench.")
                require(downstreamImagePath !== upstreamImagePath,
                        "Direction images need distinct fresh destinations.")
                require(!JSON.parse(host.backend.state()).applicationBusy,
                        "Probe requires an idle host; it must not stop an unrelated run.")
                state.downstreamItem = tabs.downstreamItem
                state.upstreamItem = tabs.upstreamItem
                state.settingsPopup = workbench.settingsPopup
                state.aboutPopup = workbench.aboutPopup
                checkCursor(downstream)
                switchDirection(upstream)
                require(!checkCursor(upstream, false).applicationBusy,
                        "Idle tab selection acquired a scientific execution lease.")
                switchDirection(downstream)
                checkCursor(downstream)
                state.idleSwitchObserved = true
                require(!workbench.configurationPopup.visible
                        && !state.settingsPopup.visible && !state.aboutPopup.visible,
                        "Probe requires initially closed shared/configuration dialogs.")
                clickControl("directionConfigure")
                next("configured-dialog")
            } else if (state.phase === "configured-dialog") {
                if (!workbench.configurationPopup.opened) {
                    return
                }
                require(workbench.configurationPopup.contextCurrent,
                        "Configure did not publish the actual accepted downstream draft.")
                state.configureWorked = true
                workbench.configurationPopup.reject()
                next("prepare")
            } else if (state.phase === "prepare") {
                if (workbench.configurationPopup.visible) {
                    return
                }
                const current = JSON.parse(workbench.runBackend.poll())
                require(!!current.accepted_options,
                        "Loaded fixture has no accepted receiver options to prepare.")
                const error = workbench.runBackend.apply(JSON.stringify(current.accepted_options),
                        workbench.settingsBackend.current())
                require(!error, "Native receiver preparation rejected the real fixture: " + error)
                workbench.refreshRun()
                workbench.runBackend.verify()
                workbench.pollRun()
                host.poll()
                next("ready")
            } else if (state.phase === "ready") {
                if (!controls.contextCurrent) {
                    return
                }
                const shell = checkCursor(downstream)
                const current = JSON.parse(workbench.runBackend.poll())
                require(current.can_start && workbench.allBlocksComplete && controls.canStart,
                        "Real loaded workbench is not ready after configure/verify: "
                        + (current.reason || current.receiver_status || "unavailable"))
                state.downstreamIdentity = shell.downstreamIdentity
                state.upstreamIdentity = shell.upstreamIdentity
                state.acceptedOptions = JSON.stringify(current.accepted_options)
                clickControl("directionStart")
                const started = JSON.parse(workbench.runBackend.poll())
                require(started.busy, "Real downstream Start did not dispatch: " + (started.reason || ""))
                state.startedRun = true
                switchDirection(upstream)
                const runningShell = checkCursor(upstream, false)
                require(runningShell.applicationBusy && runningShell.executionDirection === downstream
                        && runningShell.executionStatus.indexOf("running") >= 0,
                        "Switching during actual execution lost the downstream running lease.")
                require(runningShell.downstreamIdentity === state.downstreamIdentity
                        && runningShell.upstreamIdentity === state.upstreamIdentity,
                        "Running tab selection changed a native direction identity.")
                switchDirection(downstream)
                checkCursor(downstream)
                state.runningSwitchObserved = true
                // Same GUI turn: the existing small fixture must reach a cooperative pause.
                clickControl("directionPause")
                next("paused")
            } else if (state.phase === "paused") {
                const current = JSON.parse(workbench.runBackend.poll())
                require(current.busy,
                        "Real fixture completed before pause acknowledgement; no pause evidence.")
                if (!current.paused || !controls.paused || !controls.contextCurrent) {
                    return
                }
                state.pausedRevision = current.result_token
                checkPaused()
                capture(downstream, downstreamImagePath, "select-upstream")
            } else if (state.phase === "select-upstream") {
                checkPaused()
                switchDirection(upstream)
                next("upstream-settings")
            } else if (state.phase === "upstream-settings") {
                if (!controls.contextCurrent || controls.direction !== upstream) {
                    return
                }
                checkCursor(upstream)
                checkPaused()
                require(!controls.busy && !controls.paused && !host.upstreamState.busy,
                        "Foreign controls adopted the hidden downstream run state.")
                const action = findItem(rootWindow.menuBar, "globalSettingsAction")
                require(action && action.enabled, "Actual shared Settings menu action is unavailable.")
                action.triggered()
                next("settings-open")
            } else if (state.phase === "settings-open") {
                checkPaused()
                if (!state.settingsPopup.opened) {
                    return
                }
                require(state.settingsPopup === workbench.settingsPopup
                        && state.settingsPopup.visible && state.settingsPopup.contentItem.width > 0,
                        "Shared Settings did not open on the upstream tab.")
                state.settingsWorked = true
                state.settingsPopup.close()
                next("upstream-about")
            } else if (state.phase === "upstream-about") {
                if (state.settingsPopup.visible) {
                    return
                }
                const action = findItem(rootWindow.menuBar, "aboutAction")
                require(action && action.enabled, "Actual shared About menu action is unavailable.")
                action.triggered()
                next("about-open")
            } else if (state.phase === "about-open") {
                checkPaused()
                if (!state.aboutPopup.opened) {
                    return
                }
                require(state.aboutPopup === workbench.aboutPopup
                        && state.aboutPopup.visible && state.aboutPopup.contentItem.width > 0,
                        "Shared About did not open on the upstream tab.")
                state.aboutWorked = true
                state.aboutPopup.close()
                next("foreign-actions")
            } else if (state.phase === "foreign-actions") {
                if (state.aboutPopup.visible) {
                    return
                }
                checkCursor(upstream)
                checkPaused()
                for (const action of ["new", "open", "save"]) {
                    require(rootWindow.routeFile(action, false) === false,
                            "File action was redirected from the upstream tab: " + action)
                }
                require(rootWindow.routeFile("save", true) === false,
                        "Save As was redirected from the upstream tab.")
                state.fileRejected = true
                require(!host.runAction("stop", downstream, state.downstreamIdentity)
                        && !tabs.requestRunAction("stop", downstream, state.downstreamIdentity),
                        "Hidden downstream action bypassed the selected-direction guard.")
                for (const action of ["stop", "pause", "start"]) {
                    require(!host.runAction(action, upstream, state.upstreamIdentity),
                            "Foreign upstream action controlled the downstream lease: " + action)
                    checkPaused()
                }
                state.foreignRejected = true
                state.pausedAcrossSwitch = true
                require(host.feedback(""), "Could not clear deliberately rejected guard feedback.")
                const notice = findItem(tabs, "otherDirectionExecutionStatus")
                require(notice && notice.visible && notice.text.length > 0,
                        "Upstream tab omitted the hidden direction's execution status.")
                capture(upstream, upstreamImagePath, "select-downstream")
            } else if (state.phase === "select-downstream") {
                checkPaused()
                switchDirection(downstream)
                next("stop")
            } else if (state.phase === "stop") {
                if (!controls.contextCurrent || controls.direction !== downstream) {
                    return
                }
                checkCursor(downstream)
                checkPaused()
                require(controls.busy && controls.paused,
                        "Returning downstream lost the original paused control state.")
                clickControl("directionStop")
                next("terminal")
            } else if (state.phase === "terminal") {
                const current = JSON.parse(workbench.runBackend.poll())
                const shell = JSON.parse(host.backend.state())
                if (current.busy || shell.applicationBusy) {
                    return
                }
                checkCursor(downstream)
                require(JSON.stringify(current.accepted_options) === state.acceptedOptions
                        && workbench.hasProject && !workbench.loadingProject,
                        "Stopping the actual worker discarded accepted workbench state.")
                state.terminalObserved = true
                require(state.configureWorked && state.settingsWorked && state.aboutWorked
                        && state.fileRejected && state.foreignRejected && state.pausedAcrossSwitch
                        && state.idleSwitchObserved && state.runningSwitchObserved
                        && state.downstreamSaved && state.upstreamSaved,
                        "Probe reached terminal without completing every required invariant.")
                complete(true, "")
            } else if (state.phase !== "capturing-downstream"
                    && state.phase !== "capturing-upstream") {
                throw new Error("Unknown native direction probe phase.")
            }
        } catch (error) {
            complete(false, String(error))
        }
    }

    Component.onCompleted: {
        state.startedAt = Date.now()
        phaseTimer.start()
    }
    Component.onDestruction: phaseTimer.stop()
}
