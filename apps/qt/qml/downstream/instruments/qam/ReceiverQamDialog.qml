/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"
import "../../../shared/instruments"

WorkbenchDialog {
    id: dialog
    required property var backend
    signal contextHelpRequested(string code)
    property bool rawOnly: false
    property alias signalIndex: selection.signalIndex
    property alias viewIndex: selection.viewIndex
    property alias plcPartIndex: selection.plcPartIndex
    property alias selectionControls: selection
    readonly property var observation: presentation.value ? presentation.value.observation : ({})
    readonly property var display: presentation.value ? presentation.value.display : ({})
    readonly property string signalName: display.signalName || "Unavailable"
    property bool restoringSelection: false
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
    }

    /**
     * Couple the original receiver DTO, selection and retained display strings.
     * @param value Unmodified backend response; plot bindings share this owner.
     * @param query Original ReceiverCellControls query for those samples.
     * @returns Plain candidate admitted before any persistent replacement.
     */
    function qamCandidate(value, query) {
        const name = ["Data Profile", "NCP", "PLC", "Continuous Pilots", "Scattered Pilots"][query.signal] || "Unavailable"
        return {observation:value,query:query,
            views:query.view===2 ? ["Raw · RX FFT","Equalized · Before Slicing"] : [query.view===0 ? "Raw · RX FFT" : "Equalized · Before Slicing"],
            display:{signalName:name,title:"QAM Analyzer · "+name,
                correction:rawOnly ? "Received FFT cells · Before equalization" : value.correction_name ? "Recorded run · Equalizer: "+value.correction_name : "Equalizer unavailable · No recorded run selected",
                equalizedTitle:"Equalized · "+(value.correction_mode || "Unavailable")+" · Before Slicing",
                cells:value.available ? value.points.length+" cells · I / Q" : "No I/Q data",
                detail:value.available && value.retained>value.points.length ? "Showing "+value.points.length+" of "+value.retained+" available cells." : ""}}
    }

    /**
     * Restore accepted scalar selectors without dispatching intermediate queries.
     * @param query Previously admitted selection.
     * @returns No result; actual scientific data remains unchanged.
     */
    function restoreSelection(query) {
        restoringSelection = true
        selection.resetting = true
        selection.signalIndex = query.signal
        selection.viewIndex = query.view
        selection.plcPartIndex = query.part
        selection.runProfile = query.run_profile
        selection.profile = query.profile
        selection.order = query.order
        selection.firstBin = query.first_bin
        selection.lastBin = query.last_bin
        selection.firstSymbol = query.first_symbol
        selection.lastSymbol = query.last_symbol
        selection.resetting = false
        restoringSelection = false
    }

    /**
     * Admit the original response and its query together before displaying it.
     * @returns False preserves old samples/context and restores their selectors.
     */
    function refresh() {
        if (!backend || !selection || restoringSelection) {
            return false
        }
        const query = selection.query()
        const value = JSON.parse(backend.qam_selected(JSON.stringify(query)))
        if (!presentation.replace(qamCandidate(value,query))) {
            if (presentation.value) {
                restoreSelection(presentation.value.query)
            }
            return false
        }
        return true
    }
    Component.onCompleted: presentation.replace(qamCandidate({points:[],profiles:[]},selection.query()))
    onOpened: refresh()
    title: display.title || "QAM Analyzer"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1080, parent ? parent.width - 40 : 1080)
    height: Math.min(690, parent ? parent.height - 40 : 690)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 12
        ReceiverCellControls {
            id:selection;theme:dialog.theme;metadata:dialog.observation;showView:!dialog.rawOnly
            Layout.fillWidth:true
            onSelectionChanged:if(!dialog.restoringSelection){dialog.refresh()}
        }
        Label {objectName:"receiverQamRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {
            objectName: "receiverQamCorrection"
            text: dialog.display.correction || ""
            color: dialog.theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap
            HoverHandler { id: correctionHover }
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: correctionHover.hovered
            ToolTip.text: dialog.observation.correction_help || "Run the simulation to record the receiver correction mode."
            Accessible.description: ToolTip.text
        }
        RowLayout {
            Layout.fillWidth: true; Layout.fillHeight: true; spacing: 12
            Repeater {
                model: presentation.value ? presentation.value.views : []
                Rectangle {
                    required property string modelData
                    Layout.fillWidth: true; Layout.fillHeight: true
                    radius: 8; color: dialog.theme.background; border.color: dialog.theme.line
                    ColumnLayout {
                        anchors.fill: parent; anchors.margins: 18
                        Label {
                            objectName: modelData.indexOf("Raw")===0 ? "receiverRawTitle" : "receiverEqualizedTitle"
                            text: modelData.indexOf("Raw")===0 ? modelData
                                : dialog.display.equalizedTitle || ""
                            color: dialog.theme.text; font.bold: true; Layout.fillWidth:true; wrapMode:Text.WordWrap
                        }
                        Label {
                            id:merReading
                            objectName:modelData.indexOf("Raw")===0 ? "receiverRawMer" : "receiverEqualizedMer"
                            readonly property var reading:modelData.indexOf("Raw")===0 ? (dialog.observation.raw_reference_mer || {}) : (dialog.observation.equalized_reference_mer || {})
                            text:reading.text || "TX-reference MER unavailable"
                            color:dialog.theme.accent
                            Layout.fillWidth:true
                            wrapMode:Text.WordWrap
                            HoverHandler {id:merHover}
                            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:merHover.hovered
                            ToolTip.text:merReading.reading.help || "Select retained cells to calculate MER."
                        }
                        Label {
                            text:((modelData.indexOf("Raw")===0 ? dialog.observation.raw_mer : dialog.observation.equalized_mer) || {}).text || ""
                            color:dialog.theme.muted;font.pixelSize:11
                            Layout.fillWidth:true;wrapMode:Text.WordWrap
                            HoverHandler {id:estimateHover}
                            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:estimateHover.hovered
                            ToolTip.text:"Nearest ideal point estimate. Dense constellations and incorrect decisions can make it too high, even when received symbols are badly impaired."
                        }
                        QamPlot {
                            objectName: modelData.indexOf("Raw") === 0 ? "receiverRawScatter" : "receiverEqualizedScatter"
                            Layout.fillWidth: true; Layout.fillHeight: true
                            theme: dialog.theme
                            observation: dialog.observation
                            xCoordinate: modelData.indexOf("Raw") === 0 ? "raw_x" : "eq_x"
                            yCoordinate: modelData.indexOf("Raw") === 0 ? "raw_y" : "eq_y"
                            receivedSamples: true
                            roleMarkers: false
                            showAxesWithoutData: false
                            plotInset: 0
                            axisFontPixels: 11
                            horizontalLabelInset: 10
                            verticalLabelOffset: 12
                        }
                        ContextualStatus { theme:dialog.theme;diagnostic:dialog.observation.diagnostic || null;onHelpRequested:function(code){dialog.contextHelpRequested(code)};objectName: "receiverQamUnavailable"; visible: !dialog.observation.available; text: dialog.observation.reason || "No retained receiver results"; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; horizontalAlignment: Text.AlignHCenter }
                        Item { visible: !dialog.observation.available; Layout.fillHeight: true }
                        Label { text: dialog.display.cells || ""; color: dialog.theme.muted; Layout.alignment: Qt.AlignHCenter; font.pixelSize: 11 }
                    }
                }
            }
        }
        Label { objectName: "receiverQamDetail"; visible: text.length > 0; text: dialog.display.detail || ""; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
    }
}
