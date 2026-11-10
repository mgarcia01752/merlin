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
    property var document:({messages:[],reason:""})
    property int selectedIndex:0
    property bool showBytes:false
    readonly property var packet:(document.messages || [])[selectedIndex] || ({tlvs:[],hex_rows:[]})
    function refresh(){document=JSON.parse(backend.messages());selectedIndex=Math.min(selectedIndex,Math.max(0,document.messages.length-1))}
    onOpened:refresh()
    title:"Transmitter · OCD / DPD Messages"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1060,parent ? parent.width-40 : 1060)
    height:Math.min(680,parent ? parent.height-40 : 680)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:12
        RowLayout {
            Layout.fillWidth:true
            Label { text:"Message";color:dialog.theme.text }
            ComboBox { objectName:"txMessageSelection";Layout.fillWidth:true;model:(dialog.document.messages || []).map(function(p){return p.name+" · frame "+Math.floor(p.symbol/128)});currentIndex:dialog.selectedIndex;onActivated:function(index){dialog.selectedIndex=index} }
            Button { objectName:"txMessageBytes";text:dialog.showBytes ? "TLV Tree" : "Bytes";onClicked:dialog.showBytes=!dialog.showBytes }
        }
        Label { text:dialog.document.reason || "";visible:text.length>0;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        Label { visible:dialog.packet.name!==undefined;text:(dialog.packet.status || "")+" · "+(dialog.packet.integrity || "");color:dialog.packet.completed_frame ? dialog.theme.accent : dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        GridLayout {
            visible:dialog.packet.name!==undefined;columns:4;columnSpacing:20;Layout.fillWidth:true
            Label { text:"Message Type";color:dialog.theme.muted }
            Label { text:dialog.packet.type || "";color:dialog.theme.text }
            Label { text:"Channel ID";color:dialog.theme.muted }
            Label { text:dialog.packet.channel===undefined ? "" : dialog.packet.channel;color:dialog.theme.text }
            Label { text:"Change Count";color:dialog.theme.muted }
            Label { text:dialog.packet.change_count===undefined ? "" : dialog.packet.change_count;color:dialog.theme.text }
            Label { text:"Profile ID";color:dialog.theme.muted }
            Label { text:dialog.packet.profile===null || dialog.packet.profile===undefined ? "—" : dialog.packet.profile;color:dialog.theme.text }
            Label { text:"Source MAC";color:dialog.theme.muted }
            Label { text:dialog.packet.source || "";color:dialog.theme.text }
            Label { text:"Packet Bytes";color:dialog.theme.muted }
            Label { text:dialog.packet.bytes || "";color:dialog.theme.text }
        }
        TlvTreeGrid {
            objectName: "txTlvTree"
            visible: !dialog.showBytes
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: dialog.theme
            nodes: dialog.packet.tlvs || []
        }
        ListView {
            visible:dialog.showBytes;Layout.fillWidth:true;Layout.fillHeight:true;clip:true
            model:dialog.packet.hex_rows || [];ScrollBar.vertical:ScrollBar {}
            delegate:Label { required property var modelData;width:ListView.view.width;text:modelData.offset+"   "+modelData.hex;font.family:"monospace";color:dialog.theme.text;padding:5;wrapMode:Text.WrapAnywhere }
        }
        Label { visible:(dialog.document.pending_bytes || 0)>0;text:"Incomplete captured packet: "+dialog.document.pending_bytes+" bytes";color:dialog.theme.statusAttention }
        Label { visible:dialog.document.capture_limited || false;text:"Message capture limit reached; later messages are not shown.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    }
}
