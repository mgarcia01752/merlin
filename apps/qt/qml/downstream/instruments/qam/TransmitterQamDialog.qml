/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"

WorkbenchDialog {
    id:dialog
    required property var backend
    property int signalIndex:0
    readonly property var observation: presentation.value ? presentation.value.observation : ({})
    readonly property int displayedSignal: presentation.value ? presentation.value.signalIndex : 0
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.qamCandidate({available:false,points:[]},0,dialog.signalLabels())
    }

    /**
     * Provide the existing compiled choices for initial admission or retry.
     * @returns Unchanged signal labels; accepted refreshes reuse their alias.
     */
    function signalLabels() {
        return ["Mapped Cells · Data / NCP","Interleaved Cells · Data / NCP","PLC Data","PLC Preamble"]
    }

    /**
     * Couple the unmodified I/Q DTO, selector and its existing summary text.
     * @param value Original backend response.
     * @param signal Requested selector index.
     * @param labels Existing admitted labels or initial compiled choices.
     * @returns Plain candidate, with no copied scientific points.
     */
    function qamCandidate(value, signal, labels) {
        return {observation:value,signalIndex:signal,labels:labels,
            summary:"Showing "+(value.points || []).length+" / "+value.iq_count+" samples · missing cells: "+value.missing}
    }

    /**
     * Admit the complete response before changing its displayed signal context.
     * @returns False retains the previous response and restores the selector.
     */
    function refresh() {
        const signal = signalIndex
        const value = JSON.parse(backend.stage([4,5,7,8][signal],0))
        const labels = presentation.value ? presentation.value.labels : signalLabels()
        if (!presentation.replace(qamCandidate(value,signal,labels))) {
            if (presentation.value) {
                signalIndex = presentation.value.signalIndex
            }
            return false
        }
        return true
    }
    onOpened:refresh()
    title:"QAM Analyzer · Transmitter"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(960,parent ? parent.width-40 : 960)
    height:Math.min(650,parent ? parent.height-40 : 650)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        RowLayout {
            Label { text:"Signal";color:dialog.theme.text }
            ComboBox { objectName:"txQamSignal";Layout.fillWidth:true;model:presentation.value ? presentation.value.labels : [];currentIndex:dialog.signalIndex;onActivated:function(index){dialog.signalIndex=index;dialog.refresh()} }
        }
        Label {objectName:"txQamRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { visible:!dialog.observation.available;text:dialog.observation.reason || "No I/Q data";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        RowLayout {
            objectName: "txQamLegend"
            visible: dialog.observation.available && dialog.displayedSignal < 2
            Layout.fillWidth: true; spacing: 18
            Label { objectName: "txQamDataLegend"; text: "■ Data"; color: dialog.theme.qamData }
            Label { objectName: "txQamNcpLegend"; text: "● NCP"; color: dialog.theme.qamNcp }
            Label { text: "◆ Filler"; color: dialog.theme.qamFiller; visible: (dialog.observation.role_counts || {}).filler > 0 }
            Label { text: "■ Unclassified"; color: dialog.theme.qamUnknown; visible: (dialog.observation.role_counts || {}).unknown > 0 }
            Item { Layout.fillWidth: true }
        }
        TransmitterScatter { objectName: "txQamScatter"; theme:dialog.theme;observation:dialog.observation;Layout.fillWidth:true;Layout.fillHeight:true }
        Label { visible:dialog.observation.available;text:presentation.value ? presentation.value.summary : "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    }
}
