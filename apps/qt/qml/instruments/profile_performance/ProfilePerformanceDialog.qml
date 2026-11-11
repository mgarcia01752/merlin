/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../../inputs/profile_input"
WorkbenchDialog {
    id: dialog
    objectName: "profilePerformanceDialog"
    required property var backend
    property var runState: ({})
    property var viewport: ({available:false,profiles:[],points:[]})
    property int profileId: -1
    property string seenToken: ""
    function statusLabel(status) { return ({not_started:"Not Started",runtime_exceeded:"Budget Exceeded · Partial",cancelled:"Cancelled · Partial",failed:"Failed",complete:"Complete"})[status] || status || "" }
    function refresh(start, count) { if (backend) viewport=JSON.parse(backend.performance(profileId,start || 0,count || 0)) }
    onOpened: refresh(0,0)
    onRunStateChanged: { if (opened && runState.result_token !== seenToken) { seenToken=runState.result_token || ""; refresh(0,0) } }
    title: "Receiver Profile Performance"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1100,parent ? parent.width-32 : 1100)
    height: Math.min(750,parent ? parent.height-32 : 750)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 12
        RowLayout {
            Layout.fillWidth: true
            Label { text: "Channel " + (dialog.viewport.channel ?? "—"); color: dialog.theme.text }
            Label { text: "Profile"; color: dialog.theme.muted }
            ComboBox {
                objectName: "performanceProfileChoice"
                property var profiles: dialog.viewport.profiles || []
                model: profiles.map(p => "Profile " + p.id + " · " + dialog.statusLabel(p.status))
                currentIndex: profiles.findIndex(p => p.id === dialog.viewport.profile_id)
                Layout.preferredWidth: 260
                onActivated: function(index) { dialog.profileId=profiles[index].id; dialog.refresh(0,0) }
            }
            Label { text: dialog.runState.busy ? "Running Profile " + (dialog.runState.profile_id ?? "—") : dialog.statusLabel(dialog.viewport.status); color: dialog.theme.accent; Layout.fillWidth: true }
            Button { objectName: "performanceFullSpan"; text: "Full Span"; onClicked: dialog.refresh(0,0) }
        }
        RowLayout {
            Layout.fillWidth: true; spacing: 12
            Repeater {
                model: [{key:"total_cw",label:"TotalCW",color:dialog.theme.text},{key:"ccw",label:"CCW",color:dialog.theme.statusAttention},{key:"ufec",label:"UFEC",color:dialog.theme.statusError},{key:"clean_cw",label:"CleanCW",color:dialog.theme.statusComplete}]
                Rectangle {
                    required property var modelData
                    Layout.fillWidth: true; implicitHeight: 72; radius: 8
                    color: dialog.theme.panel; border.color: dialog.theme.line
                    Column {
                        anchors.centerIn: parent; spacing: 4
                        Label { text: modelData.label; color: dialog.theme.muted; anchors.horizontalCenter: parent.horizontalCenter }
                        Label { text: dialog.viewport.fec ? dialog.viewport.fec[modelData.key] : "—"; color: modelData.color; font.pixelSize: 23; font.bold: true; anchors.horizontalCenter: parent.horizontalCenter }
                    }
                }
            }
        }
        RowLayout {
            Layout.fillWidth: true; spacing: 24
            Label { text: "━  Receiver MER"; color: dialog.theme.dark ? "#79b8ff" : "#2563b9" }
            Label { text: "━  Ideal Profile Minimum"; color: dialog.theme.dark ? "#45cf83" : "#187344" }
            Item { Layout.fillWidth: true }
            Label { text: "Undetected Errors: " + (dialog.viewport.fec ? dialog.viewport.fec.undetected_errors : "—"); color: dialog.theme.muted }
        }
        ProfileChart {
            visible: dialog.viewport.available || false
            theme: dialog.theme; viewport: dialog.viewport; roleColors: ({})
            profileColor: function(id) { return id === 1 ? (dialog.theme.dark ? "#79b8ff" : "#2563b9") : (dialog.theme.dark ? "#45cf83" : "#187344") }
            Layout.fillWidth: true; Layout.fillHeight: true
            onViewportRequested: function(start,count) { dialog.refresh(start,count) }
        }
        Label { visible: !dialog.viewport.available; text: dialog.viewport.reason || dialog.viewport.error || "Run the simulation first."; color: dialog.theme.muted; Layout.fillHeight:true; Layout.fillWidth:true; horizontalAlignment:Text.AlignHCenter; verticalAlignment:Text.AlignVCenter; wrapMode:Text.WordWrap }
        Label { visible: dialog.viewport.available || false; text: dialog.viewport.scope || ""; color: dialog.theme.muted; font.pixelSize:11; Layout.fillWidth:true; wrapMode:Text.WordWrap }
        Label { visible: !!dialog.viewport.error; text: dialog.viewport.error || ""; color: dialog.theme.statusError; Layout.fillWidth:true; wrapMode:Text.WordWrap }
    }
}
