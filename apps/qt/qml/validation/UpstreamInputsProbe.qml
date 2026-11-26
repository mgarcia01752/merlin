/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed opt-in actual Main consumer. The runner selects real source files in
// native pickers; Rust alone decodes, associates, admits and executes them.
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

    // Finite GUI scheduling/traversal policy, independent of scientific limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 2400
    readonly property int overallTimeoutMs: 180000
    readonly property int captureSettleMs: 100
    readonly property int captureGraceMs: 2000
    readonly property int itemTraversalLimit: 8192
    readonly property real plotZoomInFactor: 0.5
    readonly property real plotAnchor: 0.5
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    readonly property var captureCases: [
        {
            tag: "ucd-binary",
            family: "ucd",
            page: false
        },
        {
            tag: "ucd-json",
            family: "ucd",
            page: false
        },
        {
            tag: "rxmer-binary",
            family: "rxmer",
            page: true
        },
        {
            tag: "rxmer-json",
            family: "rxmer",
            page: false
        },
        {
            tag: "pre-eq-binary",
            family: "pre_eq",
            page: false
        },
        {
            tag: "pre-eq-json",
            family: "pre_eq",
            page: false
        }
    ]
    readonly property var measuredCases: ["measured-50", "measured-25"]
    readonly property var configurationSections: ["ucd", "profiles", "grants", "source", "receiver", "channel", "preEqualization"]
    property QtObject state: QtObject {
        property string phase: "loading"
        property int phaseRetries: 0
        property double startedAt: 0
        property double phaseStartedAt: 0
        property bool done: false
        property bool pickerReturned: false
        property string pickerTag: ""
        property int captureIndex: 0
        property int measuredIndex: 0
        property int sectionIndex: 0
        property var downstreamItem: null
        property var upstreamItem: null
        property var capturePopup: null
        property string downstreamIdentity: ""
        property string acceptedOptions: ""
        property string inspectionIdentity: ""
        property string inspectionKey: ""
        property string heldCapture: ""
        property string heldEvidence: ""
        property var invalidCaptureView: null
        property string pageBefore: ""
        property string metadataBefore: ""
        property int requestedPageFirst: 0
        property var captures: []
        property var pages: []
        property var plotViews: []
        property string plotBefore: ""
        property var plotRequest: null
        property bool rxmerImageSaved: false
        property bool pickerCancelPreserved: false
        property bool invalidPreserved: false
        property string invalidDiagnostic: ""
        property bool pagingObserved: false
        property bool inspectionTabIsolation: false
        property bool inspectionsFinished: false
        property bool screenshotSaved: false
        property string planIdentity: ""
        property string planKey: ""
        property string planSession: ""
        property string planGeneration: ""
        property var drafts: []
        property var plans: []
        property string draftToken: ""
        property bool startRequested: false
        property string previousRun: ""
        property string activeRun: ""
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /**
     * Fail this fixed operation on the first missing actual invariant.
     * @param condition Fresh native or visible presentation condition.
     * @param message Exact failure explanation retained in the result.
     */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message);
        }
    }

    /** Move to a finite asynchronous phase without blocking the GUI thread. */
    function next(phase) {
        state.phase = phase;
        state.phaseRetries = 0;
        state.phaseStartedAt = Date.now();
    }

    /**
     * Inspect actual public visual/default child lists with finite traversal.
     * @param root Existing Main item, popup content or footer.
     * @param name Existing stable objectName.
     * @returns The real object or null, without creating an alternate consumer.
     */
    function findItem(root, name) {
        const pending = [root];
        const visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0) {
                continue;
            }
            require(visited.length < itemTraversalLimit, "Inputs probe object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name) {
                return item;
            }
            for (const key of ["children", "contentData", "data"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number") {
                    continue;
                }
                require(pending.length + children.length <= itemTraversalLimit, "Inputs probe pending traversal exceeded its bound.");
                for (const child of children) {
                    pending.push(child);
                }
            }
        }
        return null;
    }

    /** Resolve one existing persistent upstream section popup. */
    function popupFor(section) {
        const popup = state.upstreamItem.sectionDialogs[section];
        require(!!popup, "Actual upstream section is unavailable: " + section);
        return popup;
    }

    /**
     * Click an actual enabled popup action through its established connection.
     * @param popup Existing opened dialog.
     * @param name Existing action objectName, including actual Close/Cancel.
     */
    function clickPopup(popup, name) {
        const action = findItem(popup.footer, name) || findItem(popup.contentItem, name) || findItem(popup.header, name);
        require(popup.opened && action && action.enabled, "Actual input action is disabled: " + name);
        action.clicked();
    }

    /** Return exact accepted key text without converting wide identifiers. */
    function keyText() {
        return JSON.stringify({
            session: host.upstreamState.session,
            generation: host.upstreamState.generation
        });
    }

    /** Check the real direction cursor and admitted shared control context. */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction);
    }

    /** Click the existing direction tab; active scientific ownership never changes. */
    function switchDirection(direction) {
        const action = findItem(tabs, direction + "DirectionTab");
        require(action && action.enabled, "Actual direction tab is unavailable.");
        action.clicked();
    }

    /**
     * Preserve independent downstream state and idle inspection ownership.
     * @returns Fresh shell metadata for explicit lifecycle checks.
     */
    function checkIndependent() {
        const shell = JSON.parse(host.backend.state());
        const downstreamState = JSON.parse(workbench.runBackend.poll());
        require(shell.downstreamIdentity === state.downstreamIdentity && JSON.stringify(downstreamState.accepted_options) === state.acceptedOptions && !downstreamState.busy, "Upstream inputs changed or executed the independent downstream owner.");
        require(tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem, "Input navigation recreated a persistent Main workbench.");
        if (!state.inspectionsFinished) {
            require(shell.upstreamIdentity === state.inspectionIdentity && keyText() === state.inspectionKey && !shell.applicationBusy, "Capture inspection changed accepted upstream configuration or started science.");
        }
        return shell;
    }

    /** Require the exact admitted plan key throughout read-only inspection and execution. */
    function checkPlan() {
        const shell = checkIndependent();
        require(shell.upstreamIdentity === state.planIdentity && keyText() === state.planKey, "Read-only draft, cancellation, tabs or execution changed the measured-plan key.");
        return shell;
    }

    /** Return the original native capture envelope, not a reconstructed observation. */
    function nativeCapture() {
        return host.upstreamState.capture_view;
    }

    /**
     * Compare the actual inspector with its current authoritative page.
     * @returns True only after the root-supplied envelope is admitted unchanged.
     */
    function inspectorCurrent() {
        const popup = state.capturePopup;
        const captured = nativeCapture();
        return popup && popup.opened && !popup.busy && !host.upstreamState.preparing && popup.contextCurrent && !popup.presentationRejected && captured && captured.family === popup.displayedFamily && JSON.stringify(popup.displayedPresentation) === JSON.stringify(captured.view);
    }

    /**
     * Compare source evidence separately from a newly reported input diagnostic.
     * This temporary display-only copy never changes the authoritative envelope.
     * @param captured Actual native family/view envelope.
     * @returns Exact source metadata, rows and page, excluding only view.error.
     */
    function sourceEvidence(captured) {
        const view = Object.assign({}, captured.view);
        delete view.error;
        return JSON.stringify({
            family: captured.family,
            view: view
        });
    }

    /** Wait for actual configuration values, text coverage and exact native correlation. */
    function draftReady(popup) {
        return popup.opened && popup.valuesLoaded && popup.textAdmitted && popup.matchingState && !popup.pending;
    }

    /**
     * Synchronize the external runner before an actual native picker action.
     * @param tag One fixed compiled source/cancel/error/measured-plan vector.
     * @param popup Existing actual inspector or UCD dialog.
     * @param actionName Existing Load action objectName.
     * @param phase Finite phase resumed after the native call returns.
     */
    function requestPicker(tag, popup, actionName, phase) {
        const action = findItem(popup.footer, actionName);
        require(popup.opened && action && action.enabled, "Actual native input picker action is unavailable.");
        state.pickerTag = tag;
        state.pickerReturned = false;
        next(phase);
        console.log("MERLIN_UPSTREAM_INPUTS_PICKER " + tag);
        Qt.callLater(function () {
            if (state.done) {
                return;
            }
            try {
                require(action.enabled, "Native input picker action became disabled.");
                action.clicked();
                state.pickerReturned = true;
            } catch (error) {
                complete(false, String(error));
            }
        });
    }

    /**
     * Capture explicitly QML-owned inspector content after all source cases.
     * The runner captures the actual full Main X11 window at the ready marker.
     */
    function capture() {
        require(workbench.theme.dark === true && inspectorCurrent(), "Actual dark capture inspection is required for evidence.");
        const target = state.capturePopup.contentItem;
        require(target && target.width > 0 && target.height > 0 && screenshotPath.length > 0, "A real inspector frame and fresh screenshot destination are required.");
        next("capturing");
        console.log("MERLIN_UPSTREAM_CAPTURE ready");
        const started = target.grabToImage(function (result) {
            if (state.done) {
                return;
            }
            try {
                checkIndependent();
                require(inspectorCurrent() && JSON.stringify(nativeCapture()) === state.heldCapture, "The real source inspector changed during capture.");
                require(result.saveToFile(screenshotPath), "Could not save the actual capture inspector image.");
                state.screenshotSaved = true;
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        });
        require(started, "Actual QML inspector content rejected image capture.");
    }

    /**
     * Compare actual measured verification strings with the original native report.
     * @returns True after the real PRBS-RX summary, rows and block match.
     */
    function verificationCurrent() {
        const popup = popupFor("verification");
        const view = host.upstreamState.execution_view;
        require(view && view.verification && Array.isArray(view.verification.rows) && view.verification.rows.length > 0 && view.verification_block, "The actual measured run supplied no native verification rows.");
        if (!popup.opened || popup.presentationRejected) {
            return false;
        }
        const expected = view.verification.rows.map(row => row.label + ": " + row.value);
        const block = state.upstreamItem.blockPresentation("verification");
        const summary = findItem(popup.contentItem, "upstreamObservationSummary");
        return popup.displayedPresentation.summary === view.verification.summary && JSON.stringify(popup.displayedRows) === JSON.stringify(expected) && summary && summary.text === view.verification.summary && block.status === view.verification_block.status && block.reason === view.verification_block.reason;
    }

    /**
     * Finish once; failure cancels only a matching run requested by this probe.
     * @param success True only after six inspections and both actual measured runs.
     * @param message Explicit error explanation, empty on complete success.
     */
    function complete(success, message) {
        if (state.done) {
            return;
        }
        state.done = true;
        phaseTimer.stop();
        if (!success && state.startRequested) {
            const shell = JSON.parse(host.backend.state());
            const current = host.upstreamState;
            if (shell.upstreamIdentity === state.planIdentity && (current.preparing || (shell.executionDirection === upstream && current.progress && current.progress.run === state.activeRun))) {
                host.upstreamBackend.stop();
            }
        }
        finished(success, JSON.stringify({
            probe: "upstream-inputs",
            success: success,
            phase: state.phase,
            error: message,
            pickerTag: state.pickerTag,
            captures: state.captures,
            pages: state.pages,
            measurementPlots: state.plotViews,
            rxmerImageSaved: state.rxmerImageSaved,
            pickerCancelPreserved: state.pickerCancelPreserved,
            invalidPreserved: state.invalidPreserved,
            invalidDiagnostic: state.invalidDiagnostic,
            invalidCaptureView: state.invalidCaptureView,
            pagingObserved: state.pagingObserved,
            inspectionTabIsolation: state.inspectionTabIsolation,
            inspectionIdentity: state.inspectionIdentity,
            inspectionKey: state.inspectionKey,
            downstreamIdentity: state.downstreamIdentity,
            downstreamAcceptedOptionsPreserved: success,
            measuredPlans: state.plans,
            screenshot: state.screenshotSaved ? screenshotPath : null,
            captureTarget: "capture-inspector-contentItem",
            fullWindowCaptureMarker: state.screenshotSaved,
            dark: !!workbench && !!workbench.theme && workbench.theme.dark === true
        }));
    }

    /**
     * Drive only fixed actual GUI inputs with finite per-phase/overall watchdogs.
     * Native inspection remains distinct from measured-plan scientific admission.
     */
    function advance() {
        if (state.done) {
            return;
        }
        try {
            state.phaseRetries += 1;
            require(state.phaseRetries <= phaseRetryLimit, "Upstream inputs probe timed out in " + state.phase + ".");
            require(Date.now() - state.startedAt <= overallTimeoutMs, "Upstream inputs probe exceeded its overall watchdog.");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem) {
                return;
            }
            workbench.pollRun();
            host.poll();
            if (state.phase === "loading") {
                const downstreamState = JSON.parse(workbench.runBackend.poll());
                if (workbench.loadingProject || !workbench.hasProject || !downstreamState.accepted_options || !controls.contextCurrent) {
                    return;
                }
                const shell = JSON.parse(host.backend.state());
                require(rootWindow.visible && workbench.theme.dark === true && !downstreamState.busy && !shell.applicationBusy && !host.upstreamState.busy, "Probe requires visible idle dark Main with its real downstream example loaded.");
                state.downstreamItem = tabs.downstreamItem;
                state.upstreamItem = tabs.upstreamItem;
                state.downstreamIdentity = shell.downstreamIdentity;
                state.acceptedOptions = JSON.stringify(downstreamState.accepted_options);
                state.inspectionIdentity = shell.upstreamIdentity;
                state.inspectionKey = keyText();
                require(host.selectDirection(upstream), "Native upstream selection was rejected.");
                next("capture-open");
                return;
            }
            const shell = checkIndependent();
            const current = host.upstreamState;
            if (state.phase === "capture-open") {
                if (!cursorCurrent(upstream) || (state.capturePopup && state.capturePopup.visible)) {
                    return;
                }
                require(typeof state.upstreamItem.openCapture === "function", "Actual upstream capture navigation is unavailable.");
                state.upstreamItem.openCapture(captureCases[state.captureIndex].family);
                next("capture-ready");
            } else if (state.phase === "capture-ready") {
                state.capturePopup = findItem(state.upstreamItem, "upstreamCaptureDialog");
                const popup = state.capturePopup;
                if (!popup || !popup.opened || popup.busy || !popup.contextCurrent || popup.presentationRejected) {
                    return;
                }
                const spec = captureCases[state.captureIndex];
                require(popup.family === spec.family, "Actual capture navigation selected another family.");
                requestPicker(spec.tag, popup, "upstreamCaptureLoad", "capture-loaded");
            } else if (state.phase === "capture-loaded") {
                if (!state.pickerReturned || !inspectorCurrent()) {
                    return;
                }
                const captured = nativeCapture();
                const spec = captureCases[state.captureIndex];
                require(captured.family === spec.family && captured.view && !captured.view.error && Array.isArray(captured.view.metadata_rows) && captured.view.metadata_rows.length > 0 && Array.isArray(captured.view.data_rows) && captured.view.data_rows.length > 0, "The real native picker did not produce the requested nonempty capture inspection.");
                state.captures = state.captures.concat([
                    {
                        tag: spec.tag,
                        capture_view: captured
                    }
                ]);
                if (state.captureIndex === 0) {
                    state.heldCapture = JSON.stringify(captured);
                    requestPicker("cancel", state.capturePopup, "upstreamCaptureLoad", "capture-cancelled");
                } else if (spec.page) {
                    require(captured.view.page && captured.view.page.has_next === true && state.capturePopup.nextAvailable, "The real 400-reading RxMER source did not expose a bounded next page.");
                    state.requestedPageFirst = captured.view.page.next_first;
                    state.pageBefore = JSON.stringify(captured.view);
                    state.metadataBefore = JSON.stringify(captured.view.metadata_rows);
                    clickPopup(state.capturePopup, "upstreamCaptureNext");
                    next("capture-page");
                } else if (state.captureIndex === captureCases.length - 1) {
                    state.heldCapture = JSON.stringify(captured);
                    state.heldEvidence = sourceEvidence(captured);
                    requestPicker("invalid", state.capturePopup, "upstreamCaptureLoad", "capture-invalid");
                } else {
                    clickPopup(state.capturePopup, "upstreamCaptureClose");
                    next("capture-closed");
                }
            } else if (state.phase === "capture-cancelled") {
                if (!state.pickerReturned || !inspectorCurrent()) {
                    return;
                }
                require(JSON.stringify(nativeCapture()) === state.heldCapture, "Native picker Cancel changed the exact inspected capture presentation.");
                state.pickerCancelPreserved = true;
                clickPopup(state.capturePopup, "upstreamCaptureClose");
                next("capture-closed");
            } else if (state.phase === "capture-page") {
                if (!inspectorCurrent() || JSON.stringify(nativeCapture().view) === state.pageBefore) {
                    return;
                }
                const captured = nativeCapture();
                require(JSON.stringify(captured.view.metadata_rows) === state.metadataBefore && captured.view.page.has_previous === true && captured.view.data_rows.length > 0, "Native paging changed source evidence or lost its original bounded data.");
                state.pages = state.pages.concat([
                    {
                        tag: captureCases[state.captureIndex].tag,
                        requested_first: state.requestedPageFirst,
                        capture_view: captured
                    }
                ]);
                state.pagingObserved = true;
                const plot = findItem(state.capturePopup.contentItem, "upstreamCapturePlot");
                const navigation = findItem(plot, "chartNavigation");
                require(plot && plot.visible && plot.viewport === captured.view.plot && navigation && navigation.canZoom, "Original RxMER plot is not an actual shared native-data consumer.");
                state.plotViews = state.plotViews.concat([{action:"paged",capture_view:captured}]);
                state.plotBefore = JSON.stringify(captured.view);
                navigation.zoom(plotZoomInFactor, plotAnchor);
                next("capture-plot-zoom");
            } else if (state.phase === "capture-plot-zoom") {
                if (!inspectorCurrent() || JSON.stringify(nativeCapture().view) === state.plotBefore) return;
                const captured = nativeCapture();
                require(JSON.stringify(captured.view.metadata_rows) === state.metadataBefore && captured.view.plot.count > 0 && captured.view.plot.count < captured.view.plot.total, "RxMER zoom lost source provenance or original viewport bounds.");
                const plot = findItem(state.capturePopup.contentItem, "upstreamCapturePlot");
                require(plot && plot.viewport === captured.view.plot && findItem(plot,"measurementChart").available, "RxMER zoom no longer draws the exact admitted original viewport.");
                const navigation = findItem(plot,"chartNavigation");
                navigation.pointHovered(plotAnchor);
                const readout = plot.readout;
                require(readout.length > 0,"Actual original RxMER hover is unavailable.");
                host.poll();
                require(plot.readout === readout && plot.viewport === nativeCapture().view.plot,
                    "Unchanged native polling cleared original RxMER hover or source alias.");
                state.plotViews = state.plotViews.concat([{action:"zoom",capture_view:captured,hover_preserved:readout}]);
                state.plotBefore = JSON.stringify(captured.view);
                clickPopup(state.capturePopup, "upstreamCapturePlotFull");
                next("capture-plot-reset");
            } else if (state.phase === "capture-plot-reset") {
                if (!inspectorCurrent() || JSON.stringify(nativeCapture().view) === state.plotBefore) return;
                const captured = nativeCapture();
                require(captured.view.plot.start === 0 && captured.view.plot.count === captured.view.plot.total, "RxMER reset did not restore original bounded capture coordinates.");
                state.plotViews = state.plotViews.concat([{action:"reset",capture_view:captured}]);
                next("capture-plot-image");
                console.log("MERLIN_UPSTREAM_RXMER_CAPTURE ready");
                require(state.capturePopup.contentItem.grabToImage(function(result) {
                    if (state.done) return;
                    try {
                        require(result.saveToFile(screenshotPath + ".rxmer.png"), "Could not save actual original RxMER plot.");
                        state.rxmerImageSaved = true;
                        next("capture-plot-image-grace");
                    } catch (error) { complete(false,String(error)); }
                }), "Actual RxMER plot rejected capture.");
            } else if (state.phase === "capture-plot-image-grace") {
                if (Date.now() - state.phaseStartedAt < captureGraceMs) return;
                clickPopup(state.capturePopup, "upstreamCaptureClose");
                next("capture-closed");
            } else if (state.phase === "capture-plot-image") {
                return;
            } else if (state.phase === "capture-closed") {
                if (state.capturePopup.visible) {
                    return;
                }
                state.captureIndex += 1;
                require(state.captureIndex < captureCases.length, "Capture iteration exceeded its compiled cases.");
                next("capture-open");
            } else if (state.phase === "capture-invalid") {
                if (!state.pickerReturned || !inspectorCurrent()) {
                    return;
                }
                require(sourceEvidence(nativeCapture()) === state.heldEvidence, "Invalid native input replaced the exact previous inspected source evidence.");
                state.invalidDiagnostic = current.error || host.presentation.feedbackMessage || "";
                require(typeof state.invalidDiagnostic === "string" && state.invalidDiagnostic.length > 0, "Invalid native input lost its original diagnostic.");
                if (current.error) {
                    const diagnostic = findItem(state.capturePopup.contentItem, "upstreamCaptureError");
                    require(state.capturePopup.displayedPresentation.error === current.error && diagnostic && diagnostic.text === current.error, "The actual inspector lost its original native input diagnostic.");
                }
                state.invalidCaptureView = nativeCapture();
                state.heldCapture = JSON.stringify(nativeCapture());
                state.invalidPreserved = true;
                clickPopup(state.capturePopup, "upstreamCaptureClose");
                switchDirection(downstream);
                next("inspection-downstream");
            } else if (state.phase === "inspection-downstream") {
                if (!cursorCurrent(downstream) || state.capturePopup.visible) {
                    return;
                }
                require(JSON.stringify(nativeCapture()) === state.heldCapture, "Downstream tab selection changed retained capture evidence.");
                switchDirection(upstream);
                next("inspection-upstream");
            } else if (state.phase === "inspection-upstream") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                require(JSON.stringify(nativeCapture()) === state.heldCapture, "Upstream tab return changed retained capture evidence.");
                state.inspectionTabIsolation = true;
                state.upstreamItem.openCapture(captureCases[state.captureIndex].family);
                next("capture-evidence");
            } else if (state.phase === "capture-evidence") {
                if (!inspectorCurrent() || Date.now() - state.phaseStartedAt < captureSettleMs) {
                    return;
                }
                require(state.captures.length === captureCases.length && state.pickerCancelPreserved && state.invalidPreserved && state.pagingObserved && state.inspectionTabIsolation, "Source inspection omitted a required native case.");
                // Display original-coordinate rows in the actual scrolled viewport.
                // This changes presentation geometry only, never capture contents.
                const viewport = state.capturePopup.contentItem.contentItem;
                require(viewport && typeof viewport.contentY === "number", "Actual capture scroll viewport is unavailable.");
                viewport.contentY = Math.max(0, viewport.contentHeight - viewport.height);
                next("capture-scrolled");
            } else if (state.phase === "capture-scrolled") {
                if (!inspectorCurrent() || Date.now() - state.phaseStartedAt < captureSettleMs) {
                    return;
                }
                capture();
            } else if (state.phase === "capture-grace") {
                require(inspectorCurrent() && JSON.stringify(nativeCapture()) === state.heldCapture, "The original inspector changed during full-window capture grace.");
                if (Date.now() - state.phaseStartedAt >= captureGraceMs) {
                    state.inspectionsFinished = true;
                    clickPopup(state.capturePopup, "upstreamCaptureClose");
                    next("measured-open");
                }
            } else if (state.phase === "measured-open") {
                if (state.capturePopup.visible || popupFor("verification").visible || !cursorCurrent(upstream)) {
                    return;
                }
                require(!current.busy && !shell.applicationBusy, "Measured-plan loading cannot replace active work.");
                require(state.upstreamItem.openSection("ucd"), "Actual UCD navigation was rejected.");
                next("measured-ready");
            } else if (state.phase === "measured-ready") {
                const popup = popupFor("ucd");
                if (!draftReady(popup)) {
                    return;
                }
                requestPicker(measuredCases[state.measuredIndex], popup, "upstreamConfigurationLoad", "measured-loaded");
            } else if (state.phase === "measured-loaded") {
                if (!state.pickerReturned || current.preparing) {
                    return;
                }
                require(!current.error, "The real measured-plan import failed: " + (current.error || ""));
                if (!draftReady(popupFor("ucd"))) {
                    return;
                }
                require(current.configured && current.configuration_view && current.configuration_view.input_mode === "measured" && typeof current.session === "string" && typeof current.generation === "string", "Native measured Load did not accept the actual measured-plan owner.");
                state.planIdentity = shell.upstreamIdentity;
                state.planSession = current.session;
                state.planGeneration = current.generation;
                state.planKey = keyText();
                state.drafts = [];
                state.draftToken = "";
                state.sectionIndex = 0;
                next("plan-draft");
            } else if (state.phase === "plan-draft") {
                checkPlan();
                const section = configurationSections[state.sectionIndex];
                const popup = popupFor(section);
                if (!draftReady(popup)) {
                    return;
                }
                const draft = host.draftState;
                const apply = findItem(popup.footer, "upstreamDraftApply");
                require(draft.section === section && draft.options && draft.options.read_only === true && draft.can_apply === false && popup.fields.length === 0 && !popup.canApply && apply && !apply.enabled && typeof draft.token === "string" && draft.token.length > 0 && draft.token !== state.draftToken && draft.baseSession === state.planSession && draft.baseGeneration === state.planGeneration, "An actual measured configuration window exposed editable or uncorrelated synthetic fields.");
                require(typeof draft.error === "string" && draft.error.length > 0 && popup.diagnostic === draft.error, "The native measured read-only explanation was not preserved.");
                state.draftToken = draft.token;
                state.drafts = state.drafts.concat([
                    {
                        section: section,
                        draft: draft
                    }
                ]);
                clickPopup(popup, "upstreamDraftCancel");
                next("plan-draft-closed");
            } else if (state.phase === "plan-draft-closed") {
                checkPlan();
                if (popupFor(configurationSections[state.sectionIndex]).visible) {
                    return;
                }
                require(host.draftState.token === state.draftToken && host.draftState.outcome === "cancelled", "Read-only Cancel did not target the original native draft.");
                state.sectionIndex += 1;
                if (state.sectionIndex < configurationSections.length) {
                    require(state.upstreamItem.openSection(configurationSections[state.sectionIndex]), "Opening the next actual read-only configuration window failed.");
                    next("plan-draft");
                } else {
                    switchDirection(downstream);
                    next("plan-downstream");
                }
            } else if (state.phase === "plan-downstream") {
                checkPlan();
                if (!cursorCurrent(downstream)) {
                    return;
                }
                require(!current.busy && !shell.applicationBusy, "Read-only input inspection unexpectedly started a simulation.");
                switchDirection(upstream);
                next("plan-upstream");
            } else if (state.phase === "plan-upstream") {
                checkPlan();
                if (!cursorCurrent(upstream) || !controls.canStart) {
                    return;
                }
                const start = findItem(controls, "directionStart");
                require(start && start.enabled && !current.busy && !shell.applicationBusy, "Actual measured-plan Start is unavailable.");
                state.previousRun = current.result ? current.result.run : "";
                state.activeRun = "";
                state.startRequested = true;
                start.clicked();
                next("plan-terminal");
            } else if (state.phase === "plan-terminal") {
                checkPlan();
                require(!current.error, "The actual measured run failed: " + (current.error || ""));
                if (current.progress && typeof current.progress.run === "string" && current.progress.run !== state.previousRun) {
                    state.activeRun = current.progress.run;
                }
                if (current.busy || current.preparing || shell.applicationBusy) {
                    return;
                }
                require(current.has_result && current.result && typeof current.result.run === "string" && current.result.run !== state.previousRun && current.result.status === "Completed" && current.result.reference_recovered === true, "The actual measured plan did not complete with independent recovered-source verification.");
                state.activeRun = current.result.run;
                state.startRequested = false;
                require(state.upstreamItem.openSection("verification"), "Actual measured PRBS-RX navigation was rejected.");
                next("plan-verification");
            } else if (state.phase === "plan-verification") {
                checkPlan();
                if (!verificationCurrent()) {
                    return;
                }
                require(current.result.run === state.activeRun && state.drafts.length === configurationSections.length, "Measured verification lost its actual report or a read-only configuration window.");
                state.plans = state.plans.concat([
                    {
                        tag: measuredCases[state.measuredIndex],
                        identity: state.planIdentity,
                        session: state.planSession,
                        generation: state.planGeneration,
                        read_only_drafts: state.drafts,
                        cancelAndTabsPreserved: true,
                        result: current.result,
                        verification: current.execution_view.verification
                    }
                ]);
                state.measuredIndex += 1;
                if (state.measuredIndex < measuredCases.length) {
                    clickPopup(popupFor("verification"), "popupCloseButton");
                    next("measured-open");
                } else {
                    require(state.plans.length === measuredCases.length && state.screenshotSaved, "Inputs probe omitted an actual measured plan or source image.");
                    complete(true, "");
                }
            } else if (state.phase !== "capturing") {
                throw new Error("Unknown upstream inputs probe phase.");
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
