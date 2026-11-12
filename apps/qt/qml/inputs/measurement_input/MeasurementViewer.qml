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
    property int selectionCount: 0
    property bool previewEnabled: false
    property bool animationActive: false
    readonly property real capturePaneWidth: Math.max(285,Math.min(400,width*0.34))
    signal selectionToggled(string identity, bool checked)
    property int selectedIndex: -1
    property int metric: 0
    property string symbol: "0"
    property var viewport: ({points: [], x_ticks: [], y_ticks: []})
    function stopPreview() { playbackControls.playing=false;animationActive=false }
    function showFrame(position) {
        animationActive=true
        viewport=JSON.parse(backend.preview_frame(JSON.stringify(options),position,viewport.start || 0,viewport.count || 0))
        if (viewport.error) { playbackControls.playing=false }
    }
    function refresh(start, count) {
        if (animationActive) {
            viewport=JSON.parse(backend.preview_frame(JSON.stringify(options),playbackControls.frameIndex,start || 0,count || 0));return
        }
        viewport = JSON.parse(backend.view(rxmer, JSON.stringify({index: selectedIndex, metric: metric, symbol: symbol, start: start || 0, count: count || 0, options: options}), settingsBackend.current()))
    }
    function reset() { refresh(0, 0) }
    spacing: 12
    ColumnLayout {
        Layout.preferredWidth: viewer.capturePaneWidth; Layout.minimumWidth: viewer.capturePaneWidth
        Layout.maximumWidth: viewer.capturePaneWidth; Layout.fillHeight: true
        WorkbenchActionButton { theme: viewer.theme; objectName: "measurementEffective"; Layout.fillWidth: true; text: viewer.rxmer ? "Effective RxMER / playback" : "Channel response H"; onClicked: { viewer.stopPreview();viewer.selectedIndex = -1; viewer.reset() } }
        MeasurementCaptureTree { Layout.fillWidth: true; Layout.fillHeight: true; theme: viewer.theme; files: viewer.files; selectedIndex: viewer.animationActive && viewer.viewport.source_index !== undefined ? viewer.viewport.source_index : viewer.selectedIndex;
            selectionEnabled:viewer.rxmer;singleShot:viewer.options.mode === "single"
            selectedCaptures:viewer.options.selected_captures === undefined ? null : viewer.options.selected_captures
            onSelectionToggled:function(identity,checked) {viewer.selectionToggled(identity,checked)}
            onSelected: function(index) { viewer.stopPreview();viewer.selectedIndex = index; viewer.reset() } }
        Label { Layout.fillWidth: true; text: !viewer.rxmer && !viewer.files.length ? "Flat / unity H = 1 + 0i\nSynthetic · 0 dB gain / 0° phase" : viewer.files.length + " capture(s)"; color: theme.muted; font.pixelSize: 11; wrapMode: Text.WordWrap }
    }
    Rectangle { width: 1; Layout.fillHeight: true; color: theme.line }
    ColumnLayout {
        Layout.fillWidth: true; Layout.fillHeight: true
        RowLayout {
            Layout.fillWidth: true
            Label { text: viewer.animationActive ? "Capture preview · "+(playbackControls.frameIndex+1)+" / "+playbackControls.frameCount : viewer.selectedIndex < 0 ? (viewer.rxmer ? "Effective residual model" : "Channel response") : ((viewer.files[viewer.selectedIndex] || {}).designed ? "Designed capture " : "Original capture ") + (viewer.selectedIndex + 1); color: theme.accent; Layout.fillWidth: true }
            ComboBox { objectName: "measurementMetric"; visible: !viewer.rxmer; model: ["Gain (dB)", "Phase (degrees)", "Real H", "Imaginary H"]; onActivated: function(index) { viewer.metric = index; viewer.reset() } }
            WorkbenchActionButton { theme: viewer.theme; objectName: "measurementFullSpan"; text: "Full Span"; onClicked: viewer.reset() }
        }
        MeasurementPlot { Layout.fillWidth: true; Layout.fillHeight: true; theme: viewer.theme; viewport: viewer.viewport; onViewportRequested: function(start, count) { viewer.refresh(start, count) } }
        MeasurementPlaybackControls {
            id:playbackControls;objectName:"measurementPlaybackControls"
            visible:viewer.rxmer;Layout.fillWidth:true;theme:viewer.theme
            frameCount:viewer.selectionCount
            frameIndex:viewer.animationActive && viewer.viewport.frame_index !== undefined ? viewer.viewport.frame_index : 0
            suspended:!viewer.previewEnabled
            onFrameRequested:function(position) {viewer.showFrame(position)}
        }
        Slider {
            objectName:"rxmerPreviewPosition";visible:viewer.rxmer;Layout.fillWidth:true
            from:0;to:Math.max(1,playbackControls.frameCount-1);stepSize:1;snapMode:Slider.SnapAlways
            value:playbackControls.frameIndex;enabled:viewer.previewEnabled && playbackControls.frameCount>1
            Accessible.name:"Capture preview position"
            onMoved:playbackControls.jump(Math.round(value))
        }
        Label {
            visible:viewer.rxmer;Layout.fillWidth:true;Layout.minimumWidth:0;font.pixelSize:11;color:viewer.theme.muted;wrapMode:Text.WordWrap
            text:(viewer.animationActive && viewer.viewport.capture_time !== undefined ? (viewer.viewport.synthetic_time ? "Synthetic association epoch: " : "Capture UTC: ")+new Date(viewer.viewport.capture_time*1000).toISOString().replace("T"," ").replace(".000Z","")+" · " : "")+"Capture preview speed is independent of the simulation interval."
        }
    }
}
