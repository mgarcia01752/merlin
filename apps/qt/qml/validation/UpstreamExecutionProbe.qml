/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed opt-in actual Main consumer. Native Rust owns configuration, execution,
// progress and recovery; the runner supplies the real OS picker input and image.
QtObject {
    id: probe
    required property var rootWindow
    required property var workbench
    required property var tabs
    required property var host
    required property var controls
    required property var captureItem
    required property string screenshotPath
    signal finished(bool success, string message)

    // Finite GUI scheduling/traversal policies, independent of scientific limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 2400
    readonly property int overallTimeoutMs: 180000
    readonly property int captureSettleMs: 100
    readonly property int captureGraceMs: 2000
    readonly property int itemTraversalLimit: 8192

    // Require actual completed science before testing preservation on cancellation.
    readonly property int minimumCompletedGrantWitness: 1
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    property QtObject state: QtObject {
        property string phase: "loading"
        property int phaseRetries: 0
        property double startedAt: 0
        property double phaseStartedAt: 0
        property bool done: false
        property bool pickerReturned: false
        property bool configurationLoaded: false
        property bool startRequested: false
        property bool firstPauseAcknowledged: false
        property bool foreignControlsRejected: false
        property bool secondStartRejected: false
        property bool tabIsolation: false
        property bool resumedProgress: false
        property bool secondPauseAcknowledged: false
        property bool cancellingObserved: false
        property bool cancellationLeaseChecked: false
        property bool cancelledReportRetained: false
        property bool completedReportRetained: false
        property bool verificationMatched: false
        property bool executionDisplayMatched: false
        property bool screenshotSaved: false
        property var downstreamItem: null
        property var upstreamItem: null
        property string downstreamIdentity: ""
        property string upstreamIdentity: ""
        property string upstreamGeneration: ""
        property string acceptedOptions: ""
        property string firstRun: ""
        property string activeRun: ""
        property string pausedProgress: ""
        property var cancelledResult: null
        property var completedResult: null
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /**
     * Fail this fixed operation on the first absent actual invariant.
     * @param condition Fresh native or visible QML condition.
     * @param message Exact failure explanation included in terminal JSON.
     */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message);
        }
    }

    /**
     * Advance a finite asynchronous phase without blocking the GUI thread.
     * @param phase Next compiled phase tag.
     */
    function next(phase) {
        state.phase = phase;
        state.phaseRetries = 0;
        state.phaseStartedAt = Date.now();
    }

    /**
     * Inspect public visual child lists within a finite traversal budget.
     * @param root Existing actual Main item or popup footer.
     * @param name Existing stable objectName.
     * @returns The actual object, or null; no alternate control is created.
     */
    function findItem(root, name) {
        const pending = [root];
        const visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0) {
                continue;
            }
            require(visited.length < itemTraversalLimit, "Execution probe object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name) {
                return item;
            }
            for (const key of ["children", "contentData"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number") {
                    continue;
                }
                require(pending.length + children.length <= itemTraversalLimit, "Execution probe pending traversal exceeded its bound.");
                for (const child of children) {
                    pending.push(child);
                }
            }
        }
        return null;
    }

    /**
     * Resolve an existing persistent upstream popup without constructing one.
     * @param section Compiled workbench navigation tag.
     * @returns The actual section dialog.
     */
    function popupFor(section) {
        const popup = state.upstreamItem.sectionDialogs[section];
        require(!!popup, "Actual upstream section is unavailable: " + section);
        return popup;
    }

    /**
     * Check the selected native cursor and persistent children after binding settle.
     * @param direction Expected compiled direction.
     * @returns True when the real tabs and shared controls observe that cursor.
     */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction) && tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem;
    }

    /**
     * Click the actual direction tab through its existing Main signal connection.
     * @param direction Compiled target tag; no active work is redirected.
     */
    function switchDirection(direction) {
        const button = findItem(tabs, direction + "DirectionTab");
        require(button && button.enabled, "Actual direction tab is unavailable.");
        button.clicked();
    }

    /**
     * Activate an actual shared run control with its admitted opaque origin.
     * @param name Existing control objectName.
     */
    function clickControl(name) {
        const button = findItem(controls, name);
        require(controls.contextCurrent && button && button.enabled, "Actual shared control is disabled: " + name);
        button.clicked();
    }

    /**
     * Preserve independent accepted downstream state while upstream is active.
     * @returns Fresh shell metadata; no report or source is recomputed.
     */
    function checkIndependent() {
        const shell = JSON.parse(host.backend.state());
        const downstreamState = JSON.parse(workbench.runBackend.poll());
        require(shell.downstreamIdentity === state.downstreamIdentity, "Upstream execution changed the independent downstream identity.");
        require(JSON.stringify(downstreamState.accepted_options) === state.acceptedOptions, "Upstream execution changed downstream accepted_options.");
        require(!downstreamState.busy, "Upstream controls started or redirected downstream execution.");
        require(tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem, "Direction switching recreated an actual Main workbench.");
        if (state.configurationLoaded) {
            require(shell.upstreamIdentity === state.upstreamIdentity && host.upstreamState.generation === state.upstreamGeneration, "Execution or tab switching changed the accepted upstream identity/generation.");
        }
        return shell;
    }

    /**
     * Compare only authoritative reported progress, without calculating coverage.
     * @param progress Existing native bounded progress DTO.
     * @returns Exact representation of the three observed counters.
     */
    function progressSignature(progress) {
        return JSON.stringify({
            completed_grants: progress.completed_grants,
            emitted_samples: progress.emitted_samples,
            compared_bits: progress.compared_bits
        });
    }

    /**
     * Require real acknowledged pause for the first run and its shared gate.
     * @returns True only after native, shell and selected controls acknowledge it.
     */
    function paused() {
        const current = host.upstreamState;
        const shell = checkIndependent();
        return !!current.progress && current.progress.run === state.firstRun && current.progress.status === "Paused" && current.busy && shell.applicationBusy && shell.executionDirection === upstream && shell.executionStatus.indexOf("paused") >= 0;
    }

    /**
     * Observe cancellation without requiring an artificial scheduling delay.
     * A fast released gate is valid only after the actual Cancelled report arrives.
     * @returns True when the genuine terminal outcome has released the shared gate.
     */
    function cancellationTerminal() {
        const current = host.upstreamState;
        const shell = checkIndependent();
        if (shell.applicationBusy) {
            require(shell.executionDirection === upstream && current.busy && !current.can_start, "Cancellation released or redirected the real active upstream ownership.");
            require(findItem(controls, "directionStart").enabled === false, "Start became enabled before cancellation reached terminal.");
            state.cancellingObserved = state.cancellingObserved || shell.executionStatus.indexOf("cancelling") >= 0;
            state.cancellationLeaseChecked = true;
            return false;
        }
        require(!current.busy && !current.preparing && current.has_result && current.result && current.result.run === state.firstRun && current.result.status === "Cancelled", "The global slot was released without the actual retained Cancelled report.");
        state.cancellationLeaseChecked = true;
        return true;
    }

    /**
     * Synchronize the runner before the actual UCD Load button opens its OS picker.
     */
    function requestPicker() {
        const popup = popupFor("ucd");
        const button = findItem(popup.footer, "upstreamConfigurationLoad");
        require(popup.opened && popup.valuesLoaded && popup.textAdmitted && !popup.pending && button && button.enabled, "Actual UCD Load Configuration is not ready.");
        state.pickerReturned = false;
        next("picker-return");
        console.log("MERLIN_UPSTREAM_PICKER load");
        Qt.callLater(function () {
            if (state.done) {
                return;
            }
            try {
                require(button.enabled, "Load Configuration became disabled before the picker.");
                button.clicked();
                state.pickerReturned = true;
            } catch (error) {
                complete(false, String(error));
            }
        });
    }

    /**
     * Check actual shared control bindings against native display strings.
     * @param admitted Also require the independently admitted visible status copy.
     * @returns Whether the real controls consume the original execution view.
     */
    function executionControlsCurrent(admitted) {
        const view = host.upstreamState.execution_view;
        if (!view || !cursorCurrent(upstream) || controls.status !== view.status || controls.elapsed !== view.elapsed || controls.activityTooltip !== view.activity) {
            return false;
        }
        if (!admitted) {
            return true;
        }
        const held = controls.presentationCoverage.value;
        const progress = findItem(controls, "directionRunProgress");
        return !controls.presentationCoverage.rejected && held && held.status === view.status && held.elapsed === view.elapsed && held.activity === view.activity && progress && progress.text === view.status;
    }

    /**
     * Compare the real verification popup with the supplied native execution view.
     * @returns True after every retained row, summary and block matches the DTO.
     */
    function verificationCurrent() {
        const view = host.upstreamState.execution_view;
        const popup = popupFor("verification");
        require(view && view.verification && Array.isArray(view.verification.rows) && view.verification.rows.length > 0 && typeof view.verification.summary === "string" && view.verification.summary.length > 0 && view.verification_block, "The actual native report did not supply verification display rows.");
        if (!popup.opened || popup.presentationRejected) {
            return false;
        }
        const expected = view.verification.rows.map(row => row.label + ": " + row.value);
        const block = state.upstreamItem.blockPresentation("verification");
        const summary = findItem(popup.contentItem, "upstreamObservationSummary");
        return popup.displayedPresentation.summary === view.verification.summary && JSON.stringify(popup.displayedRows) === JSON.stringify(expected) && summary && summary.text === view.verification.summary && block.status === view.verification_block.status && block.reason === view.verification_block.reason;
    }

    /**
     * Capture the explicitly QML-owned verification content, never Qt-owned window
     * content. The runner captures the full actual Main X11 window at the marker.
     */
    function capture() {
        require(workbench.theme.dark === true && verificationCurrent(), "Actual dark verification content is required for evidence.");
        const target = popupFor("verification").contentItem;
        require(target && target.width > 0 && target.height > 0 && screenshotPath.length > 0, "A real verification frame and fresh image destination are required.");
        next("capturing");
        console.log("MERLIN_UPSTREAM_EXECUTION_CAPTURE ready");
        const started = target.grabToImage(function (result) {
            if (state.done) {
                return;
            }
            try {
                checkIndependent();
                require(verificationCurrent(), "The actual verification report changed before capture.");
                require(result.saveToFile(screenshotPath), "Could not save the actual verification content image.");
                state.screenshotSaved = true;
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        });
        require(started, "The actual QML verification content rejected image capture.");
    }

    /**
     * Finish once and cancel only this probe's own active upstream request on failure.
     * @param success True only after real cancellation, completion and row capture.
     * @param message Failure explanation; empty on complete success.
     */
    function complete(success, message) {
        if (state.done) {
            return;
        }
        state.done = true;
        phaseTimer.stop();
        if (!success && state.startRequested && state.configurationLoaded) {
            const shell = JSON.parse(host.backend.state());
            const current = host.upstreamState;
            if (shell.upstreamIdentity === state.upstreamIdentity && (current.preparing || (current.progress && current.progress.run === state.activeRun && shell.executionDirection === upstream))) {
                host.upstreamBackend.stop();
            }
        }
        finished(success, JSON.stringify({
            probe: "upstream-execution",
            success: success,
            phase: state.phase,
            error: message,
            chooserLoaded: state.configurationLoaded,
            firstPauseAcknowledged: state.firstPauseAcknowledged,
            foreignControlsRejected: state.foreignControlsRejected,
            secondStartRejected: state.secondStartRejected,
            tabIsolation: state.tabIsolation,
            downstreamIdentity: state.downstreamIdentity,
            downstreamAcceptedOptionsPreserved: success,
            upstreamIdentity: state.upstreamIdentity,
            upstreamGeneration: state.upstreamGeneration,
            resumedActualProgress: state.resumedProgress,
            secondPauseAcknowledged: state.secondPauseAcknowledged,
            cancellingObserved: state.cancellingObserved,
            globalSlotRetainedUntilTerminal: state.cancellationLeaseChecked,
            cancelledReportRetained: state.cancelledReportRetained,
            cancelledResult: state.cancelledResult,
            completedReportRetained: state.completedReportRetained,
            completedResult: state.completedResult,
            verificationRowsMatchedNative: state.verificationMatched,
            sharedExecutionDisplayMatchedNative: state.executionDisplayMatched,
            screenshot: state.screenshotSaved ? screenshotPath : null,
            captureTarget: "verification-contentItem",
            fullWindowCaptureMarker: state.screenshotSaved,
            dark: !!workbench && !!workbench.theme && workbench.theme.dark === true
        }));
    }

    /**
     * Drive actual shared controls and retained outcomes within finite watchdogs.
     * No result, readiness, timing or recovery quantity is calculated in QML.
     */
    function advance() {
        if (state.done) {
            return;
        }
        try {
            state.phaseRetries += 1;
            require(state.phaseRetries <= phaseRetryLimit, "Upstream execution probe timed out in " + state.phase + ".");
            require(Date.now() - state.startedAt <= overallTimeoutMs, "Upstream execution probe exceeded its overall watchdog.");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem) {
                return;
            }
            workbench.pollRun();
            host.poll();
            if (state.phase === "loading") {
                const current = JSON.parse(workbench.runBackend.poll());
                if (workbench.loadingProject || !workbench.hasProject || !current.accepted_options || !controls.contextCurrent) {
                    return;
                }
                const shell = JSON.parse(host.backend.state());
                require(rootWindow.visible && workbench.theme.dark === true && !current.busy && !shell.applicationBusy, "Probe requires the visible idle dark Main with the real downstream example loaded.");
                state.downstreamItem = tabs.downstreamItem;
                state.upstreamItem = tabs.upstreamItem;
                state.downstreamIdentity = shell.downstreamIdentity;
                state.acceptedOptions = JSON.stringify(current.accepted_options);
                require(host.selectDirection(upstream), "Native upstream selection was rejected.");
                next("ucd-open");
                return;
            }
            const shell = checkIndependent();
            const current = host.upstreamState;
            if (state.startRequested && current.progress && typeof current.progress.run === "string" && (!state.firstRun.length || current.progress.run !== state.firstRun)) {
                state.activeRun = current.progress.run;
            }
            if (state.phase === "ucd-open") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                require(!current.busy && !shell.applicationBusy, "Configuration acquisition cannot replace active work.");
                require(state.upstreamItem.openSection("ucd"), "Actual UCD navigation was rejected.");
                next("ucd-ready");
            } else if (state.phase === "ucd-ready") {
                const popup = popupFor("ucd");
                if (!popup.opened || !popup.valuesLoaded || !popup.textAdmitted || popup.pending) {
                    return;
                }
                requestPicker();
            } else if (state.phase === "picker-return") {
                if (!state.pickerReturned || current.preparing) {
                    return;
                }
                require(!current.error, "Native configuration Load failed: " + (current.error || ""));
                const popup = popupFor("ucd");
                if (!popup.opened || !popup.valuesLoaded || !popup.textAdmitted || !popup.matchingState) {
                    return;
                }
                require(current.configured && popup.fields.length > 0 && typeof current.generation === "string" && current.generation.length > 0, "The actual picker did not adopt a real configured upstream owner.");
                state.configurationLoaded = true;
                state.upstreamIdentity = shell.upstreamIdentity;
                state.upstreamGeneration = current.generation;
                const cancel = findItem(popup.footer, "upstreamDraftCancel");
                require(cancel && cancel.enabled, "The actual loaded UCD Cancel is unavailable.");
                cancel.clicked();
                next("first-start");
            } else if (state.phase === "first-start") {
                if (popupFor("ucd").visible || !cursorCurrent(upstream) || !controls.canStart) {
                    return;
                }
                state.startRequested = true;
                clickControl("directionStart");
                next("first-running");
            } else if (state.phase === "first-running") {
                require(!current.error, "The actual first upstream run was rejected: " + (current.error || ""));
                if (!current.progress || current.progress.status !== "Running") {
                    require(!current.has_result, "The fixed upstream fixture completed before its first real pause could be requested.");
                    return;
                }
                require(current.busy && shell.applicationBusy && shell.executionDirection === upstream && typeof current.progress.run === "string" && current.progress.run.length > 0, "Actual upstream Running progress has no matching global ownership.");
                state.firstRun = current.progress.run;
                state.activeRun = state.firstRun;
                require(!findItem(controls, "directionStart").enabled && !host.runAction("start", upstream, state.upstreamIdentity), "A second upstream Start was accepted while its real run was active.");
                state.secondStartRejected = true;
                clickControl("directionPause");
                next("first-paused");
            } else if (state.phase === "first-paused") {
                if (!paused() || !controls.paused) {
                    return;
                }
                state.firstPauseAcknowledged = true;
                state.pausedProgress = progressSignature(current.progress);
                switchDirection(downstream);
                next("downstream-foreign");
            } else if (state.phase === "downstream-foreign") {
                if (!cursorCurrent(downstream)) {
                    return;
                }
                require(paused() && !controls.busy && !controls.paused, "Switching to downstream redirected the actual paused upstream run.");
                for (const action of ["stop", "pause", "start"]) {
                    require(!host.runAction(action, upstream, state.upstreamIdentity) && !tabs.requestRunAction(action, upstream, state.upstreamIdentity), "A foreign hidden upstream control crossed the selected-direction guard: " + action);
                    require(!host.runAction(action, downstream, state.downstreamIdentity), "A downstream control consumed the upstream global lease: " + action);
                    require(paused(), "Rejected foreign control changed the actual acknowledged pause.");
                }
                state.foreignControlsRejected = true;
                const notice = findItem(tabs, "otherDirectionExecutionStatus");
                require(notice && notice.visible && notice.text.length > 0, "Downstream omitted the actual hidden upstream execution status.");
                require(host.feedback(""), "Could not clear deliberately rejected control feedback.");
                switchDirection(upstream);
                next("upstream-resume");
            } else if (state.phase === "upstream-resume") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                require(paused() && controls.paused && controls.busy, "The upstream tab lost its actual paused owner.");
                state.tabIsolation = true;
                clickControl("directionPause");
                next("resumed-progress");
            } else if (state.phase === "resumed-progress") {
                require(current.busy && shell.applicationBusy && shell.executionDirection === upstream, "The fixed run ended before resumed progress and cancellation could be observed.");
                if (!current.progress || current.progress.run !== state.firstRun || current.progress.status !== "Running" || progressSignature(current.progress) === state.pausedProgress || current.progress.completed_grants < minimumCompletedGrantWitness) {
                    return;
                }
                require(current.execution_view && typeof current.execution_view.status === "string" && typeof current.execution_view.elapsed === "string" && typeof current.execution_view.activity === "string", "Native running execution display strings are unavailable.");
                if (!executionControlsCurrent(false)) {
                    return;
                }
                state.resumedProgress = true;
                clickControl("directionPause");
                next("second-paused");
            } else if (state.phase === "second-paused") {
                if (!paused() || !controls.paused) {
                    return;
                }
                state.secondPauseAcknowledged = true;
                clickControl("directionStop");
                next("cancelled-terminal");
                cancellationTerminal();
            } else if (state.phase === "cancelled-terminal") {
                if (!cancellationTerminal()) {
                    return;
                }
                state.cancelledResult = current.result;
                state.cancelledReportRetained = true;
                state.startRequested = false;
                next("second-start");
            } else if (state.phase === "second-start") {
                require(state.cancelledReportRetained && !current.busy && !shell.applicationBusy, "The next run was attempted before the genuine cancelled terminal outcome.");
                if (!cursorCurrent(upstream) || !controls.canStart) {
                    return;
                }
                state.startRequested = true;
                clickControl("directionStart");
                next("completed-terminal");
            } else if (state.phase === "completed-terminal") {
                require(!current.error, "The actual completion run failed: " + (current.error || ""));
                if (current.progress && current.progress.run !== state.firstRun) {
                    state.activeRun = current.progress.run;
                }
                if (current.busy || current.preparing || shell.applicationBusy) {
                    return;
                }
                require(current.has_result && current.result && typeof current.result.run === "string" && current.result.run !== state.firstRun && current.result.status === "Completed" && typeof current.result.reference_recovered === "boolean" && (current.result.ber === null || typeof current.result.ber === "number"), "The distinct second actual run did not retain its native Completed outcome.");
                state.activeRun = current.result.run;
                state.completedResult = current.result;
                state.completedReportRetained = true;
                state.startRequested = false;
                require(state.upstreamItem.openSection("verification"), "Actual PRBS-RX / Verification navigation was rejected.");
                next("verification-ready");
            } else if (state.phase === "verification-ready") {
                if (!verificationCurrent() || !executionControlsCurrent(true) || Date.now() - state.phaseStartedAt < captureSettleMs) {
                    return;
                }
                state.verificationMatched = true;
                state.executionDisplayMatched = true;
                require(state.firstPauseAcknowledged && state.foreignControlsRejected && state.secondStartRejected && state.tabIsolation && state.resumedProgress && state.secondPauseAcknowledged && state.cancellationLeaseChecked && state.cancelledReportRetained && state.completedReportRetained, "Execution probe omitted a required real lifecycle phase.");
                capture();
            } else if (state.phase === "capture-grace") {
                require(verificationCurrent() && executionControlsCurrent(true) && !current.busy && !shell.applicationBusy && current.result.run === state.activeRun, "The actual verification owner changed during external capture grace.");
                if (Date.now() - state.phaseStartedAt >= captureGraceMs) {
                    complete(true, "");
                }
            } else if (state.phase !== "capturing") {
                throw new Error("Unknown upstream execution probe phase.");
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
