/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed actual Main consumer. Rust owns resource queries, roles and admission;
// this probe drives existing controls and records their original native results.
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

    // Finite GUI scheduling policy, independent of scientific work limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 2400
    readonly property int overallTimeoutMs: 180000
    readonly property int captureSettleMs: 100
    readonly property int captureGraceMs: 2000
    readonly property int itemTraversalLimit: 8192
    readonly property real cellCenterFraction: 0.5
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    readonly property var cases: ["grid-50", "grid-25"]
    readonly property var actions: ["resourceZoomIn", "resourceZoomOut", "resourcePanRight", "resourceFullCarriers", "frame"]
    readonly property var extentKeys: ["frame_index", "first_symbol", "symbol_count", "first_carrier", "carrier_count"]

    property QtObject state: QtObject {
        property string phase: "loading"
        property int phaseRetries: 0
        property double startedAt: 0
        property double phaseStartedAt: 0
        property double heartbeatAt: 0
        property double maximumHeartbeatGapMs: 0
        property int heartbeatCount: 0
        property bool done: false
        property bool pickerReturned: false
        property int caseIndex: 0
        property int actionIndex: 0
        property var downstreamItem: null
        property var upstreamItem: null
        property var popup: null
        property var gridView: null
        property var canvas: null
        property string downstreamIdentity: ""
        property string acceptedOptions: ""
        property string identity: ""
        property string key: ""
        property string execution: ""
        property string heldGrid: ""
        property int baselineItems: 0
        property int minimumCells: 0
        property int maximumCells: 0
        property double queryMaximumHeartbeatGapMs: 0
        property int paintedSerial: 0
        property double paintedAt: 0
        property int queryPaintSerial: 0
        property int queryCount: 0
        property int expectedQueryCount: 0
        property int queryHeartbeats: 0
        property double requestedAt: 0
        property double acknowledgedAt: 0
        property var request: null
        property var requestContext: null
        property var inspected: null
        property var inspectionContext: null
        property var snapshots: []
        property var inspections: []
        property var plans: []
        property bool screenshotSaved: false
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }
    property Connections gridSignals: Connections {
        target: state.gridView
        function onViewRequested(request, context) {
            state.queryCount += 1;
            state.request = request;
            state.requestContext = context;
            state.requestedAt = Date.now();
        }
        function onCellInspected(cell, context) {
            state.inspected = cell;
            state.inspectionContext = context;
        }
    }
    property Connections canvasSignals: Connections {
        target: state.canvas
        function onPainted() {
            state.paintedSerial += 1;
            state.paintedAt = Date.now();
        }
    }

    /** Fail at the first missing actual GUI/native invariant. */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message);
        }
    }

    /** Enter one finite asynchronous phase without blocking the GUI thread. */
    function next(phase) {
        state.phase = phase;
        state.phaseRetries = 0;
        state.phaseStartedAt = Date.now();
        console.log("MERLIN_UPSTREAM_GRID_PHASE " + cases[state.caseIndex] + " " + phase);
    }

    /** Traverse only existing public child lists, bounded by GUI policy. */
    function findItem(root, name) {
        const pending = [root];
        const visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0) {
                continue;
            }
            require(visited.length < itemTraversalLimit, "Grid probe object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name) {
                return item;
            }
            for (const key of ["children", "contentData", "data"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number") {
                    continue;
                }
                require(pending.length + children.length <= itemTraversalLimit, "Grid probe pending traversal exceeded its bound.");
                for (const child of children) {
                    pending.push(child);
                }
            }
        }
        return null;
    }

    /** Count actual visual descendants and Canvas items, without a cell delegate. */
    function visualCounts() {
        const pending = [state.gridView];
        let count = 0;
        let canvases = 0;
        while (pending.length) {
            const item = pending.pop();
            require(++count <= itemTraversalLimit, "Grid visual traversal exceeded its bound.");
            if (typeof item.getContext === "function" && typeof item.requestPaint === "function") {
                canvases += 1;
            }
            for (const child of item.children) {
                pending.push(child);
            }
        }
        return {
            items: count,
            canvases: canvases
        };
    }

    /** Click an actual enabled action, including the existing popup header X. */
    function click(root, name) {
        const action = findItem(root, name);
        require(action && action.enabled, "Actual grid action is unavailable: " + name);
        action.clicked();
    }

    /** Resolve the actual persistent configuration popup. */
    function ucdPopup() {
        return state.upstreamItem.sectionDialogs.ucd;
    }

    /** Check the existing draft's actual text admission and native correlation. */
    function draftReady() {
        const popup = ucdPopup();
        return popup.opened && popup.valuesLoaded && popup.textAdmitted && popup.matchingState && !popup.pending;
    }

    /** Preserve exact application identities without numeric conversion. */
    function keyText() {
        return JSON.stringify({
            session: host.upstreamState.session,
            generation: host.upstreamState.generation
        });
    }

    /** Snapshot only the existing native execution/source-progress presentation. */
    function executionText() {
        const current = host.upstreamState;
        return JSON.stringify({
            has_result: current.has_result,
            result: current.result,
            progress: current.progress
        });
    }

    /** Read the real shell direction and admitted shared-control context. */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction);
    }

    /** Select the existing real direction tab. */
    function switchDirection(direction) {
        click(tabs, direction + "DirectionTab");
    }

    /** Retain independent DS state and prohibit any scientific execution. */
    function checkIndependent() {
        const shell = JSON.parse(host.backend.state());
        const current = JSON.parse(workbench.runBackend.poll());
        require(shell.downstreamIdentity === state.downstreamIdentity && JSON.stringify(current.accepted_options) === state.acceptedOptions && !current.busy, "Grid observation changed or executed the independent downstream owner.");
        require(tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem, "Grid navigation recreated a persistent Main workbench.");
        require(!shell.applicationBusy, "A passive grid operation acquired the scientific execution slot.");
        if (state.key.length) {
            require(shell.upstreamIdentity === state.identity && keyText() === state.key && executionText() === state.execution, "Grid inspection changed accepted configuration, run results or source progress.");
        }
        return shell;
    }

    /** Wait for exact current-context native DTO admission before drawing. */
    function gridAdmitted() {
        const view = state.gridView;
        return state.popup && state.popup.opened && view && view.displayable && view.actionsEnabled && !view.presentationRejected && view.state === host.resourceGridState;
    }

    /** Wait separately for completed Canvas drawing of the admitted cells. */
    function gridReady() {
        return gridAdmitted() && state.canvas && state.canvas.paintedCellCount === state.gridView.grid.cells.length;
    }

    /** Check every unchanged public viewport field and its exact identity envelope. */
    function requestAcknowledged() {
        if (!gridAdmitted() || !state.request || !state.requestContext) {
            return false;
        }
        const view = state.gridView;
        const context = state.requestContext;
        require(view.sameIdentity(context.session_token, view.state.session_token) && view.sameIdentity(context.configuration_generation, view.state.configuration_generation), "Grid response belongs to a different request context.");
        for (const key of extentKeys) {
            if (view.grid[key] !== state.request[key]) {
                return false;
            }
        }
        require(view.state.max_cells === state.request.max_cells, "Grid response changed the supplied cell budget.");
        return true;
    }

    /** Record original DTOs and observed timings; no latency threshold is imposed. */
    function record(tag) {
        require(gridReady(), "Cannot record an unavailable or unpainted actual grid.");
        const view = state.gridView;
        const counts = visualCounts();
        require(counts.canvases === 1, "Resource cells were not rendered by one Canvas.");
        if (!state.baselineItems) {
            state.baselineItems = counts.items;
        }
        require(counts.items === state.baselineItems, "Visual object count changed with the bounded cell population.");
        const metadata = findItem(view, "resourceGridMetadata");
        require(metadata && metadata.visible && metadata.text.length > 0, "Actual original resource metadata is missing.");
        const cellCount = view.grid.cells.length;
        state.minimumCells = state.minimumCells ? Math.min(state.minimumCells, cellCount) : cellCount;
        state.maximumCells = Math.max(state.maximumCells, cellCount);
        const observation = {
            tag: cases[state.caseIndex],
            action: tag,
            resource_grid_state: JSON.parse(JSON.stringify(host.resourceGridState)),
            request: state.request,
            context: state.requestContext,
            painted_cells: state.canvas.paintedCellCount,
            visual_items: counts.items,
            canvas_count: counts.canvases,
            metadata: metadata.text,
            query_ms: state.request ? state.acknowledgedAt - state.requestedAt : null,
            render_ms: state.request ? state.paintedAt - state.requestedAt : null,
            heartbeat_ticks: state.request ? state.heartbeatCount - state.queryHeartbeats : null,
            maximum_query_heartbeat_gap_ms: state.request ? state.queryMaximumHeartbeatGapMs : null
        };
        state.snapshots = state.snapshots.concat([observation]);
    }

    /** Dispatch only existing real toolbar/frame-selector interaction paths. */
    function requestAction() {
        require(gridReady(), "Actual grid controls are not ready.");
        const action = actions[state.actionIndex];
        state.queryPaintSerial = state.paintedSerial;
        state.queryHeartbeats = state.heartbeatCount;
        state.queryMaximumHeartbeatGapMs = 0;
        state.expectedQueryCount = state.queryCount + 1;
        state.acknowledgedAt = 0;
        state.request = null;
        state.requestContext = null;
        if (action === "frame") {
            const frame = findItem(state.gridView, "resourceFrameChoice");
            require(frame && frame.enabled && frame.to > frame.from, "The fixed actual grid fixture has no second frame.");
            frame.value = frame.value === frame.from ? frame.to : frame.from;
            frame.valueModified();
        } else {
            click(state.gridView, action);
        }
        require(state.queryCount === state.expectedQueryCount && !!state.request, "The actual grid action emitted no single bounded viewport request.");
        require(!state.gridView.actionsEnabled || requestAcknowledged(), "Pending grid request enabled stale actions before exact native acknowledgement.");
        next("query");
    }

    /** Inspect an actual rendered original cell through the existing view operation. */
    function inspect() {
        const view = state.gridView;
        const cell = view.grid.cells[0];
        const rectangle = view.cellRectangle(cell);
        state.inspected = null;
        state.inspectionContext = null;
        view.inspectAt(rectangle.x + rectangle.width * cellCenterFraction, rectangle.y + rectangle.height * cellCenterFraction);
        require(state.inspected && JSON.stringify(state.inspected) === JSON.stringify(cell) && view.sameIdentity(state.inspectionContext.session_token, view.state.session_token) && view.sameIdentity(state.inspectionContext.configuration_generation, view.state.configuration_generation), "Actual cell inspection lost original metadata or request identity.");
        const text = findItem(view, "resourceCellInspection");
        require(text && JSON.stringify(JSON.parse(text.text)) === JSON.stringify(cell), "Visible inspection did not preserve the original cell and null values.");
        state.inspections = state.inspections.concat([
            {
                tag: cases[state.caseIndex],
                cell: state.inspected,
                context: state.inspectionContext,
                text: text.text
            }
        ]);
    }

    /** Announce one fixed real picker vector before clicking actual Load. */
    function loadConfiguration() {
        const popup = ucdPopup();
        const action = findItem(popup.footer, "upstreamConfigurationLoad");
        require(draftReady() && action && action.enabled, "Actual configuration Load is unavailable.");
        state.pickerReturned = false;
        next("loaded");
        console.log("MERLIN_UPSTREAM_GRID_PICKER " + cases[state.caseIndex]);
        Qt.callLater(function () {
            if (state.done) {
                return;
            }
            try {
                require(action.enabled, "Actual Load became disabled before the native picker.");
                action.clicked();
                state.pickerReturned = true;
            } catch (error) {
                complete(false, String(error));
            }
        });
    }

    /** Capture QML-owned actual grid content; allow runner full-window evidence. */
    function capture() {
        require(gridReady() && workbench.theme.dark === true && screenshotPath.length > 0, "Fresh actual dark grid evidence is unavailable.");
        next("capturing");
        console.log("MERLIN_UPSTREAM_GRID_CAPTURE ready");
        const started = state.popup.contentItem.grabToImage(function (result) {
            if (state.done) {
                return;
            }
            try {
                checkIndependent();
                require(gridReady() && result.saveToFile(screenshotPath), "Could not save actual QML-owned grid content.");
                state.screenshotSaved = true;
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        });
        require(started, "The actual QML grid content rejected image capture.");
    }

    /** Emit bounded evidence once; this passive probe never starts or stops science. */
    function complete(success, message) {
        if (state.done) {
            return;
        }
        state.done = true;
        phaseTimer.stop();
        finished(success, JSON.stringify({
            probe: "upstream-resource-grid",
            success: success,
            phase: state.phase,
            error: message,
            pickerTag: cases[state.caseIndex],
            snapshots: state.snapshots,
            captured_state: state.screenshotSaved ? host.resourceGridState : null,
            inspections: state.inspections,
            plans: state.plans,
            downstreamIdentity: state.downstreamIdentity,
            downstreamAcceptedOptionsPreserved: success,
            noExecutionObserved: success,
            maximumHeartbeatGapMs: state.maximumHeartbeatGapMs,
            screenshot: state.screenshotSaved ? screenshotPath : null,
            captureTarget: "resource-grid-contentItem",
            fullWindowCaptureMarker: state.screenshotSaved,
            dark: !!workbench && !!workbench.theme && workbench.theme.dark === true
        }));
    }

    /** Drive two fixed actual configurations with finite phase/overall watchdogs. */
    function advance() {
        if (state.done) {
            return;
        }
        try {
            const now = Date.now();
            if (state.heartbeatAt) {
                const gap = now - state.heartbeatAt;
                state.maximumHeartbeatGapMs = Math.max(state.maximumHeartbeatGapMs, gap);
                if (state.phase === "query") {
                    state.queryMaximumHeartbeatGapMs = Math.max(state.queryMaximumHeartbeatGapMs, gap);
                }
            }
            state.heartbeatAt = now;
            state.heartbeatCount += 1;
            state.phaseRetries += 1;
            require(state.phaseRetries <= phaseRetryLimit, "Grid probe timed out in " + state.phase + ".");
            require(now - state.startedAt <= overallTimeoutMs, "Grid probe exceeded its overall watchdog.");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem) {
                return;
            }
            workbench.pollRun();
            host.poll();
            if (state.phase === "loading") {
                const ds = JSON.parse(workbench.runBackend.poll());
                if (workbench.loadingProject || !workbench.hasProject || !ds.accepted_options || !controls.contextCurrent) {
                    return;
                }
                const shell = JSON.parse(host.backend.state());
                require(rootWindow.visible && workbench.theme.dark === true && !ds.busy && !shell.applicationBusy && !host.upstreamState.busy, "Grid probe requires real visible idle dark Main with its downstream example.");
                state.downstreamItem = tabs.downstreamItem;
                state.upstreamItem = tabs.upstreamItem;
                state.downstreamIdentity = shell.downstreamIdentity;
                state.acceptedOptions = JSON.stringify(ds.accepted_options);
                require(host.selectDirection(upstream), "Actual upstream selection was rejected.");
                next("load-open");
                return;
            }
            const shell = checkIndependent();
            const current = host.upstreamState;
            if (state.phase === "load-open") {
                if (!cursorCurrent(upstream) || (state.popup && state.popup.visible)) {
                    return;
                }
                require(state.upstreamItem.openSection("ucd"), "Actual UCD navigation was rejected.");
                next("load-ready");
            } else if (state.phase === "load-ready") {
                if (draftReady()) {
                    loadConfiguration();
                }
            } else if (state.phase === "loaded") {
                if (!state.pickerReturned || current.preparing || !draftReady()) {
                    return;
                }
                require(!current.error && current.configured && typeof current.session === "string" && typeof current.generation === "string", "The actual fixed grid configuration was not accepted.");
                state.identity = shell.upstreamIdentity;
                state.key = keyText();
                state.execution = executionText();
                state.request = null;
                state.requestContext = null;
                state.actionIndex = 0;
                state.minimumCells = 0;
                state.maximumCells = 0;
                click(ucdPopup().footer, "upstreamDraftCancel");
                next("grid-open");
            } else if (state.phase === "grid-open") {
                if (ucdPopup().visible || !cursorCurrent(upstream)) {
                    return;
                }
                require(typeof state.upstreamItem.openResourceGrid === "function", "Actual resource-grid navigation is unavailable.");
                state.upstreamItem.openResourceGrid();
                state.popup = findItem(state.upstreamItem, "upstreamResourceGridDialog");
                require(state.popup && state.popup.gridView, "Actual resource-grid popup/view was not found.");
                state.gridView = state.popup.gridView;
                state.canvas = findItem(state.gridView, "resourceGridCanvas");
                require(!!state.canvas, "Actual resource Canvas was not found.");
                next("initial");
            } else if (state.phase === "initial") {
                if (!gridReady()) {
                    return;
                }
                record("initial");
                requestAction();
            } else if (state.phase === "query") {
                if (!requestAcknowledged()) {
                    return;
                }
                if (!state.acknowledgedAt) {
                    state.acknowledgedAt = now;
                }
                if (!gridReady() || state.paintedSerial <= state.queryPaintSerial) {
                    return;
                }
                record(actions[state.actionIndex]);
                state.actionIndex += 1;
                if (state.actionIndex < actions.length) {
                    requestAction();
                } else {
                    inspect();
                    state.heldGrid = JSON.stringify(host.resourceGridState);
                    click(state.popup.header, "popupCloseButton");
                    next("grid-closed");
                }
            } else if (state.phase === "grid-closed") {
                if (state.popup.visible) {
                    return;
                }
                switchDirection(downstream);
                next("downstream");
            } else if (state.phase === "downstream") {
                if (!cursorCurrent(downstream)) {
                    return;
                }
                require(JSON.stringify(host.resourceGridState) === state.heldGrid, "Downstream tab changed retained grid evidence.");
                switchDirection(upstream);
                next("upstream");
            } else if (state.phase === "upstream") {
                if (!cursorCurrent(upstream)) {
                    return;
                }
                require(JSON.stringify(host.resourceGridState) === state.heldGrid, "Upstream tab return changed retained grid evidence.");
                require(state.maximumCells > state.minimumCells, "Actual viewport controls did not exercise increasing bounded cell populations.");
                state.plans = state.plans.concat([
                    {
                        tag: cases[state.caseIndex],
                        identity: state.identity,
                        key: state.key,
                        executionBefore: state.execution,
                        executionAfter: executionText(),
                        minimumCells: state.minimumCells,
                        maximumCells: state.maximumCells,
                        visualItems: state.baselineItems,
                        tabIsolation: true
                    }
                ]);
                if (state.caseIndex + 1 < cases.length) {
                    state.caseIndex += 1;
                    state.key = "";
                    next("load-open");
                } else {
                    state.upstreamItem.openResourceGrid();
                    next("capture-ready");
                }
            } else if (state.phase === "capture-ready") {
                if (gridReady() && now - state.phaseStartedAt >= captureSettleMs) {
                    require(state.plans.length === cases.length && state.inspections.length === cases.length, "A fixed native grid configuration or inspection was omitted.");
                    capture();
                }
            } else if (state.phase === "capture-grace") {
                require(gridReady(), "Actual grid became unavailable during screenshot grace.");
                if (now - state.phaseStartedAt >= captureGraceMs) {
                    complete(true, "");
                }
            } else if (state.phase !== "capturing") {
                throw new Error("Unknown upstream grid probe phase.");
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
