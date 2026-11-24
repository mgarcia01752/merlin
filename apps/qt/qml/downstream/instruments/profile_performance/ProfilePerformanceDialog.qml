/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"
import "../../configuration/profile_input"
WorkbenchDialog {
    id: dialog
    objectName: "profilePerformanceDialog"
    required property var backend
    property var runState: ({})
    readonly property var viewport:presentation.value ? presentation.value.viewport : ({})
    readonly property var display:presentation.value ? presentation.value.display : ({})
    property int profileId: -1
    readonly property string seenToken:presentation.value ? presentation.value.token : ""
    property PresentationOwner presentationCoverage: PresentationOwner {
        id:presentation
        theme:dialog.theme
        initialValue:dialog.performanceCandidate({available:false,profiles:[],points:[]},-1,0,0)
    }

    /**
     * Preserve the existing finite status wording without changing backend status.
     * @param status Original public result status.
     * @returns Existing display label, or the original unknown status unchanged.
     */
    function statusLabel(status) {
        return ({not_started:"Not Started",runtime_exceeded:"Budget Exceeded · Partial",cancelled:"Cancelled · Partial",failed:"Failed",complete:"Complete"})[status] || status || ""
    }

    /**
     * Couple the original viewport, query and independently retained labels.
     * @param value Original numerical viewport, aliased by the existing chart.
     * @param profile Requested profile ID; -1 retains existing default semantics.
     * @param start Original viewport start index.
     * @param count Original requested count; zero preserves full-span semantics.
     * @returns Plain candidate admitted before publication/model conversion.
     */
    function performanceCandidate(value, profile, start, count) {
        const counters = value.fec || {}
        const counterText = function(key){return value.fec ? (counters[key]===undefined || counters[key]===null ? "" : String(counters[key])) : "—"}
        return {viewport:value,profileId:profile,start:start,count:count,token:String(runState.result_token || ""),
            counters:[{key:"total_cw",label:"TotalCW"},{key:"ccw",label:"CCW"},{key:"ufec",label:"UFEC"},{key:"clean_cw",label:"CleanCW"}],
            profiles:(value.profiles || []).map(function(p){return "Profile "+p.id+" · "+statusLabel(p.status)}),
            display:{channel:"Channel "+(value.channel ?? "—"),
                counters:{total_cw:counterText("total_cw"),ccw:counterText("ccw"),ufec:counterText("ufec"),clean_cw:counterText("clean_cw")},
                status:runState.busy ? "Running Profile "+(runState.profile_id ?? "—") : statusLabel(value.status),
                undetected:"Undetected Errors: "+(value.fec ? value.fec.undetected_errors : "—")}}
    }

    /**
     * Admit the actual viewport before changing its retained profile/context.
     * @param start Existing optional viewport start index, default zero.
     * @param count Existing optional viewport count, default zero.
     * @returns False keeps the old viewport and restores its requested profile.
     */
    function refresh(start, count) {
        if (!backend) {
            return false
        }
        const first = start || 0
        const length = count || 0
        const profile = profileId
        const value = JSON.parse(backend.performance(profile,first,length))
        if (!presentation.replace(performanceCandidate(value,profile,first,length))) {
            if (presentation.value) {
                profileId = presentation.value.profileId
            }
            return false
        }
        return true
    }
    onOpened: refresh(0,0)
    onRunStateChanged: {
        if (opened && String(runState.result_token || "")!==seenToken) {
            refresh(0,0)
        } else if (presentation && presentation.value) {
            // Runtime status still changes independently of scientific queries.
            const accepted = presentation.value
            const candidate = performanceCandidate(accepted.viewport,accepted.profileId,accepted.start,accepted.count)
            candidate.token = accepted.token
            presentation.replace(candidate)
        }
    }
    title: "Receiver Profile Performance"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1100,parent ? parent.width-32 : 1100)
    height: Math.min(750,parent ? parent.height-32 : 750)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 12
        RowLayout {
            Layout.fillWidth: true
            Label { text:dialog.display.channel || ""; color: dialog.theme.text }
            Label { text: "Profile"; color: dialog.theme.muted }
            ComboBox {
                objectName: "performanceProfileChoice"
                property var profiles: dialog.viewport.profiles || []
                model:presentation.value ? presentation.value.profiles : []
                currentIndex: profiles.findIndex(p => p.id === dialog.viewport.profile_id)
                Layout.preferredWidth: 260
                onActivated: function(index) { dialog.profileId=profiles[index].id; dialog.refresh(0,0) }
            }
            Label { text:dialog.display.status || ""; color: dialog.theme.accent; Layout.fillWidth: true }
            WorkbenchActionButton { theme: dialog.theme; objectName: "performanceFullSpan"; text: "Full Span"; onClicked: dialog.refresh(0,0) }
        }
        Label {objectName:"profilePerformanceRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        RowLayout {
            Layout.fillWidth: true; spacing: 12
            Repeater {
                model:presentation.value ? presentation.value.counters : []
                Rectangle {
                    required property var modelData
                    Layout.fillWidth: true; implicitHeight: 72; radius: 8
                    color: dialog.theme.panel; border.color: dialog.theme.line
                    Column {
                        anchors.centerIn: parent; spacing: 4
                        Label { text: modelData.label; color: dialog.theme.muted; anchors.horizontalCenter: parent.horizontalCenter }
                        Label { text:(dialog.display.counters || {})[modelData.key] || ""; color:modelData.key==="ccw" ? dialog.theme.statusAttention : modelData.key==="ufec" ? dialog.theme.statusError : modelData.key==="clean_cw" ? dialog.theme.statusComplete : dialog.theme.text; font.pixelSize: 23; font.bold: true; anchors.horizontalCenter: parent.horizontalCenter }
                    }
                }
            }
        }
        RowLayout {
            Layout.fillWidth: true; spacing: 24
            Label { text: "━  Receiver MER"; color: dialog.theme.dark ? "#79b8ff" : "#2563b9" }
            Label { text: "━  Ideal Profile Minimum"; color: dialog.theme.dark ? "#45cf83" : "#187344" }
            Label { text: "▰  PLC"; color: dialog.theme.dark ? "#f2bd40" : "#946a08"; visible: (dialog.viewport.bands || []).length>0 }
            Item { Layout.fillWidth: true }
            Label { text:dialog.display.undetected || ""; color: dialog.theme.muted }
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
