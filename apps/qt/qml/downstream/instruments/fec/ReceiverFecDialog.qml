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
    id: dialog
    required property var backend
    property var runState: ({})
    readonly property string seenToken:presentation.value ? presentation.value.token : ""
    property int profileId: -1
    readonly property var statistics:presentation.value ? presentation.value.statistics : ({})
    readonly property var display:presentation.value ? presentation.value.display : ({})
    property PresentationOwner presentationCoverage: PresentationOwner {
        id:presentation
        theme:dialog.theme
    }

    /**
     * Preserve the existing unavailable-value rendering for retained table text.
     * @param number Original optional numerical/display value.
     * @returns Original value text; missing values remain an em dash.
     */
    function value(number) {
        return number===undefined || number===null ? "—" : String(number)
    }

    /**
     * Preserve the existing status wording without changing run outcomes.
     * @param status Original public result status.
     * @returns Existing label or the original unknown status unchanged.
     */
    function statusLabel(status) {
        return ({complete:"Run complete",cancelled:"Stopped · Partial",runtime_exceeded:"Budget Exceeded · Partial",failed:"Failed · Partial",not_started:"Not Started"})[status] || status || "No Results"
    }

    /**
     * Couple original FEC data and all independent converted tables/display text.
     * @param statistics Original backend DTO, including unchanged words/charts.
     * @param profile Original requested profile ID.
     * @returns Plain complete candidate admitted before table model conversion.
     */
    function allocationText(allocation) {
        return (allocation && allocation.segments || []).map(function(segment){return {
            description:segment.label+": "+segment.count,count:String(segment.count),legend:segment.label+" "+segment.count
        }})
    }
    function fecCandidate(statistics, profile) {
        const counters = statistics.fec || {}
        return {statistics:statistics,profileId:profile,token:String(runState.result_token || ""),
            profiles:(statistics.profiles || []).map(function(p){return "Profile "+p.id}),
            counters:[{key:"total_cw",label:"TotalCW"},{key:"ccw",label:"Corrected · CCW"},{key:"ufec",label:"Uncorrectable · UFEC"}],
            iterations:statistics.iterations || ({points:[],maximum:0}),
            iterationText:{description:((statistics.iterations || {}).points || []).map(function(point){return point.iteration+" iterations: "+point.count+" codewords"}).join(", "),
                tooltips:((statistics.iterations || {}).points || []).map(function(point){return point.iteration+" iterations · "+point.count+" codewords"})},
            outcomeText:allocationText(statistics.outcomes),recoveryText:allocationText(statistics.recovery),
            control:controlTable.prepare(statistics),codewords:wordTable.prepare(statistics.words || [],statistics.omitted_words || 0),
            display:{status:statusLabel(statistics.status),error:statistics.error ? JSON.stringify(statistics.error) : statistics.reason || "",
                counters:{total_cw:value(counters.total_cw),ccw:value(counters.ccw),ufec:value(counters.ufec)},
                mean:"Mean: "+value(statistics.mean_iterations_text)+" iterations",
                recovered:value(statistics.recovered)+" / "+value(statistics.generated)+" Recovered",
                compared:"Compared Bits: "+value(statistics.compared_bits),bitErrors:"Bit Errors: "+value(statistics.bit_errors),
                ber:"BER: "+(statistics.ber===undefined || statistics.ber===null ? "—" : statistics.ber===0 ? "0" : Number(statistics.ber).toExponential(3)),
                ldpc:"LDPC failures: "+value(statistics.ldpc_failures),bch:"BCH failures: "+value(statistics.bch_failures),
                bchCorrected:"BCH corrected bits: "+value(statistics.bch_corrected_bits),codewordErrors:"Codeword errors: "+value(statistics.codeword_errors)}}
    }

    /**
     * Admit the whole profile and both tables before changing any displayed data.
     * @returns False preserves all old tables/statistics and restores profile ID.
     */
    function refresh() {
        if (!backend) {
            return false
        }
        const profile = profileId
        const statistics = JSON.parse(backend.fec_profile(profile))
        if (!presentation.replace(fecCandidate(statistics,profile))) {
            if (presentation.value) {
                profileId = presentation.value.profileId
            }
            return false
        }
        return true
    }
    Component.onCompleted: presentation.replace(fecCandidate({available:false,words:[]},-1))
    onOpened: refresh()
    onRunStateChanged: {
        if (opened && String(runState.result_token || "")!==seenToken) {
            refresh()
        }
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
                model:presentation.value ? presentation.value.profiles : []
                currentIndex: profiles.findIndex(profile => profile.id === dialog.statistics.profile_id)
                enabled: profiles.length > 0
                Accessible.name: "FEC result profile"
                Layout.preferredWidth: 220
                onActivated: function(index) { dialog.profileId = profiles[index].id; dialog.refresh() }
            }
            Label { objectName: "receiverFecStatus"; text:dialog.display.status || ""; color: dialog.theme.accent; Layout.fillWidth: true; horizontalAlignment: Text.AlignRight }
        }
        Label {objectName:"receiverFecRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { visible: !dialog.statistics.available || !!dialog.statistics.error; text:dialog.display.error || ""; color: dialog.statistics.error ? dialog.theme.statusError : dialog.theme.muted; wrapMode: Text.WordWrap; Layout.fillWidth: true }
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
                        model:presentation.value ? presentation.value.counters : []
                        Rectangle {
                            required property var modelData
                            Layout.fillWidth: true; implicitHeight: 80
                            color: dialog.theme.panel; border.color: dialog.theme.line; radius: 6
                            Column {
                                anchors.fill: parent; anchors.margins: 12; spacing: 4
                                Label { text: modelData.label; color: dialog.theme.accent; font.bold: true }
                                Label { objectName: "fecCounter_" + modelData.key; text:(dialog.display.counters || {})[modelData.key] || ""; color: dialog.theme.text; font.pixelSize: 26; font.bold: true }
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
                            FecAllocationChart { objectName: "fecOutcomeChart"; theme: dialog.theme; allocation: dialog.statistics.outcomes;presentation:presentationCoverage.value ? presentationCoverage.value.outcomeText : []; Layout.fillWidth: true; Layout.fillHeight: true }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; Layout.fillHeight: true; Layout.minimumHeight: 160
                        color: dialog.theme.panel; border.color: dialog.theme.line; radius: 6
                        ColumnLayout {
                            anchors.fill: parent; anchors.margins: 12
                            Label { text: "LDPC Decoder Effort · Whole Run"; color: dialog.theme.accent; font.bold: true }
                            FecIterationChart { objectName: "fecIterationChart"; theme: dialog.theme; histogram:presentation.value ? presentation.value.iterations : ({});presentationText:presentation.value ? presentation.value.iterationText : ({}); Layout.fillWidth: true; Layout.fillHeight: true }
                            Label { text: dialog.display.mean || ""; color: dialog.theme.muted }
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
                            Label { text: dialog.display.recovered || ""; color: dialog.theme.text }
                        }
                        FecAllocationChart { objectName: "fecRecoveryChart"; theme: dialog.theme; allocation: dialog.statistics.recovery;presentation:presentationCoverage.value ? presentationCoverage.value.recoveryText : []; emptyText: "No generated payloads."; Layout.fillWidth: true; Layout.fillHeight: true }
                        RowLayout {
                            Layout.fillWidth: true
                            Label { text: dialog.display.compared || ""; color: dialog.theme.text; Layout.fillWidth: true }
                            Label { text: dialog.display.bitErrors || ""; color: dialog.theme.text; Layout.fillWidth: true }
                            Label { text: dialog.display.ber || ""; color: dialog.theme.text }
                        }
                    }
                }
            }
            ColumnLayout {
                spacing: 14
                Label { text: "Control Channels · Whole Run"; color: dialog.theme.accent; font.bold: true }
                FecControlTable {
                    id:controlTable
                    objectName: "fecControlTable"
                    theme: dialog.theme
                    projection:presentation.value ? presentation.value.control : ({})
                    Layout.fillWidth: true
                    Layout.maximumWidth: 600
                    Layout.alignment: Qt.AlignLeft
                }
                Item { Layout.fillHeight: true }
            }
            ColumnLayout {
                RowLayout {
                    Layout.fillWidth: true
                    Label { text: dialog.display.ldpc || ""; color: dialog.theme.text; Layout.fillWidth: true }
                    Label { text: dialog.display.bch || ""; color: dialog.theme.text; Layout.fillWidth: true }
                    Label { text: dialog.display.bchCorrected || ""; color: dialog.theme.text }
                }
                Label { text: dialog.display.codewordErrors || ""; color: dialog.theme.muted }
                FecCodewordTable { id: wordTable; theme: dialog.theme; projection:presentation.value ? presentation.value.codewords : ({}); Layout.fillWidth: true; Layout.fillHeight: true }
            }
        }
    }
}
