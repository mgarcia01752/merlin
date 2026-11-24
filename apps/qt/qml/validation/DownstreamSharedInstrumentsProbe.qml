/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Window

// Fixed opt-in actual Main consumer. Original native projections are evidence;
// this probe neither reconstructs measurements nor changes scientific settings.
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

    // GUI scheduling, traversal and display geometry only, not scientific limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 4800
    readonly property int overallTimeoutMs: 300000
    readonly property int itemTraversalLimit: 8192
    readonly property int renderSettleMs: 100
    readonly property int popupReadyTimeoutMs: 10000
    readonly property int maximumPhaseDiagnostics: 256
    readonly property int captureGraceMs: 2000
    readonly property real displayPanFraction: 0.1
    readonly property real spectrumZoomFactor: 0.5
    readonly property real centeredAnchor: 0.5
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    // Existing compiled DS widget selectors; these are not a scientific catalog.
    readonly property var txSignals: [0, 1, 2, 3]
    readonly property var rxSelections: [
        {
            signal: 0,
            part: 0,
            view: 0
        },
        {
            signal: 0,
            part: 0,
            view: 1
        },
        {
            signal: 0,
            part: 0,
            view: 2
        },
        {
            signal: 1,
            part: 0,
            view: 0
        },
        {
            signal: 1,
            part: 0,
            view: 1
        },
        {
            signal: 1,
            part: 0,
            view: 2
        },
        {
            signal: 2,
            part: 0,
            view: 0
        },
        {
            signal: 2,
            part: 0,
            view: 1
        },
        {
            signal: 2,
            part: 0,
            view: 2
        },
        {
            signal: 3,
            part: 0,
            view: 0
        },
        {
            signal: 3,
            part: 0,
            view: 1
        },
        {
            signal: 3,
            part: 0,
            view: 2
        },
        {
            signal: 4,
            part: 0,
            view: 0
        },
        {
            signal: 4,
            part: 0,
            view: 1
        },
        {
            signal: 4,
            part: 0,
            view: 2
        },
        {
            signal: 2,
            part: 1,
            view: 0
        },
        {
            signal: 2,
            part: 1,
            view: 1
        },
        {
            signal: 2,
            part: 1,
            view: 2
        }
    ]
    readonly property var spectrumSelections: [
        {
            owner: "tx",
            tap: 0
        },
        {
            owner: "rx",
            tap: 0
        },
        {
            owner: "rx",
            tap: 3
        }
    ]
    readonly property var responseSelections: [
        {
            source: 0,
            view: 0
        },
        {
            source: 0,
            view: 4
        },
        {
            source: 1,
            view: 0
        },
        {
            source: 1,
            view: 4
        },
        {
            source: 2,
            view: 0
        }
    ]
    readonly property var measurementFamilies: ["rxmer", "response"]
    property QtObject state: QtObject {
        id: observations
        property string phase: "loading"
        property int retries: 0
        property double startedAt: 0
        property double phaseAt: 0
        property double heartbeatAt: 0
        property double maximumHeartbeatGapMs: 0
        property int heartbeatCount: 0
        property bool done: false
        property bool startedRun: false
        property bool busyObserved: false
        property var downstreamItem: null
        property var upstreamItem: null
        property string downstreamIdentity: ""
        property string upstreamIdentity: ""
        property string upstreamScience: ""
        property string acceptedOptions: ""
        property string initialRevision: ""
        property string completedScience: ""
        property var completedState: null
        property var popup: null
        property var parentPopup: null
        property var phaseDiagnostics: []
        property var viewer: null
        property int caseIndex: 0
        property int measurementMetric: 0
        property var qam: []
        property var spectra: []
        property var responses: []
        property var measurements: []
        property var tabIsolation: []
        property bool screenshotSaved: false
    }
    property Timer timer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /** Fail on an actual owner/widget invariant without substituting data. */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message);
        }
    }

    /** Record only finite popup lifecycle scalars, never another scientific DTO. */
    function popupDiagnostics(popup) {
        if (!popup) {
            return null;
        }
        const content = popup.contentItem, parent = popup.parent;
        return {
            qmlType: String(popup),
            title: popup.title,
            visible: popup.visible,
            opened: popup.opened,
            openAdmissionRejected: popup.openAdmissionRejected,
            presentationPresent: !!popup.presentationCoverage,
            presentationRejected: popup.presentationCoverage ? popup.presentationCoverage.rejected : null,
            parentType: parent ? String(parent) : null,
            parentHasWindow: !!parent && !!parent.Window.window,
            contentHasWindow: !!content && !!content.Window.window,
            contentWindowMatchesMain: !!content && content.Window.window === rootWindow,
            contentWidth: content ? content.width : null,
            contentHeight: content ? content.height : null
        };
    }

    /** Enter one finite phase with bounded actual popup/window diagnostics. */
    function next(phase) {
        require(observations.phaseDiagnostics.length < maximumPhaseDiagnostics, "DS probe phase diagnostics exceeded their finite bound.");
        observations.phaseDiagnostics = observations.phaseDiagnostics.concat([
            {
                phase: phase,
                previous: observations.phase,
                previousElapsedMs: Date.now() - observations.phaseAt,
                previousPolls: observations.retries,
                popup: popupDiagnostics(observations.popup),
                parentPopup: popupDiagnostics(observations.parentPopup)
            }
        ]);
        observations.phase = phase;
        observations.retries = 0;
        observations.phaseAt = Date.now();
    }

    /** Inspect bounded existing QObject lists, including real popup content. */
    function findObject(root, predicate) {
        const pending = [root], visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0) {
                continue;
            }
            require(visited.length < itemTraversalLimit, "DS instrument traversal exceeded its bound.");
            visited.push(item);
            if (predicate(item)) {
                return item;
            }
            for (const key of ["children", "contentData", "data"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number") {
                    continue;
                }
                require(pending.length + children.length <= itemTraversalLimit, "DS pending traversal exceeded its bound.");
                for (const child of children) {
                    pending.push(child);
                }
            }
        }
        return null;
    }

    /** Find one existing control by its stable source objectName. */
    function findItem(root, name) {
        return findObject(root, item => item.objectName === name);
    }

    /** Activate the actual enabled button through its original connection. */
    function click(root, name) {
        const button = findItem(root, name);
        require(button && button.enabled, "Actual DS action is disabled or absent: " + name);
        button.clicked();
    }

    /** Activate an existing compiled selector, preserving its own query handler. */
    function select(root, name, index) {
        const selector = findItem(root, name);
        require(selector && selector.enabled && index >= 0 && index < selector.count, "Actual DS selector is unavailable: " + name);
        selector.currentIndex = index;
        selector.activated(index);
    }

    /** Preserve actual view context/data while excluding global readiness text. */
    function observationScience(value) {
        if (!value) {
            return value;
        }
        return {
            query_sequence: value.query_sequence,
            session_token: value.session_token,
            configuration_generation: value.configuration_generation,
            run: value.run,
            snapshot: value.snapshot
        };
    }

    /** Compare independent US science while allowing global busy/readiness flags. */
    function upstreamScience(value) {
        return JSON.stringify({
            session: value.session,
            generation: value.generation,
            configured: value.configured,
            has_result: value.has_result,
            result: value.result,
            progress: value.progress,
            capture_view: value.capture_view,
            resource_grid_view: observationScience(value.resource_grid_view),
            instrument_view: observationScience(value.instrument_view)
        });
    }

    /** Exclude only live timing/readiness text from completed DS owner comparison. */
    function downstreamScience(value) {
        return JSON.stringify({
            result_token: value.result_token,
            has_result: value.has_result,
            result: value.result,
            completed_symbols: value.completed_symbols
        });
    }

    /** Require persistent owners, exact accepted options and independent US key. */
    function checkOwners() {
        const shell = JSON.parse(host.backend.state());
        const current = JSON.parse(workbench.runBackend.poll());
        require(shell.downstreamIdentity === observations.downstreamIdentity && shell.upstreamIdentity === observations.upstreamIdentity, "Instrument interaction changed an independent native identity.");
        require(tabs.downstreamItem === observations.downstreamItem && tabs.upstreamItem === observations.upstreamItem, "Instrument interaction recreated a persistent workbench.");
        require(JSON.stringify(current.accepted_options) === observations.acceptedOptions, "Instrument interaction changed DS accepted scientific options.");
        require(upstreamScience(JSON.parse(host.upstreamBackend.state())) === observations.upstreamScience, "DS inspection or execution changed independent US science.");
        if (observations.completedScience.length) {
            require(!current.busy && !shell.applicationBusy && downstreamScience(current) === observations.completedScience, "Render-only inspection started work or advanced/replaced completed DS science.");
        }
        return current;
    }

    /** Wait for the actual direction controls to match the native selected owner. */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction);
    }

    /** Fail immediately on real admission rejection and bound missing-window waits. */
    function popupReady() {
        const popup = observations.popup;
        require(!!popup, "DS navigation did not identify its actual popup.");
        require(!popup.openAdmissionRejected, "Actual DS popup visibility admission was rejected: " + popup.title);
        require(popup.presentationCoverage && !popup.presentationCoverage.rejected, "Actual DS popup presentation admission was rejected: " + popup.title);
        const ready = popup.opened && popup.contentItem && popup.contentItem.Window.window === rootWindow;
        require(ready || Date.now() - observations.phaseAt <= popupReadyTimeoutMs, "Actual DS popup did not acquire its Main window: " + JSON.stringify(popupDiagnostics(popup)));
        return ready && Date.now() - observations.phaseAt >= renderSettleMs;
    }

    /** Open the actual parent workbench before any nested instrument popup. */
    function openParent(owner, block, phase) {
        require(!workbench.transmitterPopup.visible && !workbench.receiverPopup.visible, "Previous DS workbench popup remains open during parent navigation.");
        observations.parentPopup = owner;
        observations.popup = owner;
        click(workbench, block);
        next(phase);
    }

    /** Close a child first, then its actual parent before changing workbenches. */
    function closeParent(phase) {
        if (observations.popup.visible) {
            return;
        }
        require(observations.parentPopup && observations.parentPopup.opened, "DS nested instrument lost its actual parent workbench.");
        click(observations.parentPopup.header, "popupCloseButton");
        next(phase);
    }

    /** Close through the existing header without applying a measurement draft. */
    function closePopup(nextPhase) {
        click(observations.popup.header, "popupCloseButton");
        next(nextPhase);
    }

    /** Count actual rendered descendants and Canvases without a latency promise. */
    function visualCounts(root) {
        const pending = [root];
        let objects = 0, canvases = 0;
        while (pending.length) {
            const item = pending.pop();
            require(++objects <= itemTraversalLimit, "DS visual object count exceeded its bound.");
            if (typeof item.getContext === "function" && typeof item.requestPaint === "function") {
                canvases++;
            }
            for (const child of item.children || []) {
                pending.push(child);
            }
        }
        return {
            objects: objects,
            canvases: canvases
        };
    }

    /** Locate an actual shared chart whose viewport is the parent's admitted alias. */
    function chart(root, viewport, type) {
        const value = findObject(root, item => String(item).indexOf(type) >= 0 && item.viewport === viewport && typeof item.viewportRequested === "function");
        require(!!value, "Actual shared " + type + " with the original viewport alias is absent.");
        return value;
    }

    /** Record one original scatter and verify display navigation leaves it intact. */
    function scatterEvidence(plot, observation) {
        require(plot && plot.observation === observation && plot.points === observation.points && typeof plot.pointPosition === "function" && typeof plot.resetView === "function", "DS scatter stopped sharing its admitted original point array.");
        require(plot.magnification === plot.minimumMagnification && plot.panX === 0 && plot.panY === 0, "DS scatter default display transform changed.");
        const original = JSON.stringify(observation);
        const record = {
            qmlType: String(plot),
            xCoordinate: plot.xCoordinate,
            yCoordinate: plot.yCoordinate,
            receivedSamples: plot.receivedSamples,
            roleMarkers: plot.roleMarkers,
            pointsAlias: true,
            paintedPointCount: plot.paintedPointCount,
            defaultMagnification: plot.magnification,
            defaultPanX: plot.panX,
            defaultPanY: plot.panY,
            firstPoint: observation.points.length ? observation.points[0] : null
        };
        if (observation.available && observation.points.length) {
            const point = observation.points[0], before = plot.pointPosition(point);
            require(plot.zoom(plot.zoomFactor) && plot.pan(plot.plotEdge * displayPanFraction, 0), "Shared scatter rejected finite screen navigation.");
            const after = plot.pointPosition(point);
            record.displayNavigation = {
                before: {
                    x: before.x,
                    y: before.y
                },
                after: {
                    x: after.x,
                    y: after.y
                },
                magnification: plot.magnification,
                panX: plot.panX,
                panY: plot.panY
            };
            plot.resetView();
            const restored = plot.pointPosition(point);
            require(restored.x === before.x && restored.y === before.y && plot.magnification === plot.minimumMagnification && plot.panX === 0 && plot.panY === 0 && JSON.stringify(observation) === original, "DS screen navigation altered original I/Q or failed exact reset.");
            record.displayNavigation.resetExact = true;
        }
        return record;
    }

    /** Retain original TX observation, its actual labels and shared Canvas identity. */
    function recordTx() {
        const popup = observations.popup, signal = txSignals[observations.caseIndex];
        require(popup.signalIndex === signal && popup.displayedSignal === signal, "TX QAM selector and data differ.");
        const plot = findItem(popup.contentItem, "txQamScatter");
        const value = popup.observation;
        require(Array.isArray(value.points), "Native TX QAM has no bounded point array.");
        if (value.available && (!plot || plot.paintedPointCount !== value.points.length)) {
            return false;
        }
        observations.qam = observations.qam.concat([
            {
                consumer: "tx",
                signal: signal,
                label: findItem(popup.contentItem, "txQamSignal").currentText,
                observation: value,
                unsupported: value.available ? null : value.reason || "Native observation unavailable",
                scatter: scatterEvidence(plot, value),
                visual: visualCounts(popup.contentItem)
            }
        ]);
        checkOwners();
        return true;
    }

    /** Use actual receiver selectors; PLC parts remain independent native choices. */
    function selectRx() {
        const selection = rxSelections[observations.caseIndex];
        select(observations.popup.contentItem, "receiverQamSignal", selection.signal);
        next("rx-part");
    }

    /** Record native raw/equalized/pairing data without reconstructing cell identity. */
    function recordRx() {
        const popup = observations.popup, wanted = rxSelections[observations.caseIndex];
        const query = popup.selectionControls.query(), value = popup.observation;
        require(query.signal === wanted.signal && query.part === wanted.part && query.view === wanted.view, "RX QAM retained the wrong signal, PLC part or view.");
        require(Array.isArray(value.points), "Native RX QAM has no bounded point array.");
        const plots = [];
        for (const name of ["receiverRawScatter", "receiverEqualizedScatter"]) {
            const plot = findItem(popup.contentItem, name);
            if (!plot) {
                continue;
            }
            if (value.available && plot.paintedPointCount !== value.points.length) {
                return false;
            }
            plots.push(scatterEvidence(plot, value));
        }
        require(plots.length === (wanted.view === 2 ? 2 : 1), "RX QAM view has the wrong shared scatter count.");
        observations.qam = observations.qam.concat([
            {
                consumer: "rx",
                query: query,
                observation: value,
                unsupported: value.available ? null : value.reason || "Native observation unavailable",
                signalLabel: findItem(popup.contentItem, "receiverQamSignal").currentText,
                viewLabel: findItem(popup.contentItem, "receiverQamView").currentText,
                plcPartLabel: wanted.signal === 2 ? findItem(popup.contentItem, "receiverPlcPart").currentText : null,
                scatters: plots,
                visual: visualCounts(popup.contentItem)
            }
        ]);
        checkOwners();
        return true;
    }

    /** Enter the actual parent route; do not create a detached nested popup. */
    function openSpectrum() {
        const selection = spectrumSelections[observations.caseIndex];
        const owner = selection.owner === "tx" ? workbench.transmitterPopup : workbench.receiverPopup;
        openParent(owner, selection.owner === "tx" ? "transmitterBlock" : "receiverBlock", "spectrum-parent");
    }

    /** Activate the real retained tap card after its parent is fully open. */
    function openSpectrumChild() {
        const selection = spectrumSelections[observations.caseIndex];
        const owner = observations.parentPopup;
        if (selection.owner === "tx") {
            click(owner.contentItem, "txStageTap6");
        } else {
            click(owner.contentItem, selection.tap === 0 ? "receiverInputTap" : "receiverFftTap");
        }
        observations.popup = owner.spectrumPopup;
        next("spectrum-record");
    }

    /** Record unmodified PSD/PLC/calibration metadata and actual point readout. */
    function recordSpectrum(action) {
        const popup = observations.popup, value = popup.viewport;
        require(popup.autoGenerate === false && popup.state.can_generate === false, "Retained DS spectrum unexpectedly enables simulation dispatch.");
        const plot = chart(popup.contentItem, value, "SpectrumPlot");
        const points = value.points || [];
        if (points.length) {
            plot.selectedIndex = points.length - 1;
            require(plot.selectedPoint === points[points.length - 1], "Spectrum selection copied or changed native data.");
        }
        require(visualCounts(plot).canvases === 1, "Shared SpectrumPlot no longer has one Canvas.");
        observations.spectra = observations.spectra.concat([
            {
                selection: spectrumSelections[observations.caseIndex],
                action: action,
                state: popup.state,
                viewport: value,
                displayedContext: popup.displayedContext,
                captureFrequencyAxis: popup.captureFrequencyAxis,
                selectedPoint: plot.selectedPoint,
                qmlType: String(plot),
                viewportAlias: plot.viewport === value,
                unsupported: points.length ? null : value.error || popup.state.reason || "No retained spectrum",
                visual: visualCounts(plot)
            }
        ]);
        checkOwners();
    }

    /** Change only existing response selectors; original native metadata stays intact. */
    function selectResponse() {
        const selection = responseSelections[observations.caseIndex];
        select(observations.popup.contentItem, "receiverResponseSource", selection.source);
        select(observations.popup.contentItem, "receiverResponseView", selection.view);
        next("response-record");
    }

    /** Record configured H, actual estimate or its honest absence, and backend echo. */
    function recordResponse() {
        const popup = observations.popup, wanted = responseSelections[observations.caseIndex], value = popup.response;
        require(popup.sourceIndex === wanted.source && popup.viewIndex === wanted.view, "Response selectors and original response differ.");
        const plot = chart(popup.contentItem, value, "SpectrumPlot");
        if ((value.points || []).length) {
            plot.selectedIndex = value.points.length - 1;
        }
        const comparison = wanted.source === 2 ? findItem(popup.contentItem, "receiverResponseComparison") : null;
        observations.responses = observations.responses.concat([
            {
                selection: wanted,
                query: popup.presentationCoverage.value.query,
                response: value,
                unsupported: value.available ? null : value.reason || value.error || "Native response unavailable",
                estimatedComparisonUnsupported: comparison && !comparison.viewport.available ? comparison.viewport.reason || comparison.viewport.error : null,
                sourceLabel: findItem(popup.contentItem, "receiverResponseSource").currentText,
                viewLabel: findItem(popup.contentItem, "receiverResponseView").currentText,
                snapshotLabels: popup.presentationCoverage.value.snapshots,
                qmlType: String(plot),
                viewportAlias: plot.viewport === value,
                selectedPoint: plot.selectedPoint,
                visual: visualCounts(popup.contentItem)
            }
        ]);
        checkOwners();
    }

    /** Open existing original capture session; do not Apply, design or play it. */
    function openMeasurement() {
        observations.popup = measurementFamilies[observations.caseIndex] === "rxmer" ? workbench.rxmerPopup : workbench.responsePopup;
        observations.parentPopup = null;
        click(workbench, measurementFamilies[observations.caseIndex] === "rxmer" ? "rxmerBlock" : "channelEstimateBlock");
        next("measurement-select");
    }

    /** Select the first actual original file through its existing tree signal. */
    function selectMeasurement() {
        const popup = observations.popup;
        observations.viewer = findItem(popup.contentItem, "sharedMeasurementViewer");
        require(observations.viewer && popup.draftSynchronized && popup.metadata.files.length > 0, "Loaded workbench did not expose its original native measurement file.");
        const list = findItem(observations.viewer, "measurementCaptureTree");
        require(list && list.parent && typeof list.parent.selected === "function", "Actual capture tree is unavailable.");
        const first = popup.metadata.files[0];
        require(!first.designed && typeof first.sha256 === "string" && first.sha256.length > 0 && first.bytes > 0, "Measurement evidence is not the original captured payload.");
        list.parent.selected(first.index);
        list.parent.toggleExpanded(first.index);
        observations.measurementMetric = 0;
        next("measurement-record");
    }

    /** Exercise actual point hover/readout and retain native original coordinates. */
    function recordMeasurement() {
        const popup = observations.popup, viewer = observations.viewer, value = viewer.viewport;
        if (!viewer.contextCurrent) {
            return false;
        }
        require(!viewer.animationActive && !viewer.previewRequested && viewer.selectedIndex === popup.metadata.files[0].index, "Original measurement unexpectedly selected a derived/playback view.");
        const plot = chart(viewer, value, "MeasurementPlot");
        const navigation = findItem(plot, "chartNavigation"), points = value.points || [];
        if (points.length) {
            require(navigation && typeof points[0].readout === "string", "Original native measurement lacks its point readout.");
            navigation.pointHovered(points[0].x);
            require(plot.readout === points[0].readout, "Measurement hover changed an original native coordinate/readout.");
        }
        const list = findItem(viewer, "measurementCaptureTree");
        require(visualCounts(plot).canvases === 1, "Shared MeasurementPlot no longer has one Canvas.");
        observations.measurements = observations.measurements.concat([
            {
                family: measurementFamilies[observations.caseIndex],
                metric: viewer.metric,
                metadata: popup.metadata,
                options: popup.options,
                viewport: value,
                selectedIndex: viewer.selectedIndex,
                displayedContext: viewer.displayedContext,
                expandedMetadata: list.parent.expanded,
                displayedTreeRowCount: list.count,
                readout: plot.readout,
                qmlType: String(plot),
                viewportAlias: plot.viewport === value,
                filesAlias: viewer.files === popup.metadata.files,
                unsupported: points.length ? null : value.error || "Original measurement has no displayable points",
                visual: visualCounts(plot)
            }
        ]);
        checkOwners();
        return true;
    }

    /** Capture only a QML-owned visible popup; runner may also capture actual Main. */
    function capture() {
        require(popupReady() && observations.viewer.contextCurrent && workbench.theme.dark === true && screenshotPath.length > 0, "Fresh dark original-measurement capture is unavailable.");
        next("capturing");
        console.log("MERLIN_DOWNSTREAM_SHARED_CAPTURE ready");
        const started = observations.popup.contentItem.grabToImage(function (result) {
            if (observations.done) {
                return;
            }
            try {
                checkOwners();
                require(observations.popup.opened && result.saveToFile(screenshotPath), "Could not save the actual DS popup image.");
                observations.screenshotSaved = true;
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        });
        require(started, "Actual DS popup rejected its QML image capture.");
    }

    /** Emit once; failure may stop only this probe's actual still-active DS worker. */
    function complete(success, error) {
        if (observations.done) {
            return;
        }
        observations.done = true;
        timer.stop();
        if (!success && observations.startedRun) {
            const current = JSON.parse(workbench.runBackend.poll()), shell = JSON.parse(host.backend.state());
            if (current.busy && shell.executionDirection === downstream && shell.downstreamIdentity === observations.downstreamIdentity) {
                workbench.runBackend.stop();
            }
        }
        const result = JSON.stringify({
            probe: "downstream-shared-instruments",
            success: success,
            phase: observations.phase,
            error: error,
            phaseDiagnostics: observations.phaseDiagnostics,
            lastPopup: popupDiagnostics(observations.popup),
            lastParentPopup: popupDiagnostics(observations.parentPopup),
            sourceWorkbench: "examples/workbench/single/workbench.merlin.json",
            downstreamIdentity: observations.downstreamIdentity,
            upstreamIdentity: observations.upstreamIdentity,
            acceptedOptions: observations.acceptedOptions.length ? JSON.parse(observations.acceptedOptions) : null,
            completedState: observations.completedState,
            busyObserved: observations.busyObserved,
            qam: observations.qam,
            spectra: observations.spectra,
            responses: observations.responses,
            measurements: observations.measurements,
            tabIsolation: observations.tabIsolation,
            upstreamSciencePreserved: success,
            noRenderOnlySimulationAdvance: success,
            dark: workbench.theme.dark === true,
            heartbeatCount: observations.heartbeatCount,
            maximumHeartbeatGapMs: observations.maximumHeartbeatGapMs,
            heartbeatScope: "Observed GUI timer gaps only; no performance threshold or latency guarantee.",
            screenshot: observations.screenshotSaved ? screenshotPath : null,
            captureTarget: "original-channel-response-popup-contentItem",
            fullWindowCaptureMarker: observations.screenshotSaved
        });
        console.log("MERLIN_DOWNSTREAM_SHARED_PROBE " + result);
        finished(success, result);
    }

    /** Drive one real DS run followed only by compiled existing inspection routes. */
    function advance() {
        if (observations.done) {
            return;
        }
        try {
            const now = Date.now();
            if (observations.heartbeatAt) {
                observations.maximumHeartbeatGapMs = Math.max(observations.maximumHeartbeatGapMs, now - observations.heartbeatAt);
            }
            observations.heartbeatAt = now;
            observations.heartbeatCount++;
            require(++observations.retries <= phaseRetryLimit, "DS shared probe timed out in " + observations.phase + ".");
            require(now - observations.startedAt <= overallTimeoutMs, "DS shared probe exceeded its overall watchdog.");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem) {
                return;
            }
            workbench.pollRun();
            host.poll();
            if (observations.phase === "loading") {
                const ds = JSON.parse(workbench.runBackend.poll()), shell = JSON.parse(host.backend.state());
                if (workbench.loadingProject || !workbench.hasProject || !ds.accepted_options || !controls.contextCurrent) {
                    return;
                }
                require(rootWindow.visible && workbench.theme.dark === true && !ds.busy && !shell.applicationBusy, "DS shared probe requires actual idle dark Main with its original example.");
                observations.downstreamItem = tabs.downstreamItem;
                observations.upstreamItem = tabs.upstreamItem;
                observations.downstreamIdentity = shell.downstreamIdentity;
                observations.upstreamIdentity = shell.upstreamIdentity;
                observations.acceptedOptions = JSON.stringify(ds.accepted_options);
                observations.upstreamScience = upstreamScience(JSON.parse(host.upstreamBackend.state()));
                observations.initialRevision = ds.result_token;
                require(host.selectDirection(downstream), "Actual downstream selection was rejected.");
                next("start");
                return;
            }
            const current = checkOwners();
            if (observations.phase === "start") {
                if (!cursorCurrent(downstream)) {
                    return;
                }
                require(current.can_start, "Original DS example is not ready for real Start: " + current.reason);
                click(controls, "directionStart");
                observations.startedRun = true;
                next("running");
            } else if (observations.phase === "running") {
                observations.busyObserved = observations.busyObserved || current.busy;
                if (current.busy || !current.has_result || current.result_token === observations.initialRevision) {
                    return;
                }
                require(current.result && current.result.status === "complete", "Actual DS run did not complete: " + current.reason);
                observations.completedState = current;
                observations.completedScience = downstreamScience(current);
                observations.caseIndex = 0;
                openParent(workbench.transmitterPopup, "transmitterBlock", "tx-parent");
            } else if (observations.phase === "tx-parent") {
                if (popupReady()) {
                    // Existing reusable analyzer navigation, after the actual parent.
                    require(observations.parentPopup.qamPopup.openChecked(), "Actual TX QAM opening was rejected.");
                    observations.popup = observations.parentPopup.qamPopup;
                    next("tx-select");
                }
            } else if (observations.phase === "tx-select") {
                if (popupReady()) {
                    select(observations.popup.contentItem, "txQamSignal", txSignals[observations.caseIndex]);
                    next("tx-record");
                }
            } else if (observations.phase === "tx-record") {
                if (popupReady() && recordTx()) {
                    if (++observations.caseIndex < txSignals.length) {
                        next("tx-select");
                    } else {
                        closePopup("tx-parent-close");
                    }
                }
            } else if (observations.phase === "tx-parent-close") {
                closeParent("rx-open");
            } else if (observations.phase === "rx-open") {
                if (observations.parentPopup.visible) {
                    return;
                }
                observations.caseIndex = 0;
                openParent(workbench.receiverPopup, "receiverBlock", "rx-parent");
            } else if (observations.phase === "rx-parent") {
                if (popupReady()) {
                    click(observations.parentPopup.contentItem, "receiverDataQam");
                    observations.popup = observations.parentPopup.qamPopup;
                    next("rx-select");
                }
            } else if (observations.phase === "rx-select") {
                if (popupReady() && observations.popup.selectionControls.contextCurrent) {
                    selectRx();
                }
            } else if (observations.phase === "rx-part") {
                if (popupReady() && observations.popup.selectionControls.contextCurrent) {
                    const selection = rxSelections[observations.caseIndex];
                    if (selection.signal === 2) {
                        select(observations.popup.contentItem, "receiverPlcPart", selection.part);
                    }
                    next("rx-view");
                }
            } else if (observations.phase === "rx-view") {
                if (popupReady() && observations.popup.selectionControls.contextCurrent) {
                    select(observations.popup.contentItem, "receiverQamView", rxSelections[observations.caseIndex].view);
                    next("rx-record");
                }
            } else if (observations.phase === "rx-record") {
                if (popupReady() && observations.popup.selectionControls.contextCurrent && recordRx()) {
                    if (++observations.caseIndex < rxSelections.length) {
                        next("rx-select");
                    } else {
                        observations.caseIndex = 0;
                        closePopup("rx-parent-close");
                    }
                }
            } else if (observations.phase === "rx-parent-close") {
                closeParent("spectrum-open");
            } else if (observations.phase === "spectrum-open") {
                if (!observations.parentPopup.visible) {
                    openSpectrum();
                }
            } else if (observations.phase === "spectrum-parent") {
                if (popupReady()) {
                    openSpectrumChild();
                }
            } else if (observations.phase === "spectrum-record") {
                if (popupReady()) {
                    recordSpectrum("baseband-full");
                    const plot = chart(observations.popup.contentItem, observations.popup.viewport, "SpectrumPlot");
                    const navigation = findItem(plot, "chartNavigation");
                    if (navigation && navigation.canZoom) {
                        navigation.zoom(spectrumZoomFactor, centeredAnchor);
                    }
                    next("spectrum-zoom");
                }
            } else if (observations.phase === "spectrum-zoom") {
                if (popupReady()) {
                    recordSpectrum("existing-navigation-zoom");
                    click(observations.popup.contentItem, "saFullSpan");
                    next("spectrum-full");
                }
            } else if (observations.phase === "spectrum-full") {
                if (popupReady()) {
                    recordSpectrum("existing-full-span");
                    if (observations.popup.frequencyAxisAvailable && observations.popup.state.has_result) {
                        click(observations.popup.contentItem, "saCaptureAxis");
                    }
                    next("spectrum-capture-axis");
                }
            } else if (observations.phase === "spectrum-capture-axis") {
                if (popupReady()) {
                    recordSpectrum(observations.popup.captureFrequencyAxis ? "capture-frequency" : "capture-frequency-unavailable");
                    if (observations.popup.captureFrequencyAxis && observations.popup.frequencyAxisAvailable) {
                        click(observations.popup.contentItem, "saBasebandAxis");
                    }
                    observations.caseIndex++;
                    closePopup("spectrum-parent-close");
                }
            } else if (observations.phase === "spectrum-parent-close") {
                closeParent(observations.caseIndex < spectrumSelections.length ? "spectrum-open" : "response-open");
            } else if (observations.phase === "response-open") {
                if (!observations.parentPopup.visible) {
                    observations.caseIndex = 0;
                    openParent(workbench.receiverPopup, "receiverBlock", "response-parent");
                }
            } else if (observations.phase === "response-parent") {
                if (popupReady()) {
                    const action = findObject(observations.parentPopup.contentItem, item => item.text === "Channel Response / Echo" && typeof item.clicked === "function");
                    require(action && action.enabled, "Actual received-response navigation is unavailable.");
                    action.clicked();
                    observations.popup = observations.parentPopup.responsePopup;
                    next("response-select");
                }
            } else if (observations.phase === "response-select") {
                if (popupReady()) {
                    selectResponse();
                }
            } else if (observations.phase === "response-record") {
                if (popupReady()) {
                    recordResponse();
                    if (++observations.caseIndex < responseSelections.length) {
                        next("response-select");
                    } else {
                        observations.caseIndex = 0;
                        closePopup("response-parent-close");
                    }
                }
            } else if (observations.phase === "response-parent-close") {
                closeParent("measurement-open");
            } else if (observations.phase === "measurement-open") {
                if (!observations.popup.visible && (!observations.parentPopup || !observations.parentPopup.visible)) {
                    openMeasurement();
                }
            } else if (observations.phase === "measurement-select") {
                if (popupReady()) {
                    selectMeasurement();
                }
            } else if (observations.phase === "measurement-record") {
                if (popupReady() && recordMeasurement()) {
                    if (measurementFamilies[observations.caseIndex] === "response" && ++observations.measurementMetric < observations.viewer.display.metrics.length) {
                        select(observations.viewer, "measurementMetric", observations.measurementMetric);
                        next("measurement-record");
                    } else if (++observations.caseIndex < measurementFamilies.length) {
                        closePopup("measurement-open");
                    } else {
                        capture();
                    }
                }
            } else if (observations.phase === "capture-grace") {
                require(observations.popup.opened && observations.viewer.contextCurrent, "Actual measurement disappeared during capture grace.");
                if (now - observations.phaseAt >= captureGraceMs) {
                    closePopup("switch-upstream");
                }
            } else if (observations.phase === "switch-upstream") {
                if (!observations.popup.visible) {
                    click(tabs, "upstreamDirectionTab");
                    next("check-upstream");
                }
            } else if (observations.phase === "check-upstream") {
                if (cursorCurrent(upstream)) {
                    observations.tabIsolation = observations.tabIsolation.concat([
                        {
                            direction: upstream,
                            shell: JSON.parse(host.backend.state()),
                            upstream: JSON.parse(host.upstreamBackend.state())
                        }
                    ]);
                    click(tabs, "downstreamDirectionTab");
                    next("check-downstream");
                }
            } else if (observations.phase === "check-downstream") {
                if (cursorCurrent(downstream)) {
                    observations.tabIsolation = observations.tabIsolation.concat([
                        {
                            direction: downstream,
                            shell: JSON.parse(host.backend.state()),
                            run: current
                        }
                    ]);
                    require(observations.qam.length === txSignals.length + rxSelections.length && observations.spectra.length === spectrumSelections.length * 4 && observations.responses.length === responseSelections.length && observations.measurements.length > measurementFamilies.length && observations.screenshotSaved, "Actual DS shared consumer evidence is incomplete.");
                    require(observations.qam.some(entry => entry.consumer === "tx" && entry.observation.available) && observations.qam.filter(entry => entry.consumer === "rx" && entry.query.signal === 0 && entry.observation.available).length === 3, "Actual shared scatters did not display TX and received raw/equalized/paired data.");
                    require(observations.spectra.some(entry => entry.selection.owner === "tx" && entry.viewport.points.length > 0) && observations.spectra.some(entry => entry.selection.owner === "rx" && entry.viewport.points.length > 0), "Actual shared spectrum lacked a retained TX or RX consumer.");
                    require(observations.responses.some(entry => entry.selection.source === 0 && entry.selection.view === 0 && entry.response.available) && observations.responses.some(entry => entry.selection.source === 0 && entry.selection.view === 4 && entry.response.available && entry.response.detail), "Configured H and original backend impulse/echo evidence are incomplete.");
                    require(observations.measurements.every(entry => entry.viewport.points.length > 0 && entry.filesAlias), "An original measurement payload was not displayed through its parent alias.");
                    complete(true, "");
                }
            }
        } catch (error) {
            complete(false, String(error));
        }
    }
    Component.onCompleted: {
        observations.startedAt = Date.now();
        observations.phaseAt = observations.startedAt;
        timer.start();
    }
}
