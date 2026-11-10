/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

WorkbenchDialog {
    id:dialog
    required property var backend
    property int stageIndex:0
    property int pageOffset:0
    property var observation:({rows:[],available:false,reason:""})
    function refresh(){ observation=JSON.parse(backend.stage(stageIndex,pageOffset)) }
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
            ComboBox { objectName:"txStageSelection";Layout.fillWidth:true;model:["Source Bits","LDPC Codeword","Interleaved Bits","Cell Labels","Mapped Cells · Data / NCP","Interleaved Cells · Data / NCP","Physical FFT Bins","PLC Data I/Q","PLC Preamble I/Q"];currentIndex:dialog.stageIndex;onActivated:function(index){dialog.stageIndex=index;dialog.pageOffset=0;dialog.refresh()} }
        }
        Label { visible:dialog.observation.available;text:"Symbol: "+(dialog.observation.symbol===null ? "—" : dialog.observation.symbol)+" · "+(dialog.observation.phase || "")+" · Captured: "+dialog.observation.retained+" / "+dialog.observation.total;color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        Label { visible:!dialog.observation.available;text:dialog.observation.reason || "";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        Label { text:"Index                         Value";color:dialog.theme.accent;font.bold:true }
        ListView {
            Layout.fillWidth:true;Layout.fillHeight:true;clip:true;spacing:3
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
            Button { objectName:"txPreviousPage";text:"Previous";enabled:dialog.pageOffset>0;onClicked:{dialog.pageOffset=Math.max(0,dialog.pageOffset-(dialog.stageIndex<3 ? 2048 : 64));dialog.refresh()} }
            Button { objectName:"txNextPage";text:"Next";enabled:dialog.observation.available && dialog.observation.end<dialog.observation.retained;onClicked:{dialog.pageOffset=dialog.observation.end;dialog.refresh()} }
            Label { text:dialog.observation.available ? dialog.pageOffset+"–"+(dialog.observation.end-1) : "";color:dialog.theme.muted }
            Item { Layout.fillWidth:true }
        }
    }
}
