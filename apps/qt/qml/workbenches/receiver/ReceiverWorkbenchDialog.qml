/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../../instruments/fec"
import "../../instruments/qam"
import "../../instruments/response"
import "../../instruments/slicer"
import "../../instruments/spectrum"

WorkbenchDialog {
    id: dialog
    required property var backend
    required property var runState
    property string seenToken: ""
    signal configurationRequested()
    onRunStateChanged: { if (runState.result_token !== seenToken) { seenToken=runState.result_token || ""; qam.refresh(); selectTap(selectedTap) } }
    property alias qamPopup: qam
    property alias spectrumPopup: spectrum
    property alias responsePopup: response
    property alias slicerPopup: slicer
    property alias fecPopup: fec
    property int selectedTap: 0
    property var spectrumState: ({has_result: false, can_generate: false, busy: false})
    function openQam(signal, view) { qam.signalIndex = signal; qam.viewIndex = view; qam.open() }
    function selectTap(index) { if (!backend) { return }; selectedTap = index; spectrum.clearView(); spectrumState = JSON.parse(backend.spectrum(index)); spectrum.updateResult() }
    function openSpectrum(index) { selectTap(index); spectrum.open() }
    function openSlicer() { slicer.useSelection(qam.selectionControls.query()); slicer.open() }
    title: "Receiver Workbench"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1140, parent ? parent.width - 40 : 1140)
    height: Math.min(560, parent ? parent.height - 40 : 560)
    modal: true; standardButtons: Dialog.Close
    closePolicy: Popup.CloseOnEscape
    contentItem: ColumnLayout {
        spacing: 14
        RowLayout { Layout.fillWidth:true; Label { text: "Receiver Signal Path"; font.pixelSize:19; font.bold:true; color:dialog.theme.text; Layout.fillWidth:true } Button { objectName:"receiverFecShortcut"; text:"FEC Statistics"; onClicked:fec.open() } Button { objectName:"receiverConfigure"; text:"Configure Receiver / Run…"; onClicked:dialog.configurationRequested() } }
        Rectangle {
            Layout.fillWidth: true; Layout.preferredHeight: 142
            color: dialog.theme.background; radius: 12; border.color: dialog.theme.line
            RowLayout {
                anchors.fill: parent; anchors.margins: 16; spacing: 10
                ColumnLayout {
                    Layout.fillWidth: true; Layout.preferredWidth: 1; Layout.minimumWidth: 0; spacing: 10
                    ReceiverChip { theme: dialog.theme; text: "Sync / CP"; Layout.fillWidth: true; stage: true; Layout.preferredHeight: 64; onClicked: dialog.openSpectrum(1) }
                    ReceiverChip { theme: dialog.theme; objectName: "receiverInputTap"; text: "● RX Input · SA"; Layout.preferredHeight: 28; Layout.fillWidth: true; onClicked: dialog.openSpectrum(0) }
                }
                Label { text: "→"; color: dialog.theme.muted; font.pixelSize: 18; Layout.alignment: Qt.AlignTop; Layout.topMargin: 26 }
                ColumnLayout {
                    Layout.fillWidth: true; Layout.preferredWidth: 1; Layout.minimumWidth: 0; spacing: 10
                    ReceiverChip { theme: dialog.theme; text: "FFT"; Layout.fillWidth: true; stage: true; Layout.preferredHeight: 64; onClicked: dialog.openQam(qam.signalIndex, 0) }
                    ReceiverChip { theme: dialog.theme; objectName: "receiverFftTap"; text: "● FFT Bins"; Layout.preferredHeight: 28; Layout.fillWidth: true; onClicked: dialog.openSpectrum(3) }
                }
                Label { text: "→"; color: dialog.theme.muted; font.pixelSize: 18; Layout.alignment: Qt.AlignTop; Layout.topMargin: 26 }
                ColumnLayout {
                    Layout.fillWidth: true; Layout.preferredWidth: 1; Layout.minimumWidth: 0; spacing: 10
                    ReceiverChip { theme: dialog.theme; text: "CH-EST /\nEqualizer"; Layout.fillWidth: true; stage: true; Layout.preferredHeight: 64; onClicked: response.open() }
                    ReceiverChip { theme: dialog.theme; text: "● Equalized I/Q"; Layout.preferredHeight: 28; Layout.fillWidth: true; onClicked: dialog.openQam(qam.signalIndex, 1) }
                }
                Label { text: "→"; color: dialog.theme.muted; font.pixelSize: 18; Layout.alignment: Qt.AlignTop; Layout.topMargin: 26 }
                ColumnLayout {
                    Layout.fillWidth: true; Layout.preferredWidth: 1; Layout.minimumWidth: 0; spacing: 10
                    ReceiverChip { theme: dialog.theme; text: "Received Roles"; Layout.fillWidth: true; stage: true; Layout.preferredHeight: 64; onClicked: dialog.openQam(qam.signalIndex, qam.viewIndex) }
                    Label { text: "PLC / NCP / Data"; color: dialog.theme.muted; Layout.fillWidth: true; Layout.preferredHeight: 28; horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter; font.pixelSize: 11 }
                }
                Label { text: "→"; color: dialog.theme.muted; font.pixelSize: 18; Layout.alignment: Qt.AlignTop; Layout.topMargin: 26 }
                ColumnLayout {
                    Layout.fillWidth: true; Layout.preferredWidth: 1; Layout.minimumWidth: 0; spacing: 10
                    ReceiverChip { theme: dialog.theme; text: "Soft Demap /\nDecode"; Layout.fillWidth: true; stage: true; Layout.preferredHeight: 64; onClicked: dialog.openSlicer() }
                    Label { text: "Branch-specific FEC"; color: dialog.theme.muted; Layout.fillWidth: true; Layout.preferredHeight: 28; horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter; font.pixelSize: 11 }
                }
            }
        }
        Label { text: "QAM Instruments"; color: dialog.theme.accent; font.bold: true }
        RowLayout {
            Layout.fillWidth: true
            ReceiverChip { theme: dialog.theme; objectName: "receiverDataQam"; text: "Data Profile QAM"; Layout.fillWidth: true; onClicked: dialog.openQam(0, qam.viewIndex) }
            ReceiverChip { theme: dialog.theme; objectName: "receiverNcpQam"; text: "NCP QAM"; Layout.fillWidth: true; onClicked: dialog.openQam(1, qam.viewIndex) }
            ReceiverChip { theme: dialog.theme; objectName: "receiverPlcQam"; text: "PLC QAM"; Layout.fillWidth: true; onClicked: dialog.openQam(2, qam.viewIndex) }
        }
        Rectangle {
            Layout.fillWidth: true; Layout.preferredHeight: 110
            color: dialog.theme.panel; border.color: dialog.theme.line; radius: 12
            ColumnLayout {
                anchors.fill: parent; anchors.margins: 12
                Label { text: "Diagnostic Instruments"; color: dialog.theme.text; font.bold: true }
                RowLayout {
                    ReceiverChip { theme: dialog.theme; objectName: "receiverSa"; text: "Spectrum Analyzer"; Layout.fillWidth: true; onClicked: dialog.openSpectrum(dialog.selectedTap) }
                    ReceiverChip { theme: dialog.theme; text: "Channel Response / Echo"; Layout.fillWidth: true; onClicked: response.open() }
                    ReceiverChip { theme: dialog.theme; text: "Slicer / LLR"; Layout.fillWidth: true; onClicked: dialog.openSlicer() }
                    ReceiverChip { theme: dialog.theme; text: "FEC Monitor"; Layout.fillWidth: true; onClicked: fec.open() }
                }
            }
        }
        Item { Layout.fillHeight: true }
        Label { visible: !dialog.runState.result && !dialog.runState.can_start && !dialog.runState.busy; text: dialog.runState.reason || "Configure Receiver and enable PRBS-RX before starting."; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
    }
    ReceiverQamDialog { id: qam; theme: dialog.theme; backend: dialog.backend }
    ReceiverResponseDialog { id:response;theme:dialog.theme;backend:dialog.backend;runState:dialog.runState }
    ReceiverSlicerDialog { id:slicer;theme:dialog.theme;backend:dialog.backend;runState:dialog.runState }
    ReceiverFecDialog { id:fec;theme:dialog.theme;backend:dialog.backend;runState:dialog.runState }
    QtObject {
        id: spectrumProxy
        function cancel() {}
        function view(start,count,captureAxis) { return dialog.backend.view_tap(dialog.selectedTap,start,count,captureAxis) }
    }
    SpectrumAnalyzerDialog {
        id: spectrum; theme: dialog.theme; backend: spectrumProxy; state: dialog.spectrumState
        autoGenerate: false; showTiming: false; frequencyAxisAvailable: dialog.selectedTap < 3
        tapOptions: ["RX Input / Channel Output", "RX Framed Samples", "RX Useful Samples", "RX FFT Bin Power"]
        selectedTap: dialog.selectedTap
        onTapRequested: function(index) { dialog.selectTap(index) }
        generateDescription: "Use Start on the main workbench to run the simulation."
    }
}
