/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed, opt-in actual Main consumer. The external runner answers genuine OS
// pickers with its isolated files; this component supplies no scientific state.
QtObject {
    id: probe
    required property var rootWindow
    required property var workbench
    required property var tabs
    required property var host
    required property var controls
    required property var dirtyDialog
    required property var captureItem
    required property string screenshotPath
    signal finished(bool success, string message)

    // GUI scheduling and object traversal policies, unrelated to science limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseTimeoutMs: 45000
    readonly property int runTimeoutMs: 90000
    readonly property int overallTimeoutMs: 300000
    readonly property int captureGraceMs: 2000
    readonly property int itemTraversalLimit: 8192
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    readonly property string downstreamSavedName: "downstream-a.merlin.json"
    readonly property string downstreamOpenedName: "downstream-open.merlin.json"
    readonly property string upstreamSavedName: "upstream-a.merlin.json"
    // Distinct valid scalar edit anchors for the existing real input fixtures.
    readonly property int firstDownstreamSeed: 2
    readonly property int secondDownstreamSeed: 3
    readonly property string firstUpstreamSeed: "2"
    readonly property string secondUpstreamSeed: "3"

    property QtObject state: QtObject {
        property string phase: "loading"
        property double startedAt: 0
        property double phaseStartedAt: 0
        property bool done: false
        property bool pickerReturned: false
        property bool screenshotSaved: false
        property var downstreamItem: null
        property var upstreamItem: null
        property var before: null
        property var records: []
        property string downstreamPath: ""
        property string upstreamPath: ""
        property string originalUpstreamSeed: ""
        property string beforeMessage: ""
        property string editDirection: ""
        property string afterEdit: ""
        property string upstreamEditValue: ""
        property int downstreamEditValue: 0
        property string runBefore: ""
        property string activeRun: ""
        property bool runStarted: false
        property bool closeCancelled: false
        property bool secondCloseCancelled: false
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /** Fail on a missing actual native or presentation invariant. */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message);
        }
    }

    /** Move to the next compiled finite phase without blocking GUI polling. */
    function next(phase) {
        state.phase = phase;
        state.phaseStartedAt = Date.now();
    }

    /** Find a real named item through bounded existing QML child collections. */
    function findItem(root, name) {
        const pending = [root];
        const visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0) {
                continue;
            }
            require(visited.length < itemTraversalLimit, "Project probe object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name) {
                return item;
            }
            for (const key of ["children", "contentData", "menus"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number") {
                    continue;
                }
                require(pending.length + children.length <= itemTraversalLimit, "Project probe pending traversal exceeded its bound.");
                for (const child of children) {
                    pending.push(child);
                }
            }
        }
        return null;
    }

    /** Read fresh native metadata; identities remain opaque strings throughout. */
    function nativeState() {
        return {
            shell: JSON.parse(host.backend.state()),
            ds: JSON.parse(workbench.runBackend.poll()),
            us: JSON.parse(host.upstreamBackend.state())
        };
    }

    /**
     * Capture the fixed fixture's accepted state, without copying source payloads.
     * DS exposes paths only on actual Open/Save completion, recorded separately.
     */
    function snapshot() {
        const value = nativeState();
        return {
            downstream: {
                identity: value.shell.downstreamIdentity,
                dirty: value.shell.downstreamDirty,
                options: JSON.stringify(value.ds.accepted_options || null),
                observations: JSON.stringify(value.ds.accepted_observations || null),
                result: JSON.stringify(value.ds.result || null)
            },
            upstream: {
                identity: value.shell.upstreamIdentity,
                dirty: value.shell.upstreamDirty,
                session: value.us.session,
                generation: value.us.generation,
                configured: value.us.configured,
                path: value.us.project ? value.us.project.path : null,
                result: JSON.stringify(value.us.result || null)
            }
        };
    }

    /** Compare actual accepted owners, including their native dirty/path state. */
    function unchanged(before, direction) {
        const current = snapshot();
        for (const name of direction ? [direction] : [downstream, upstream]) {
            require(JSON.stringify(current[name]) === JSON.stringify(before[name]), "File action changed the preserved " + name + " owner.");
        }
    }

    /** Record one distinct completed file requirement and fresh native evidence. */
    function record(name, details) {
        const entry = {
            name: name,
            state: snapshot(),
            details: details || {}
        };
        state.records.push(entry);
        console.log("MERLIN_PROJECT_FILE_CHECKPOINT " + JSON.stringify(entry));
    }

    /** Require persistent tabs and the actual selected native control cursor. */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction) && tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem;
    }

    /** Click the actual direction tab, exercising its normal Main connection. */
    function select(direction) {
        const button = findItem(tabs, direction + "DirectionTab");
        require(button && button.enabled, "Actual direction tab is unavailable: " + direction);
        button.clicked();
    }

    /** Route through the same actual Main handler used by File menu actions. */
    function route(action, saveAs) {
        require(controls.contextCurrent && !dirtyDialog.visible, "The actual file action has a stale or modal context.");
        require(rootWindow.routeFile(action, saveAs), "Main rejected the actual File " + action + " route.");
    }

    /** Synchronize the runner before a genuine blocking native picker opens. */
    function picker(tag, action, saveAs, following) {
        state.pickerReturned = false;
        next(following);
        console.log("MERLIN_PROJECT_FILE_PICKER " + tag);
        Qt.callLater(function () {
            if (state.done) {
                return;
            }
            try {
                route(action, saveAs);
                state.pickerReturned = true;
            } catch (error) {
                complete(false, String(error));
            }
        });
    }

    /** Wait only for actual native file workers and normal popup settlement. */
    function fileIdle() {
        const current = nativeState();
        return !workbench.loadingProject && !workbench.savingProject && !current.us.preparing && rootWindow.pendingFileDirection === rootWindow.noFileDirection && !dirtyDialog.visible;
    }

    /** Check native target tags before using the real dirty-dialog controls. */
    function promptReady(direction, action) {
        require(!workbench.theme.presentationAdmissionRejected, "The actual project prompt was rejected by presentation admission.");
        if (!dirtyDialog.opened || !dirtyDialog.decisionPending) {
            return false;
        }
        require(dirtyDialog.direction === direction && dirtyDialog.action === action && dirtyDialog.contextCurrent && dirtyDialog.decisionPending, "The actual dirty prompt does not match its native target/action.");
        return true;
    }

    /** Activate exactly one actual admitted dirty-dialog button. */
    function decide(name) {
        const button = findItem(dirtyDialog.footer, name);
        require(dirtyDialog.opened && button && button.enabled, "Actual dirty-dialog action is disabled: " + name);
        button.clicked();
    }

    /** Resolve an existing persistent upstream configuration popup. */
    function popupFor(section) {
        const popup = state.upstreamItem.sectionDialogs[section];
        require(!!popup, "Actual upstream section is unavailable: " + section);
        return popup;
    }

    /** Require actual native draft, retained content and editable-text admission. */
    function draftReady(popup) {
        if (popup.presentationCoverage && popup.presentationCoverage.rejected) {
            throw new Error("The actual upstream draft was rejected by presentation admission.");
        }
        return popup.opened && popup.valuesLoaded && popup.textAdmitted && popup.matchingState && !popup.pending;
    }

    /** Click an actual admitted upstream footer, never an alternate signal path. */
    function draftButton(popup, name) {
        const button = findItem(popup.footer, name);
        require(popup.opened && button && button.enabled, "Actual upstream footer is disabled: " + name);
        button.clicked();
    }

    /** Begin a fixed scalar edit through the existing scientific configuration UI. */
    function beginEdit(direction, value, following) {
        state.before = snapshot();
        state.editDirection = direction;
        state.afterEdit = following;
        if (direction === downstream) {
            state.downstreamEditValue = value;
        } else {
            state.upstreamEditValue = value;
        }
        select(direction);
        next("edit-open");
    }

    /** Read the actual source descriptor; do not infer its opaque transport key. */
    function sourceField(popup) {
        const field = popup.fields.find(value => value.field === "source_seed");
        require(field && field.editable && field.kind === "integer", "The actual native source_seed editor is unavailable.");
        return field;
    }

    /** Exercise real bounded TextField textEdited and actual Apply controls. */
    function editUpstream(popup) {
        const field = sourceField(popup);
        const editor = findItem(popup.contentItem, field.editorObjectName);
        require(popup.editable && popup.fieldAvailable(field) && editor && editor.enabled && typeof editor.textEdited === "function", "The actual source TextField is not admitted/editable.");
        require(state.upstreamEditValue.length <= editor.maximumLength && popup.valueFor(field.key) !== state.upstreamEditValue, "The fixed source edit anchor is not distinct or exceeds its actual text capacity.");
        editor.text = state.upstreamEditValue;
        editor.textEdited();
        require(popup.valueFor(field.key) === state.upstreamEditValue && popup.canApply, "The real source draft did not preserve/permit its changed string.");
        draftButton(popup, "upstreamDraftApply");
    }

    /** Capture QML-owned prompt content; the runner also captures the full window. */
    function captureDirtyPrompt() {
        require(workbench.theme.dark === true && screenshotPath.length > 0, "A dark actual Main and fresh screenshot path are required.");
        const target = dirtyDialog.contentItem.parent;
        require(target && target.width > 0 && target.height > 0, "The actual dirty prompt has no capturable QML content.");
        console.log("MERLIN_PROJECT_FILE_CAPTURE dirty");
        next("new-ds-capture");
        require(target.grabToImage(function (image) {
            if (state.done) {
                return;
            }
            try {
                require(dirtyDialog.opened && image.saveToFile(screenshotPath), "Could not save the actual dirty-dialog content.");
                state.screenshotSaved = true;
                next("new-ds-capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        }), "The actual dirty-dialog content rejected capture.");
    }

    /** Require an actual successful destination result, never a remembered path. */
    function downstreamSaved(path) {
        if (!fileIdle()) {
            return false;
        }
        const result = workbench.loadState;
        require(!result.error, "Actual downstream Save failed: " + (result.error || ""));
        return result.saved === true && result.path === path;
    }

    /** Capture only observations from a genuine native run started by this probe. */
    function observedRun(current) {
        const run = current.progress ? current.progress.run : current.result ? current.result.run : "";
        if (state.runStarted && typeof run === "string" && run !== state.runBefore) {
            state.activeRun = run;
        }
    }

    /** Emit one terminal receipt; on failure stop only this probe's actual worker. */
    function complete(success, message) {
        if (state.done) {
            return;
        }
        state.done = true;
        phaseTimer.stop();
        if (!success && state.runStarted && state.activeRun.length) {
            const current = JSON.parse(host.upstreamBackend.state());
            const run = current.progress ? current.progress.run : "";
            if (current.busy && run === state.activeRun && cursorCurrent(upstream)) {
                const stop = findItem(controls, "directionStop");
                if (stop && stop.enabled) {
                    stop.clicked();
                }
            }
        }
        const result = {
            probe: "project-file",
            success: success,
            phase: state.phase,
            error: message,
            cases: state.records,
            downstreamObservedSavedPath: state.downstreamPath,
            upstreamObservedSavedPath: state.upstreamPath,
            closeCancel: state.closeCancelled,
            firstDiscardThenSecondCancel: state.secondCloseCancelled,
            postCloseRun: state.activeRun || null,
            finalClose: "Runner-owned; not executed by this probe.",
            screenshot: state.screenshotSaved ? screenshotPath : null,
            captureTarget: "project-dirty-popup",
            fullWindowCaptureMarker: state.screenshotSaved,
            elapsedMs: Date.now() - state.startedAt
        };
        const json = JSON.stringify(result);
        console.log("MERLIN_PROJECT_FILE_PROBE " + json);
        finished(success, json);
    }

    /** Drive the compiled FILE01 cases using actual Main and native lifecycle. */
    function advance() {
        if (state.done) {
            return;
        }
        try {
            const timeout = state.phase === "close-run-terminal" ? runTimeoutMs : phaseTimeoutMs;
            require(Date.now() - state.startedAt <= overallTimeoutMs, "Project file probe exceeded its overall watchdog.");
            require(Date.now() - state.phaseStartedAt <= timeout, "Project file probe timed out in " + state.phase + ".");
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
            require(rootWindow.visible, "Main closed before its actual project decisions were resolved.");
            require(!actual.ds.busy, "File lifecycle unexpectedly started downstream execution.");
            require(state.runStarted || ((!actual.us.busy || actual.us.preparing) && !actual.shell.applicationBusy), "File lifecycle unexpectedly started scientific execution.");
            if (state.phase === "loading") {
                if (workbench.loadingProject || !workbench.hasProject || !actual.ds.accepted_options || !controls.contextCurrent) {
                    return;
                }
                require(workbench.theme.dark === true && !actual.us.configured, "Start with the real dark DS example and an unconfigured US owner.");
                state.downstreamItem = tabs.downstreamItem;
                state.upstreamItem = tabs.upstreamItem;
                require(cursorCurrent(downstream), "The initial actual Main direction is not downstream.");
                state.before = snapshot();
                picker("save-ds-a", "save", true, "initial-ds-save");
            } else if (state.phase === "initial-ds-save") {
                require(!state.pickerReturned || workbench.loadingProject || !workbench.loadState.error, "Actual initial DS Save failed: " + (workbench.loadState.error || ""));
                if (!state.pickerReturned || !fileIdle() || !workbench.loadState.saved) {
                    return;
                }
                require(typeof workbench.loadState.path === "string" && workbench.loadState.path.endsWith("/" + downstreamSavedName), "The runner did not supply the fixed isolated DS destination.");
                state.downstreamPath = workbench.loadState.path;
                unchanged(state.before, upstream);
                require(!actual.shell.downstreamDirty, "Successful DS Save did not acknowledge its exact native revision.");
                record("downstream-save-as", {
                    path: state.downstreamPath
                });
                select(upstream);
                next("configuration-open");
            } else if (state.phase === "configuration-open") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                state.before = snapshot();
                require(state.upstreamItem.openSection("ucd"), "Actual unconfigured UCD open was rejected.");
                next("configuration-ready");
            } else if (state.phase === "configuration-ready") {
                const popup = popupFor("ucd");
                if (!draftReady(popup)) {
                    return;
                }
                require(popup.fields.length === 0 && !popup.canApply, "The unconfigured native draft invented editable configuration.");
                state.pickerReturned = false;
                next("configuration-loaded");
                console.log("MERLIN_PROJECT_FILE_PICKER configuration");
                Qt.callLater(function () {
                    try {
                        draftButton(popup, "upstreamConfigurationLoad");
                        state.pickerReturned = true;
                    } catch (error) {
                        complete(false, String(error));
                    }
                });
            } else if (state.phase === "configuration-loaded") {
                require(!state.pickerReturned || actual.us.preparing || !actual.us.error, "Actual native configuration Load failed: " + (actual.us.error || ""));
                if (!state.pickerReturned || actual.us.preparing || !draftReady(popupFor("ucd"))) {
                    return;
                }
                require(!actual.us.error && actual.us.configured && actual.shell.upstreamDirty, "The actual native configuration picker failed to adopt/dirty its US owner.");
                unchanged(state.before, downstream);
                draftButton(popupFor("ucd"), "upstreamDraftCancel");
                next("initial-us-save-request");
            } else if (state.phase === "initial-us-save-request") {
                if (popupFor("ucd").visible || !cursorCurrent(upstream)) {
                    return;
                }
                state.before = snapshot();
                picker("save-us-a", "save", true, "initial-us-save");
            } else if (state.phase === "initial-us-save") {
                if (!state.pickerReturned || !fileIdle() || actual.us.project.status === "saving") {
                    return;
                }
                require(actual.us.project.status === "saved" && !actual.shell.upstreamDirty && typeof actual.us.project.path === "string" && actual.us.project.path.endsWith("/" + upstreamSavedName), "The actual US Save As failed or used the wrong isolated destination.");
                state.upstreamPath = actual.us.project.path;
                unchanged(state.before, downstream);
                record("upstream-save-as", {
                    path: state.upstreamPath
                });
                state.before = snapshot();
                state.beforeMessage = host.presentation.feedbackMessage || actual.us.error || "";
                picker("save-us-protected", "save", true, "save-us-protected");
            } else if (state.phase === "tabs-downstream") {
                if (!cursorCurrent(downstream)) {
                    return;
                }
                unchanged(state.before);
                select(upstream);
                next("tabs-upstream");
            } else if (state.phase === "tabs-upstream") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                unchanged(state.before);
                record("tab-switch-independent-clean-owners");
                beginEdit(downstream, firstDownstreamSeed, "first-us-edit");
            } else if (state.phase === "edit-open") {
                if (!cursorCurrent(state.editDirection)) {
                    return;
                }
                if (state.editDirection === downstream) {
                    workbench.configureProject();
                } else {
                    require(state.upstreamItem.openSection("source"), "The actual source editor could not open.");
                }
                next("edit-ready");
            } else if (state.phase === "edit-ready") {
                if (state.editDirection === downstream) {
                    const popup = workbench.configurationPopup;
                    if (!popup.opened || !popup.contextCurrent || !popup.presentationCoverage.textAdmitted) {
                        require(!popup.presentationCoverage.rejected, "Actual DS configuration admission failed.");
                        return;
                    }
                    const editor = findItem(popup.contentItem, "wbNoiseSeed");
                    const apply = findItem(popup.footer, "wbConfigurationApply");
                    require(editor && editor.enabled && apply && apply.enabled && editor.value !== state.downstreamEditValue && typeof editor.valueModified === "function", "The actual DS seed editor/Apply is unavailable or its anchor is unchanged.");
                    editor.value = state.downstreamEditValue;
                    editor.valueModified();
                    // Direct test assignment removes a QML binding. Restore the
                    // existing draft binding before reusing this persistent editor.
                    editor.value = Qt.binding(function () {
                        return popup.draft.noise_seed;
                    });
                    require(popup.draft.noise_seed === state.downstreamEditValue, "The real DS editor did not update its existing draft slot.");
                    apply.clicked();
                } else {
                    const popup = popupFor("source");
                    if (!draftReady(popup)) {
                        return;
                    }
                    if (!state.originalUpstreamSeed.length) {
                        state.originalUpstreamSeed = popup.valueFor(sourceField(popup).key);
                    }
                    editUpstream(popup);
                }
                next("edit-applied");
            } else if (state.phase === "edit-applied") {
                const popup = state.editDirection === downstream ? workbench.configurationPopup : popupFor("source");
                if (popup.visible || actual.us.preparing) {
                    return;
                }
                const other = state.editDirection === downstream ? upstream : downstream;
                unchanged(state.before, other);
                if (state.editDirection === downstream) {
                    require(actual.ds.accepted_options.noise_seed === state.downstreamEditValue && actual.shell.downstreamDirty, "Actual DS Apply did not preserve its edit and dirty revision.");
                } else {
                    require(actual.us.generation !== state.before.upstream.generation && actual.shell.upstreamDirty, "Actual US Apply did not advance its native generation/dirty revision.");
                }
                record("edit-" + state.afterEdit, {
                    direction: state.editDirection
                });
                next(state.afterEdit);
            } else if (state.phase === "first-us-edit") {
                beginEdit(upstream, firstUpstreamSeed, "new-ds-request");
            } else if (state.phase === "new-ds-request") {
                select(downstream);
                next("new-ds-route");
            } else if (state.phase === "new-ds-route") {
                if (!cursorCurrent(downstream)) {
                    return;
                }
                state.before = snapshot();
                require(state.before.downstream.dirty && state.before.upstream.dirty, "Both real edited projects must be dirty before New/Close coverage.");
                route("new", false);
                next("new-ds-prompt");
            } else if (state.phase === "new-ds-prompt") {
                if (promptReady(downstream, "new")) {
                    captureDirtyPrompt();
                }
            } else if (state.phase === "new-ds-capture-grace") {
                if (Date.now() - state.phaseStartedAt < captureGraceMs) {
                    return;
                }
                require(promptReady(downstream, "new"), "The actual New prompt disappeared before Cancel.");
                decide("projectDirtyCancel");
                next("new-ds-cancelled");
            } else if (state.phase === "new-ds-cancelled") {
                if (!fileIdle()) {
                    return;
                }
                unchanged(state.before);
                record("new-downstream-cancel-preserves-both-dirty");
                route("new", false);
                next("new-ds-save-prompt");
            } else if (state.phase === "new-ds-save-prompt") {
                if (!promptReady(downstream, "new")) {
                    return;
                }
                require(dirtyDialog.saveAllowed, "Actual dirty DS New cannot Save its existing project.");
                decide("projectDirtySave");
                next("new-ds-created");
            } else if (state.phase === "new-ds-created") {
                if (!fileIdle() || actual.shell.downstreamIdentity === state.before.downstream.identity) {
                    return;
                }
                require(!actual.ds.can_start, "New DS did not clear the prior runnable accepted project.");
                unchanged(state.before, upstream);
                record("new-downstream-save-before-replace");
                select(upstream);
                next("reopen-ds-request");
            } else if (state.phase === "reopen-ds-request") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                state.before = snapshot();
                picker("reopen-ds", "open", false, "reopen-ds");
            } else if (state.phase === "reopen-ds") {
                if (!state.pickerReturned) {
                    return;
                }
                if (dirtyDialog.opened && dirtyDialog.decisionPending) {
                    require(promptReady(downstream, "open"), "The blank DS replacement prompt targeted the wrong owner.");
                    decide("projectDirtyDiscard");
                    return;
                }
                if (!fileIdle() || actual.shell.downstreamIdentity === state.before.downstream.identity) {
                    return;
                }
                require(actual.ds.accepted_options.noise_seed === firstDownstreamSeed, "Save-before-New did not persist the actual DS accepted edit.");
                unchanged(state.before, upstream);
                require(cursorCurrent(downstream), "Generic Open did not select its actual DS target.");
                record("reopen-saved-downstream-edit");
                beginEdit(downstream, secondDownstreamSeed, "target-us-cancel-request");
            } else if (state.phase === "target-us-cancel-request") {
                state.before = snapshot();
                require(cursorCurrent(downstream) && state.before.upstream.dirty, "Target-aware US Open requires the other tab selected and a dirty US target.");
                picker("open-us-cancel", "open", false, "target-us-cancel-prompt");
            } else if (state.phase === "target-us-cancel-prompt") {
                if (!state.pickerReturned || !promptReady(upstream, "open")) {
                    return;
                }
                require(cursorCurrent(downstream), "Generic Open switched tabs before resolving its target dirty decision.");
                decide("projectDirtyCancel");
                next("target-us-cancelled");
            } else if (state.phase === "target-us-cancelled") {
                if (!fileIdle()) {
                    return;
                }
                unchanged(state.before);
                record("target-aware-upstream-open-cancel");
                picker("open-us-discard", "open", false, "target-us-discard-prompt");
            } else if (state.phase === "target-us-discard-prompt") {
                if (state.pickerReturned && promptReady(upstream, "open")) {
                    decide("projectDirtyDiscard");
                    next("target-us-opened");
                }
            } else if (state.phase === "target-us-opened") {
                if (!fileIdle() || actual.shell.upstreamIdentity === state.before.upstream.identity || !cursorCurrent(upstream)) {
                    return;
                }
                require(actual.us.configured && !actual.shell.upstreamDirty && actual.us.project.path === state.upstreamPath, "Actual US Discard/Open did not restore its selected saved project.");
                unchanged(state.before, downstream);
                record("target-aware-upstream-open-discard");
                state.before = snapshot();
                require(state.upstreamItem.openSection("source"), "The reopened actual source editor could not open.");
                next("restored-source-ready");
            } else if (state.phase === "restored-source-ready") {
                const popup = popupFor("source");
                if (!draftReady(popup)) {
                    return;
                }
                require(popup.valueFor(sourceField(popup).key) === state.originalUpstreamSeed, "Open/Discard did not restore the actual original saved US source value.");
                draftButton(popup, "upstreamDraftCancel");
                next("target-ds-save-request");
            } else if (state.phase === "target-ds-save-request") {
                if (popupFor("source").visible) {
                    return;
                }
                unchanged(state.before);
                require(state.before.downstream.dirty && cursorCurrent(upstream), "Target-aware DS Open needs a dirty DS target while US is selected.");
                picker("open-ds-save", "open", false, "target-ds-save-prompt");
            } else if (state.phase === "target-ds-save-prompt") {
                if (!state.pickerReturned || !promptReady(downstream, "open")) {
                    return;
                }
                require(cursorCurrent(upstream) && dirtyDialog.saveAllowed, "The target DS dirty Save was redirected or disabled.");
                decide("projectDirtySave");
                next("target-ds-opened");
            } else if (state.phase === "target-ds-opened") {
                if (!fileIdle() || actual.shell.downstreamIdentity === state.before.downstream.identity || !cursorCurrent(downstream)) {
                    return;
                }
                require(!actual.shell.downstreamDirty && actual.ds.accepted_options.noise_seed !== secondDownstreamSeed, "Actual DS Save/Open did not adopt the distinct original project.");
                unchanged(state.before, upstream);
                record("target-aware-downstream-open-save", {
                    previousSavedPath: state.downstreamPath
                });
                route("save", false);
                next("target-ds-destination-saved");
            } else if (state.phase === "target-ds-destination-saved") {
                if (!fileIdle() || !workbench.loadState.saved) {
                    return;
                }
                require(typeof workbench.loadState.path === "string" && workbench.loadState.path.endsWith("/" + downstreamOpenedName), "The actual normal DS Save did not use the newly opened isolated destination.");
                unchanged(state.before, upstream);
                state.downstreamPath = workbench.loadState.path;
                record("downstream-open-destination-from-actual-save", {
                    path: state.downstreamPath
                });
                beginEdit(upstream, secondUpstreamSeed, "new-us-request");
            } else if (state.phase === "new-us-request") {
                state.before = snapshot();
                route("new", false);
                next("new-us-prompt");
            } else if (state.phase === "new-us-prompt") {
                if (promptReady(upstream, "new")) {
                    decide("projectDirtyDiscard");
                    next("new-us-created");
                }
            } else if (state.phase === "new-us-created") {
                if (!fileIdle() || actual.shell.upstreamIdentity === state.before.upstream.identity) {
                    return;
                }
                require(!actual.us.configured && !actual.us.can_start && actual.us.project.path === null, "New US did not clear accepted inputs and its independent destination.");
                unchanged(state.before, downstream);
                record("new-upstream-discard-clears-only-upstream");
                state.before = snapshot();
                picker("reopen-us", "open", false, "reopen-us");
            } else if (state.phase === "reopen-us") {
                if (!state.pickerReturned) {
                    return;
                }
                if (dirtyDialog.opened && dirtyDialog.decisionPending) {
                    require(promptReady(upstream, "open"), "The blank US replacement prompt targeted the wrong owner.");
                    decide("projectDirtyDiscard");
                    return;
                }
                if (!fileIdle() || actual.shell.upstreamIdentity === state.before.upstream.identity) {
                    return;
                }
                require(actual.us.configured && actual.us.project.path === state.upstreamPath, "Generic Open did not reload the real US saved project.");
                unchanged(state.before, downstream);
                record("reopen-upstream-after-new");
                state.before = snapshot();
                picker("open-cancel", "open", false, "open-cancelled");
            } else if (state.phase === "open-cancelled") {
                if (!state.pickerReturned || !fileIdle()) {
                    return;
                }
                unchanged(state.before);
                record("native-open-picker-cancel");
                state.beforeMessage = workbench.fileMessage;
                picker("open-invalid", "open", false, "open-invalid");
            } else if (state.phase === "open-invalid") {
                if (!state.pickerReturned || !fileIdle()) {
                    return;
                }
                require(workbench.fileMessage.length > 0 && workbench.fileMessage !== state.beforeMessage, "Invalid native Open omitted its actual visible diagnostic.");
                unchanged(state.before);
                record("native-open-failure-preserves-both", {
                    error: workbench.fileMessage
                });
                picker("save-us-cancel", "save", true, "save-us-cancelled");
            } else if (state.phase === "save-us-cancelled") {
                if (!state.pickerReturned || !fileIdle()) {
                    return;
                }
                unchanged(state.before);
                record("native-save-as-picker-cancel");
                route("save", false);
                next("normal-us-after-cancel");
            } else if (state.phase === "save-us-protected") {
                if (!state.pickerReturned || !fileIdle()) {
                    return;
                }
                const error = actual.us.error || host.presentation.feedbackMessage || "";
                require(error.length > 0 && error !== state.beforeMessage, "Protected-source Save As omitted its actual native diagnostic.");
                unchanged(state.before);
                record("protected-source-write-rejected", {
                    error: error
                });
                route("save", false);
                next("normal-us-save");
            } else if (state.phase === "normal-us-save") {
                if (!fileIdle() || actual.us.project.status === "saving") {
                    return;
                }
                require(actual.us.project.status === "saved" && actual.us.project.path === state.upstreamPath, "Save As Cancel/failure changed the actual US normal Save destination.");
                unchanged(state.before);
                record("upstream-destination-preserved-after-source-rejection");
                state.before = snapshot();
                select(downstream);
                next("tabs-downstream");
            } else if (state.phase === "normal-us-after-cancel") {
                if (!fileIdle() || actual.us.project.status === "saving") {
                    return;
                }
                require(actual.us.project.status === "saved" && actual.us.project.path === state.upstreamPath, "Save As Cancel changed the actual normal Save destination.");
                unchanged(state.before);
                record("upstream-destination-preserved-after-picker-cancel");
                select(downstream);
                next("normal-ds-save-request");
            } else if (state.phase === "normal-ds-save-request") {
                if (!cursorCurrent(downstream)) {
                    return;
                }
                state.before = snapshot();
                route("save", false);
                next("normal-ds-save");
            } else if (state.phase === "normal-ds-save") {
                if (!downstreamSaved(state.downstreamPath)) {
                    return;
                }
                unchanged(state.before);
                record("downstream-destination-preserved-after-failures", {
                    path: workbench.loadState.path
                });
                beginEdit(downstream, secondDownstreamSeed, "close-us-edit");
            } else if (state.phase === "close-us-edit") {
                beginEdit(upstream, firstUpstreamSeed, "close-cancel-request");
            } else if (state.phase === "close-cancel-request") {
                state.before = snapshot();
                require(state.before.downstream.dirty && state.before.upstream.dirty, "The real Close cases require both directions dirty.");
                rootWindow.close();
                next("close-cancel-prompt");
            } else if (state.phase === "close-cancel-prompt") {
                if (promptReady(downstream, "close")) {
                    decide("projectDirtyCancel");
                    next("close-cancelled");
                }
            } else if (state.phase === "close-cancelled") {
                if (!fileIdle()) {
                    return;
                }
                unchanged(state.before);
                state.closeCancelled = true;
                record("close-cancel-preserves-both-dirty");
                rootWindow.close();
                next("close-first-prompt");
            } else if (state.phase === "close-first-prompt") {
                if (promptReady(downstream, "close")) {
                    decide("projectDirtyDiscard");
                    next("close-second-prompt");
                }
            } else if (state.phase === "close-second-prompt") {
                if (promptReady(upstream, "close")) {
                    unchanged(state.before);
                    decide("projectDirtyCancel");
                    next("close-second-cancelled");
                }
            } else if (state.phase === "close-second-cancelled") {
                if (!fileIdle()) {
                    return;
                }
                unchanged(state.before);
                state.secondCloseCancelled = true;
                record("close-first-discard-second-cancel-is-atomic");
                select(upstream);
                next("close-run-start");
            } else if (state.phase === "close-run-start") {
                if (!cursorCurrent(upstream) || !controls.canStart) {
                    return;
                }
                unchanged(state.before);
                const start = findItem(controls, "directionStart");
                require(start && start.enabled, "Actual US Start is unavailable after aborted Close.");
                state.runBefore = actual.us.result ? actual.us.result.run : "";
                state.runStarted = true;
                start.clicked();
                next("close-run-terminal");
            } else if (state.phase === "close-run-terminal") {
                observedRun(actual.us);
                require(!actual.us.error, "Actual post-Close US Start failed: " + (actual.us.error || ""));
                unchanged(state.before, downstream);
                require(actual.shell.upstreamIdentity === state.before.upstream.identity && actual.us.generation === state.before.upstream.generation, "Aborted Close/actual execution changed accepted US identity or generation.");
                if (actual.us.busy || actual.shell.applicationBusy || !state.activeRun.length || !actual.us.has_result || !actual.us.result || actual.us.result.run !== state.activeRun) {
                    return;
                }
                require(actual.us.result.status === "Completed", "The actual post-Close worker did not retain its Completed result.");
                state.runStarted = false;
                record("actual-upstream-start-after-aborted-close", {
                    run: state.activeRun,
                    status: actual.us.result.status,
                    referenceRecovered: actual.us.result.reference_recovered
                });
                complete(true, "");
            } else {
                require(state.phase === "new-ds-capture", "Unknown compiled project file probe phase: " + state.phase);
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
