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
    readonly property var document: presentation.value ? presentation.value.document : ({})
    property int selectedIndex:0
    property bool showBytes:false
    readonly property var packet:(document.messages || [])[selectedIndex] || ({})
    readonly property var packetDisplay:presentation.value ? presentation.value.packets[selectedIndex] || ({}) : ({})
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.messageCandidate({messages:[],reason:""})
    }

    /**
     * Couple original message data with its separately retained display strings.
     * @param value Unmodified backend document; packet/TLV aliases are not cloned.
     * @returns Plain candidate including selector names and byte-row/status text.
     */
    function messageCandidate(value) {
        const messages = value.messages || []
        return {document:value,
            labels:messages.map(function(p){return p.name+" · frame "+Math.floor(p.symbol/128)}),
            packets:messages.map(function(p){return {
                status:(p.status || "")+" · "+(p.integrity || ""),
                bytes:(p.hex_rows || []).map(function(row){return row.offset+"   "+row.hex})
            }}),
            pending:"Incomplete captured packet: "+value.pending_bytes+" bytes"}
    }

    /**
     * Admit document and derived selectors together before clamping selection.
     * @returns False preserves the old document, selected packet and byte view.
     */
    function refresh() {
        const value = JSON.parse(backend.messages())
        const selected = Math.min(selectedIndex,Math.max(0,value.messages.length-1))
        if (!presentation.replace(messageCandidate(value))) {
            return false
        }
        selectedIndex = selected
        return true
    }
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
            ComboBox { objectName:"txMessageSelection";Layout.fillWidth:true;model:presentation.value ? presentation.value.labels : [];currentIndex:dialog.selectedIndex;onActivated:function(index){dialog.selectedIndex=index} }
            WorkbenchActionButton { theme: dialog.theme; objectName:"txMessageBytes";text:dialog.showBytes ? "TLV Tree" : "Bytes";onClicked:dialog.showBytes=!dialog.showBytes }
        }
        Label {objectName:"txMessageRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { text:dialog.document.reason || "";visible:text.length>0;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        Label { visible:dialog.packet.name!==undefined;text:dialog.packetDisplay.status || "";color:dialog.packet.completed_frame ? dialog.theme.accent : dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
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
            model:dialog.packetDisplay.bytes || [];ScrollBar.vertical:ScrollBar {}
            delegate:Label { required property var modelData;width:ListView.view.width;text:modelData;font.family:"monospace";color:dialog.theme.text;padding:5;wrapMode:Text.WrapAnywhere }
        }
        Label { visible:(dialog.document.pending_bytes || 0)>0;text:presentation.value ? presentation.value.pending : "";color:dialog.theme.statusAttention }
        Label { visible:dialog.document.capture_limited || false;text:"Message capture limit reached; later messages are not shown.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    }
}
