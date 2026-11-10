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
    required property var profileBackend
    required property var sourceBackend
    required property var aboutBackend
    required property var measurementBackend
    required property var fileBackend
    property string fileMessage: ""
    function openProject() {
        if (runState.busy) { return }
        const result=JSON.parse(fileBackend.open())
        if (result.error) { fileMessage=result.error;fileFeedback.open();return }
        if (!result.opened) { return }
        settingsBackend.apply(JSON.stringify(result.transmission))
        profileBackend.reload();sourceBackend.reload();measurementBackend.reload()
        runBackend.restore(JSON.stringify(result.receiver),result.verified,settingsBackend.current())
        theme.dark=result.dark
        refreshProfileState();refreshSourceState();pollRun()
        fileMessage="Opened "+result.path
    }
    function saveProject(saveAs) {
        if (runState.busy) { return }
        const result=JSON.parse(fileBackend.save(saveAs,settingsBackend.current(),JSON.stringify(runState.accepted_options || null),runState.verified,theme.dark))
        if (result.error) { fileMessage=result.error;fileFeedback.open() }
        else if (result.saved) { fileMessage="Saved "+result.path;fileFeedback.open() }
    }
    required property var runBackend
    property var runState: ({receiver_status:"attention",verify_status:"attention",reason:"Configure Receiver",busy:false,can_start:false,paused:false,options:{},verified:false})
    function pollRun() { runState=JSON.parse(runBackend.poll()) }
    function refreshRun() { runState=JSON.parse(runBackend.configure(settingsBackend.current())) }
    property alias receiverConfigPopup: receiverConfig
    property alias verificationPopup: verificationDialog
    required property var receiverBackend
    property alias receiverPopup: receiverDialog
    property alias rxmerPopup: rxmerDialog
    property alias responsePopup: responseDialog
    property alias channelPopup: channelDialog
    property var measurementState: ({rxmer: {status: "attention", reason: "Configure RxMER"}, response: {status: "attention", reason: "Apply DPD"}, channel: {status: "attention", reason: "Configure channel inputs"}, flat: true})
    function refreshMeasurements() { measurementState = JSON.parse(measurementBackend.state(settingsBackend.current())); refreshRun() }
    required property var transmitterBackend
    property alias settingsPopup: settingsDialog
    property alias aboutPopup: aboutDialog
    property alias transmitterPopup: transmitterDialog
    property var transmitterState: ({status: "attention", reason: "Accept compatible OCD, DPD and PRBS-TX inputs to configure the transmitter.", busy: false, can_generate: false, has_result: false, result_token: ""})
    function refreshTransmitterState() { transmitterState = JSON.parse(transmitterBackend.configure(settingsBackend.current())) }
    property alias sourcePopup: sourceDialog
    property var sourceState: ({status: "attention", reason: "Apply a validated PRBS15 or file source before starting."})
    function refreshSourceState() { sourceState = JSON.parse(sourceBackend.state()); refreshTransmitterState(); refreshRun() }
    property alias profilePopup: profileDialog
    property var profileState: ({selected: false, profile_status: "attention", ofdm_status: "attention", reason: "A validated modulation-profile input is required.", summary: {}})
    function refreshProfileState() {
        profileState = JSON.parse(profileBackend.state(settingsBackend.current()))
        refreshTransmitterState()
        refreshMeasurements()
    }
    Component.onCompleted: { refreshProfileState(); refreshSourceState() }
    property alias ofdmPopup: ofdmDialog
    property alias sectionPopup: sectionDialog
    property string iconRoot: "qrc:/branding/"
    readonly property var diagramBlocks: [ofdmBlock, sourceBlock, transmitterBlock, channelBlock, receiverBlock, verificationBlock, profileBlock, channelEstimateBlock, rxmerBlock]
    readonly property bool allBlocksComplete: diagramBlocks.every(block => block.blockStatus === "complete")
    // Start requires accepted block readiness and one idle Rust worker.
    readonly property bool runIntegrationAvailable: true
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
                Label { text: "MERLIN — DOCSIS OFDM Simulation Toolkit"; color: theme.text; font.pixelSize: 23; font.weight: Font.DemiBold }
            }
            Item { Layout.fillWidth: true }
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
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 300; fromY: 360; toX: 300; toY: 280 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 615; fromY: 125; toX: 615; toY: 190 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 777.5; fromY: 360; toX: 777.5; toY: 325; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 680; fromY: 325; toX: 880; toY: 325; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 680; fromY: 325; toX: 680; toY: 280 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 880; fromY: 325; toX: 880; toY: 280 }
                Label { x: 652.5; y: 455; width: 250; horizontalAlignment: Text.AlignHCenter; text: "Supplied H · reference correction"; color: theme.muted; font.pixelSize: 10 }
                WorkbenchBlock {
                    id: ofdmBlock; objectName: "ofdmBlock"; x: 10; y: 190; width: 140; height: 90
                    theme: workbench.theme; heading: "OFDM Channel\nDescriptor (OCD)"
                    blockStatus: workbench.profileState.ofdm_status
                    statusReason: workbench.profileState.reason
                    onClicked: ofdmDialog.open()
                }
                WorkbenchBlock {
                    id: sourceBlock; objectName: "sourceBlock"; x: 215; y: 35; width: 170; height: 90
                    theme: workbench.theme; heading: "PRBS-TX"
                    blockStatus: workbench.sourceState.status
                    statusReason: workbench.sourceState.reason
                    onClicked: sourceDialog.open()
                }
                WorkbenchBlock {
                    id: transmitterBlock; objectName: "transmitterBlock"; x: 215; y: 190; width: 170; height: 90
                    theme: workbench.theme; heading: "Transmitter"
                    blockStatus: workbench.transmitterState.status
                    statusReason: workbench.transmitterState.reason
                    onClicked: transmitterDialog.open()
                }
                WorkbenchBlock {
                    id: channelBlock; objectName: "channelBlock"; x: 520; y: 190; width: 190; height: 90
                    theme: workbench.theme; heading: "Channel Simulator"
                    blockStatus: workbench.measurementState.channel.status
                    statusReason: workbench.measurementState.channel.reason
                    onClicked: channelDialog.open()
                }
                WorkbenchBlock {
                    id: receiverBlock; objectName: "receiverBlock"; x: 845; y: 190; width: 170; height: 90
                    theme: workbench.theme; heading: "Receiver"
                    blockStatus: workbench.runState.receiver_status
                    statusReason: workbench.runState.reason
                    onClicked: receiverDialog.open()
                }
                WorkbenchBlock {
                    id: verificationBlock; objectName: "verificationBlock"; x: 845; y: 35; width: 170; height: 90
                    theme: workbench.theme; heading: "PRBS-RX"
                    blockStatus: workbench.runState.verify_status
                    statusReason: workbench.runState.verified ? "Independent recovered-source comparison enabled." : "Enable independent source comparison in PRBS-RX."
                    onClicked: verificationDialog.open()
                }
                WorkbenchBlock {
                    id: profileBlock; objectName: "profileBlock"; x: 215; y: 360; width: 170; height: 90
                    theme: workbench.theme; heading: "Downstream Profile\nDescriptor (DPD)"
                    blockStatus: workbench.profileState.profile_status
                    statusReason: workbench.profileState.reason; inputBlock: true
                    onClicked: profileDialog.open()
                }
                WorkbenchBlock {
                    id: channelEstimateBlock; objectName: "channelEstimateBlock"; x: 692.5; y: 360; width: 170; height: 90
                    theme: workbench.theme; heading: "DS Channel Est."
                    blockStatus: workbench.measurementState.response.status
                    statusReason: workbench.measurementState.response.reason; inputBlock: true
                    onClicked: responseDialog.open()
                }
                WorkbenchBlock {
                    id: rxmerBlock; objectName: "rxmerBlock"; x: 530; y: 35; width: 170; height: 90
                    theme: workbench.theme; heading: "RxMER"
                    blockStatus: workbench.measurementState.rxmer.status
                    statusReason: workbench.measurementState.rxmer.reason; inputBlock: true
                    onClicked: rxmerDialog.open()
                }
            }
        }
        RowLayout {
            Layout.fillWidth: true; spacing: 10
            ColumnLayout {
                Label { text: workbench.runState.busy ? (workbench.runState.paused ? "PAUSED" : "RUNNING") : "SIMULATION WORKBENCH"; color: theme.accent; font.pixelSize: 11; font.letterSpacing: 1.5 }
                Label { text: workbench.runState.busy ? "Completed observation symbols: "+workbench.runState.completed_symbols : workbench.runState.result ? "Run "+workbench.runState.result.status+" · "+workbench.runState.result.bits_tested+" bits compared · "+workbench.runState.result.bit_errors+" bit errors" : workbench.runState.reason; color: theme.muted; font.pixelSize: 12 }
            }
            Item { Layout.fillWidth: true }
            Button {
                objectName: "startButton"; text: "▶  Start"
                enabled: workbench.allBlocksComplete && workbench.runState.can_start
                onClicked: { workbench.runBackend.start();workbench.pollRun() }
                Accessible.name: "Start simulation"
                Accessible.description: !workbench.allBlocksComplete ? "Every workbench block must be green before starting." : "The Rust worker must be idle before starting."
            }
            Button { objectName: "stopButton"; text: "■  Stop"; enabled: workbench.runState.busy; onClicked: workbench.runBackend.stop(); Accessible.name: "Stop simulation" }
            Button { objectName: "pauseButton"; text: workbench.runState.paused ? "▶  Resume" : "Ⅱ  Pause"; enabled: workbench.runState.busy; onClicked: { workbench.runBackend.pause();workbench.pollRun() } Accessible.name: "Pause simulation" }
        }
    }
    RxmerInputDialog {
        id: rxmerDialog; theme: workbench.theme; backend: workbench.measurementBackend; settingsBackend: workbench.settingsBackend
        onMeasurementApplied: workbench.refreshMeasurements()
    }
    ChannelResponseDialog {
        id: responseDialog; theme: workbench.theme; backend: workbench.measurementBackend; settingsBackend: workbench.settingsBackend
        onMeasurementApplied: workbench.refreshMeasurements()
    }
    ChannelConfigurationDialog {
        id: channelDialog; theme: workbench.theme; configuration: workbench.measurementState
        onRxmerRequested: rxmerDialog.open()
        onResponseRequested: responseDialog.open()
    }
    ReceiverWorkbenchDialog { id: receiverDialog; theme: workbench.theme; backend: workbench.receiverBackend; runState: workbench.runState; onConfigurationRequested: receiverConfig.open() }
    ReceiverConfigurationDialog { id: receiverConfig; theme: workbench.theme; backend: workbench.runBackend; settingsBackend: workbench.settingsBackend; runState: workbench.runState; onConfigurationApplied: workbench.pollRun() }
    VerificationDialog { id: verificationDialog; theme: workbench.theme; backend: workbench.runBackend; runState: workbench.runState; onConfigurationApplied: workbench.pollRun() }
    WorkbenchFileFeedback { id: fileFeedback; theme:workbench.theme; message:workbench.fileMessage }
    SettingsDialog { id: settingsDialog; theme: workbench.theme }
    AboutDialog { id: aboutDialog; theme: workbench.theme; backend: workbench.aboutBackend; iconRoot: workbench.iconRoot }
    OfdmSettingsDialog {
        id: ofdmDialog
        theme: workbench.theme
        backend: workbench.settingsBackend
        profileSummary: workbench.profileState.summary
        onSettingsApplied: workbench.refreshProfileState()
    }
    ProfileInputDialog {
        id: profileDialog
        theme: workbench.theme
        backend: workbench.profileBackend
        settingsBackend: workbench.settingsBackend
        onInputApplied: workbench.refreshProfileState()
    }
    SourceInputDialog {
        id: sourceDialog
        theme: workbench.theme
        backend: workbench.sourceBackend
        onSourceApplied: workbench.refreshSourceState()
    }
    Timer {
        interval: 100; running: true; repeat: true
        onTriggered: { workbench.transmitterState = JSON.parse(workbench.transmitterBackend.poll());workbench.pollRun() }
    }
    SpectrumAnalyzerDialog {
        id: transmitterDialog
        theme: workbench.theme
        backend: workbench.transmitterBackend
        state: workbench.transmitterState
    }
    SectionPreviewDialog {
        id: sectionDialog
        theme: workbench.theme
        title: workbench.selectedSection
        description: workbench.sectionDescription
    }
}
