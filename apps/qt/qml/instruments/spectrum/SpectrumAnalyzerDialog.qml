/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"

WorkbenchDialog {
    id: dialog
    objectName: "spectrumAnalyzerDialog"
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
    property string generateDescription: "Run the transmitter preview and recalculate its spectrum."
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
    function selectFrequencyAxis(capture) {
        if (captureFrequencyAxis === capture) { return }
        captureFrequencyAxis = capture
        if (state.has_result) { showView(viewport.start, viewport.count) }
    }
    ButtonGroup { id: frequencyAxes }
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            Layout.fillWidth: true
            Label { visible: dialog.tapOptions.length > 0; text: "Test Point"; color: dialog.theme.text }
            ComboBox { visible: dialog.tapOptions.length > 0; objectName: "saTestPoint"; model: dialog.tapOptions; currentIndex: dialog.selectedTap; Layout.preferredWidth: 280; onActivated: function(index) { dialog.tapRequested(index) } }
            Item { Layout.fillWidth: true }
            CheckBox {
                id: detailsToggle
                objectName: "saDetailsToggle"
                text: "Details"
                checked: true
                ToolTip.visible: hovered
                ToolTip.text: "Show acquisition details below the plot"
            }
            WorkbenchActionButton { theme: dialog.theme; objectName: "saFullSpan"; text: "Full Span"; enabled: dialog.state.has_result; onClicked: dialog.showView(0, 0) }
        }
        RowLayout {
            Layout.fillWidth: true
            visible: dialog.frequencyAxisAvailable
            RadioButton {
                objectName: "saBasebandAxis"
                text: "Complex baseband"
                ButtonGroup.group: frequencyAxes
                enabled: dialog.state.has_result || false
                checked: !dialog.captureFrequencyAxis
                onClicked: dialog.selectFrequencyAxis(false)
            }
            RadioButton {
                objectName: "saCaptureAxis"
                text: "Capture frequency mapping"
                ButtonGroup.group: frequencyAxes
                enabled: dialog.state.has_result || false
                checked: dialog.captureFrequencyAxis
                onClicked: dialog.selectFrequencyAxis(true)
            }
            Item { Layout.fillWidth: true }
        }
        SpectrumPlot { id: plot; theme: dialog.theme; viewport: dialog.viewport; detailsText: dialog.viewport.summary || ""; showDetails: detailsToggle.checked; zoomEnabled: dialog.state.has_result; onViewportRequested: function(start, count) { dialog.showView(start, count) } Layout.fillWidth: true; Layout.fillHeight: true }
        Label { objectName:"saChannelPower";visible:!!dialog.viewport.channel_power_text;text:dialog.viewport.channel_power_text || "";color:dialog.theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        Label {visible:!!dialog.viewport.level_warning;text:dialog.viewport.level_warning || "";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {visible:!!dialog.viewport.error;text:dialog.viewport.error || "";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        SymbolTimingPanel {
            id: timingPanel
            objectName: "saTimingPanel"
            Layout.fillWidth: true; Layout.preferredHeight: implicitHeight
            visible: dialog.showTiming
            theme: dialog.theme; backend: dialog.backend
            hasResult: dialog.showTiming && (dialog.state.has_result || false)
            resultToken: dialog.state.result_token || ""
        }

    }
    footer: Pane {
        objectName: "saFooter"
        padding: 10
        contentItem: RowLayout {
            spacing: 10
            WorkbenchActionButton {
                theme: dialog.theme
                objectName: "saGenerate"; text: "Run simulation"
                enabled: dialog.state.can_generate; onClicked: backend.generate()
                ToolTip.visible: hovered
                ToolTip.text: dialog.generateDescription + "\n" + (dialog.viewport.error || dialog.state.message || dialog.state.reason)
                Accessible.description: dialog.generateDescription
            }
            WorkbenchActionButton {
                theme: dialog.theme
                objectName: "saCancel"; text: "Cancel preview"
                visible: dialog.state.busy || false
                onClicked: backend.cancel()
                ToolTip.visible: hovered
                ToolTip.text: "Cancel this TX preview. The main simulation uses Stop on the workbench."
            }
            Item { Layout.fillWidth: true }
            WorkbenchActionButton { theme: dialog.theme; objectName: "saClose"; text: "Close"; onClicked: dialog.close() }
        }
    }
}
