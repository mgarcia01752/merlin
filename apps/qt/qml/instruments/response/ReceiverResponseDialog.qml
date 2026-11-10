/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../spectrum"
WorkbenchDialog {
    id:dialog
    required property var backend
    property var runState: ({})
    property var response: ({points:[],x_ticks:[],y_ticks:[]})
    property string seenToken:""
    property int sourceIndex:0
    property int snapshotIndex:0
    property int viewIndex:0
    property int velocityPercent:87
    function refresh(){if (!backend){return};response=JSON.parse(backend.response(JSON.stringify({source:sourceIndex,snapshot:snapshotIndex,view:viewIndex,velocity_percent:velocityPercent})))}
    onOpened:refresh()
    onRunStateChanged: {if (opened && runState.result_token!==seenToken){seenToken=runState.result_token || "";refresh()} }
    title:"Channel Response / Echo"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1120,parent ? parent.width-40 : 1120);height:Math.min(690,parent ? parent.height-40 : 690)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:10
        RowLayout {
            Label { text:"Response";color:dialog.theme.text }
            ComboBox { objectName:"receiverResponseSource";model:["Applied Reference H","Receiver H-hat"];currentIndex:dialog.sourceIndex;Layout.preferredWidth:215;onActivated:function(index){dialog.sourceIndex=index;dialog.snapshotIndex=0;dialog.refresh()} }
            Label { text:"View";color:dialog.theme.text }
            ComboBox { objectName:"receiverResponseView";model:["Gain","Wrapped Phase","Real","Imaginary","Impulse / Echo"];currentIndex:dialog.viewIndex;Layout.preferredWidth:165;onActivated:function(index){dialog.viewIndex=index;dialog.refresh()} }
            Label { text:"VF (%)";color:dialog.theme.text }
            SpinBox { objectName:"receiverVelocity";from:1;to:100;editable:true;value:dialog.velocityPercent;onValueModified:{dialog.velocityPercent=value;dialog.refresh()} }
            Item { Layout.fillWidth:true }
        }
        RowLayout {
            visible:dialog.sourceIndex===1
            Label { text:"RX Symbol";color:dialog.theme.muted }
            ComboBox { objectName:"receiverResponseSnapshot";model:dialog.response.snapshots || [];currentIndex:dialog.snapshotIndex;enabled:model.length>0;onActivated:function(index){dialog.snapshotIndex=index;dialog.refresh()} }
        }
        Label { text:dialog.response.summary || dialog.response.reason || "";color:dialog.theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        SpectrumPlot { theme:dialog.theme;viewport:dialog.response;Layout.fillWidth:true;Layout.fillHeight:true }
        Label { text:dialog.response.detail || "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11 }
    }
}
