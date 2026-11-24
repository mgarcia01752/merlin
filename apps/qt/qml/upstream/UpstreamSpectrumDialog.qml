/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "../shared/instruments"
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/presentation"

WorkbenchDialog {
    id: dialog
    required property var host
    signal instrumentRequested(string kind, var selection, var context)
    readonly property var incomingState: host && host.instrumentState ? host.instrumentState : ({})
    readonly property var envelope: coverage.value ? coverage.value.envelope : ({})
    readonly property var snapshot: envelope.snapshot || ({})
    readonly property var selection: coverage.value ? coverage.value.selection : ({})
    readonly property var tapChoices: snapshot.choices ? snapshot.choices.taps || [] : []
    readonly property bool contextCurrent: coverage.value !== null && envelope === incomingState
    readonly property bool contextAvailable: envelope.query_sequence !== undefined && envelope.session_token !== undefined && envelope.configuration_generation !== undefined && envelope.run !== undefined
    readonly property bool presentationRejected: coverage.rejected
    readonly property bool pending: coverage.value ? coverage.value.pending : false
    readonly property bool displayable: contextCurrent && contextAvailable && !coverage.rejected && envelope.status === "ready" && snapshot.kind === "spectrum" && snapshot.status === "ready"
    readonly property bool actionsEnabled: contextCurrent && contextAvailable && !coverage.rejected && envelope.status === "ready" && snapshot.kind === "spectrum" && !pending
    readonly property string statusText: coverage.rejected ? coverage.message : envelope.reason || snapshot.message || (displayable ? "" : "Spectrum unavailable.")
    readonly property var emptyViewport: ({
            points: [],
            x_ticks: [],
            y_ticks: []
        })
    readonly property real preferredWidth: 1100
    readonly property real preferredHeight: 780
    readonly property real windowMargin: 24
    readonly property real sectionSpacing: 12
    readonly property real controlSpacing: 8
    readonly property real tapWidth: 280
    readonly property real plotMinimumHeight: 360
    readonly property int detailIndentSpaces: 2
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: coverage
        theme: dialog.theme
    }

    /** Preserve the native correlation envelope without number conversion. */
    function context() {
        return {
            query_sequence: envelope.query_sequence,
            session_token: envelope.session_token,
            configuration_generation: envelope.configuration_generation,
            run: envelope.run
        };
    }

    /** Admit one original envelope plus only its independently derived labels. */
    function candidate(value, draft, waiting) {
        const result = value.snapshot || {};
        return {
            envelope: value,
            selection: draft,
            pending: waiting,
            rowStrings: (result.metadata_rows || []).map(row => row.label + ": " + row.value),
            metadataText: result.metadata ? JSON.stringify(result.metadata, null, detailIndentSpaces) : ""
        };
    }

    /** Consume current native data only when this existing consumer is opened. */
    function receiveState() {
        const value = incomingState;
        const sameQuery = coverage.value && value.query_sequence === envelope.query_sequence && value.session_token === envelope.session_token && value.configuration_generation === envelope.configuration_generation && value.run === envelope.run;
        // Loading can retain another instrument's previous snapshot. A ready
        // reply adopts its own authoritative selector even with the same context.
        const acceptedReply = value.status === "ready" && envelope.status === "loading";
        const keepDraft = sameQuery && !acceptedReply;
        const waiting = keepDraft && pending && value.status !== "error" && value.status !== "unavailable";
        const draft = keepDraft ? selection : value.snapshot && value.snapshot.selection ? value.snapshot.selection : {};
        return coverage.replace(candidate(value, draft, waiting));
    }

    /** Select only an available tap from the supplied native choices. */
    function editTap(index) {
        if (!actionsEnabled || index < 0 || index >= tapChoices.length || tapChoices[index].available !== true) {
            return false;
        }
        return coverage.replace(candidate(envelope, Object.assign({}, selection, {
            tap: tapChoices[index].value
        }), false));
    }

    /** Check only that an explicit supplied tap is currently available. */
    function tapAvailable() {
        return tapChoices.some(choice => choice.value === selection.tap && choice.available === true);
    }

    /** Send unchanged backend viewport coordinates after coupled presentation admission. */
    function requestViewport(first, count) {
        if (!actionsEnabled || !tapAvailable() || first === undefined || count === undefined) {
            return false;
        }
        const value = {
            tap: selection.tap,
            first: first,
            count: count
        };
        const originalContext = context();
        if (!coverage.replace(candidate(envelope, value, true))) {
            return false;
        }
        instrumentRequested("spectrum", value, originalContext);
        return true;
    }

    onIncomingStateChanged: if (visible)
        receiveState()
    onOpened: receiveState()
    Component.onDestruction: coverage.clear()
    title: "Upstream Spectrum"
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
                objectName: "upstreamSpectrumSummary"
                text: dialog.snapshot.summary || ""
                textFormat: Text.PlainText
                color: dialog.theme.text
                wrapMode: Text.WordWrap
                Layout.fillWidth: true
            }
            Label {
                objectName: "upstreamSpectrumStatus"
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
                    text: "Tap"
                    color: dialog.theme.text
                }
                ComboBox {
                    objectName: "upstreamSpectrumTap"
                    width: dialog.tapWidth
                    model: dialog.tapChoices
                    textRole: "label"
                    enabled: dialog.actionsEnabled
                    currentIndex: dialog.tapChoices.findIndex(choice => choice.value === dialog.selection.tap)
                    delegate: ItemDelegate {
                        required property var modelData
                        text: modelData.label
                        enabled: modelData.available === true
                    }
                    onActivated: function (index) {
                        dialog.editTap(index);
                    }
                }
                WorkbenchActionButton {
                    objectName: "upstreamSpectrumRefresh"
                    theme: dialog.theme
                    text: "Refresh"
                    enabled: dialog.actionsEnabled && dialog.tapAvailable() && dialog.selection.first !== undefined && dialog.selection.count !== undefined
                    onClicked: dialog.requestViewport(dialog.selection.first, dialog.selection.count)
                }
            }

            SpectrumPlot {
                id: spectrumPlot
                objectName: "upstreamSpectrumPlot"
                Layout.fillWidth: true
                Layout.preferredHeight: dialog.plotMinimumHeight
                theme: dialog.theme
                valueAxisWidth: extendedValueAxisWidth
                viewport: dialog.displayable ? dialog.snapshot.viewport || dialog.emptyViewport : dialog.emptyViewport
                zoomEnabled: dialog.actionsEnabled
                detailsText: dialog.snapshot.summary || ""
                onViewportRequested: function (first, count) {
                    dialog.requestViewport(first, count);
                }
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
                objectName: "upstreamSpectrumDetails"
                text: "Details"
            }
            TextArea {
                Layout.fillWidth: true
                visible: details.checked
                readOnly: true
                textFormat: TextEdit.PlainText
                text: coverage.value ? coverage.value.metadataText : ""
                wrapMode: TextEdit.WrapAnywhere
            }
        }
    }
}
