/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    required property var theme
    required property var backend
    property alias qamPopup: qam
    property alias spectrumPopup: spectrum
    property alias diagnosticPopup: diagnostic
    property int selectedTap: 0
    property var spectrumState: ({has_result: false, can_generate: false, busy: false})
    function openQam(signal, view) { qam.signalIndex = signal; qam.viewIndex = view; qam.open() }
    function selectTap(index) { selectedTap = index; spectrumState = JSON.parse(backend.spectrum(index)); spectrum.clearView() }
    function openSpectrum(index) { selectTap(index); spectrum.open() }
    function showDiagnostic(name, point, detail) { diagnostic.showInstrument(name, point, detail) }
    title: "Receiver Workbench"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1140, parent ? parent.width - 40 : 1140)
    height: Math.min(560, parent ? parent.height - 40 : 560)
    modal: true; standardButtons: Dialog.Close
    closePolicy: Popup.CloseOnEscape
    contentItem: ColumnLayout {
        spacing: 14
        Label { text: "Receiver Signal Path"; font.pixelSize: 19; font.bold: true; color: dialog.theme.text }
        Label { text: "Select a stage or test point to open its instrument. Receiver-run result wiring is pending."; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
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
                    ReceiverChip { theme: dialog.theme; text: "CH-EST /\nEqualizer"; Layout.fillWidth: true; stage: true; Layout.preferredHeight: 64; onClicked: dialog.showDiagnostic("Channel Response / Echo", "rx.channel_estimation.response", "Receiver-estimated H-hat is distinct from the channel-input response H. Response/echo result display is pending.") }
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
                    ReceiverChip { theme: dialog.theme; text: "Soft Demap /\nDecode"; Layout.fillWidth: true; stage: true; Layout.preferredHeight: 64; onClicked: dialog.showDiagnostic("Slicer / LLR", "rx.ncp.llrs · rx.data.llrs", "Soft demapping uses received I/Q, not hard-slicer decisions. NCP, PLC and payload have separate decode paths.") }
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
                    ReceiverChip { theme: dialog.theme; text: "Channel Response / Echo"; Layout.fillWidth: true; onClicked: dialog.showDiagnostic("Channel Response / Echo", "rx.channel_estimation.response", "Inspect estimated response H-hat and delay/echo results independently of spectral power. Result connection is pending.") }
                    ReceiverChip { theme: dialog.theme; text: "Slicer / LLR"; Layout.fillWidth: true; onClicked: dialog.showDiagnostic("Slicer / LLR", "rx.slicer.decisions · rx.ncp.llrs · rx.data.llrs", "Slicing is an optional diagnostic. The registered NCP/data soft records are separate from I/Q; streamed slicer decisions are still pending.") }
                    ReceiverChip { theme: dialog.theme; text: "FEC Monitor"; Layout.fillWidth: true; onClicked: dialog.showDiagnostic("FEC Monitor", "Receiver decoder outcomes", "Inspect real branch-specific decoder iterations and codeword outcomes. PRBS comparison applies to recovered payload only. Result connection is pending.") }
                }
            }
        }
        Item { Layout.fillHeight: true }
        Label { text: "No receiver results yet · instrument selection does not run the simulation · Receiver readiness remains yellow"; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
    }
    ReceiverQamDialog { id: qam; theme: dialog.theme; backend: dialog.backend }
    ReceiverDiagnosticDialog { id: diagnostic; theme: dialog.theme }
    SpectrumAnalyzerDialog {
        id: spectrum; theme: dialog.theme; backend: dialog.backend; state: dialog.spectrumState
        autoGenerate: false; showTiming: false
        tapOptions: ["RX Input / Channel Output", "RX Framed Samples", "RX Useful Samples", "RX FFT Bin Power"]
        selectedTap: dialog.selectedTap
        onTapRequested: function(index) { dialog.selectTap(index) }
        generateDescription: "Full-loop UI dispatch is pending. This receiver instrument never substitutes a transmitter preview."
    }
}
