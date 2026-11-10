/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    id: workbench
    required property var theme
    required property var settingsBackend
    property alias ofdmPopup: ofdmDialog
    property alias sectionPopup: sectionDialog
    property string iconRoot: "qrc:/branding/"
    readonly property var diagramBlocks: [ofdmBlock, sourceBlock, transmitterBlock, channelBlock, receiverBlock, verificationBlock, profileBlock, channelEstimateBlock, rxmerBlock]
    readonly property bool allBlocksComplete: diagramBlocks.every(block => block.blockStatus === "complete")
    // Run dispatch is not connected yet; readiness alone cannot start a job.
    readonly property bool runIntegrationAvailable: false
    property string selectedSection: ""
    property string sectionDescription: ""
    // Opens explanatory section content; editors will use Rust validation later.
    function openSection(title, description) {
        selectedSection = title
        sectionDescription = description
        sectionDialog.open()
    }
    Rectangle {
        width: 72; height: parent.height
        color: theme.panel
        Image {
            anchors.top: parent.top; anchors.topMargin: 22
            anchors.horizontalCenter: parent.horizontalCenter
            width: 48; height: 48; fillMode: Image.PreserveAspectFit
            source: theme.dark ? iconRoot + "merlin-icon-dark.png" : iconRoot + "merlin-icon-light.png"
        }
        Label {
            anchors.top: parent.top; anchors.topMargin: 88
            anchors.horizontalCenter: parent.horizontalCenter
            text: "WB"; color: theme.accent; font.bold: true; font.pixelSize: 14
        }
        Rectangle {
            anchors.right: parent.right; width: 1; height: parent.height; color: theme.line
        }
    }
    ColumnLayout {
        anchors.fill: parent; anchors.leftMargin: 96; anchors.rightMargin: 24
        anchors.topMargin: 24; anchors.bottomMargin: 20; spacing: 16
        RowLayout {
            Layout.fillWidth: true
            ColumnLayout {
                spacing: 5
                Label { text: "MERLIN"; color: theme.accent; font.pixelSize: 13; font.letterSpacing: 3 }
                Label { text: "Simulation Workbench"; color: theme.text; font.pixelSize: 25; font.weight: Font.DemiBold }
                Label { text: "Single-channel downstream OFDM"; color: theme.muted; font.pixelSize: 13 }
            }
            Item { Layout.fillWidth: true }
            Button {
                objectName: "themeToggle"
                text: theme.dark ? "Light mode" : "Dark mode"
                onClicked: theme.dark = !theme.dark
            }
        }
        Rectangle {
            Layout.fillWidth: true; implicitHeight: 42; radius: 6
            color: theme.accentSoft
            Label {
                anchors.centerIn: parent
                text: "Start with OFDM Channel Descriptor (OCD)  ·  Click any block to explore its section"
                color: theme.accent; font.pixelSize: 13
            }
        }
        Rectangle {
            Layout.fillWidth: true; Layout.fillHeight: true
            color: theme.background; border.color: theme.line; radius: 12
            clip: true
            Canvas {
                id: grid
                anchors.fill: parent
                onWidthChanged: requestPaint()
                onHeightChanged: requestPaint()
                Connections { target: theme; function onDarkChanged() { grid.requestPaint() } }
                onPaint: {
                    const ctx = getContext("2d")
                    ctx.reset(); ctx.fillStyle = theme.line
                    for (let x = 16; x < width; x += 24) {
                        for (let y = 16; y < height; y += 24) {
                            ctx.fillRect(x, y, 1, 1)
                        }
                    }
                }
            }
            Item {
                id: diagram
                anchors.centerIn: parent
                width: 1040; height: 470
                scale: Math.min((parent.width - 24) / width, (parent.height - 20) / height)
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 150; fromY: 235; toX: 215; toY: 235 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 385; fromY: 235; toX: 520; toY: 235 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 710; fromY: 235; toX: 845; toY: 235 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 300; fromY: 125; toX: 300; toY: 190 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 930; fromY: 190; toX: 930; toY: 125 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 385; fromY: 80; toX: 845; toY: 80; comparison: true }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 300; fromY: 360; toX: 300; toY: 280 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 530; fromY: 360; toX: 580; toY: 280 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 725; fromY: 360; toX: 655; toY: 280 }
                Label { x: 470; y: 48; width: 300; horizontalAlignment: Text.AlignHCenter; text: "Original ↔ recovered data comparison"; color: theme.muted; font.pixelSize: 12 }
                WorkbenchBlock {
                    id: ofdmBlock; objectName: "ofdmBlock"; x: 10; y: 190; width: 140; height: 90
                    theme: workbench.theme; heading: "OFDM Channel\nDescriptor (OCD)"
                    blockStatus: "attention"
                    statusReason: "A validated modulation-profile PNM input is required. Profile import is not connected yet."
                    onClicked: ofdmDialog.open()
                }
                WorkbenchBlock {
                    id: sourceBlock; objectName: "sourceBlock"; x: 215; y: 35; width: 170; height: 90
                    theme: workbench.theme; heading: "PRBS-TX"
                    blockStatus: "attention"
                    statusReason: "Source configuration and simulation integration are pending."
                    onClicked: workbench.openSection(heading, "Select continuous PRBS15 or exact text/binary payload bytes. File selection and explicit PRBS15 fill controls will be added here.\n\nOriginal source data remains the independent reference for recovered-data verification.")
                }
                WorkbenchBlock {
                    id: transmitterBlock; objectName: "transmitterBlock"; x: 215; y: 190; width: 170; height: 90
                    theme: workbench.theme; heading: "Transmitter"
                    blockStatus: "attention"
                    statusReason: "Transmitter integration and a validated run configuration are pending."
                    onClicked: workbench.openSection(heading, "Inspect the Rust transmitter stages: FEC coding, mapping, interleaving and waveform generation.\n\nTransmission settings and the applied modulation profile configure the next run.")
                }
                WorkbenchBlock {
                    id: channelBlock; objectName: "channelBlock"; x: 520; y: 190; width: 190; height: 90
                    theme: workbench.theme; heading: "Channel Simulator"
                    blockStatus: "attention"
                    statusReason: "Channel configuration and simulation integration are pending."
                    onClicked: workbench.openSection(heading, "Configure the reference channel or measurement-driven impairments using the existing Rust engine.\n\nChannel-estimation and RxMER inputs belong to this section. No measurement files are loaded yet.")
                }
                WorkbenchBlock {
                    id: receiverBlock; objectName: "receiverBlock"; x: 845; y: 190; width: 170; height: 90
                    theme: workbench.theme; heading: "Receiver"
                    blockStatus: "attention"
                    statusReason: "Receiver integration and a validated run configuration are pending."
                    onClicked: workbench.openSection(heading, "Inspect receiver acquisition, FFT, channel correction and decoding.\n\nSupplied-H reference correction and pilot-estimated H-hat remain separate modes. Stage observations will come from bounded Rust test points.")
                }
                WorkbenchBlock {
                    id: verificationBlock; objectName: "verificationBlock"; x: 845; y: 35; width: 170; height: 90
                    theme: workbench.theme; heading: "PRBS-RX"
                    blockStatus: "attention"
                    statusReason: "Recovered-data comparison setup and simulation integration are pending."
                    onClicked: workbench.openSection(heading, "Compare recovered bits with the independent original source and inspect BER and completion. File payloads retain exact-byte verification.\n\nThis comparison never feeds back into the source. There are no simulation results yet.")
                }
                WorkbenchBlock {
                    id: profileBlock; objectName: "profileBlock"; x: 215; y: 360; width: 170; height: 90
                    theme: workbench.theme; heading: "Modulation Profile"
                    blockStatus: "attention"
                    statusReason: "No validated modulation-profile PNM input is selected. Profile import is pending."; inputBlock: true
                    onClicked: workbench.openSection(heading, "Select a supported decoded modulation-profile PNM JSON document.\n\nThe file picker and Rust import validation are planned. Native binary PNM decoding is not part of this workbench.")
                }
                WorkbenchBlock {
                    id: channelEstimateBlock; objectName: "channelEstimateBlock"; x: 445; y: 360; width: 170; height: 90
                    theme: workbench.theme; heading: "DS Channel Est."
                    blockStatus: "attention"
                    statusReason: "Optional channel-estimation input is not configured. Input integration is pending."; inputBlock: true
                    onClicked: workbench.openSection(heading, "Supply downstream channel-estimation input through the Rust decoded-JSON importer.\n\nThe channel uses response H; supplied-H receiver correction is a separately labeled reference mode. File selection is planned.")
                }
                WorkbenchBlock {
                    id: rxmerBlock; objectName: "rxmerBlock"; x: 640; y: 360; width: 170; height: 90
                    theme: workbench.theme; heading: "RxMER"
                    blockStatus: "attention"
                    statusReason: "Optional RxMER input is not configured. Input integration is pending."; inputBlock: true
                    onClicked: workbench.openSection(heading, "Supply RxMER observations for the channel's residual-error model.\n\nMultiple inputs require an explicit validated epoch schedule. Listing files alone will not start playback. File selection is planned.")
                }
            }
        }
        RowLayout {
            Layout.fillWidth: true; spacing: 10
            ColumnLayout {
                Label { text: "WORKBENCH PREVIEW"; color: theme.accent; font.pixelSize: 11; font.letterSpacing: 1.5 }
                Label { text: "OCD settings are editable. Simulation controls are not connected yet."; color: theme.muted; font.pixelSize: 12 }
            }
            Item { Layout.fillWidth: true }
            Button {
                objectName: "startButton"; text: "▶  Start"
                enabled: workbench.allBlocksComplete && workbench.runIntegrationAvailable
                Accessible.name: "Start simulation"
                Accessible.description: !workbench.allBlocksComplete ? "Every workbench block must be green before starting." : "Simulation run integration is pending."
            }
            Button { objectName: "stopButton"; text: "■  Stop"; enabled: false; Accessible.name: "Stop simulation" }
            Button { objectName: "pauseButton"; text: "Ⅱ  Pause"; enabled: false; Accessible.name: "Pause simulation" }
        }
    }
    OfdmSettingsDialog {
        id: ofdmDialog
        theme: workbench.theme
        backend: workbench.settingsBackend
    }
    SectionPreviewDialog {
        id: sectionDialog
        theme: workbench.theme
        title: workbench.selectedSection
        description: workbench.sectionDescription
    }
}
