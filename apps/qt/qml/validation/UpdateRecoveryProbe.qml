/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed actual Main restart consumer. The runner supplies real project files and
// launches save/restore as separate processes; no installer or science is mocked.
QtObject {
    id: probe
    required property var rootWindow
    required property var workbench
    required property var tabs
    required property var host
    required property var controls
    required property var fileBackend
    required property var upstreamBackend
    required property string screenshotPath
    required property string mode
    signal finished(bool success, string message)

    // Independent GUI scheduling/traversal policies, unrelated to science limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseTimeoutMs: 45000
    readonly property int runTimeoutMs: 90000
    readonly property int overallTimeoutMs: 180000
    readonly property int captureGraceMs: 2000
    readonly property int itemTraversalLimit: 8192
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    readonly property string downstreamName: "recovery-downstream.merlin.json"
    readonly property string upstreamName: "recovery-upstream.merlin.json"
    // Explicit distinct valid input-edit anchors for the real fixed fixtures.
    readonly property int downstreamNoiseSeed: 2
    readonly property string upstreamSourceSeed: "2"

    property QtObject state: QtObject {
        property string phase: "loading"
        property double startedAt: 0
        property double phaseStartedAt: 0
        property bool done: false
        property bool pickerReturned: false
        property bool screenshotSaved: false
        property bool bundleSaved: false
        property bool runStarted: false
        property string activeRun: ""
        property string runBefore: ""
        property string downstreamPath: ""
        property string observedSourceSeed: ""
        property string downstreamOptions: ""
        property string upstreamGeneration: ""
        property var downstreamItem: null
        property var upstreamItem: null
        property var before: null
        property var checkpoints: []
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /** Fail on the first missing actual native or presentation invariant. */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message);
        }
    }

    /** Move to one finite compiled phase without blocking the GUI thread. */
    function next(phase) {
        state.phase = phase;
        state.phaseStartedAt = Date.now();
    }

    /** Search actual named QML items with independent finite traversal bounds. */
    function findItem(root, name) {
        const pending = [root];
        const visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0) {
                continue;
            }
            require(visited.length < itemTraversalLimit, "Recovery probe object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name) {
                return item;
            }
            for (const key of ["children", "contentData", "menus"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number") {
                    continue;
                }
                require(pending.length + children.length <= itemTraversalLimit, "Recovery probe pending traversal exceeded its bound.");
                for (const child of children) {
                    pending.push(child);
                }
            }
        }
        return null;
    }

    /** Read fresh native owners; all session/generation identifiers stay strings. */
    function nativeState() {
        return {
            shell: JSON.parse(host.backend.state()),
            ds: JSON.parse(workbench.runBackend.poll()),
            us: JSON.parse(upstreamBackend.state())
        };
    }

    /** Check a fixed isolated fixture basename without deriving any native path. */
    function namedPath(path, name) {
        return typeof path === "string" && (path.endsWith("/" + name) || path.endsWith("\\" + name));
    }

    /** Check actual selected controls and both persistent workbench instances. */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction) && tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem;
    }

    /** Select through the actual direction tab and its existing Main connection. */
    function select(direction) {
        const button = findItem(tabs, direction + "DirectionTab");
        require(button && button.enabled, "The actual recovery direction tab is unavailable.");
        button.clicked();
    }

    /** Resolve the existing persistent source editor without another data owner. */
    function sourcePopup() {
        const popup = state.upstreamItem.sectionDialogs.source;
        require(!!popup, "The actual upstream source editor is unavailable.");
        return popup;
    }

    /** Wait for native draft and actual content/text admission before interacting. */
    function draftReady(popup) {
        require(!popup.presentationRejected, "The actual source draft exceeded presentation admission.");
        return popup.opened && popup.valuesLoaded && popup.textAdmitted && popup.matchingState && !popup.pending;
    }

    /** Preserve the native semantic descriptor and opaque field key verbatim. */
    function sourceField(popup) {
        const field = popup.fields.find(value => value.field === "source_seed");
        require(field && field.kind === "integer" && field.editable, "The actual source_seed descriptor is unavailable.");
        return field;
    }

    /** Click an actual admitted footer action rather than calling its handler. */
    function clickFooter(popup, name) {
        const button = findItem(popup.footer, name);
        require(popup.opened && button && button.enabled, "The actual recovery editor footer is disabled: " + name);
        button.clicked();
    }

    /** Snapshot finite native configuration metadata, not source payload buffers. */
    function snapshot() {
        const actual = nativeState();
        return {
            downstreamIdentity: actual.shell.downstreamIdentity,
            upstreamIdentity: actual.shell.upstreamIdentity,
            downstreamDirty: actual.shell.downstreamDirty,
            upstreamDirty: actual.shell.upstreamDirty,
            selectedDirection: actual.shell.selectedDirection,
            downstreamPath: state.downstreamPath,
            upstreamPath: actual.us.project ? actual.us.project.path : null,
            downstreamNoiseSeed: actual.ds.accepted_options ? actual.ds.accepted_options.noise_seed : null,
            upstreamSourceSeed: state.observedSourceSeed,
            downstreamAcceptedOptions: JSON.stringify(actual.ds.accepted_options || null),
            downstreamAcceptedObservations: JSON.stringify(actual.ds.accepted_observations || null),
            upstreamSession: actual.us.session,
            upstreamGeneration: actual.us.generation,
            dark: workbench.theme.dark
        };
    }

    /** Publish one finite checkpoint for independent runner byte/path comparison. */
    function checkpoint(name) {
        const record = {
            name: name,
            state: snapshot()
        };
        state.checkpoints.push(record);
        console.log("MERLIN_RECOVERY_CHECKPOINT " + JSON.stringify(record));
    }

    /** Require the native dirty pair and independent actual normal destinations. */
    function configuredPair(actual) {
        require(actual.us.configured && actual.ds.accepted_options, "Both genuine direction projects must be configured.");
        require(actual.ds.accepted_options.noise_seed === downstreamNoiseSeed && state.observedSourceSeed === upstreamSourceSeed, "The accepted real scalar edits were not preserved.");
        require(actual.shell.downstreamDirty && actual.shell.upstreamDirty && actual.us.project.dirty, "Recovery acknowledged a normal Save or lost independent dirty flags.");
        require(namedPath(state.downstreamPath, downstreamName) && namedPath(actual.us.project.path, upstreamName) && state.downstreamPath !== actual.us.project.path, "Recovery lost the independent real normal Save destinations.");
        require(cursorCurrent(upstream) && workbench.theme.dark === true, "Recovery did not preserve the actual upstream tab and dark theme.");
    }

    /** Capture the real visible QML-owned upstream canvas; runner captures Main. */
    function capture() {
        const target = state.upstreamItem;
        require(target && target.visible && target.width > 0 && target.height > 0 && screenshotPath.length > 0 && workbench.theme.dark === true, "A visible dark actual workbench and fresh image path are required.");
        console.log("MERLIN_RECOVERY_CAPTURE " + mode);
        next("capturing");
        require(target.grabToImage(function (image) {
            if (state.done) {
                return;
            }
            try {
                require(image.saveToFile(screenshotPath), "Could not save the actual recovery workbench image.");
                state.screenshotSaved = true;
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        }), "The actual upstream canvas rejected image capture.");
    }

    /** Emit exactly one receipt and stop only an observed worker this probe began. */
    function complete(success, message) {
        if (state.done) {
            return;
        }
        state.done = true;
        phaseTimer.stop();
        if (!success && state.runStarted && state.activeRun.length && cursorCurrent(upstream)) {
            const current = JSON.parse(upstreamBackend.state());
            if (current.busy && current.progress && current.progress.run === state.activeRun) {
                const stop = findItem(controls, "directionStop");
                if (stop && stop.enabled) {
                    stop.clicked();
                }
            }
        }
        const json = JSON.stringify({
            probe: "update-recovery",
            mode: mode,
            success: success,
            phase: state.phase,
            error: message,
            bundleSaved: state.bundleSaved,
            checkpoints: state.checkpoints,
            restoredExecutionRun: state.activeRun || null,
            screenshot: state.screenshotSaved ? screenshotPath : null,
            captureTarget: "upstream-workbench",
            fullWindowCaptureMarker: state.screenshotSaved,
            installation: "Not requested or executed.",
            elapsedMs: Date.now() - state.startedAt
        });
        console.log("MERLIN_RECOVERY_PROBE " + json);
        finished(success, json);
    }

    /** Drive only the fixed native recovery export or actual startup restoration. */
    function advance() {
        if (state.done) {
            return;
        }
        try {
            const timeout = state.phase === "run-terminal" ? runTimeoutMs : phaseTimeoutMs;
            require(mode === "save" || mode === "restore", "Recovery probe mode must be save or restore.");
            require(Date.now() - state.startedAt <= overallTimeoutMs && Date.now() - state.phaseStartedAt <= timeout, "Recovery probe exceeded its finite watchdog in " + state.phase + ".");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem) {
                return;
            }
            if (workbench.fileFeedbackPopup.visible) {
                workbench.fileFeedbackPopup.close();
                return;
            }
            workbench.pollRun();
            host.poll();
            const actual = nativeState();
            require(rootWindow.visible && !actual.ds.busy, "Main closed or recovery redirected downstream execution.");
            require(state.runStarted || ((!actual.us.busy || actual.us.preparing) && !actual.shell.applicationBusy), "Recovery unexpectedly started scientific execution.");
            if (state.phase === "loading") {
                if (rootWindow.restoringUpdate || workbench.loadingProject || !workbench.hasProject || !actual.ds.accepted_options || !controls.contextCurrent) {
                    return;
                }
                state.downstreamItem = tabs.downstreamItem;
                state.upstreamItem = tabs.upstreamItem;
                // Normal startup status is a transient dirty-decision tag. In
                // save mode record the explicit CLI destination expectation;
                // the runner independently checks the native bundle's actual
                // destination. Restore metadata carries its actual adopted path.
                const args = Qt.application.arguments;
                const workbenchArgument = args.indexOf("--workbench");
                const path = mode === "restore" ? workbench.loadState.path : workbenchArgument >= 0 && workbenchArgument + 1 < args.length ? args[workbenchArgument + 1] : null;
                require(namedPath(path, downstreamName), "Actual restored or explicit startup DS destination is unavailable.");
                state.downstreamPath = path;
                if (mode === "restore") {
                    if (!actual.us.configured || !cursorCurrent(upstream)) {
                        return;
                    }
                    state.downstreamOptions = JSON.stringify(actual.ds.accepted_options);
                    require(state.upstreamItem.openSection("source"), "The actual restored source editor could not open.");
                    next("source-inspect-ready");
                } else {
                    require(cursorCurrent(downstream) && !actual.us.configured && !actual.shell.downstreamDirty, "Save mode requires the real clean DS fixture and blank US owner.");
                    state.before = snapshot();
                    state.pickerReturned = false;
                    console.log("MERLIN_RECOVERY_PICKER upstream-project");
                    next("upstream-opened");
                    Qt.callLater(function () {
                        if (state.done) {
                            return;
                        }
                        try {
                            require(rootWindow.routeFile("open", false), "Main rejected the actual Open route for the upstream project.");
                            state.pickerReturned = true;
                        } catch (error) {
                            complete(false, String(error));
                        }
                    });
                }
            } else if (state.phase === "upstream-opened") {
                require(!state.pickerReturned || workbench.loadingProject || !workbench.loadState.error, "Actual upstream project Open failed: " + (workbench.loadState.error || ""));
                if (!state.pickerReturned || workbench.loadingProject || actual.us.preparing || !actual.us.configured || !cursorCurrent(upstream)) {
                    return;
                }
                require(namedPath(actual.us.project.path, upstreamName) && !actual.shell.upstreamDirty, "Actual native Open did not adopt the isolated upstream saved project cleanly.");
                require(actual.shell.downstreamIdentity === state.before.downstreamIdentity && JSON.stringify(actual.ds.accepted_options) === state.before.downstreamAcceptedOptions && !actual.shell.downstreamDirty, "Upstream project Open changed the independent accepted downstream owner.");
                select(downstream);
                next("downstream-edit-open");
            } else if (state.phase === "downstream-edit-open") {
                if (!cursorCurrent(downstream)) {
                    return;
                }
                workbench.configureProject();
                next("downstream-edit-ready");
            } else if (state.phase === "downstream-edit-ready") {
                const popup = workbench.configurationPopup;
                require(!popup.presentationCoverage.rejected, "The actual DS configuration exceeded presentation admission.");
                if (!popup.opened || !popup.contextCurrent || !popup.presentationCoverage.textAdmitted) {
                    return;
                }
                const editor = findItem(popup.contentItem, "wbNoiseSeed");
                const apply = findItem(popup.footer, "wbConfigurationApply");
                require(editor && editor.enabled && typeof editor.valueModified === "function" && apply && apply.enabled && editor.value !== downstreamNoiseSeed, "Actual DS scalar editor/Apply is unavailable or its fixed anchor is unchanged.");
                editor.value = downstreamNoiseSeed;
                editor.valueModified();

                // Direct probe assignment removes the existing persistent binding.
                editor.value = Qt.binding(function () {
                    return popup.draft.noise_seed;
                });
                require(popup.draft.noise_seed === downstreamNoiseSeed, "The actual DS editor did not retain its changed scalar.");
                apply.clicked();
                next("downstream-edited");
            } else if (state.phase === "downstream-edited") {
                if (workbench.configurationPopup.visible) {
                    return;
                }
                require(actual.ds.accepted_options.noise_seed === downstreamNoiseSeed && actual.shell.downstreamDirty, "Real DS Apply did not preserve the edit and native dirty state.");
                state.downstreamOptions = JSON.stringify(actual.ds.accepted_options);
                select(upstream);
                next("upstream-edit-open");
            } else if (state.phase === "upstream-edit-open") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                state.upstreamGeneration = actual.us.generation;
                require(state.upstreamItem.openSection("source"), "The actual upstream source editor could not open.");
                next("upstream-edit-ready");
            } else if (state.phase === "upstream-edit-ready") {
                const popup = sourcePopup();
                if (!draftReady(popup)) {
                    return;
                }
                const field = sourceField(popup);
                const editor = findItem(popup.contentItem, field.editorObjectName);
                require(popup.editable && popup.fieldAvailable(field) && editor && editor.enabled && typeof editor.textEdited === "function" && upstreamSourceSeed.length <= editor.maximumLength && popup.valueFor(field.key) !== upstreamSourceSeed, "Actual US source text editor is unavailable or its fixed anchor is unchanged.");
                editor.text = upstreamSourceSeed;
                editor.textEdited();
                require(popup.valueFor(field.key) === upstreamSourceSeed && popup.canApply, "The actual source draft did not admit its exact changed text.");
                clickFooter(popup, "upstreamDraftApply");
                next("upstream-edited");
            } else if (state.phase === "upstream-edited") {
                if (sourcePopup().visible || actual.us.preparing) {
                    return;
                }
                require(actual.us.generation !== state.upstreamGeneration && actual.shell.upstreamDirty && JSON.stringify(actual.ds.accepted_options) === state.downstreamOptions, "Real US Apply failed or changed the independent accepted DS options.");
                require(state.upstreamItem.openSection("source"), "The actual accepted source editor could not reopen.");
                next("source-inspect-ready");
            } else if (state.phase === "source-inspect-ready") {
                const popup = sourcePopup();
                if (!draftReady(popup)) {
                    return;
                }
                state.observedSourceSeed = popup.valueFor(sourceField(popup).key);
                configuredPair(actual);
                clickFooter(popup, "upstreamDraftCancel");
                next("pair-ready");
            } else if (state.phase === "pair-ready") {
                if (sourcePopup().visible || actual.us.preparing) {
                    return;
                }
                configuredPair(actual);
                state.before = snapshot();
                checkpoint(mode === "save" ? "before-export" : "restored-pair");
                if (mode === "save") {
                    const result = JSON.parse(fileBackend.save_update_recovery(upstreamBackend, workbench.saveOptionsPacket()));
                    require(!result.error && result.preparing === true, "Actual recovery export was not queued: " + (result.error || ""));
                    next("export-poll");
                } else {
                    next("run-start");
                }
            } else if (state.phase === "export-poll") {
                const result = JSON.parse(fileBackend.poll_update_recovery(upstreamBackend));
                require(!result.error, "Actual recovery export failed: " + (result.error || ""));
                if (result.preparing === true) {
                    return;
                }
                require(result.saved === true, "Actual recovery export completed without its native saved acknowledgement.");
                require(JSON.stringify(snapshot()) === JSON.stringify(state.before), "Recovery export changed accepted identities, dirty flags, destinations, selection or settings.");
                state.bundleSaved = true;
                checkpoint("after-export-preserved");
                capture();
            } else if (state.phase === "run-start") {
                if (!cursorCurrent(upstream) || !controls.canStart) {
                    return;
                }
                const start = findItem(controls, "directionStart");
                require(start && start.enabled, "The actual restored US Start control is disabled.");
                state.runBefore = actual.us.result ? actual.us.result.run : "";
                state.runStarted = true;
                start.clicked();
                next("run-terminal");
            } else if (state.phase === "run-terminal") {
                const run = actual.us.progress ? actual.us.progress.run : actual.us.result ? actual.us.result.run : "";
                if (typeof run === "string" && run !== state.runBefore) {
                    state.activeRun = run;
                }
                require(!actual.us.error, "Actual restored execution failed: " + (actual.us.error || ""));
                configuredPair(actual);
                require(JSON.stringify(snapshot()) === JSON.stringify(state.before), "Actual restored execution changed accepted configuration metadata or the independent DS owner.");
                if (actual.us.busy || actual.shell.applicationBusy || !state.activeRun.length || !actual.us.has_result || !actual.us.result || actual.us.result.run !== state.activeRun) {
                    return;
                }
                require(actual.us.result.status === "Completed", "Actual restored execution did not retain its Completed report.");
                state.runStarted = false;
                checkpoint("actual-restored-run-completed");
                capture();
            } else if (state.phase === "capture-grace") {
                if (Date.now() - state.phaseStartedAt >= captureGraceMs) {
                    configuredPair(actual);
                    complete(true, "");
                }
            } else {
                require(state.phase === "capturing", "Unknown compiled recovery probe phase: " + state.phase);
            }
        } catch (error) {
            complete(false, String(error));
        }
    }

    Component.onCompleted: {
        state.startedAt = Date.now();
        state.phaseStartedAt = state.startedAt;
        phaseTimer.start();
    }
    Component.onDestruction: phaseTimer.stop()
}
