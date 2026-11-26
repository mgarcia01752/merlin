/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/instruments"
import "../shared/presentation"

WorkbenchDialog {
    id: dialog
    required property var host
    signal instrumentRequested(string kind, var selection, var context)
    readonly property var incomingState: host && host.instrumentState ? host.instrumentState : ({})
    readonly property var envelope: coverage.value ? coverage.value.envelope : ({})
    readonly property var snapshot: envelope.snapshot && envelope.snapshot.kind === "fec" ? envelope.snapshot : ({})
    readonly property var selection: snapshot.selection || ({})
    readonly property var choices: snapshot.choices || ({})
    readonly property var page: snapshot.page || ({})
    readonly property bool contextCurrent: coverage.value !== null && envelope === incomingState
    readonly property bool contextAvailable: envelope.query_sequence !== undefined && envelope.session_token !== undefined && envelope.configuration_generation !== undefined && envelope.run !== undefined
    readonly property bool actionsEnabled: contextCurrent && contextAvailable && !coverage.rejected && envelope.status === "ready" && snapshot.kind === "fec" && !coverage.value.pending
    readonly property bool presentationRejected: coverage.rejected
    readonly property string statusText: coverage.rejected ? coverage.message : envelope.reason || snapshot.message || ""
    // Native display geometry, independent of scientific retention/copy limits.
    readonly property real preferredWidth: 1120
    readonly property real preferredHeight: 780
    readonly property real windowMargin: 24
    readonly property real sectionSpacing: 12
    readonly property real actionSpacing: 8
    readonly property real selectorWidth: 290
    readonly property real tableHeight: 230
    readonly property real chartHeight: 210
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: coverage
        theme: dialog.theme
    }

    /** Couple original native data and all derived strings before retaining either. */
    function candidate(value, pending) {
        const result = value.snapshot && value.snapshot.kind === "fec" ? value.snapshot : {};
        return {envelope: value, pending: pending,
            rows: (result.metadata_rows || []).map(row => row.label + ": " + row.value)};
    }

    /** Repeated polling retains the existing immutable witness and table position. */
    function receiveState() {
        if (coverage.value && envelope === incomingState && !coverage.rejected) {
            return true;
        }
        return coverage.replace(candidate(incomingState, false));
    }

    /** Forward one supplied selector under its complete original native context. */
    function requestSelection(value) {
        if (!actionsEnabled || !value) {
            return false;
        }
        const context = {query_sequence: envelope.query_sequence,
            session_token: envelope.session_token,
            configuration_generation: envelope.configuration_generation, run: envelope.run};
        if (!coverage.replace(candidate(envelope, true))) {
            return false;
        }
        instrumentRequested("fec", value, context);
        return true;
    }

    /** Request the backend-supplied initial selector for a distinct observation plane. */
    function requestMode(index) {
        const modes = choices.modes || [];
        if (index < 0 || index >= modes.length || !choices.mode_requests) {
            return false;
        }
        return requestSelection(choices.mode_requests[modes[index].value]);
    }

    /** Navigate original word/cell/bit coordinates supplied by the current page. */
    function requestPage(first) {
        if (!Number.isInteger(first) || first < 0) {
            return false;
        }
        const key = selection.mode === "words" ? "first_word"
            : selection.mode === "soft_cells" ? "first_index" : "first_bit";
        const changed = {};
        changed[key] = first;
        return requestSelection(Object.assign({}, selection, changed));
    }

    onIncomingStateChanged: if (visible)
        receiveState()
    onOpened: receiveState()
    Component.onDestruction: coverage.clear()
    title: "Upstream LDPC / Soft Observations"
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
        // Reserve the real outer scrollbar width so it cannot intercept pointer
        // drags intended for the independently scrolling original-data table.
        contentWidth: Math.max(0, availableWidth - scroll.ScrollBar.vertical.width)
        ColumnLayout {
            width: scroll.contentWidth
            spacing: dialog.sectionSpacing
            Label {
                objectName: "upstreamFecSummary"
                Layout.fillWidth: true
                text: dialog.snapshot.summary || "Run upstream to inspect actual decoder and soft observations."
                textFormat: Text.PlainText
                color: dialog.theme.text
                wrapMode: Text.WordWrap
            }
            Label {
                objectName: "upstreamFecStatus"
                Layout.fillWidth: true
                visible: text.length > 0
                text: dialog.statusText
                textFormat: Text.PlainText
                color: dialog.theme.statusAttention
                wrapMode: Text.WordWrap
            }
            Flow {
                Layout.fillWidth: true
                spacing: dialog.actionSpacing
                ComboBox {
                    objectName: "upstreamFecMode"
                    width: dialog.selectorWidth
                    enabled: dialog.actionsEnabled
                    model: dialog.choices.modes || []
                    textRole: "label"
                    currentIndex: (dialog.choices.modes || []).findIndex(value => value.value === dialog.selection.mode)
                    onActivated: function(index) { dialog.requestMode(index); }
                }
                ComboBox {
                    objectName: "upstreamFecGrant"
                    width: dialog.selectorWidth
                    visible: dialog.selection.mode === "words"
                    enabled: dialog.actionsEnabled
                    model: dialog.choices.grants || []
                    textRole: "label"
                    currentIndex: (dialog.choices.grants || []).findIndex(value => value.value === dialog.selection.grant_index)
                    onActivated: function(index) {
                        const values = dialog.choices.grants || [];
                        if (index >= 0 && index < values.length) {
                            dialog.requestSelection(Object.assign({}, dialog.selection, {grant_index: values[index].value, first_word: 0}));
                        }
                    }
                }
                ComboBox {
                    objectName: "upstreamFecSoftWord"
                    width: dialog.selectorWidth
                    visible: dialog.selection.mode === "soft_word"
                    enabled: dialog.actionsEnabled && count > 0
                    model: dialog.choices.soft_words || []
                    textRole: "label"
                    currentIndex: (dialog.choices.soft_words || []).findIndex(value => value.value === dialog.selection.retained_index)
                    onActivated: function(index) {
                        const values = dialog.choices.soft_words || [];
                        if (index >= 0 && index < values.length) {
                            dialog.requestSelection(Object.assign({}, dialog.selection, {retained_index: values[index].value, first_bit: 0}));
                        }
                    }
                }
                WorkbenchActionButton {
                    objectName: "upstreamFecRefresh"
                    theme: dialog.theme
                    text: "Refresh"
                    enabled: dialog.actionsEnabled
                    onClicked: dialog.requestSelection(dialog.selection)
                }
                WorkbenchActionButton {
                    objectName: "upstreamFecFirst"
                    theme: dialog.theme
                    text: "First"
                    enabled: dialog.actionsEnabled && Number.isInteger(dialog.page.first) && dialog.page.first > 0
                    onClicked: dialog.requestPage(0)
                }
                WorkbenchActionButton {
                    objectName: "upstreamFecPrevious"
                    theme: dialog.theme
                    text: "Previous"
                    enabled: dialog.actionsEnabled && Number.isInteger(dialog.page.previous)
                    onClicked: dialog.requestPage(dialog.page.previous)
                }
                WorkbenchActionButton {
                    objectName: "upstreamFecNext"
                    theme: dialog.theme
                    text: "Next"
                    enabled: dialog.actionsEnabled && Number.isInteger(dialog.page.next)
                    onClicked: dialog.requestPage(dialog.page.next)
                }
            }
            ObservationTable {
                objectName: "upstreamFecTable"
                rowsObjectName: "upstreamFecRows"
                Layout.fillWidth: true
                Layout.preferredHeight: dialog.tableHeight
                theme: dialog.theme
                headings: dialog.snapshot.table ? dialog.snapshot.table.headings || [] : []
                rows: dialog.snapshot.table ? dialog.snapshot.table.rows || [] : []
                emptyText: dialog.snapshot.status === "unavailable" ? dialog.snapshot.message || "Unavailable" : "No original observations in this page."
            }
            CountBarChart {
                objectName: "upstreamFecIterations"
                Layout.fillWidth: true
                Layout.preferredHeight: dialog.chartHeight
                visible: dialog.selection.mode === "words"
                theme: dialog.theme
                heading: "Actual iterations for selected completed words"
                chart: dialog.snapshot.chart || ({points: []})
                categoryAxisLabel: "Original completed-word index"
                zeroMessage: "Zero iterations in the selected actual outcomes"
                emptyText: "No completed decoder outcomes in this page."
            }
            CheckBox {
                id: details
                objectName: "upstreamFecDetails"
                text: "Details"
                checked: false
            }
            Repeater {
                model: details.checked && coverage.value ? coverage.value.rows : []
                Label {
                    required property string modelData
                    Layout.fillWidth: true
                    text: modelData
                    textFormat: Text.PlainText
                    color: dialog.theme.muted
                    wrapMode: Text.WrapAnywhere
                }
            }
        }
    }
}
