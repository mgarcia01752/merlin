/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/presentation"

// Draw only accepted ResourceGridSnapshot cells. The host owns all queries,
// scientific admission and lossless identity/timing transport.
ColumnLayout {
    id: view
    required property var theme
    required property var host
    signal viewRequested(var request, var context)
    signal grantRequested(var grantId, var context)
    signal cellInspected(var cell, var context)
    readonly property var incomingState: host && host.resourceGridState ? host.resourceGridState : ({})
    readonly property var state: presentationOwner.value ? presentationOwner.value.state : ({})
    readonly property bool presentationRejected: presentationOwner.rejected
    readonly property bool actionsEnabled: displayable && presentationOwner.textAdmitted && !presentationOwner.rejected && state === incomingState
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentationOwner
        theme: view.theme
    }
    property Binding frameTextBinding: Binding {
        target: frameChoice.contentItem
        property: "text"
        value: presentationOwner.textAdmitted ? frameChoice.displayText : ""
    }
    property bool presentationReady: false
    Component.onCompleted: {
        presentationReady = true
        receiveState()
    }
    Component.onDestruction: {
        presentationReady = false
        if (presentationOwner) {
            presentationOwner.clear()
        }
    }
    readonly property var snapshot: state.snapshot || null
    readonly property var grid: snapshot && snapshot.grid ? snapshot.grid : null
    readonly property string exportFormat: "merlin.upstream.resource_grid"
    readonly property int exportSchemaVersion: 1
    readonly property real sectionSpacing: 12
    readonly property real toolbarSpacing: 8
    readonly property real grantChoiceWidth: 220
    readonly property real chartHeight: 420
    readonly property real legendMarkerSize: 12
    readonly property real leftAxisMargin: 76
    readonly property real rightAxisMargin: 24
    readonly property real topAxisMargin: 24
    readonly property real bottomAxisMargin: 56
    readonly property real axisFontPixels: 12
    readonly property real cellDividerPixels: 6
    readonly property real selectionBorderPixels: 2
    readonly property real pointerDragThresholdPixels: 6
    readonly property real zoomInFactor: 0.5
    readonly property real zoomOutFactor: 2
    // Viewport-center anchor is independent of the equal-valued zoom-in scale.
    readonly property real zoomAnchorFraction: 0.5
    readonly property real axisLabelInset: 16
    readonly property real inspectionMinimumHeight: 100
    readonly property int metadataFontPixels: 12
    readonly property int inspectionJsonIndentSpaces: 2
    readonly property var roles: presentationOwner.value ? presentationOwner.value.roles : []
    function roleDefinitions() {
        return [
        {
            key: "data",
            label: "Data"
        },
        {
            key: "complementary",
            label: "Complementary"
        },
        {
            key: "known_pilot",
            label: "Known pilot"
        },
        {
            key: "zero_loaded",
            label: "Zero loaded"
        },
        {
            key: "excluded",
            label: "Excluded"
        },
        {
            key: "unallocated",
            label: "Unallocated"
        }
    ]
    }

    // Resolve the palette once per theme change, rather than once per drawn cell.
    readonly property var roleColors: ({
            data: theme.qamData,
            complementary: theme.txScatteredPilot,
            known_pilot: theme.txPlc,
            zero_loaded: theme.statusAttention,
            excluded: theme.statusError,
            unallocated: theme.line
        })
    readonly property var pendingWindow: presentationOwner.value ? presentationOwner.value.pendingWindow : null
    readonly property var pendingWindowContext: presentationOwner.value ? presentationOwner.value.pendingWindowContext : null
    readonly property var pendingGrant: presentationOwner.value ? presentationOwner.value.pendingGrant : null
    readonly property var inspectedCell: presentationOwner.value ? presentationOwner.value.inspectedCell : null
    readonly property var inspectionContext: presentationOwner.value ? presentationOwner.value.inspectionContext : null
    readonly property var hoveredCell: presentationOwner.value ? presentationOwner.value.hoveredCell : null
    readonly property bool contextMatches: snapshot !== null && sameIdentity(state.session_token, state.snapshot_session_token) && sameIdentity(state.configuration_generation, snapshot.configuration_generation)
    readonly property bool supportedExport: snapshot !== null && snapshot.format === exportFormat && snapshot.schema_version === exportSchemaVersion
    readonly property bool withinDrawingBudget: grid !== null && state.max_cells > 0 && (grid.cells || []).length <= state.max_cells
    readonly property bool pending: pendingWindow !== null || pendingGrant !== null || state.status === "loading"
    readonly property bool displayable: state.status === "ready" && contextMatches && supportedExport && withinDrawingBudget && !pending && grid.symbol_count > 0 && grid.carrier_count > 0 && (grid.cells || []).length > 0
    readonly property var grantChoices: presentationOwner.value ? presentationOwner.value.grantChoices : []
    readonly property string unavailableText: {
        if (presentationOwner.rejected) {
            return presentationOwner.message;
        }
        if (pending) {
            return state.reason || "Waiting for an accepted resource view.";
        }
        if (state.status !== "ready" || !snapshot) {
            return state.reason || "Resource grid unavailable.";
        }
        if (!contextMatches) {
            return "Resource grid is out of date for this session or configuration.";
        }
        if (!supportedExport) {
            return "Resource-grid export format is unavailable.";
        }
        if (!withinDrawingBudget) {
            return "Resource view exceeds the supplied drawing budget or has no supplied budget.";
        }
        return "No resource cells are available.";
    }
    readonly property string inspectionPrompt: "Select a reported cell to inspect its original metadata."
    readonly property string inspectedText: displayable && inspectedCell && inspectionContext && sameIdentity(inspectionContext.session_token, state.session_token) && sameIdentity(inspectionContext.configuration_generation, state.configuration_generation) ? presentationOwner.value.inspectedText : inspectionPrompt
    spacing: sectionSpacing

    /**
     * Compare application identities without numeric conversion.
     * @param left First lossless token or display scalar.
     * @param right Second lossless token or display scalar.
     * @returns Whether both values exist and have the same exact string form.
     */
    function sameIdentity(left, right) {
        return left !== undefined && left !== null && right !== undefined && right !== null && String(left) === String(right);
    }

    /**
     * Capture the current application owner without modifying scientific state.
     * @returns Session and configuration tokens for query/inspection routing.
     */
    function currentContext() {
        return {
            session_token: state.session_token,
            configuration_generation: state.configuration_generation
        };
    }

    /**
     * Map a reported role to the theme without deriving allocation or loading.
     * @param role Original ResourceGridRole string.
     * @returns Its presentation color, or the muted color for an unknown token.
     */
    function roleColor(role) {
        return roleColors[role] || theme.muted;
    }

    /**
     * Map an original cell to a screen rectangle without compacting physical holes.
     * @param cell Unchanged reported ResourceGridCell.
     * @returns Pixel rectangle, or an empty rectangle when no region exists.
     */
    function cellRectangle(cell) {
        if (!grid) {
            return Qt.rect(0, 0, 0, 0);
        }
        return Qt.rect(leftAxisMargin + (cell.frame_symbol - grid.first_symbol) * canvas.cellWidth, topAxisMargin + (grid.first_carrier + grid.carrier_count - 1 - cell.physical_carrier) * canvas.cellHeight, canvas.cellWidth, canvas.cellHeight);
    }

    /**
     * Find an original reported cell under a screen coordinate.
     * @param x Horizontal Canvas position in pixels.
     * @param y Vertical Canvas position in pixels.
     * @returns Original cell or null outside a currently owned rendered region.
     */
    function cellAt(x, y) {
        if (!actionsEnabled || x < leftAxisMargin || x >= canvas.width - rightAxisMargin || y < topAxisMargin || y >= canvas.height - bottomAxisMargin) {
            return null;
        }
        const symbol = grid.first_symbol + Math.floor((x - leftAxisMargin) / canvas.cellWidth);
        const carrier = grid.first_carrier + grid.carrier_count - 1 - Math.floor((y - topAxisMargin) / canvas.cellHeight);
        for (const cell of grid.cells) {
            if (cell.frame_symbol === symbol && cell.physical_carrier === carrier) {
                return cell;
            }
        }
        return null;
    }

    /**
     * Select an owned original cell and emit its unchanged metadata and context.
     * @param x Horizontal Canvas position in pixels.
     * @param y Vertical Canvas position in pixels.
     * Missing, stale and pending views produce no inspection event.
     */
    function inspectAt(x, y) {
        const cell = cellAt(x, y);
        if (!cell) {
            return;
        }
        const context = currentContext();
        if (!replaceState(state, {inspectedCell: cell, inspectionContext: context})) {
            return;
        }
        cellInspected(cell, context);
        canvas.requestPaint();
    }

    /**
     * Request a bounded original-coordinate viewport; the host performs the API query.
     * @param firstSymbol Requested original frame-symbol origin.
     * @param symbolCount Requested horizontal cell count.
     * @param firstCarrier Requested original physical-carrier origin.
     * @param carrierCount Requested vertical cell count.
     * @param frameIndex Relative frame inside the reported grant bounds.
     * Counts/origins are clipped only for viewport geometry and the supplied work
     * budget. The old grid becomes unavailable until a matching owned view arrives.
     * @returns Whether presentation admission allowed the request to be emitted.
     */
    function requestWindow(firstSymbol, symbolCount, firstCarrier, carrierCount, frameIndex) {
        if (!actionsEnabled) {
            return false;
        }
        const carriers = Math.max(1, Math.min(grid.fft_samples, Math.round(carrierCount), state.max_cells));
        const symbols = Math.max(1, Math.min(grid.frame_symbols, Math.round(symbolCount), Math.floor(state.max_cells / carriers)));
        const request = {
            frame_index: Math.max(0, Math.min(grid.grant_frames - 1, Math.round(frameIndex))),
            first_symbol: Math.max(0, Math.min(grid.frame_symbols - symbols, Math.round(firstSymbol))),
            symbol_count: symbols,
            first_carrier: Math.max(0, Math.min(grid.fft_samples - carriers, Math.round(firstCarrier))),
            carrier_count: carriers,
            max_cells: state.max_cells
        };
        const context = currentContext();
        if (!replaceState(state, {inspectedCell: null, inspectionContext: null, hoveredCell: null,
                pendingWindow: request, pendingWindowContext: context})) {
            return false;
        }
        viewRequested(request, context);
        return true;
    }

    /**
     * Request zoom around a screen anchor without synthesizing resource metadata.
     * @param factor Display extent multiplier.
     * @param horizontalAnchor Horizontal fraction measured from the plot left.
     * @param verticalAnchor Physical-carrier fraction measured from the plot bottom.
     * Pending or unavailable views emit no request.
     */
    function zoom(factor, horizontalAnchor, verticalAnchor) {
        if (!actionsEnabled) {
            return;
        }
        const symbols = Math.max(1, Math.round(grid.symbol_count * factor));
        const carriers = Math.max(1, Math.round(grid.carrier_count * factor));
        const symbolOrigin = grid.first_symbol + horizontalAnchor * grid.symbol_count;
        const carrierOrigin = grid.first_carrier + verticalAnchor * grid.carrier_count;
        requestWindow(symbolOrigin - horizontalAnchor * symbols, symbols, carrierOrigin - verticalAnchor * carriers, carriers, grid.frame_index);
    }

    /**
     * Request original-coordinate panning without relabelling existing cells.
     * @param symbolOffset Horizontal cell displacement.
     * @param carrierOffset Vertical physical-carrier displacement.
     * Pending or unavailable views emit no request.
     */
    function pan(symbolOffset, carrierOffset) {
        if (actionsEnabled) {
            requestWindow(grid.first_symbol + symbolOffset, grid.symbol_count, grid.first_carrier + carrierOffset, grid.carrier_count, grid.frame_index);
        }
    }

    /**
     * Request the full reported physical-carrier span within the supplied budget.
     * The host resolves a fresh view; full-plan capacity is never recalculated.
     */
    function fullCarriers() {
        if (actionsEnabled) {
            requestWindow(0, grid.frame_symbols, 0, grid.fft_samples, grid.frame_index);
        }
    }

    /**
     * Request a real host-configured grant session and freeze scientific actions.
     * @param choice Host inventory entry with grant ID and opaque target session.
     * Selection never edits a configuration or invents an API grant parameter.
     */
    function selectGrant(choice) {
        if (!actionsEnabled || !choice || sameIdentity(choice.session_token, state.session_token)) {
            return;
        }
        const nextGrant = {grant_id: choice.grant_id, session_token: choice.session_token};
        if (!replaceState(state, {pendingGrant: nextGrant, inspectedCell: null,
                inspectionContext: null, hoveredCell: null})) {
            return;
        }
        const context = currentContext();
        context.target_session_token = choice.session_token;
        grantRequested(choice.grant_id, context);
    }

    /**
     * Adopt an owned host update and discard stale inspection state.
     * Matching original extents acknowledge a pending window; the selected token
     * and reported grant ID acknowledge a grant change. Errors clear pending UI.
     */
    function receiveState() {
        const nextState = incomingState;
        // QML may notify an unchanged var binding when its containing host changes.
        // Preserve local inspection and pending context for the same admitted view.
        if (nextState === state && !presentationOwner.rejected) {
            return true;
        }
        const nextSnapshot = nextState.snapshot || null;
        const nextGrid = nextSnapshot && nextSnapshot.grid ? nextSnapshot.grid : null;
        let nextWindow = pendingWindow, nextWindowContext = pendingWindowContext, nextGrant = pendingGrant;
        if (nextState.status === "error" || nextState.status === "unavailable") {
            nextWindow = null;
            nextWindowContext = null;
            nextGrant = null;
        } else if (nextState.status === "ready" && nextSnapshot && nextGrid
                && sameIdentity(nextState.session_token, nextState.snapshot_session_token)
                && sameIdentity(nextState.configuration_generation, nextSnapshot.configuration_generation)
                && nextSnapshot.format === exportFormat && nextSnapshot.schema_version === exportSchemaVersion) {
            if (nextGrant && sameIdentity(nextGrant.session_token, nextState.session_token) && sameIdentity(nextGrant.grant_id, nextGrid.grant_id)) {
                nextGrant = null;
            }
            if (nextWindow && nextWindowContext && sameIdentity(nextWindowContext.session_token, nextState.session_token)
                    && sameIdentity(nextWindowContext.configuration_generation, nextState.configuration_generation)
                    && nextGrid.frame_index === nextWindow.frame_index && nextGrid.first_symbol === nextWindow.first_symbol
                    && nextGrid.symbol_count === nextWindow.symbol_count && nextGrid.first_carrier === nextWindow.first_carrier
                    && nextGrid.carrier_count === nextWindow.carrier_count) {
                nextWindow = null;
                nextWindowContext = null;
            }
        }
        const accepted = replaceState(nextState, {inspectedCell: null, inspectionContext: null, hoveredCell: null,
            pendingWindow: nextWindow, pendingWindowContext: nextWindowContext, pendingGrant: nextGrant});
        canvas.requestPaint();
        return accepted;
    }

    /**
     * Format original metadata without numerical conversion or missing-value zeros.
     * @param value Original scalar, including lossless strings for wide integers.
     * @returns Exact string form, or an explicit unavailable marker.
     */
    function scalar(value) {
        return value === null || value === undefined ? "Unavailable" : String(value);
    }

    /**
     * Admit the existing DTO, interaction state and independent display strings.
     * @param nextState Unchanged host resource-grid presentation DTO.
     * @param changes Only the coupled local request/inspection fields being changed.
     * @returns False preserves the entire previous view and emits no action.
     */
    function replaceState(nextState, changes) {
        if (!presentationReady || !presentationOwner.reserveTextFields([frameChoice.contentItem])) {
            return false;
        }
        const local = Object.assign({pendingWindow: pendingWindow, pendingWindowContext: pendingWindowContext,
            pendingGrant: pendingGrant, inspectedCell: inspectedCell, inspectionContext: inspectionContext,
            hoveredCell: hoveredCell}, changes);
        const nextSnapshot = nextState.snapshot || null;
        const nextGrid = nextSnapshot && nextSnapshot.grid ? nextSnapshot.grid : null;
        const choices = nextState.grants || (nextGrid ? [{grant_id: nextGrid.grant_id,
            label: "Grant " + String(nextGrid.grant_id), session_token: nextState.session_token}] : []);
        const roleRows = roleDefinitions().map(role => Object.assign({}, role, {
            availableText: role.label + ": " + scalar(nextGrid ? (nextGrid.counts || {})[role.key] : null),
            unavailableText: role.label + ": Unavailable"}));
        return presentationOwner.replace(Object.assign({state: nextState, grantChoices: choices, roles: roleRows,
            metadataText: metadataText(nextSnapshot, nextGrid),
            inspectedText: local.inspectedCell ? JSON.stringify(local.inspectedCell, null, inspectionJsonIndentSpaces) : inspectionPrompt,
            hoverText: local.hoveredCell ? "Symbol " + local.hoveredCell.frame_symbol + ", physical carrier " + local.hoveredCell.physical_carrier + ": " + local.hoveredCell.role : ""}, local));
    }

    /** Return the existing metadata label from original reported values only. */
    function metadataText(nextSnapshot, nextGrid) {
        if (!nextGrid) {
            return "";
        }
        return "Generation " + scalar(nextSnapshot.configuration_generation) + " · Grant " + scalar(nextGrid.grant_id) + " · Epoch sample " + scalar(nextGrid.grant_epoch_sample) + " · First frame " + scalar(nextGrid.grant_first_frame) + " · Grant frames " + scalar(nextGrid.grant_frames) + "\nProfile " + scalar(nextGrid.profile_id) + " · IUC " + scalar(nextGrid.iuc) + " · Relative frame " + scalar(nextGrid.frame_index) + " · Absolute frame " + scalar(nextGrid.absolute_frame) + " · Frame prefix sample " + scalar(nextGrid.frame_prefix_sample) + "\nFFT samples " + scalar(nextGrid.fft_samples) + " · Sample rate Hz " + scalar(nextGrid.sample_rate_hz) + " · Spacing Hz " + scalar(nextGrid.spacing_hz) + " · Prefix samples " + scalar(nextGrid.prefix_samples) + " · Roll-off samples " + scalar(nextGrid.rolloff_samples) + " · Stride samples " + scalar(nextGrid.symbol_stride_samples) + "\nWindow first symbol " + scalar(nextGrid.first_symbol) + " / count " + scalar(nextGrid.symbol_count) + " · First physical carrier " + scalar(nextGrid.first_carrier) + " / count " + scalar(nextGrid.carrier_count) + "\nFull-frame coded bits " + scalar(nextGrid.frame_bits) + " · Full-grant coded bits " + scalar(nextGrid.grant_bits) + " · Full-frame visits " + scalar(nextGrid.full_frame_visits) + " · Transmission planned " + scalar(nextSnapshot.transmission_planned) + "\nReference plane " + scalar(nextGrid.reference_plane);
    }

    /** Retain a hover alias and its derived tooltip only after admission. */
    function setHoveredCell(cell) {
        return replaceState(state, {hoveredCell: cell});
    }

    onIncomingStateChanged: receiveState()
    onDisplayableChanged: canvas.requestPaint()
    onInspectedTextChanged: canvas.requestPaint()

    Flow {
        Layout.fillWidth: true
        spacing: view.toolbarSpacing
        WorkbenchActionButton {
            objectName: "resourceZoomIn"
            theme: view.theme
            text: "Zoom In"
            enabled: view.actionsEnabled
            onClicked: view.zoom(view.zoomInFactor, view.zoomAnchorFraction, view.zoomAnchorFraction)
        }
        WorkbenchActionButton {
            objectName: "resourceZoomOut"
            theme: view.theme
            text: "Zoom Out"
            enabled: view.actionsEnabled
            onClicked: view.zoom(view.zoomOutFactor, view.zoomAnchorFraction, view.zoomAnchorFraction)
        }
        WorkbenchActionButton {
            objectName: "resourceFullCarriers"
            theme: view.theme
            text: "Full Carriers"
            enabled: view.actionsEnabled
            onClicked: view.fullCarriers()
        }
        WorkbenchActionButton {
            objectName: "resourcePanLeft"
            theme: view.theme
            text: "←"
            compact: true
            enabled: view.actionsEnabled
            Accessible.name: "Previous symbols"
            onClicked: view.pan(-view.grid.symbol_count, 0)
        }
        WorkbenchActionButton {
            objectName: "resourcePanRight"
            theme: view.theme
            text: "→"
            compact: true
            enabled: view.actionsEnabled
            Accessible.name: "Next symbols"
            onClicked: view.pan(view.grid.symbol_count, 0)
        }
        WorkbenchActionButton {
            objectName: "resourcePanUp"
            theme: view.theme
            text: "↑"
            compact: true
            enabled: view.actionsEnabled
            Accessible.name: "Higher physical carriers"
            onClicked: view.pan(0, view.grid.carrier_count)
        }
        WorkbenchActionButton {
            objectName: "resourcePanDown"
            theme: view.theme
            text: "↓"
            compact: true
            enabled: view.actionsEnabled
            Accessible.name: "Lower physical carriers"
            onClicked: view.pan(0, -view.grid.carrier_count)
        }
        ComboBox {
            objectName: "resourceGrantChoice"
            width: view.grantChoiceWidth
            model: view.grantChoices
            textRole: "label"
            currentIndex: view.grantChoices.findIndex(choice => view.sameIdentity(choice.session_token, view.state.session_token))
            enabled: view.actionsEnabled && count > 1
            Accessible.name: "Configured grant session"
            onActivated: function (index) {
                view.selectGrant(view.grantChoices[index]);
            }
        }
        Label {
            text: "Frame"
            color: view.theme.text
            height: frameChoice.height
            verticalAlignment: Text.AlignVCenter
        }
        SpinBox {
            id: frameChoice
            objectName: "resourceFrameChoice"
            from: 0
            to: view.grid ? view.grid.grant_frames - 1 : 0
            value: view.grid ? view.grid.frame_index : 0
            enabled: view.actionsEnabled
            Accessible.name: "Relative frame index"
            onValueModified: {
                if (!view.requestWindow(view.grid.first_symbol, view.grid.symbol_count, view.grid.first_carrier, view.grid.carrier_count, value)) {
                    // Restore the binding too, so later accepted snapshots still update it.
                    value = Qt.binding(function() { return view.grid ? view.grid.frame_index : frameChoice.from; });
                }
            }
        }
    }
    Label {
        objectName: "resourceGridStatus"
        textFormat: Text.PlainText
        Layout.fillWidth: true
        text: presentationOwner.rejected ? presentationOwner.message : view.displayable ? "Configured CM transmit resources before Pre-Eq C. This is not physical H or received C×H." : view.unavailableText
        wrapMode: Text.WordWrap
        color: view.displayable ? view.theme.accent : view.theme.muted
    }
    Canvas {
        id: canvas
        objectName: "resourceGridCanvas"
        Layout.fillWidth: true
        Layout.preferredHeight: view.chartHeight
        readonly property real plotWidth: Math.max(1, width - view.leftAxisMargin - view.rightAxisMargin)
        readonly property real plotHeight: Math.max(1, height - view.topAxisMargin - view.bottomAxisMargin)
        readonly property real cellWidth: view.grid && view.grid.symbol_count > 0 ? plotWidth / view.grid.symbol_count : 0
        readonly property real cellHeight: view.grid && view.grid.carrier_count > 0 ? plotHeight / view.grid.carrier_count : 0
        property int paintedCellCount: 0
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        Connections {
            target: view.theme
            function onDarkChanged() {
                canvas.requestPaint();
            }
        }
        onPaint: {
            const context = getContext("2d");
            context.reset();
            context.fillStyle = view.theme.panel;
            context.fillRect(0, 0, width, height);
            if (!view.displayable) {
                paintedCellCount = 0;
                return;
            }
            let drawnCells = 0;
            for (const cell of view.grid.cells) {
                const rectangle = view.cellRectangle(cell);
                context.fillStyle = view.roleColor(cell.role);
                context.fillRect(rectangle.x, rectangle.y, rectangle.width, rectangle.height);
                if (rectangle.width >= view.cellDividerPixels && rectangle.height >= view.cellDividerPixels) {
                    context.strokeStyle = view.theme.background;
                    context.strokeRect(rectangle.x, rectangle.y, rectangle.width, rectangle.height);
                }
                drawnCells += 1;
            }
            paintedCellCount = drawnCells;
            if (view.inspectedCell) {
                const rectangle = view.cellRectangle(view.inspectedCell);
                context.strokeStyle = view.theme.text;
                context.lineWidth = view.selectionBorderPixels;
                context.strokeRect(rectangle.x, rectangle.y, rectangle.width, rectangle.height);
            }
            context.fillStyle = view.theme.text;
            context.font = view.axisFontPixels + "px sans-serif";
            context.textAlign = "center";
            context.fillText("Frame symbol index", view.leftAxisMargin + plotWidth / 2, height - view.axisLabelInset);
            context.fillText(String(view.grid.first_symbol), view.leftAxisMargin + cellWidth / 2, height - view.bottomAxisMargin + view.axisLabelInset);
            if (view.grid.symbol_count > 1) {
                context.fillText(String(view.grid.first_symbol + view.grid.symbol_count - 1), width - view.rightAxisMargin - cellWidth / 2, height - view.bottomAxisMargin + view.axisLabelInset);
            }
            context.textAlign = "right";
            context.fillText(String(view.grid.first_carrier), view.leftAxisMargin - view.axisLabelInset, height - view.bottomAxisMargin - cellHeight / 2);
            context.fillText(String(view.grid.first_carrier + view.grid.carrier_count - 1), view.leftAxisMargin - view.axisLabelInset, view.topAxisMargin + cellHeight / 2);
            context.save();
            context.translate(view.axisLabelInset, view.topAxisMargin + plotHeight / 2);
            context.rotate(-Math.PI / 2);
            context.textAlign = "center";
            context.fillText("Physical carrier index", 0, 0);
            context.restore();
        }
        MouseArea {
            id: pointer
            objectName: "resourceGridNavigation"
            anchors.fill: parent
            hoverEnabled: true
            enabled: view.actionsEnabled
            focus: true
            property real pressX: 0
            property real pressY: 0
            Accessible.name: "Configured resource grid navigation"
            Accessible.description: "Click a cell to inspect. Scroll or plus/minus zoom; drag or arrow keys pan. Home requests full carriers."
            onPressed: function (mouse) {
                forceActiveFocus();
                pressX = mouse.x;
                pressY = mouse.y;
            }
            onPositionChanged: function (mouse) {
                view.setHoveredCell(view.cellAt(mouse.x, mouse.y));
            }
            onExited: view.setHoveredCell(null)
            onReleased: function (mouse) {
                const horizontal = mouse.x - pressX;
                const vertical = mouse.y - pressY;
                if (Math.abs(horizontal) < view.pointerDragThresholdPixels && Math.abs(vertical) < view.pointerDragThresholdPixels) {
                    view.inspectAt(mouse.x, mouse.y);
                } else {
                    view.pan(-Math.round(horizontal / canvas.cellWidth), Math.round(vertical / canvas.cellHeight));
                }
            }
            onWheel: function (wheel) {
                const delta = wheel.angleDelta.y || wheel.pixelDelta.y;
                if (!delta || !view.cellAt(wheel.x, wheel.y)) {
                    wheel.accepted = false;
                    return;
                }
                const horizontal = Math.max(0, Math.min(1, (wheel.x - view.leftAxisMargin) / canvas.plotWidth));
                const vertical = Math.max(0, Math.min(1, 1 - (wheel.y - view.topAxisMargin) / canvas.plotHeight));
                view.zoom(delta > 0 ? view.zoomInFactor : view.zoomOutFactor, horizontal, vertical);
                wheel.accepted = true;
            }
            Keys.onPressed: function (event) {
                if (event.key === Qt.Key_Left) {
                    view.pan(-view.grid.symbol_count, 0);
                } else if (event.key === Qt.Key_Right) {
                    view.pan(view.grid.symbol_count, 0);
                } else if (event.key === Qt.Key_Up) {
                    view.pan(0, view.grid.carrier_count);
                } else if (event.key === Qt.Key_Down) {
                    view.pan(0, -view.grid.carrier_count);
                } else if (event.key === Qt.Key_Home) {
                    view.fullCarriers();
                } else if (event.key === Qt.Key_Plus || event.key === Qt.Key_Equal) {
                    view.zoom(view.zoomInFactor, view.zoomAnchorFraction, view.zoomAnchorFraction);
                } else if (event.key === Qt.Key_Minus) {
                    view.zoom(view.zoomOutFactor, view.zoomAnchorFraction, view.zoomAnchorFraction);
                } else {
                    event.accepted = false;
                    return;
                }
                event.accepted = true;
            }
            ToolTip.delay: view.theme.tooltipDelayMs
            ToolTip.visible: containsMouse && view.actionsEnabled && view.hoveredCell !== null
            ToolTip.text: presentationOwner.value ? presentationOwner.value.hoverText : ""
        }
    }
    Flow {
        Layout.fillWidth: true
        spacing: view.sectionSpacing
        Repeater {
            model: view.roles
            Row {
                id: roleEntry
                required property var modelData
                spacing: view.toolbarSpacing
                Rectangle {
                    width: view.legendMarkerSize
                    height: width
                    anchors.verticalCenter: parent.verticalCenter
                    color: view.roleColor(roleEntry.modelData.key)
                }
                Label {
                    text: view.displayable ? roleEntry.modelData.availableText : roleEntry.modelData.unavailableText
                    color: view.theme.text
                }
            }
        }
    }
    Label {
        objectName: "resourceGridMetadata"
        Layout.fillWidth: true
        visible: view.displayable
        color: view.theme.muted
        font.pixelSize: view.metadataFontPixels
        wrapMode: Text.WrapAnywhere
        textFormat: Text.PlainText
        text: view.displayable && presentationOwner.value ? presentationOwner.value.metadataText : ""
    }
    TextArea {
        objectName: "resourceCellInspection"
        textFormat: TextEdit.PlainText
        Layout.fillWidth: true
        Layout.minimumHeight: view.inspectionMinimumHeight
        text: view.inspectedText
        readOnly: true
        selectByMouse: true
        wrapMode: TextEdit.WrapAnywhere
        color: view.theme.text
        background: Rectangle {
            color: view.theme.panel
            border.color: view.theme.line
        }
        Accessible.name: "Original resource cell metadata"
    }
}
