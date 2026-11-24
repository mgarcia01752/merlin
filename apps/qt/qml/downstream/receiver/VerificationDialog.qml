/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/components"
import "../../shared/presentation"

WorkbenchDialog {
    id: dialog
    required property var backend
    required property var diagnosticsBackend
    required property var runState
    readonly property var statistics: presentation.value ? presentation.value.statistics : ({available:false})
    readonly property string seenToken: presentation.value ? presentation.value.token : ""
    readonly property var display:presentation.value ? presentation.value.display : ({})
    property PresentationOwner presentationCoverage: PresentationOwner {id:presentation;theme:dialog.theme}
    readonly property bool hasResult: !runState.busy && !!runState.result
    readonly property var comparisonResult: presentation.value ? presentation.value.result : ({})
    readonly property string statusText: runState.busy ? (runState.paused ? "Paused" : "Running") : hasResult ? ({complete: "Complete", cancelled: "Cancelled · Partial Result", runtime_exceeded: "Runtime Limit · Partial Result", failed: "Failed · Partial Result"})[comparisonResult.status] || comparisonResult.status || "No Run Results Yet" : "No Run Results Yet"
    signal configurationApplied()
    function number(value) {
        if (value === undefined || value === null) {
            return "—"
        }
        return Number(value).toLocaleString(Qt.locale(), "f", 0)
    }
    /** Admit FEC charts and independent comparison strings as one result context. */
    function refresh() {
        const statistics=hasResult ? JSON.parse(diagnosticsBackend.fec()) : ({available:false})
        const result=hasResult ? runState.result : ({})
        const charts=statistics.verification_charts || {}
        const chartText=function(chart){return ((chart || {}).points || []).map(function(point){return point.label+": "+point.count}).join(", ")}
        return presentation.replace({statistics:statistics,result:result,token:runState.result_token || "",display:{
            counters:[{label:"Compared Bits",value:number(result.bits_tested),alert:false},
                {label:"Bit Errors",value:number(result.bit_errors),alert:result.bit_errors>0},
                {label:"Bit Error Ratio",value:!hasResult ? "—" : result.ber===null || result.ber===undefined ? "Unavailable" : result.ber===0 ? "0" : Number(result.ber).toExponential(3),alert:result.bit_errors>0},
                {label:"Recovered Payloads",value:number(result.recovered_payloads),alert:false}],
            recoveryDescription:chartText(charts.recovery),errorDescription:chartText(charts.word_errors),
            omitted:hasResult && statistics.available && (charts.omitted>0 || charts.uncompared_retained>0) ? "Chart: up to 64 payloads · omitted: "+charts.omitted+" · without compared bits: "+charts.uncompared_retained : ""
        }})
    }
    Component.onCompleted: refresh()
    onOpened: refresh()
    onHasResultChanged: {
        if (opened) {
            refresh()
        }
    }
    onRunStateChanged: {
        if (opened && (seenToken !== (runState.result_token || "") || !hasResult)) {
            refresh()
        }
    }
    title: "PRBS-RX / Recovered Data"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(900, parent ? parent.width - 40 : 900)
    height: Math.min(650, parent ? parent.height - 40 : 650)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ScrollView {
        clip: true
        contentWidth: availableWidth
        ColumnLayout {
            width: parent.width
            spacing: 16
            Rectangle {
                Layout.fillWidth: true; implicitHeight: 76; radius: 10
                color: dialog.theme.accentSoft; border.color: dialog.theme.line
                RowLayout {
                    anchors.fill: parent; anchors.margins: 16; spacing: 16
                    ColumnLayout {
                        Layout.fillWidth: true; spacing: 5
                        Label { text: "Independent Source Comparison"; color: dialog.theme.accent; font.pixelSize: 17; font.bold: true }
                        Label { objectName: "verificationStatus"; text: dialog.statusText; color: dialog.theme.text; font.pixelSize: 13 }
                    }
                    Label {
                        visible: dialog.runState.verified
                        text: "✓  Comparison Enabled"; color: dialog.theme.statusComplete; font.bold: true
                    }
                    WorkbenchActionButton {
                        theme: dialog.theme
                        objectName: "verificationApply"; text: "Enable Comparison"
                        visible: !dialog.runState.verified; enabled: !dialog.runState.busy
                        onClicked: { dialog.backend.verify(); dialog.configurationApplied() }
                    }
                }
            }
            Label {
                Layout.fillWidth: true; wrapMode: Text.WordWrap; color: dialog.theme.muted
                text: "Recovered data is compared with the original PRBS15 sequence or file, including configured fill."
            }
            Label {visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
            GridLayout {
                Layout.fillWidth: true; columns: width < 620 ? 2 : 4; columnSpacing: 10; rowSpacing: 10
                Repeater {
                    model:dialog.display.counters || []
                    delegate: Rectangle {
                        required property var modelData
                        Layout.fillWidth: true; Layout.preferredWidth: 1; implicitHeight: 92; radius: 9
                        color: dialog.theme.panel; border.color: dialog.theme.line
                        ColumnLayout {
                            anchors.fill: parent; anchors.margins: 13; spacing: 8
                            Label { text: modelData.label; color: dialog.theme.muted; font.pixelSize: 11; Layout.fillWidth: true }
                            Label { text: modelData.value; color: modelData.alert ? dialog.theme.statusAttention : dialog.theme.text; font.pixelSize: 23; font.weight: Font.DemiBold; Layout.fillWidth: true; fontSizeMode: Text.HorizontalFit; minimumPixelSize: 14 }
                        }
                    }
                }
            }
            GridLayout {
                Layout.fillWidth: true; columns: width < 620 ? 1 : 2; columnSpacing: 12; rowSpacing: 12
                VerificationBarChart {
                    Layout.fillWidth: true; Layout.preferredWidth: 1; Layout.preferredHeight: 210
                    theme: dialog.theme; heading: "Payload Recovery";descriptionText:dialog.display.recoveryDescription || ""; horizontal: true
                    chart: dialog.hasResult && dialog.statistics.available ? dialog.statistics.verification_charts.recovery : ({points: []})
                    emptyText: dialog.runState.busy ? "Results appear when the run finishes or stops." : "Run the simulation to compare recovered data."
                }
                VerificationBarChart {
                    objectName: "verificationWordErrors"
                    Layout.fillWidth: true; Layout.preferredWidth: 1; Layout.preferredHeight: 210
                    theme: dialog.theme; heading: "Bit Errors by Payload";descriptionText:dialog.display.errorDescription || ""
                    chart: dialog.hasResult && dialog.statistics.available ? dialog.statistics.verification_charts.word_errors : ({points: []})
                    emptyText: "No retained payload comparisons."
                }
            }
            Label {
                Layout.fillWidth: true; color: dialog.theme.muted; font.pixelSize: 11; wrapMode: Text.WordWrap
                visible: text.length > 0
                text: dialog.display.omitted || ""
            }
            Label {
                Layout.fillWidth: true; color: dialog.theme.muted; wrapMode: Text.WordWrap; font.pixelSize: 11
                visible: dialog.hasResult && (dialog.comparisonResult.ber === null || dialog.comparisonResult.ber === undefined)
                text: "BER unavailable: no compared bits."
            }
        }
    }
}
