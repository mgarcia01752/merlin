/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/presentation"
import "../shared/instruments"

WorkbenchDialog {
    id: dialog
    required property var host
    signal instrumentRequested(string kind, var selection, var context)
    readonly property var incomingState: host && host.instrumentState ? host.instrumentState : ({})
    readonly property var envelope: coverage.value ? coverage.value.envelope : ({})
    readonly property var snapshot: envelope.snapshot || ({})
    readonly property var selection: coverage.value ? coverage.value.selection : ({})
    readonly property var choices: snapshot.choices || ({})
    readonly property var limits: snapshot.limits || ({})
    readonly property bool contextCurrent: coverage.value !== null && envelope === incomingState
    readonly property bool contextAvailable: envelope.query_sequence !== undefined && envelope.session_token !== undefined && envelope.configuration_generation !== undefined && envelope.run !== undefined
    readonly property bool presentationRejected: coverage.rejected
    readonly property bool pending: coverage.value ? coverage.value.pending : false
    readonly property bool displayable: contextCurrent && contextAvailable && !coverage.rejected && envelope.status === "ready" && snapshot.kind === "qam" && snapshot.status === "ready"
    readonly property bool actionsEnabled: contextCurrent && contextAvailable && !coverage.rejected && envelope.status === "ready" && snapshot.kind === "qam" && !pending
    readonly property string statusText: coverage.rejected ? coverage.message : envelope.reason || snapshot.message || (displayable ? "" : "QAM unavailable.")
    readonly property var emptyScatter: ({
            points: []
        })
    readonly property var seriesColors: ({
            actual: theme.accent,
            unit_reference: theme.muted,
            left: theme.accent,
            right: theme.qamNcp,
            reference: theme.muted
        })
    readonly property real preferredWidth: 1100
    readonly property real preferredHeight: 820
    readonly property real windowMargin: 24
    readonly property real sectionSpacing: 12
    readonly property real controlSpacing: 8
    readonly property real modeWidth: 170
    readonly property real tapWidth: 220
    readonly property real loadingWidth: 160
    readonly property real plotMinimumHeight: 280
    readonly property real inspectionMinimumHeight: 100
    readonly property int detailIndentSpaces: 2
    readonly property real pointerDragThresholdPixels: 6
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: coverage
        theme: dialog.theme
    }

    /** Preserve the original native context without number conversion. */
    function context() {
        return {
            query_sequence: envelope.query_sequence,
            session_token: envelope.session_token,
            configuration_generation: envelope.configuration_generation,
            run: envelope.run
        };
    }

    /** Admit original DTO aliases and independently retained display strings together. */
    function candidate(value, draft, selected, waiting) {
        const result = value.snapshot || {};
        const scatter = result.scatter || emptyScatter;
        return {
            envelope: value,
            selection: draft,
            pending: waiting,
            selected: selected,
            referenceScatter: {
                points: scatter.references || []
            },
            rowStrings: (result.metadata_rows || []).map(row => row.label + ": " + row.value),
            metadataText: result.metadata ? JSON.stringify(result.metadata, null, detailIndentSpaces) : "",
            inspectionText: selected ? (selected.readout || "") + "\n" + (typeof selected.detail === "string" ? selected.detail : JSON.stringify(selected.detail, null, detailIndentSpaces) || "") : ""
        };
    }

    /** Consume host updates only for an opened consumer; constructors never query. */
    function receiveState() {
        const value = incomingState;
        const sameQuery = coverage.value && value.query_sequence === envelope.query_sequence && value.session_token === envelope.session_token && value.configuration_generation === envelope.configuration_generation && value.run === envelope.run;
        // Loading can retain another instrument's previous snapshot. A ready
        // reply adopts its own authoritative selector even with the same context.
        const acceptedReply = value.status === "ready" && envelope.status === "loading";
        const keepDraft = sameQuery && !acceptedReply;
        const waiting = keepDraft && pending && value.status !== "error" && value.status !== "unavailable";
        const draft = keepDraft ? selection : value.snapshot && value.snapshot.selection ? value.snapshot.selection : {};
        return coverage.replace(candidate(value, draft, keepDraft ? coverage.value.selected : null, waiting));
    }

    /** Keep local selector drafts covered before publishing any change. */
    function edit(key, value) {
        if (!actionsEnabled) {
            return false;
        }
        const draft = Object.assign({}, selection);
        draft[key] = value;
        return coverage.replace(candidate(envelope, draft, coverage.value.selected, false));
    }

    /** Match supplied choice values exactly, including null and lossless strings. */
    function choiceIndex(values, value) {
        return (values || []).findIndex(choice => choice.value === value);
    }

    /** Available taps are supplied by native admission, never a local catalog. */
    function tapAvailable(value) {
        const taps = choices.taps || [];
        const index = choiceIndex(taps, value);
        return index >= 0 && taps[index].available === true;
    }

    /** Require supplied bounded work policies; Rust remains their validator. */
    function hasLimit(key) {
        return typeof limits[key] === "number" && Number.isFinite(limits[key]) && limits[key] > 0;
    }

    /** Build only the accepted flat public request variant from actual choices. */
    function requestSelection() {
        if (!actionsEnabled || choiceIndex(choices.modes, selection.mode) < 0 || choiceIndex(choices.loadings, selection.loading) < 0) {
            return null;
        }
        if (selection.mode === "raw" && tapAvailable(selection.tap) && hasLimit("maximum_points")) {
            return {
                mode: selection.mode,
                tap: selection.tap,
                loading: selection.loading,
                maximum_points: limits.maximum_points
            };
        }
        if (selection.mode === "constellation" && selection.loading !== null && tapAvailable(selection.tap) && hasLimit("maximum_points") && hasLimit("maximum_references")) {
            return {
                mode: selection.mode,
                tap: selection.tap,
                loading: selection.loading,
                maximum_points: limits.maximum_points,
                maximum_references: limits.maximum_references
            };
        }
        if (selection.mode === "pairing" && tapAvailable(selection.left) && tapAvailable(selection.right) && hasLimit("maximum_pairs")) {
            return {
                mode: selection.mode,
                left: selection.left,
                right: selection.right,
                loading: selection.loading,
                maximum_pairs: limits.maximum_pairs
            };
        }
        return null;
    }

    /** Explicitly request fresh native observations; queued is never accepted. */
    function request() {
        const value = requestSelection();
        if (!value) {
            return false;
        }
        const originalContext = context();
        if (!coverage.replace(candidate(envelope, selection, coverage.value.selected, true))) {
            return false;
        }
        instrumentRequested("qam", value, originalContext);
        return true;
    }

    /** Inspect the nearest original point using screen geometry only. */
    function inspectAt(x, y) {
        if (!actionsEnabled || !actualPlot.containsPlotPosition(x, y)) {
            return false;
        }
        let selected = null;
        let distance = Infinity;
        for (const points of [snapshot.scatter ? snapshot.scatter.points || [] : [], snapshot.scatter ? snapshot.scatter.references || [] : []]) {
            for (const point of points) {
                const position = actualPlot.pointPosition(point);
                if (!actualPlot.containsPlotPosition(position.x, position.y)) {
                    continue;
                }
                const dx = position.x - x;
                const dy = position.y - y;
                const nextDistance = dx * dx + dy * dy;
                if (nextDistance < distance) {
                    selected = point;
                    distance = nextDistance;
                }
            }
        }
        return selected !== null && coverage.replace(candidate(envelope, selection, selected, false));
    }

    onIncomingStateChanged: if (visible)
        receiveState()
    onOpened: receiveState()
    Component.onDestruction: coverage.clear()
    title: "Upstream QAM"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(preferredWidth, parent ? parent.width - windowMargin * 2 : preferredWidth)
    height: Math.min(preferredHeight, parent ? parent.height - windowMargin * 2 : preferredHeight)
    modal: true
    focus: true
    standardButtons: Dialog.Close
    closePolicy: Popup.CloseOnEscape
    contentItem: ScrollView {
        id: scroll
        clip: true
        contentWidth: availableWidth
        ColumnLayout {
            width: scroll.availableWidth
            spacing: dialog.sectionSpacing
            Label {
                objectName: "upstreamQamSummary"
                text: dialog.snapshot.summary || ""
                textFormat: Text.PlainText
                color: dialog.theme.text
                wrapMode: Text.WordWrap
                Layout.fillWidth: true
            }
            Label {
                objectName: "upstreamQamStatus"
                text: dialog.statusText
                textFormat: Text.PlainText
                visible: text.length > 0
                color: dialog.theme.statusAttention
                wrapMode: Text.WordWrap
                Layout.fillWidth: true
            }
            Flow {
                Layout.fillWidth: true
                spacing: dialog.controlSpacing
                Label {
                    text: "Mode"
                    color: dialog.theme.text
                }
                ComboBox {
                    objectName: "upstreamQamMode"
                    width: dialog.modeWidth
                    model: dialog.choices.modes || []
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.choiceIndex(model, dialog.selection.mode)
                    onActivated: function (index) {
                        dialog.edit("mode", model[index].value);
                    }
                }
                Label {
                    text: dialog.selection.mode === "pairing" ? "Left tap" : "Tap"
                    color: dialog.theme.text
                }
                ComboBox {
                    objectName: "upstreamQamTap"
                    width: dialog.tapWidth
                    model: dialog.choices.taps || []
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.choiceIndex(model, dialog.selection.mode === "pairing" ? dialog.selection.left : dialog.selection.tap)
                    delegate: ItemDelegate {
                        required property var modelData
                        text: modelData.label
                        enabled: modelData.available === true
                    }
                    onActivated: function (index) {
                        if (model[index].available === true)
                            dialog.edit(dialog.selection.mode === "pairing" ? "left" : "tap", model[index].value);
                    }
                }
                Label {
                    text: "Right tap"
                    visible: dialog.selection.mode === "pairing"
                    color: dialog.theme.text
                }
                ComboBox {
                    objectName: "upstreamQamRightTap"
                    width: dialog.tapWidth
                    visible: dialog.selection.mode === "pairing"
                    model: dialog.choices.taps || []
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.choiceIndex(model, dialog.selection.right)
                    delegate: ItemDelegate {
                        required property var modelData
                        text: modelData.label
                        enabled: modelData.available === true
                    }
                    onActivated: function (index) {
                        if (model[index].available === true)
                            dialog.edit("right", model[index].value);
                    }
                }
                Label {
                    text: "Loading"
                    color: dialog.theme.text
                }
                ComboBox {
                    objectName: "upstreamQamLoading"
                    width: dialog.loadingWidth
                    model: dialog.choices.loadings || []
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.choiceIndex(model, dialog.selection.loading)
                    onActivated: function (index) {
                        dialog.edit("loading", model[index].value);
                    }
                }
                WorkbenchActionButton {
                    objectName: "upstreamQamRefresh"
                    theme: dialog.theme
                    text: "Refresh"
                    enabled: dialog.requestSelection() !== null
                    onClicked: dialog.request()
                }
            }
            Flow {
                Layout.fillWidth: true
                spacing: dialog.controlSpacing
                WorkbenchActionButton {
                    objectName: "upstreamQamZoomIn"
                    theme: dialog.theme
                    text: "Zoom In"
                    enabled: dialog.actionsEnabled && actualPlot.magnification < actualPlot.maximumMagnification
                    onClicked: actualPlot.zoom(actualPlot.zoomFactor)
                }
                WorkbenchActionButton {
                    objectName: "upstreamQamZoomOut"
                    theme: dialog.theme
                    text: "Zoom Out"
                    enabled: dialog.actionsEnabled && actualPlot.magnification > actualPlot.minimumMagnification
                    onClicked: actualPlot.zoom(1 / actualPlot.zoomFactor)
                }
                WorkbenchActionButton {
                    objectName: "upstreamQamReset"
                    theme: dialog.theme
                    text: "Reset"
                    enabled: dialog.actionsEnabled
                    onClicked: actualPlot.resetView()
                }
            }

            Item {
                Layout.fillWidth: true
                Layout.preferredHeight: dialog.plotMinimumHeight
                QamPlot {
                    id: actualPlot
                    objectName: "upstreamQamActualPlot"
                    anchors.fill: parent
                    theme: dialog.theme
                    observation: dialog.displayable ? dialog.snapshot.scatter || dialog.emptyScatter : dialog.emptyScatter
                    receivedSamples: true
                    roleMarkers: false
                    showAxesWithoutData: true
                    seriesColors: dialog.seriesColors
                    clipToPlot: true
                    horizontalLabelInset: axisFontPixels
                    verticalLabelOffset: axisFontPixels
                }
                QamPlot {
                    objectName: "upstreamQamReferencePlot"
                    anchors.fill: parent
                    theme: dialog.theme
                    observation: dialog.displayable && coverage.value ? coverage.value.referenceScatter : dialog.emptyScatter
                    receivedSamples: true
                    roleMarkers: false
                    seriesColors: dialog.seriesColors
                    clipToPlot: true
                    horizontalLabelInset: actualPlot.horizontalLabelInset
                    verticalLabelOffset: actualPlot.verticalLabelOffset
                    magnification: actualPlot.magnification
                    panX: actualPlot.panX
                    panY: actualPlot.panY
                }
                MouseArea {
                    objectName: "upstreamQamInspectionArea"
                    anchors.fill: parent
                    preventStealing: true
                    enabled: dialog.actionsEnabled
                    property real pressX: 0
                    property real pressY: 0
                    onPressed: function (mouse) {
                        if (!actualPlot.containsPlotPosition(mouse.x, mouse.y)) {
                            mouse.accepted = false;
                            return;
                        }
                        pressX = mouse.x;
                        pressY = mouse.y;
                    }
                    onReleased: function (mouse) {
                        const dx = mouse.x - pressX;
                        const dy = mouse.y - pressY;
                        if (Math.abs(dx) < dialog.pointerDragThresholdPixels && Math.abs(dy) < dialog.pointerDragThresholdPixels) {
                            dialog.inspectAt(mouse.x, mouse.y);
                        } else {
                            actualPlot.pan(dx, dy);
                        }
                    }
                    onWheel: function (wheel) {
                        const delta = wheel.angleDelta.y || wheel.pixelDelta.y;
                        if (!delta || actualPlot.plotEdge <= 0 || !actualPlot.containsPlotPosition(wheel.x, wheel.y)) {
                            wheel.accepted = false;
                            return;
                        }
                        actualPlot.zoom(delta > 0 ? actualPlot.zoomFactor : 1 / actualPlot.zoomFactor, (wheel.x - actualPlot.plotLeft) / actualPlot.plotEdge, (wheel.y - actualPlot.plotTop) / actualPlot.plotEdge);
                        wheel.accepted = true;
                    }
                }
            }
            TextArea {
                objectName: "upstreamQamInspection"
                Layout.fillWidth: true
                Layout.minimumHeight: dialog.inspectionMinimumHeight
                readOnly: true
                selectByMouse: true
                textFormat: TextEdit.PlainText
                text: coverage.value ? coverage.value.inspectionText : ""
                wrapMode: TextEdit.WrapAnywhere
            }
            Repeater {
                model: coverage.value ? coverage.value.rowStrings : []
                Label {
                    required property string modelData
                    text: modelData
                    textFormat: Text.PlainText
                    color: dialog.theme.muted
                    wrapMode: Text.WrapAnywhere
                    Layout.fillWidth: true
                }
            }
            CheckBox {
                id: findDetails
                objectName: "upstreamQamDetails"
                text: "Details"
            }
            TextArea {
                Layout.fillWidth: true
                visible: findDetails.checked
                readOnly: true
                textFormat: TextEdit.PlainText
                text: coverage.value ? coverage.value.metadataText : ""
                wrapMode: TextEdit.WrapAnywhere
            }
        }
    }
}
