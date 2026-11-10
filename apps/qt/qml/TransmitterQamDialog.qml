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
    property int signalIndex:0
    property var observation:({available:false,points:[]})
    function refresh(){observation=JSON.parse(backend.stage([4,5,7,8][signalIndex],0))}
    onOpened:refresh()
    title:"QAM Analyzer · Transmitter"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(960,parent ? parent.width-40 : 960)
    height:Math.min(650,parent ? parent.height-40 : 650)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        RowLayout {
            Label { text:"Signal";color:dialog.theme.text }
            ComboBox { objectName:"txQamSignal";Layout.fillWidth:true;model:["Mapped Cells · Data / NCP","Interleaved Cells · Data / NCP","PLC Data","PLC Preamble"];onActivated:function(index){dialog.signalIndex=index;dialog.refresh()} }
        }
        Label { visible:!dialog.observation.available;text:dialog.observation.reason || "No I/Q data";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        TransmitterScatter { theme:dialog.theme;observation:dialog.observation;Layout.fillWidth:true;Layout.fillHeight:true }
        Label { visible:dialog.observation.available;text:"Showing "+(dialog.observation.points || []).length+" / "+dialog.observation.iq_count+" samples · missing cells: "+dialog.observation.missing;color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    }
}
