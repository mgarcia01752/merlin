/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
RowLayout {
    id: viewer
    required property var theme
    required property var backend
    required property var settingsBackend
    required property bool rxmer
    required property var options
    property var files: []
    property int selectedIndex: -1
    property int metric: 0
    property string symbol: "0"
    property var viewport: ({points: [], x_ticks: [], y_ticks: []})
    function refresh(start, count) {
        viewport = JSON.parse(backend.view(rxmer, JSON.stringify({index: selectedIndex, metric: metric, symbol: symbol, start: start || 0, count: count || 0, options: options}), settingsBackend.current()))
    }
    function reset() { refresh(0, 0) }
    spacing: 12
    ColumnLayout {
        Layout.preferredWidth: 285; Layout.fillHeight: true
        WorkbenchActionButton { theme: viewer.theme; objectName: "measurementEffective"; Layout.fillWidth: true; text: viewer.rxmer ? "Effective RxMER / playback" : "Channel response H"; onClicked: { viewer.selectedIndex = -1; viewer.reset() } }
        MeasurementCaptureTree { Layout.fillWidth: true; Layout.fillHeight: true; theme: viewer.theme; files: viewer.files; selectedIndex: viewer.selectedIndex; onSelected: function(index) { viewer.selectedIndex = index; viewer.reset() } }
        Label { Layout.fillWidth: true; text: !viewer.rxmer && !viewer.files.length ? "Flat / unity H = 1 + 0i\nSynthetic · 0 dB gain / 0° phase" : viewer.files.length + " capture(s)"; color: theme.muted; font.pixelSize: 11; wrapMode: Text.WordWrap }
    }
    Rectangle { width: 1; Layout.fillHeight: true; color: theme.line }
    ColumnLayout {
        Layout.fillWidth: true; Layout.fillHeight: true
        RowLayout {
            Layout.fillWidth: true
            Label { text: viewer.selectedIndex < 0 ? (viewer.rxmer ? "Effective residual model" : "Channel response") : "Original capture " + (viewer.selectedIndex + 1); color: theme.accent; Layout.fillWidth: true }
            ComboBox { objectName: "measurementMetric"; visible: !viewer.rxmer; model: ["Gain (dB)", "Phase (degrees)", "Real H", "Imaginary H"]; onActivated: function(index) { viewer.metric = index; viewer.reset() } }
            WorkbenchActionButton { theme: viewer.theme; objectName: "measurementFullSpan"; text: "Full Span"; onClicked: viewer.reset() }
        }
        MeasurementPlot { Layout.fillWidth: true; Layout.fillHeight: true; theme: viewer.theme; viewport: viewer.viewport; onViewportRequested: function(start, count) { viewer.refresh(start, count) } }
    }
}
