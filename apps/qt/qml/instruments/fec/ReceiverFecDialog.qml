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
    property var runState: ({})
    property string seenToken: ""
    property int profileId: -1
    property var statistics: ({available:false,words:[]})
    function refresh() {
        if (backend) { statistics = JSON.parse(backend.fec_profile(profileId)) }
    }
    function value(number) { return number === undefined || number === null ? "—" : String(number) }
    function statusLabel(status) { return ({complete:"Run complete",cancelled:"Stopped · Partial",runtime_exceeded:"Budget Exceeded · Partial",failed:"Failed · Partial",not_started:"Not Started"})[status] || status || "No Results" }
    onOpened: refresh()
    onRunStateChanged: {
        if (opened && runState.result_token !== seenToken) { seenToken = runState.result_token || ""; refresh() }
    }
    title: "FEC Monitor"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1080, parent ? parent.width - 40 : 1080)
    height: Math.min(730, parent ? parent.height - 40 : 730)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            Layout.fillWidth: true
            ComboBox {
                objectName: "fecProfileChoice"
                property var profiles: dialog.statistics.profiles || []
                model: profiles.map(profile => "Profile " + profile.id)
                currentIndex: profiles.findIndex(profile => profile.id === dialog.statistics.profile_id)
                enabled: profiles.length > 0
                Accessible.name: "FEC result profile"
                Layout.preferredWidth: 220
                onActivated: function(index) { dialog.profileId = profiles[index].id; dialog.refresh() }
            }
            Label { objectName: "receiverFecStatus"; text: dialog.statusLabel(dialog.statistics.status); color: dialog.theme.accent; Layout.fillWidth: true; horizontalAlignment: Text.AlignRight }
        }
        Label { visible: !dialog.statistics.available || !!dialog.statistics.error; text: dialog.statistics.error ? JSON.stringify(dialog.statistics.error) : dialog.statistics.reason || ""; color: dialog.statistics.error ? dialog.theme.statusError : dialog.theme.muted; wrapMode: Text.WordWrap; Layout.fillWidth: true }
        TabBar {
            id: tabs; Layout.fillWidth: true
            TabButton { objectName: "fecOverviewTab"; text: "Overview" }
            TabButton { objectName: "fecControlTab"; text: "Control Channels" }
            TabButton { objectName: "fecWordsTab"; text: "Codewords" }
        }
        StackLayout {
            currentIndex: tabs.currentIndex
            Layout.fillWidth: true; Layout.fillHeight: true
            ColumnLayout {
                spacing: 12
                RowLayout {
                    Layout.fillWidth: true; spacing: 12
                    Repeater {
                        objectName: "fecCounters"
                        model: [{key:"total_cw",label:"TotalCW"},{key:"ccw",label:"Corrected · CCW"},{key:"ufec",label:"Uncorrectable · UFEC"}]
                        Rectangle {
                            required property var modelData
                            Layout.fillWidth: true; implicitHeight: 80
                            color: dialog.theme.panel; border.color: dialog.theme.line; radius: 6
                            Column {
                                anchors.fill: parent; anchors.margins: 12; spacing: 4
                                Label { text: modelData.label; color: dialog.theme.accent; font.bold: true }
                                Label { objectName: "fecCounter_" + modelData.key; text: dialog.value((dialog.statistics.fec || {})[modelData.key]); color: dialog.theme.text; font.pixelSize: 26; font.bold: true }
                            }
                        }
                    }
                }
                Label { text: "Observation window · TotalCW = Clean + CCW + UFEC"; color: dialog.theme.muted }
                RowLayout {
                    Layout.fillWidth: true; Layout.fillHeight: true; spacing: 12
                    Rectangle {
                        Layout.fillWidth: true; Layout.fillHeight: true; Layout.minimumHeight: 160
                        color: dialog.theme.panel; border.color: dialog.theme.line; radius: 6
                        ColumnLayout {
                            anchors.fill: parent; anchors.margins: 12
                            Label { text: "Codeword Outcomes"; color: dialog.theme.accent; font.bold: true }
                            FecAllocationChart { objectName: "fecOutcomeChart"; theme: dialog.theme; allocation: dialog.statistics.outcomes; Layout.fillWidth: true; Layout.fillHeight: true }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; Layout.fillHeight: true; Layout.minimumHeight: 160
                        color: dialog.theme.panel; border.color: dialog.theme.line; radius: 6
                        ColumnLayout {
                            anchors.fill: parent; anchors.margins: 12
                            Label { text: "LDPC Decoder Effort · Whole Run"; color: dialog.theme.accent; font.bold: true }
                            FecIterationChart { objectName: "fecIterationChart"; theme: dialog.theme; histogram: dialog.statistics.iterations || ({points:[],maximum:0}); Layout.fillWidth: true; Layout.fillHeight: true }
                            Label { text: "Mean: " + dialog.value(dialog.statistics.mean_iterations_text) + " iterations"; color: dialog.theme.muted }
                        }
                    }
                }
                Rectangle {
                    Layout.fillWidth: true; implicitHeight: 138
                    color: dialog.theme.panel; border.color: dialog.theme.line; radius: 6
                    ColumnLayout {
                        anchors.fill: parent; anchors.margins: 12
                        RowLayout {
                            Label { text: "Payload Recovery · Whole Run"; color: dialog.theme.accent; font.bold: true; Layout.fillWidth: true }
                            Label { text: dialog.value(dialog.statistics.recovered) + " / " + dialog.value(dialog.statistics.generated) + " Recovered"; color: dialog.theme.text }
                        }
                        FecAllocationChart { objectName: "fecRecoveryChart"; theme: dialog.theme; allocation: dialog.statistics.recovery; emptyText: "No generated payloads."; Layout.fillWidth: true; Layout.fillHeight: true }
                        RowLayout {
                            Layout.fillWidth: true
                            Label { text: "Compared Bits: " + dialog.value(dialog.statistics.compared_bits); color: dialog.theme.text; Layout.fillWidth: true }
                            Label { text: "Bit Errors: " + dialog.value(dialog.statistics.bit_errors); color: dialog.theme.text; Layout.fillWidth: true }
                            Label { text: "BER: " + dialog.value(dialog.statistics.ber); color: dialog.theme.text }
                        }
                    }
                }
            }
            ColumnLayout {
                spacing: 14
                Label { text: "Control Channels · Whole Run"; color: dialog.theme.accent; font.bold: true }
                FecControlTable {
                    objectName: "fecControlTable"
                    theme: dialog.theme
                    statistics: dialog.statistics
                    Layout.fillWidth: true
                    Layout.maximumWidth: 600
                    Layout.alignment: Qt.AlignLeft
                }
                Item { Layout.fillHeight: true }
            }
            ColumnLayout {
                RowLayout {
                    Layout.fillWidth: true
                    Label { text: "LDPC failures: " + dialog.value(dialog.statistics.ldpc_failures); color: dialog.theme.text; Layout.fillWidth: true }
                    Label { text: "BCH failures: " + dialog.value(dialog.statistics.bch_failures); color: dialog.theme.text; Layout.fillWidth: true }
                    Label { text: "BCH corrected bits: " + dialog.value(dialog.statistics.bch_corrected_bits); color: dialog.theme.text }
                }
                Label { text: "Codeword errors: " + dialog.value(dialog.statistics.codeword_errors); color: dialog.theme.muted }
                FecCodewordTable { id: wordTable; theme: dialog.theme; words: dialog.statistics.words || []; omitted: dialog.statistics.omitted_words || 0; Layout.fillWidth: true; Layout.fillHeight: true }
            }
        }
    }
}
