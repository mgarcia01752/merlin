/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "../../../shared/instruments"
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"

WorkbenchDialog {
    id: dialog
    objectName: "spectrumAnalyzerDialog"
    required property var backend
    required property var state
    readonly property var viewport: presentation.value ? presentation.value.viewport : ({points:[],x_ticks:[],y_ticks:[]})
    readonly property string displayedToken: presentation.value ? presentation.value.token : ""
    readonly property var displayedContext: presentation.value ? presentation.value.context : ({})
    readonly property var display: presentation.value ? presentation.value.display : ({})
    property PresentationOwner presentationCoverage: PresentationOwner {id:presentation;theme:dialog.theme}
    property bool captureFrequencyAxis: false
    property bool frequencyAxisAvailable: true
    property bool autoGenerate: true
    property bool showTiming: true
    property var tapOptions: []
    property int selectedTap: 0
    property string generateDescription: "Run the transmitter preview and recalculate its spectrum."
    signal tapRequested(int index)
    title: dialog.display.title || "Spectrum Analyzer"
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
    // Coalesce one parent transaction before querying its admitted tap context.
    onStateChanged: Qt.callLater(dialog.updateResult)
    /**
     * Admit the spectrum and its exact token/tap/axis context before publication.
     * @param value Unmodified bounded backend viewport.
     * @param token Empty for the existing no-result view, otherwise result identity.
     * @returns False preserves old samples, axes, selector labels and title.
     */
    function adoptViewport(value,token) {
        const context = {state:state,tapOptions:tapOptions,selectedTap:selectedTap,captureAxis:captureFrequencyAxis}
        return presentation.replace({viewport:value,token:token,context:context,display:{
            title:"Spectrum Analyzer · "+(state.tap_name || "OFDM Samples"),
            timingToken:(state.result_token || "")+"/"+(state.configuration_token || ""),
            generate:generateDescription+"\n"+(value.error || state.message || state.reason)
        }})
    }
    /** Admit the existing empty view; failure preserves the previous observation. */
    function clearView() {
        return adoptViewport({points:[],x_ticks:[],y_ticks:[],axis_label:state.axis_label || "",error:state.reason || "No retained observations"},"")
    }
    /** Refresh only a changed backend token; do not publish its identity early. */
    function updateResult() {
        if (state.has_result && state.result_token !== displayedToken) {
            return showView(0,0)
        }
        if (!state.has_result) {
            return clearView()
        }
        return true
    }
    /** Query through the existing backend and admit before replacing a viewport. */
    function showView(start,count) {
        return adoptViewport(JSON.parse(backend.view(start,count,captureFrequencyAxis)),state.result_token || "")
    }
    /** Keep the displayed axis context unchanged when a new axis is rejected. */
    function selectFrequencyAxis(capture) {
        if (captureFrequencyAxis === capture) {
            return
        }
        const previous=captureFrequencyAxis
        captureFrequencyAxis=capture
        if (state.has_result && !showView(viewport.start,viewport.count)) {
            captureFrequencyAxis=previous
        }
    }
    Component.onCompleted: updateResult()
    ButtonGroup { id: frequencyAxes }
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            Layout.fillWidth: true
            Label { visible: (dialog.displayedContext.tapOptions || []).length > 0; text: "Test Point"; color: dialog.theme.text }
            ComboBox { visible: (dialog.displayedContext.tapOptions || []).length > 0; objectName: "saTestPoint"; model: dialog.displayedContext.tapOptions || []; currentIndex: dialog.displayedContext.selectedTap || 0; Layout.preferredWidth: 280; onActivated: function(index) { dialog.tapRequested(index) } }
            Item { Layout.fillWidth: true }
            CheckBox {
                id: detailsToggle
                objectName: "saDetailsToggle"
                text: "Details"
                checked: true
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered
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
        Label {visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        SpectrumPlot { id: plot; theme: dialog.theme; viewport: dialog.viewport; detailsText: dialog.viewport.summary || ""; showDetails: detailsToggle.checked; zoomEnabled: dialog.state.has_result; onViewportRequested: function(start, count) { dialog.showView(start, count) } Layout.fillWidth: true; Layout.fillHeight: true }
        Label { objectName:"saPlcSummary";visible:!!dialog.viewport.plc_summary;text:dialog.viewport.plc_summary || "";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
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
            resultToken: dialog.display.timingToken || ""
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
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered
                ToolTip.text: dialog.display.generate || ""
                Accessible.description: dialog.generateDescription
            }
            WorkbenchActionButton {
                theme: dialog.theme
                objectName: "saCancel"; text: "Cancel preview"
                visible: dialog.state.busy || false
                onClicked: backend.cancel()
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered
                ToolTip.text: "Cancel this TX preview. The main simulation uses Stop on the workbench."
            }
            Item { Layout.fillWidth: true }
            WorkbenchActionButton { theme: dialog.theme; objectName: "saClose"; text: "Close"; onClicked: dialog.close() }
        }
    }
}
