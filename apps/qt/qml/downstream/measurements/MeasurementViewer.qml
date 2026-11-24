/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/components"
import "../../shared/presentation"
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
    property bool contextAvailable: true
    readonly property bool animationActive: !!displayedContext && displayedContext.animationActive
    property bool previewRequested: false
    readonly property real capturePaneWidth: Math.max(285,Math.min(400,width*0.34))
    signal selectionToggled(string identity, bool checked)
    property int selectedIndex: -1
    property int metric: 0
    property string symbol: "0"
    readonly property var viewport: presentation.value ? presentation.value.viewport : ({})
    readonly property var display: presentation.value ? presentation.value.display : ({})
    readonly property var displayedContext: presentation.value ? presentation.value.context : null
    readonly property bool contextCurrent: !!displayedContext
        && displayedContext.files === files && displayedContext.options === options
        && displayedContext.selectionCount === selectionCount
        && displayedContext.selectedIndex === selectedIndex
        && displayedContext.metric === metric && displayedContext.symbol === symbol
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: viewer.theme
        initialValue: viewer.viewportCandidate({points:[],x_ticks:[],y_ticks:[]},false)
    }
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    function stopPreview() {
        playbackControls.playing = false
        previewRequested = false
    }

    // The accepted data and its display context move together after admission.
    function viewportCandidate(candidate,preview) {
        const context={animationActive:preview,selectedIndex:selectedIndex,metric:metric,symbol:symbol,selectionCount:selectionCount,files:files,options:options}
        const frame=candidate.frame_index === undefined ? 0 : candidate.frame_index
        return {viewport:candidate,context:context,display:{
            title:preview ? "Capture preview · "+(frame+1)+" / "+selectionCount : selectedIndex<0 ? (rxmer ? "Effective residual model" : "Channel response") : ((files[selectedIndex] || {}).designed ? "Designed capture " : "Original capture ")+(selectedIndex+1),
            files:!rxmer && !files.length ? "Flat / unity H = 1 + 0i\nSynthetic · 0 dB gain / 0° phase" : files.length+" capture(s)",
            time:(preview && candidate.capture_time!==undefined ? (candidate.synthetic_time ? "Synthetic association epoch: " : "Capture UTC: ")+new Date(candidate.capture_time*1000).toISOString().replace("T"," ").replace(".000Z","")+" · " : "")+"Capture preview speed is independent of the simulation interval.",
            metrics:["Gain (dB)","Phase (degrees)","Real H","Imaginary H"]
        }}
    }
    function replaceViewport(candidate, preview) {
        if (!presentation.replace(viewportCandidate(candidate,preview))) {
            playbackControls.playing = false
            if (displayedContext) {
                selectedIndex=displayedContext.selectedIndex
                metric=displayedContext.metric
                symbol=displayedContext.symbol
            }
            return false
        }
        previewRequested = preview
        return true
    }
    function showFrame(position) {
        if (!contextAvailable) {return false}
        const candidate = JSON.parse(backend.preview_frame(JSON.stringify(options),position,viewport.start || 0,viewport.count || 0))
        if (!replaceViewport(candidate, true)) {
            return false
        }
        if (viewport.error) {
            playbackControls.playing = false
        }
        return true
    }
    function refresh(start, count) {
        if (!contextAvailable) {return false}
        if (previewRequested) {
            const candidate = JSON.parse(backend.preview_frame(JSON.stringify(options),playbackControls.frameIndex,start || 0,count || 0))
            return replaceViewport(candidate, true)
        }
        const candidate = JSON.parse(backend.view(rxmer, JSON.stringify({index: selectedIndex, metric: metric, symbol: symbol, start: start || 0, count: count || 0, options: options}), settingsBackend.current()))
        return replaceViewport(candidate, false)
    }
    function reset() { return refresh(0, 0) }
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
        Label { Layout.fillWidth: true; text: viewer.display.files || ""; color: theme.muted; font.pixelSize: 11; wrapMode: Text.WordWrap }
    }
    Rectangle { width: 1; Layout.fillHeight: true; color: theme.line }
    ColumnLayout {
        Layout.fillWidth: true; Layout.fillHeight: true
        RowLayout {
            Layout.fillWidth: true
            Label { text: viewer.display.title || ""; color: theme.accent; Layout.fillWidth: true }
            ComboBox { objectName: "measurementMetric"; currentIndex:viewer.metric;visible: !viewer.rxmer; model: viewer.display.metrics || []; onActivated: function(index) { viewer.metric = index; viewer.reset() } }
            WorkbenchActionButton {theme:viewer.theme;objectName:"measurementZoomPlc";visible:viewer.rxmer;enabled:!!viewer.viewport.plc_view;text:"Zoom to PLC";onClicked:viewer.refresh(viewer.viewport.plc_view.start,viewer.viewport.plc_view.count)}
            WorkbenchActionButton { theme: viewer.theme; objectName: "measurementFullSpan"; text: "Full Span"; onClicked: viewer.reset() }
        }
        MeasurementPlot { Layout.fillWidth: true; Layout.fillHeight: true; theme: viewer.theme; viewport: viewer.viewport; onViewportRequested: function(start, count) { viewer.refresh(start, count) } }
        Label {visible:viewer.rxmer && text.length>0;text:viewer.viewport.plc_summary || "";color:viewer.theme.muted;font.pixelSize:11;Layout.fillWidth:true}
        Label {
            visible: presentation.rejected
            text: presentation.message
            color: viewer.theme.statusAttention
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
        }
        MeasurementPlaybackControls {
            id:playbackControls;objectName:"measurementPlaybackControls"
            visible:viewer.rxmer;Layout.fillWidth:true;theme:viewer.theme
            frameCount:viewer.displayedContext ? viewer.displayedContext.selectionCount : 0
            frameIndex:viewer.animationActive && viewer.viewport.frame_index !== undefined ? viewer.viewport.frame_index : 0
            suspended:!viewer.previewEnabled || !viewer.contextCurrent || presentation.rejected
            onFrameRequested:function(position) {viewer.showFrame(position)}
        }
        Slider {
            objectName:"rxmerPreviewPosition";visible:viewer.rxmer;Layout.fillWidth:true
            from:0;to:Math.max(1,playbackControls.frameCount-1);stepSize:1;snapMode:Slider.SnapAlways
            value:playbackControls.frameIndex;enabled:viewer.previewEnabled && viewer.contextCurrent && !presentation.rejected && playbackControls.frameCount>1
            Accessible.name:"Capture preview position"
            onMoved:playbackControls.jump(Math.round(value))
        }
        Label {
            visible:viewer.rxmer;Layout.fillWidth:true;Layout.minimumWidth:0;font.pixelSize:11;color:viewer.theme.muted;wrapMode:Text.WordWrap
            text:viewer.display.time || ""
        }
    }
}
