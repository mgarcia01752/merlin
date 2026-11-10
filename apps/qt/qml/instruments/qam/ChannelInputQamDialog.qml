/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id: dialog
    required property var backend
    property var observation: ({available:false,points:[]})
    function refresh() { if (backend) { observation=JSON.parse(backend.channel_qam(first.value,last.value)) } }
    onOpened: refresh()
    title: "QAM Analyzer · Channel Input"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(960,parent ? parent.width-40 : 960)
    height: Math.min(650,parent ? parent.height-40 : 650)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        Label { text:"TX Physical FFT Bins · Before Channel";color:dialog.theme.accent }
        Label { visible:dialog.observation.available;text:"Retained Symbol: "+dialog.observation.symbol;color:dialog.theme.muted }
        RowLayout {
            Label {text:"First Bin";color:dialog.theme.muted}
            SpinBox {id:first;objectName:"channelInputFirstBin";from:-1;to:8191;value:-1;editable:true;textFromValue:function(value){return value<0 ? "All" : value.toString()};valueFromText:function(text){return text==="All" ? -1 : Number(text)};onValueModified:dialog.refresh()}
            Label {text:"Last Bin";color:dialog.theme.muted}
            SpinBox {id:last;objectName:"channelInputLastBin";from:-1;to:8191;value:-1;editable:true;textFromValue:function(value){return value<0 ? "All" : value.toString()};valueFromText:function(text){return text==="All" ? -1 : Number(text)};onValueModified:dialog.refresh()}
            Item {Layout.fillWidth:true}
            Label {text:"■ Physical Bins";color:dialog.theme.qamUnknown}
        }
        TransmitterScatter { theme:dialog.theme;observation:dialog.observation;Layout.fillWidth:true;Layout.fillHeight:true }
        Label { visible:!dialog.observation.available;text:dialog.observation.reason || "Run the simulation to capture channel input.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    }
}
