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
    readonly property bool contextCurrent: coverage.value !== null && envelope === incomingState
    readonly property bool contextAvailable: envelope.query_sequence !== undefined && envelope.session_token !== undefined && envelope.configuration_generation !== undefined && envelope.run !== undefined
    readonly property bool presentationRejected: coverage.rejected
    readonly property bool pending: coverage.value ? coverage.value.pending : false
    readonly property bool displayable: contextCurrent && contextAvailable && !coverage.rejected && envelope.status === "ready" && snapshot.kind === "response" && snapshot.status === "ready"
    readonly property bool actionsEnabled: contextCurrent && contextAvailable && !coverage.rejected && envelope.status === "ready" && snapshot.kind === "response" && !pending
    readonly property string statusText: coverage.rejected ? coverage.message : envelope.reason || snapshot.message || (snapshot.viewport ? snapshot.viewport.error : "") || (displayable ? "" : "Response unavailable.")
    readonly property var emptyViewport: ({
            points: [],
            x_ticks: [],
            y_ticks: []
        })
    readonly property real preferredWidth: 1100
    readonly property real preferredHeight: 820
    readonly property real windowMargin: 24
    readonly property real sectionSpacing: 12
    readonly property real controlSpacing: 8
    readonly property real sourceWidth: 300
    readonly property real metricWidth: 240
    readonly property real frameWidth: 380
    readonly property real plotMinimumHeight: 320
    readonly property real inspectionMinimumHeight: 100
    readonly property int detailIndentSpaces: 2
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: coverage
        theme: dialog.theme
    }

    /** Preserve every field in the strict native context without numeric conversion. */
    function context() {
        return {
            query_sequence: envelope.query_sequence,
            session_token: envelope.session_token,
            configuration_generation: envelope.configuration_generation,
            run: envelope.run
        };
    }

    /** Couple the original native DTO and only independently retained display text. */
    function candidate(value, draft, selected, waiting) {
        const result = value.snapshot || {};
        return {
            envelope: value,
            selection: draft,
            selected: selected,
            pending: waiting,
            rowStrings: (result.metadata_rows || []).map(row => row.label + ": " + row.value),
            metadataText: result.metadata ? JSON.stringify(result.metadata, null, detailIndentSpaces) : "",
            inspectionText: selected ? (selected.readout || "") + "\n" + (typeof selected.detail === "string" ? selected.detail : JSON.stringify(selected.detail, null, detailIndentSpaces) || "") : ""
        };
    }

    /** Retain a same-query local selector draft while polling unchanged native data. */
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

    /** Locate an actual supplied choice, never an inferred array ordinal or catalog. */
    function choiceIndex(values, value) {
        return (values || []).findIndex(choice => choice.value === value);
    }

    /** Native false disables a choice; omitted availability is the native default. */
    function choiceAvailable(values, value) {
        const index = choiceIndex(values, value);
        return index >= 0 && values[index].available !== false;
    }

    /** Admit a changed supplied source/metric/frame before updating any selector. */
    function editChoice(key, values, index) {
        if (!actionsEnabled || index < 0 || index >= values.length || values[index].available === false)
            return false;
        const draft = Object.assign({}, selection);
        draft[key] = values[index].value;
        return coverage.replace(candidate(envelope, draft, coverage.value.selected, false));
    }

    /** Require all original request fields and actual source/metric/frame choices. */
    function canRequest() {
        return actionsEnabled && choiceAvailable(choices.sources, selection.source) && choiceAvailable(choices.metrics, selection.metric) && (selection.source !== "received_estimate" || choiceAvailable(choices.frames, selection.retained_index)) && selection.grant_symbol !== undefined && selection.retained_index !== undefined && selection.first !== undefined && selection.count !== undefined;
    }

    /** Forward unchanged backend viewport coordinates through the strict flat request. */
    function requestViewport(first, count) {
        if (!canRequest() || first === undefined || count === undefined)
            return false;
        const value = {
            source: selection.source,
            metric: selection.metric,
            grant_symbol: selection.grant_symbol,
            retained_index: selection.retained_index,
            first: first,
            count: count
        };
        const originalContext = context();
        if (!coverage.replace(candidate(envelope, value, coverage.value.selected, true)))
            return false;
        instrumentRequested("response", value, originalContext);
        return true;
    }

    /** Retain exact selected native details, including zero or missing readings. */
    function inspectSelected() {
        if (!displayable || !actionsEnabled)
            return false;
        const point = responsePlot.selectedPoint;
        if (coverage.value.selected === point)
            return true;
        return coverage.replace(candidate(envelope, selection, point, false));
    }

    onIncomingStateChanged: if (visible)
        receiveState()
    onOpened: receiveState()
    Component.onDestruction: coverage.clear()
    title: "Upstream Response"
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
                objectName: "upstreamResponseSummary"
                text: dialog.snapshot.summary || ""
                textFormat: Text.PlainText
                color: dialog.theme.text
                wrapMode: Text.WordWrap
                Layout.fillWidth: true
            }
            Label {
                objectName: "upstreamResponseStatus"
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
                    text: "Source"
                    color: dialog.theme.text
                }
                ComboBox {
                    objectName: "upstreamResponseSource"
                    width: dialog.sourceWidth
                    model: dialog.choices.sources || []
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.choiceIndex(model, dialog.selection.source)
                    delegate: ItemDelegate {
                        required property var modelData
                        text: modelData.label
                        enabled: modelData.available !== false
                    }
                    onActivated: function (index) {
                        dialog.editChoice("source", model, index);
                    }
                }
                Label {
                    text: "Metric"
                    color: dialog.theme.text
                }
                ComboBox {
                    objectName: "upstreamResponseMetric"
                    width: dialog.metricWidth
                    model: dialog.choices.metrics || []
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.choiceIndex(model, dialog.selection.metric)
                    delegate: ItemDelegate {
                        required property var modelData
                        text: modelData.label
                        enabled: modelData.available !== false
                    }
                    onActivated: function (index) {
                        dialog.editChoice("metric", model, index);
                    }
                }
                Label {
                    text: "Actual retained frame"
                    color: dialog.theme.text
                    visible: dialog.selection.source === "received_estimate"
                }
                ComboBox {
                    objectName: "upstreamResponseFrame"
                    width: dialog.frameWidth
                    visible: dialog.selection.source === "received_estimate"
                    model: dialog.choices.frames || []
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.choiceIndex(model, dialog.selection.retained_index)
                    onActivated: function (index) {
                        dialog.editChoice("retained_index", model, index);
                    }
                }
                WorkbenchActionButton {
                    objectName: "upstreamResponseRefresh"
                    theme: dialog.theme
                    text: "Refresh"
                    enabled: dialog.canRequest()
                    onClicked: dialog.requestViewport(dialog.selection.first, dialog.selection.count)
                }
            }

            SpectrumPlot {
                id: responsePlot
                objectName: "upstreamResponsePlot"
                visible: dialog.displayable
                Layout.fillWidth: true
                Layout.preferredHeight: dialog.plotMinimumHeight
                theme: dialog.theme
                valueAxisWidth: extendedValueAxisWidth
                viewport: dialog.displayable ? dialog.snapshot.viewport || dialog.emptyViewport : dialog.emptyViewport
                zoomEnabled: dialog.actionsEnabled && dialog.displayable
                detailsText: dialog.snapshot.summary || ""
                onViewportRequested: function (first, count) {
                    dialog.requestViewport(first, count);
                }
                onSelectedPointChanged: Qt.callLater(dialog.inspectSelected)
            }
            TextArea {
                objectName: "upstreamResponseInspection"
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
                id: details
                objectName: "upstreamResponseDetails"
                text: "Details"
            }
            TextArea {
                objectName: "upstreamResponseMetadata"
                Layout.fillWidth: true
                visible: details.checked
                readOnly: true
                selectByMouse: true
                textFormat: TextEdit.PlainText
                text: coverage.value ? coverage.value.metadataText : ""
                wrapMode: TextEdit.WrapAnywhere
            }
        }
    }
}
