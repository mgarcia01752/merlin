/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    objectName: "spectrumAnalyzerDialog"
    required property var theme
    required property var backend
    required property var state
    property var viewport: ({points: [], x_ticks: [], y_ticks: []})
    property string displayedToken: ""
    property bool captureFrequencyAxis: false
    property bool frequencyAxisAvailable: true
    property bool autoGenerate: true
    property bool showTiming: true
    property var tapOptions: []
    property int selectedTap: 0
    property string generateDescription: "Run the transmitter preview and recalculate its spectrum. Channel and receiver are not connected yet."
    signal tapRequested(int index)
    title: "Spectrum Analyzer · " + (dialog.state.tap_name || "OFDM Samples")
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(1120, parent ? parent.width - 40 : 1120)
    height: Math.min(740, parent ? parent.height - 40 : 740)
    modal: true
    onOpened: {
        updateResult()
        if (dialog.autoGenerate && dialog.state.can_generate && !dialog.state.has_result) { backend.generate() }
    }
    onClosed: backend.cancel()
    onStateChanged: updateResult()
    function clearView() {
        displayedToken = ""
        viewport = ({points: [], x_ticks: [], y_ticks: [], axis_label: dialog.state.axis_label || "", error: dialog.state.reason || "No retained observations"})
    }
    function updateResult() {
        if (dialog.state.has_result && dialog.state.result_token !== displayedToken) {
            displayedToken = dialog.state.result_token
            showView(0, 0)
        } else if (!dialog.state.has_result) {
            clearView()
        }
    }
    function showView(start, count) { viewport = JSON.parse(backend.view(start, count, captureFrequencyAxis)) }
    function zoom(factor) {
        if (!viewport.total) { return }
        const count = Math.max(2, Math.min(viewport.total, Math.round(viewport.count * factor)))
        const anchor = viewport.start + plot.selectedFraction * (viewport.count - 1)
        const start = Math.max(0, Math.min(viewport.total - count, Math.round(anchor - count / 2)))
        showView(start, count)
    }
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            Layout.fillWidth: true
            Button { objectName: "saGenerate"; text: "Run simulation"; enabled: dialog.state.can_generate; onClicked: backend.generate()
                ToolTip.visible: hovered
                ToolTip.text: dialog.generateDescription + "\n" + (dialog.viewport.error || dialog.state.message || dialog.state.reason)
                Accessible.description: dialog.viewport.error || dialog.state.message || dialog.state.reason
            }
            Button { objectName: "saCancel"; text: "Cancel"; enabled: dialog.state.busy; onClicked: backend.cancel() }
            Item { Layout.fillWidth: true }
            Button { objectName: "saZoomIn"; text: "Zoom in"; enabled: dialog.state.has_result; onClicked: dialog.zoom(0.5) }
            Button { objectName: "saZoomOut"; text: "Zoom out"; enabled: dialog.state.has_result; onClicked: dialog.zoom(2) }
            Button { objectName: "saFullSpan"; text: "Full span"; enabled: dialog.state.has_result; onClicked: dialog.showView(0, 0) }
        }
        Label { text: dialog.viewport.summary || (dialog.state.tap_description || "Complex baseband I/Q sample snapshot"); color: theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        RowLayout {
            visible: dialog.tapOptions.length > 0
            Label { text: "Test Point"; color: dialog.theme.text }
            ComboBox { objectName: "saTestPoint"; model: dialog.tapOptions; currentIndex: dialog.selectedTap; Layout.preferredWidth: 280; onActivated: function(index) { dialog.tapRequested(index) } }
            Label { text: dialog.state.reason || ""; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 11 }
        }
        RowLayout {
            Layout.fillWidth: true
            Label { text: dialog.frequencyAxisAvailable ? "Frequency axis" : "Physical FFT frequency grid"; color: theme.text }
            ComboBox {
                objectName: "saFrequencyAxis"
                visible: dialog.frequencyAxisAvailable
                enabled: dialog.state.has_result || false
                Layout.preferredWidth: 235
                model: ["Complex baseband", "Capture frequency mapping"]
                currentIndex: dialog.captureFrequencyAxis ? 1 : 0
                onActivated: function(index) {
                    dialog.captureFrequencyAxis = index === 1
                    if (dialog.state.has_result) { dialog.showView(dialog.viewport.start, dialog.viewport.count) }
                }
            }
            Label { text: "Tap: " + (dialog.state.tap_id || "—"); color: theme.muted; Layout.fillWidth: true; elide: Text.ElideRight }
        }
        SpectrumPlot { id: plot; theme: dialog.theme; viewport: dialog.viewport; Layout.fillWidth: true; Layout.fillHeight: true }
        SymbolTimingPanel {
            objectName: "saTimingPanel"
            Layout.fillWidth: true; Layout.preferredHeight: 210
            visible: dialog.showTiming
            theme: dialog.theme; backend: dialog.backend
            hasResult: dialog.showTiming && (dialog.state.has_result || false)
            resultToken: dialog.state.result_token || ""
        }

    }
    footer: DialogButtonBox { standardButtons: DialogButtonBox.Close; onRejected: dialog.close() }
}
