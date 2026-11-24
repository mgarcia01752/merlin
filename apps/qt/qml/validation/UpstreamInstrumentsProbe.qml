/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed actual Main consumer: native pickers, controls and public observations.
// Rust alone creates science; this probe records original native projections.
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
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 2400
    readonly property int overallTimeoutMs: 240000
    readonly property int itemTraversalLimit: 8192
    readonly property int captureSettleMs: 100
    readonly property int captureGraceMs: 2000
    readonly property real panFraction: 0.1
    readonly property real spectrumZoomInFactor: 0.5
    readonly property real spectrumZoomOutFactor: 2
    readonly property real centeredAnchor: 0.5
    readonly property real graphEdgeTolerancePixels: 1
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    readonly property var cases: ["instruments-50", "instruments-25"]
    readonly property var requiredActions: ["qam-initial", "qam-raw", "qam-unit-reference", "qam-exact-pairing", "spectrum-initial", "spectrum-channel-output", "spectrum-zoom", "spectrum-reset", "response-H", "response-C", "response-composite", "response-impulse", "response-received-amplitude", "response-received-phase"]
    property QtObject state: QtObject {
        property string phase: "loading"
        property int retries: 0
        property double startedAt: 0
        property double phaseAt: 0
        property double heartbeatAt: 0
        property double maximumHeartbeatGapMs: 0
        property int heartbeatCount: 0
        property bool done: false
        property int caseIndex: 0
        property var downstreamItem: null
        property var upstreamItem: null
        property var popup: null
        property var qamPopup: null
        property var spectrumPopup: null
        property var responsePopup: null
        property string downstreamIdentity: ""
        property string downstreamOptions: ""
        property string downstreamExecution: ""
        property string identity: ""
        property string key: ""
        property string run: ""
        property string previousRun: ""
        property bool startRequested: false
        property string report: ""
        property string terminalProgress: ""
        property bool pickerReturned: false
        property string previousSequence: ""
        property int requestCount: 0
        property var request: null
        property var requestContext: null
        property double requestedAt: 0
        property int requestedHeartbeat: 0
        property string requestedKind: ""
        property string requestedAction: ""
        property string nextPhase: ""
        property var snapshots: []
        property var plans: []
        property var preRun: []
        property var navigation: []
        property var screen: []
        property bool screenshotSaved: false
        property var instrumentImages: ({})
        property string imageAfterPhase: ""
    }
    property Timer timer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }
    property Connections qamRequests: Connections {
        target: state.qamPopup
        function onInstrumentRequested(kind, selection, context) {
            probe.observeRequest(kind, selection, context);
        }
    }
    property Connections spectrumRequests: Connections {
        target: state.spectrumPopup
        function onInstrumentRequested(kind, selection, context) {
            probe.observeRequest(kind, selection, context);
        }
    }

    property Connections responseRequests: Connections {
        target: state.responsePopup
        function onInstrumentRequested(kind, selection, context) {
            probe.observeRequest(kind, selection, context);
        }
    }

    /** Fail on an actual native/widget invariant, never substitute observations. */
    function require(condition, message) {
        if (!condition)
            throw new Error(message);
    }
    /** Enter a finite asynchronous phase. */
    function next(phase) {
        state.phase = phase;
        state.retries = 0;
        state.phaseAt = Date.now();
    }

    /** Traverse only finite existing public child lists, including popup content. */
    function findItem(root, name) {
        const pending = [root], visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0)
                continue;
            require(visited.length < itemTraversalLimit, "Instrument object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name)
                return item;
            for (const key of ["children", "contentData", "data"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number")
                    continue;
                require(pending.length + children.length <= itemTraversalLimit, "Instrument pending traversal exceeded its bound.");
                for (const child of children)
                    pending.push(child);
            }
        }
        return null;
    }

    /** Count actual visual objects and Canvas items; no latency threshold is added. */
    function visualCounts(root) {
        const pending = [root];
        let count = 0, canvases = 0;
        while (pending.length) {
            const item = pending.pop();
            require(++count <= itemTraversalLimit, "Instrument visual traversal exceeded its bound.");
            if (typeof item.getContext === "function" && typeof item.requestPaint === "function")
                canvases++;
            for (const child of item.children)
                pending.push(child);
        }
        return {
            items: count,
            canvases: canvases
        };
    }

    /** Click the original enabled action through its established handler. */
    function click(root, name) {
        const action = findItem(root, name);
        require(action && action.enabled, "Actual instrument action is unavailable: " + name);
        action.clicked();
    }
    /** Preserve exact identities without converting wide strings to numbers. */
    function keyText() {
        return JSON.stringify({
            session: host.upstreamState.session,
            generation: host.upstreamState.generation
        });
    }
    /** Snapshot native execution metadata; passive inspection must not advance it. */
    function executionText(value) {
        return JSON.stringify({
            has_result: value.has_result,
            result: value.result,
            progress: value.progress
        });
    }
    /** Require selected direction and admitted shared control ownership. */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction);
    }
    /** Preserve independent DS config, result and persistent workbench identities. */
    function checkIndependent() {
        const shell = JSON.parse(host.backend.state());
        const ds = JSON.parse(workbench.runBackend.poll());
        require(shell.downstreamIdentity === state.downstreamIdentity && JSON.stringify(ds.accepted_options) === state.downstreamOptions && executionText(ds) === state.downstreamExecution && !ds.busy, "Upstream instruments changed or executed independent downstream state.");
        require(tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem, "Instrument navigation recreated a persistent workbench.");
        if (state.key.length)
            require(shell.upstreamIdentity === state.identity && keyText() === state.key, "Instrument work changed the accepted upstream key.");
        if (state.report.length) {
            const current = host.upstreamState;
            require(!shell.applicationBusy && JSON.stringify({
                has_result: current.has_result,
                result: current.result
            }) === state.report, "Passive observations advanced or replaced completed upstream science.");
            // The native worker temporarily moves its controller. A missing transient
            // progress projection during preparation is not new scientific progress.
            if (!current.preparing)
                require(JSON.stringify(current.progress) === state.terminalProgress, "Passive observation changed terminal native source progress.");
        }
        return shell;
    }

    /** Return the real UCD configuration popup. */
    function ucd() {
        return state.upstreamItem.sectionDialogs.ucd;
    }
    /** Wait for actual admitted draft/text and matching native token/base. */
    function draftReady() {
        const p = ucd();
        return p.opened && p.valuesLoaded && p.textAdmitted && p.matchingState && !p.pending;
    }
    /** Announce one compiled fixture before the actual native Load click. */
    function load() {
        const action = findItem(ucd().footer, "upstreamConfigurationLoad");
        require(draftReady() && action && action.enabled, "Actual upstream configuration Load is unavailable.");
        state.pickerReturned = false;
        next("loaded");
        console.log("MERLIN_UPSTREAM_INSTRUMENTS_PICKER " + cases[state.caseIndex]);
        Qt.callLater(function () {
            if (state.done)
                return;
            try {
                require(action.enabled, "Configuration Load became disabled.");
                action.clicked();
                state.pickerReturned = true;
            } catch (error) {
                complete(false, String(error));
            }
        });
    }

    /** Open only the root-provided actual instrument destination. */
    function openInstrument(kind) {
        require(typeof state.upstreamItem.openInstrument === "function", "Actual upstream instrument navigation is unavailable.");
        state.upstreamItem.openInstrument(kind);
        state.popup = findItem(state.upstreamItem, kind === "qam" ? "upstreamQamDialog" : kind === "spectrum" ? "upstreamSpectrumDialog" : "upstreamResponseDialog");
        require(!!state.popup, "Actual instrument dialog is unavailable.");
        if (kind === "qam")
            state.qamPopup = state.popup;
        else if (kind === "spectrum")
            state.spectrumPopup = state.popup;
        else
            state.responsePopup = state.popup;
    }
    /** Wait for matching admitted native data, without optimistic queue acceptance. */
    function ready(kind) {
        const value = host.instrumentState;
        const admitted = state.popup && state.popup.opened && !state.popup.presentationRejected && state.popup.contextCurrent && state.popup.actionsEnabled && value && value.status === "ready" && value.snapshot && value.snapshot.kind === kind && value.snapshot.status === "ready" && state.popup.envelope === value && !host.upstreamState.preparing;
        if (!admitted || kind !== "qam")
            return admitted;
        const actual = findItem(state.popup.contentItem, "upstreamQamActualPlot");
        const reference = findItem(state.popup.contentItem, "upstreamQamReferencePlot");
        return actual && reference && actual.paintedPointCount === value.snapshot.scatter.points.length && reference.paintedPointCount === value.snapshot.scatter.references.length;
    }
    /** Forward only a real supplied choice via its existing ComboBox signal. */
    function select(name, value) {
        const selector = findItem(state.popup.contentItem, name);
        require(selector && selector.enabled, "Actual instrument selector is disabled: " + name);
        const index = selector.model.findIndex(choice => choice.value === value);
        require(index >= 0 && selector.model[index].available !== false, "Native choice is unavailable: " + name);
        selector.currentIndex = index;
        selector.activated(index);
    }
    /** Record existing request signals and their original context unchanged. */
    function observeRequest(kind, selection, context) {
        state.requestCount++;
        state.request = selection;
        state.requestContext = context;
        state.requestedAt = Date.now();
        state.requestedHeartbeat = state.heartbeatCount;
        state.requestedKind = kind;
    }
    /** Prepare an actual request's acknowledgment phase. */
    function request(action, after) {
        state.previousSequence = host.instrumentState.query_sequence;
        state.request = null;
        state.requestContext = null;
        state.requestedAction = action;
        state.nextPhase = after;
        const before = state.requestCount;
        click(state.popup.contentItem, state.popup === state.qamPopup ? "upstreamQamRefresh" : state.popup === state.spectrumPopup ? "upstreamSpectrumRefresh" : "upstreamResponseRefresh");
        require(state.requestCount === before + 1 && state.request && state.requestContext, "Actual Refresh emitted no single native request.");
        next("query");
    }
    /** Compare flat native selector fields independently of JSON object order. */
    function sameSelection(left, right) {
        const keys = Object.keys(left).sort();
        if (keys.join(",") !== Object.keys(right).sort().join(","))
            return false;
        return keys.every(key => left[key] === right[key]);
    }
    /** Check exact echoed public selection and original report/context identity. */
    function acknowledged() {
        if (!ready(state.requestedKind) || host.instrumentState.query_sequence === state.previousSequence)
            return false;
        const value = host.instrumentState;
        require(value.session_token === state.requestContext.session_token && value.configuration_generation === state.requestContext.configuration_generation && value.run === state.requestContext.run && value.run === state.run, "Instrument reply lost its original session/generation/run context.");
        require(sameSelection(value.snapshot.selection, state.request), "Native reply did not acknowledge the exact public selection.");
        return true;
    }
    /** Record original complete envelopes for independent Rust/Python comparison. */
    function record(action) {
        const kind = host.instrumentState.snapshot.kind;
        require(ready(kind), "Cannot record an unavailable native instrument.");
        const counts = visualCounts(state.popup.contentItem);
        const observation = host.instrumentState;
        require(sameSelection(state.popup.selection, observation.snapshot.selection), "Qt selector fields differ from the accepted native observation.");
        if (kind === "spectrum") {
            const plot = findItem(state.popup.contentItem, "upstreamSpectrumPlot");
            require(counts.canvases === 1 && plot.viewport === observation.snapshot.viewport && (observation.snapshot.viewport.bands || []).length === 0, "Upstream spectrum lost its original viewport or gained a DS overlay.");
        } else if (kind === "qam") {
            const actual = findItem(state.popup.contentItem, "upstreamQamActualPlot");
            require(actual.points === observation.snapshot.scatter.points && visualCounts(actual).canvases === 1, "QAM plot copied/replaced authoritative points or created point delegates.");
        }
        if (kind === "response") {
            const plot = findItem(state.popup.contentItem, "upstreamResponsePlot");
            require(counts.canvases === 1 && plot.viewport === observation.snapshot.viewport, "Response renderer lost its original backend viewport.");
            require(state.popup.inspectSelected() && state.popup.presentationCoverage.value.selected === plot.selectedPoint, "Response point inspection lost original backend details.");
        }
        const graph = kind === "qam" ? findItem(state.popup.contentItem,"upstreamQamActualPlot") : findItem(state.popup.contentItem,"saCanvas");
        require(graph && graph.visible,"Actual scientific graph is hidden.");
        const top = graph.mapToItem(state.popup.contentItem,0,0);
        const bottom = graph.mapToItem(state.popup.contentItem,graph.width,graph.height);
        require(top.x >= 0 && top.y >= 0 && bottom.x <= state.popup.contentItem.width + graphEdgeTolerancePixels && bottom.y <= state.popup.contentItem.height + graphEdgeTolerancePixels,
            "The actual scientific graph is clipped by initial metadata or controls.");
        state.snapshots = state.snapshots.concat([
            {
                tag: cases[state.caseIndex],
                action: action,
                instrument_state: JSON.parse(JSON.stringify(observation)),
                request: state.request,
                context: state.requestContext,
                visual: counts,
                query_ms: state.request ? Date.now() - state.requestedAt : null,
                heartbeat_ticks: state.request ? state.heartbeatCount - state.requestedHeartbeat : null
            }
        ]);
    }

    /** Inspect a visible original point, then prove screen navigation emits no query. */
    function inspectAndNavigate() {
        const popup = state.qamPopup;
        const actual = findItem(popup.contentItem, "upstreamQamActualPlot");
        const reference = findItem(popup.contentItem, "upstreamQamReferencePlot");
        const original = JSON.stringify(host.instrumentState);
        const requests = state.requestCount;
        click(popup.contentItem, "upstreamQamZoomIn");
        actual.pan(actual.plotEdge * panFraction, actual.plotEdge * panFraction);
        require(reference.magnification === actual.magnification && reference.panX === actual.panX && reference.panY === actual.panY, "QAM reference/actual screen transforms diverged.");
        let selected = null, position = null;
        for (const point of actual.points) {
            const pixel = actual.pointPosition(point);
            if (actual.containsPlotPosition(pixel.x, pixel.y)) {
                selected = point;
                position = pixel;
                break;
            }
        }
        require(selected && popup.inspectAt(position.x, position.y), "No actual visible point could be inspected after zoom/pan.");
        const inspected = popup.presentationCoverage.value.selected;
        const text = findItem(popup.contentItem, "upstreamQamInspection").text;
        const detail = typeof selected.detail === "string" ? selected.detail : JSON.stringify(selected.detail, null, popup.detailIndentSpaces) || "";
        require(JSON.stringify(inspected) === JSON.stringify(selected) && text === (selected.readout || "") + "\n" + detail, "Selected actual point/readout/detail was not preserved.");
        const inspection = {
            point: inspected,
            text: text,
            magnification: actual.magnification,
            panX: actual.panX,
            panY: actual.panY
        };
        click(popup.contentItem, "upstreamQamReset");
        require(actual.magnification === actual.minimumMagnification && actual.panX === 0 && actual.panY === 0 && reference.magnification === actual.magnification && state.requestCount === requests && JSON.stringify(host.instrumentState) === original, "Display-only QAM navigation changed observations or requested science.");
        state.screen = state.screen.concat([
            {
                tag: cases[state.caseIndex],
                inspection: inspection,
                reset: true,
                noQuery: true
            }
        ]);
    }

    /** Save an additional actual native popup before moving to the next consumer. */
    function saveInstrumentImage(suffix, after) {
        require(state.popup && state.popup.opened, "Native instrument is not open for its image.");
        next("saving-image");
        const destination = screenshotPath + suffix;
        require(state.popup.contentItem.grabToImage(function(result) {
            if (state.done) return;
            try {
                require(result.saveToFile(destination), "Could not save actual instrument image.");
                const images = Object.assign({},state.instrumentImages);
                images[suffix] = destination;
                state.instrumentImages = images;
                state.imageAfterPhase = after;
                console.log(suffix === ".qam.png" ? "MERLIN_UPSTREAM_QAM_CAPTURE ready" : "MERLIN_UPSTREAM_RESPONSE_CAPTURE ready");
                next("instrument-image-grace");
            } catch (error) { complete(false,String(error)); }
        }), "Actual native instrument rejected capture.");
    }

    /** Capture QML-owned spectrum content; the runner also captures actual Main. */
    function capture() {
        require(ready("spectrum") && workbench.theme.dark === true && screenshotPath.length > 0, "Actual dark spectrum evidence is unavailable.");
        next("capturing");
        console.log("MERLIN_UPSTREAM_INSTRUMENTS_CAPTURE ready");
        const started = state.popup.contentItem.grabToImage(function (result) {
            if (state.done)
                return;
            try {
                checkIndependent();
                require(ready("spectrum") && result.saveToFile(screenshotPath), "Could not save actual instrument content.");
                state.screenshotSaved = true;
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        });
        require(started, "Actual QML instrument content rejected capture.");
    }
    /** Emit exact success/error JSON once; only cancel this probe's own worker. */
    function complete(success, error) {
        if (state.done)
            return;
        state.done = true;
        timer.stop();
        if (!success && state.startRequested && state.key.length) {
            const current = host.upstreamState, shell = JSON.parse(host.backend.state());
            if (shell.upstreamIdentity === state.identity && (current.preparing || (current.progress && current.progress.run === state.run && shell.executionDirection === upstream)))
                host.upstreamBackend.stop();
        }
        const result = JSON.stringify({
            probe: "upstream-instruments",
            success: success,
            phase: state.phase,
            error: error,
            pickerTag: cases[state.caseIndex],
            snapshots: state.snapshots,
            plans: state.plans,
            preRun: state.preRun,
            screenNavigation: state.screen,
            instrumentImages: state.instrumentImages,
            tabIsolation: state.navigation,
            downstreamIdentity: state.downstreamIdentity,
            downstreamOptionsPreserved: success,
            downstreamExecutionPreserved: success,
            maximumHeartbeatGapMs: state.maximumHeartbeatGapMs,
            spectrumSharedCanvas: success,
            dark: !!workbench && !!workbench.theme && workbench.theme.dark === true,
            heartbeatScope: "GUI timer observations including native picker scheduling; no latency guarantee.",
            capturedState: state.screenshotSaved ? host.instrumentState : null,
            screenshot: state.screenshotSaved ? screenshotPath : null,
            captureTarget: "upstream-spectrum-contentItem",
            fullWindowCaptureMarker: state.screenshotSaved,
            downstreamSharedRuntime: "Separate actual retained DS QAM/spectrum run required; this probe preserves independent DS science."
        });
        console.log("MERLIN_UPSTREAM_INSTRUMENTS_PROBE " + result);
        finished(success, result);
    }

    /** Drive only two fixed real configurations and their finite actual observations. */
    function advance() {
        if (state.done)
            return;
        try {
            const now = Date.now();
            if (state.heartbeatAt)
                state.maximumHeartbeatGapMs = Math.max(state.maximumHeartbeatGapMs, now - state.heartbeatAt);
            state.heartbeatAt = now;
            state.heartbeatCount++;
            state.retries++;
            require(state.retries <= phaseRetryLimit, "Instrument probe timed out in " + state.phase + ".");
            require(now - state.startedAt <= overallTimeoutMs, "Instrument probe exceeded its overall watchdog.");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem)
                return;
            workbench.pollRun();
            host.poll();
            if (state.phase === "loading") {
                const ds = JSON.parse(workbench.runBackend.poll()), shell = JSON.parse(host.backend.state());
                if (workbench.loadingProject || !workbench.hasProject || !ds.accepted_options || !controls.contextCurrent)
                    return;
                require(rootWindow.visible && workbench.theme.dark === true && !ds.busy && !shell.applicationBusy, "Instrument probe requires actual idle dark Main with its DS example.");
                state.downstreamItem = tabs.downstreamItem;
                state.upstreamItem = tabs.upstreamItem;
                state.downstreamIdentity = shell.downstreamIdentity;
                state.downstreamOptions = JSON.stringify(ds.accepted_options);
                state.downstreamExecution = executionText(ds);
                require(host.selectDirection(upstream), "Actual upstream selection was rejected.");
                next("load-open");
                return;
            }
            const shell = checkIndependent(), current = host.upstreamState;
            if (state.phase === "load-open") {
                if (!cursorCurrent(upstream) || (state.popup && state.popup.visible))
                    return;
                require(state.upstreamItem.openSection("ucd"), "Actual UCD navigation was rejected.");
                next("load-ready");
            } else if (state.phase === "load-ready") {
                if (draftReady())
                    load();
            } else if (state.phase === "loaded") {
                if (!state.pickerReturned || current.preparing || !draftReady())
                    return;
                require(!current.error && current.configured, "The actual instrument fixture failed native configuration.");
                state.identity = shell.upstreamIdentity;
                state.key = keyText();
                state.previousRun = current.result ? current.result.run : "";
                click(ucd().footer, "upstreamDraftCancel");
                next("pre-run-open");
            } else if (state.phase === "pre-run-open") {
                if (ucd().visible)
                    return;
                openInstrument("qam");
                next("pre-run");
            } else if (state.phase === "pre-run") {
                if (!state.popup.opened || current.preparing || host.instrumentState.status === "loading")
                    return;
                const value = host.instrumentState;
                require(!current.has_result && (value.status === "error" || value.status === "unavailable") && (value.reason || (value.snapshot && value.snapshot.message)), "Pre-run observation did not remain explicitly unavailable.");
                state.preRun = state.preRun.concat([
                    {
                        tag: cases[state.caseIndex],
                        instrument_state: value
                    }
                ]);
                click(state.popup.header, "popupCloseButton");
                next("start");
            } else if (state.phase === "start") {
                if (state.popup.visible || !cursorCurrent(upstream) || !controls.canStart)
                    return;
                state.startRequested = true;
                state.run = "";
                click(controls, "directionStart");
                next("terminal");
            } else if (state.phase === "terminal") {
                if (current.progress && current.progress.run !== state.previousRun)
                    state.run = current.progress.run;
                if (current.busy || current.preparing || shell.applicationBusy)
                    return;
                require(current.has_result && current.result && current.result.run !== state.previousRun && current.result.status === "Completed", "The real instrument run did not complete.");
                state.run = current.result.run;
                state.startRequested = false;
                state.report = JSON.stringify({
                    has_result: current.has_result,
                    result: current.result
                });
                state.terminalProgress = JSON.stringify(current.progress);
                state.plans = state.plans.concat([
                    {
                        tag: cases[state.caseIndex],
                        identity: state.identity,
                        key: state.key,
                        result: current.result
                    }
                ]);
                openInstrument("qam");
                next("qam-initial");
            } else if (state.phase === "qam-initial") {
                if (!ready("qam"))
                    return;
                state.request = null;
                state.requestContext = null;
                record("qam-initial");
                select("upstreamQamMode", "raw");
                select("upstreamQamTap", "RxEqualized");
                select("upstreamQamLoading", null);
                request("qam-raw", "qam-unit");
            } else if (state.phase === "query") {
                if (!acknowledged())
                    return;
                record(state.requestedAction);
                next(state.nextPhase);
            } else if (state.phase === "qam-unit") {
                const choices = host.instrumentState.snapshot.choices;
                const tap = choices.taps.find(choice => choice.available === true && choice.unit_reference === true && choice.value === "RxEqualized");
                const loading = choices.loadings.find(choice => choice.value !== null && choice.value > 0);
                require(tap && loading, "Native retained unit-reference choices are unavailable.");
                select("upstreamQamMode", "constellation");
                select("upstreamQamTap", tap.value);
                select("upstreamQamLoading", loading.value);
                request("qam-unit-reference", "qam-pair");
            } else if (state.phase === "qam-pair") {
                require(host.instrumentState.snapshot.scatter.references.length > 0 && host.instrumentState.snapshot.scatter.points.some(point => point.series === "unit_reference"), "Actual unit-reference observations or canonical references are absent.");
                select("upstreamQamMode", "pairing");
                select("upstreamQamTap", "TxGrid");
                select("upstreamQamRightTap", "RxEqualized");
                select("upstreamQamLoading", null);
                request("qam-exact-pairing", "qam-screen");
            } else if (state.phase === "qam-screen") {
                const points = host.instrumentState.snapshot.scatter.points;
                require(points.some(point => point.series === "left") && points.some(point => point.series === "right"), "Actual exact left/right pair observations are absent.");
                inspectAndNavigate();
                if (state.caseIndex === cases.length - 1) {
                    saveInstrumentImage(".qam.png", "qam-close");
                } else { next("qam-close"); }
            } else if (state.phase === "qam-close") {
                click(state.popup.header, "popupCloseButton");
                next("spectrum-open");
            } else if (state.phase === "spectrum-open") {
                if (state.popup.visible)
                    return;
                openInstrument("spectrum");
                next("spectrum-initial");
            } else if (state.phase === "spectrum-initial") {
                if (!ready("spectrum"))
                    return;
                state.request = null;
                state.requestContext = null;
                record("spectrum-initial");
                select("upstreamSpectrumTap", "ChannelOutput");
                request("spectrum-channel-output", "spectrum-zoom");
            } else if (state.phase === "spectrum-zoom" || state.phase === "spectrum-reset") {
                const navigation = findItem(state.popup.contentItem, "chartNavigation");
                require(navigation && navigation.canZoom, "Actual spectrum ChartNavigation is unavailable.");
                state.previousSequence = host.instrumentState.query_sequence;
                state.requestedAction = state.phase;
                state.nextPhase = state.phase === "spectrum-zoom" ? "spectrum-reset" : "response-close";
                const before = state.requestCount;
                navigation.zoom(state.phase === "spectrum-zoom" ? spectrumZoomInFactor : spectrumZoomOutFactor, centeredAnchor);
                require(state.requestCount === before + 1, "Actual spectrum navigation emitted no single viewport request.");
                next("query");
            } else if (state.phase === "response-close") {
                click(state.popup.header, "popupCloseButton");
                next("response-open");
            } else if (state.phase === "response-open") {
                if (state.popup.visible) return;
                openInstrument("response");
                next("response-H");
            } else if (state.phase === "response-H") {
                if (!ready("response")) return;
                state.request = null;
                state.requestContext = null;
                record("response-H");
                select("upstreamResponseSource", "cm_pre_eq");
                request("response-C", "response-composite");
            } else if (state.phase === "response-composite") {
                select("upstreamResponseSource", "composite");
                select("upstreamResponseMetric", "phase");
                request("response-composite", "response-impulse");
            } else if (state.phase === "response-impulse") {
                select("upstreamResponseSource", "physical_channel");
                select("upstreamResponseMetric", "impulse");
                request("response-impulse", "response-received-amplitude");
            } else if (state.phase === "response-received-amplitude") {
                select("upstreamResponseMetric", "amplitude");
                select("upstreamResponseSource", "received_estimate");
                const frames = host.instrumentState.snapshot.choices.frames;
                require(frames.length > 0, "The native workbench did not retain an actual estimate frame.");
                select("upstreamResponseFrame", frames[0].value);
                request("response-received-amplitude", "response-received-phase");
            } else if (state.phase === "response-received-phase") {
                select("upstreamResponseMetric", "phase");
                request("response-received-phase", "tabs-close");
            } else if (state.phase === "tabs-close") {
                if (state.caseIndex === cases.length - 1 && !state.instrumentImages[".response.png"]) {
                    saveInstrumentImage(".response.png", "tabs-close");
                    return;
                }
                click(state.popup.header, "popupCloseButton");
                next("tabs-downstream");
            } else if (state.phase === "tabs-downstream") {
                if (state.popup.visible)
                    return;
                click(tabs, "downstreamDirectionTab");
                next("tabs-return");
            } else if (state.phase === "tabs-return") {
                if (!cursorCurrent(downstream))
                    return;
                click(tabs, "upstreamDirectionTab");
                next("tabs-upstream");
            } else if (state.phase === "tabs-upstream") {
                if (!cursorCurrent(upstream))
                    return;
                state.navigation = state.navigation.concat([
                    {
                        tag: cases[state.caseIndex],
                        key: state.key,
                        run: state.run,
                        preserved: true
                    }
                ]);
                if (state.caseIndex + 1 < cases.length) {
                    state.caseIndex++;
                    state.key = "";
                    state.report = "";
                    next("load-open");
                } else {
                    openInstrument("spectrum");
                    next("capture-ready");
                }
            } else if (state.phase === "capture-ready") {
                if (ready("spectrum") && now - state.phaseAt >= captureSettleMs) {
                    require(state.plans.length === cases.length && state.preRun.length === cases.length && state.screen.length === cases.length && state.navigation.length === cases.length, "A fixed actual run, unavailable view, screen navigation or tab case was omitted.");
                    for (const tag of cases) {
                        require(requiredActions.every(action => state.snapshots.some(value => value.tag === tag && value.action === action)), "A required real observation action was omitted: " + tag);
                    }
                    capture();
                }
            } else if (state.phase === "instrument-image-grace") {
                if (now - state.phaseAt >= captureGraceMs) next(state.imageAfterPhase);
            } else if (state.phase === "capture-grace") {
                require(ready("spectrum"), "The actual native spectrum disappeared during capture grace.");
                if (now - state.phaseAt >= captureGraceMs)
                    complete(true, "");
            } else if (state.phase !== "capturing" && state.phase !== "saving-image")
                throw new Error("Unknown instruments probe phase.");
        } catch (error) {
            complete(false, String(error));
        }
    }
    Component.onCompleted: {
        state.startedAt = Date.now();
        state.phaseAt = state.startedAt;
        timer.start();
    }
    Component.onDestruction: timer.stop()
}
