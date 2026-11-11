/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id:dialog
    required property var backend
    property int tap:0
    property string tapName:"Before Amplifier"
    property var observation:({available:false,points:[]})
    function refresh(){if(backend){observation=JSON.parse(backend.tap_qam(tap,JSON.stringify(selection.query())))}}
    onOpened:refresh()
    title:"QAM Analyzer · "+tapName
    parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(1060,parent ? parent.width-40 : 1060);height:Math.min(700,parent ? parent.height-40 : 700)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        ReceiverCellControls {id:selection;visible:dialog.tap===2;showView:false;viewIndex:0;theme:dialog.theme;metadata:dialog.observation;Layout.fillWidth:true;onSelectionChanged:if(dialog.opened){dialog.refresh()}}
        RowLayout {
            visible:dialog.tap<2;Layout.fillWidth:true
            Label {text:"First Bin";color:dialog.theme.muted}
            SpinBox {objectName:"tapQamFirstBin";from:-1;to:8191;value:-1;editable:true;textFromValue:function(v){return v<0 ? "All" : v.toString()};valueFromText:function(t){return t==="All" ? -1 : Number(t)};onValueModified:{selection.firstBin=value;dialog.refresh()}}
            Label {text:"Last Bin";color:dialog.theme.muted}
            SpinBox {objectName:"tapQamLastBin";from:-1;to:8191;value:-1;editable:true;textFromValue:function(v){return v<0 ? "All" : v.toString()};valueFromText:function(t){return t==="All" ? -1 : Number(t)};onValueModified:{selection.lastBin=value;dialog.refresh()}}
            Item {Layout.fillWidth:true}
        }
        TransmitterScatter {theme:dialog.theme;observation:dialog.observation;Layout.fillWidth:true;Layout.fillHeight:true}
        Label {visible:!!dialog.observation.extent;text:"Raw I/Q Axis: ±"+Number(dialog.observation.extent || 0).toPrecision(4)+" normalized units";color:dialog.theme.muted}
        Label {visible:!dialog.observation.available;text:dialog.observation.reason || "Run the simulation to retain this TapPoint.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
    }
}
