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
    property int stageIndex:0
    property int pageOffset:0
    readonly property var observation: presentation.value ? presentation.value.observation : ({})
    readonly property var display: presentation.value ? presentation.value.display : ({})
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.stageCandidate({rows:[],available:false,reason:""},0,0,dialog.stageLabels())
    }

    /**
     * Provide the existing compiled selector labels for initial admission/retry.
     * @returns The unchanged stage choices; accepted refreshes reuse their alias.
     */
    function stageLabels() {
        return ["Source Bits · PRBS-TX","LDPC Codeword","Interleaved Bits","Cell Labels","Mapped Cells · Data / NCP","Interleaved Cells · Data / NCP","Physical FFT Bins","PLC Data I/Q","PLC Preamble I/Q"]
    }

    /**
     * Couple the response and its existing display strings with query context.
     * @param value Original backend DTO, without scientific reconstruction.
     * @param stage Requested stage index.
     * @param offset Requested page offset.
     * @param labels Independently retained selector strings under this same owner.
     * @returns Plain candidate measured before persistent replacement.
     */
    function stageCandidate(value, stage, offset, labels) {
        return {observation:value,stageIndex:stage,pageOffset:offset,labels:labels,display:{
            summary:"Symbol: "+(value.symbol===null ? "—" : value.symbol)+" · "+(value.phase || "")+" · Captured: "+value.retained+" / "+value.total+" · Missing: "+(value.missing || 0),
            omitted:"Role-band display limit reached · omitted "+(value.graph || {}).omitted_bands,
            page:value.available ? offset+"–"+(value.end-1) : ""
        }}
    }

    /**
     * Query the original backend and admit the complete response/context pair.
     * @returns False preserves old science/display context and restores controls.
     */
    function refresh() {
        const stage = stageIndex
        const offset = pageOffset
        const value = JSON.parse(backend.stage(stage,offset))
        const labels = presentation.value ? presentation.value.labels : stageLabels()
        if (!presentation.replace(stageCandidate(value,stage,offset,labels))) {
            if (presentation.value) {
                stageIndex = presentation.value.stageIndex
                pageOffset = presentation.value.pageOffset
            }
            return false
        }
        return true
    }
    onOpened:{pageOffset=0;refresh()}
    title:"Transmitter · Stage Inspector"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1000,parent ? parent.width-40 : 1000)
    height:Math.min(650,parent ? parent.height-40 : 650)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:12
        RowLayout {
            Layout.fillWidth:true
            Label { text:"Test Point";color:dialog.theme.text }
            ComboBox { objectName:"txStageSelection";Layout.fillWidth:true;model:presentation.value ? presentation.value.labels : [];currentIndex:dialog.stageIndex;onActivated:function(index){dialog.stageIndex=index;dialog.pageOffset=0;dialog.refresh()} }
        }
        Label {objectName:"txStageRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { visible:dialog.observation.available;text:dialog.display.summary || "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        Label { visible:!dialog.observation.available;text:dialog.observation.reason || "";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        TransmitterStageLegend {theme:dialog.theme;graph:dialog.observation.graph || ({});observation:dialog.observation;Layout.fillWidth:true}
        Label {visible:(dialog.observation.graph || {}).omitted_bands>0;text:dialog.display.omitted || "";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        ColumnLayout {
            Layout.fillWidth:true;Layout.fillHeight:true;spacing:10
            Label { visible:!!(dialog.observation.graph || {}).before;text:((dialog.observation.graph || {}).before || {}).title || "";color:dialog.theme.muted }
            TransmitterStagePlot { visible:!!(dialog.observation.graph || {}).before;Layout.fillWidth:true;Layout.fillHeight:true;theme:dialog.theme;observation:dialog.observation;graph:((dialog.observation.graph || {}).before || ({})) }
            Label { visible:!!(dialog.observation.graph || {}).before;text:(dialog.observation.graph || {}).title || "";color:dialog.theme.muted }
            TransmitterStagePlot {objectName:"txStageGraph";Layout.fillWidth:true;Layout.fillHeight:true;theme:dialog.theme;observation:dialog.observation}
        }
        CheckBox {id:details;objectName:"txStageDetails";text:"Details";checked:false}
        Label { visible:details.checked;text:"Index                         Value";color:dialog.theme.accent;font.bold:true }
        ListView {
            visible:details.checked;Layout.fillWidth:true;Layout.preferredHeight:130;clip:true;spacing:3
            model:dialog.observation.rows || [];ScrollBar.vertical:ScrollBar {}
            delegate:Rectangle {
                required property var modelData
                width:ListView.view.width;height:Math.max(32,row.implicitHeight+12);radius:5;color:dialog.theme.panel
                RowLayout {
                    id:row;anchors.left:parent.left;anchors.right:parent.right;anchors.margins:8;anchors.verticalCenter:parent.verticalCenter
                    Label { text:modelData.index;color:dialog.theme.muted;Layout.preferredWidth:90 }
                    Label { text:modelData.value;font.family:"monospace";color:dialog.theme.text;Layout.fillWidth:true;wrapMode:Text.WrapAnywhere }
                }
            }
        }
        RowLayout {
            WorkbenchActionButton { theme: dialog.theme; objectName:"txPreviousPage";text:"Previous";enabled:dialog.pageOffset>0;onClicked:{dialog.pageOffset=Math.max(0,dialog.pageOffset-(dialog.stageIndex===1 ? 2048 : dialog.stageIndex===0 || dialog.stageIndex===2 ? 128 : 64));dialog.refresh()} }
            WorkbenchActionButton { theme: dialog.theme; objectName:"txNextPage";text:"Next";enabled:dialog.observation.available && dialog.observation.end<dialog.observation.retained;onClicked:{dialog.pageOffset=dialog.observation.end;dialog.refresh()} }
            Label { text:dialog.display.page || "";color:dialog.theme.muted }
            Item { Layout.fillWidth:true }
        }
    }
}
