/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../about"
import "../components"
import "../inputs/measurement_input"
import "../inputs/ofdm_settings"
import "../inputs/profile_input"
import "../inputs/source_input"
import "../workbenches/channel"
import "../workbenches/receiver"
import "../workbenches/workbench"
import "../workbenches/transmitter"
import "../workbenches/verification"

Item {
    id: workbench
    required property var theme
    required property var settingsBackend
    required property var profileBackend
    required property var sourceBackend
    required property var aboutBackend
    required property var measurementBackend
    required property var fileBackend
    property bool hasProject: false
    property alias faqPopup: faqDialog
    function openHelp(code) { faqDialog.showEntry(code, code ? runState.diagnostic : null) }
    property alias configurationPopup: wbConfig
    property alias entryPopup: workbenchEntry
    function configureProject() {
        if (runState.busy) { return }
        if (hasProject) { wbConfig.open() }
        else { workbenchEntry.open() }
    }
    function requestNewProject() {
        if (runState.busy) { return }
        if (hasProject) { newConfirmation.open() }
        else { newProject() }
    }
    function newProject() {
        if (runState.busy) { return }
        const result=JSON.parse(fileBackend.new_workbench())
        if (result.created) { adoptProject(result);fileMessage="New workbench" }
        return !!result.created
    }
    property string fileMessage: ""
    function openProject() {
        if (runState.busy) { return }
        const result=JSON.parse(fileBackend.open())
        if (result.error) { fileMessage=result.error;fileFeedback.open();return }
        if (!result.opened) { return }
        adoptProject(result)
        fileMessage="Opened "+result.path
        return true
    }
    function adoptProject(result) {
        hasProject=true
        settingsBackend.apply(JSON.stringify(result.transmission))
        profileBackend.reload();sourceBackend.reload();measurementBackend.reload()
        runBackend.restore(JSON.stringify(result.receiver),result.verified,settingsBackend.current())
        if (result.dark !== null && result.dark !== undefined) { theme.dark=result.dark }
        refreshProfileState();refreshSourceState();pollRun()
    }
    function restoreUpdate() {
        const result=JSON.parse(fileBackend.restore_update())
        if (result.error) { fileMessage=result.error;fileFeedback.open();return }
        if (result.opened) { adoptProject(result) }
    }
    function saveUpdateRecovery() {
        return JSON.parse(fileBackend.save_update_recovery(settingsBackend.current(),JSON.stringify(runState.accepted_options || null),runState.verified,theme.dark))
    }
    function saveProject(saveAs) {
        if (runState.busy) { return }
        const result=JSON.parse(fileBackend.save(saveAs,settingsBackend.current(),JSON.stringify(runState.accepted_options || null),runState.verified,theme.dark))
        if (result.error) { fileMessage=result.error;fileFeedback.open() }
        else if (result.saved) { hasProject=true;fileMessage="Saved "+result.path;fileFeedback.open() }
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
    ColumnLayout {
        anchors.fill: parent; anchors.leftMargin: 24; anchors.rightMargin: 24
        anchors.topMargin: 24; anchors.bottomMargin: 20; spacing: 16
        Item {
            id: workbenchHeader
            Layout.fillWidth: true; Layout.preferredHeight: 88
            Text {
                id: toolkitTitle
                x: 0; y: 29
                width: Math.min(implicitWidth, workbenchHeader.width * 0.72)
                height: 34
                text: "MERLIN — DOCSIS OFDM Simulation Toolkit"
                color: theme.text; font.pixelSize: 23; font.weight: Font.DemiBold
                fontSizeMode: Text.HorizontalFit; minimumPixelSize: 18
                verticalAlignment: Text.AlignVCenter
            }
            Image {
                id: headerLogo
                anchors.right: parent.right; anchors.top: parent.top
                width: 145; height: 78
                fillMode: Image.PreserveAspectFit
                source: workbench.iconRoot + (theme.dark ? "merlin-logo-dark.png" : "merlin-logo-light.png")
            }
            Canvas {
                id: headerPulse
                x: toolkitTitle.x + toolkitTitle.width + 10; y: toolkitTitle.y + 4
                width: Math.max(0, headerLogo.x - x - 12); height: 34
                visible: width >= 45
                onWidthChanged: requestPaint()
                Connections { target: theme; function onDarkChanged() { headerPulse.requestPaint() } }
                onPaint: {
                    const ctx = getContext("2d")
                    ctx.reset(); ctx.strokeStyle = theme.accent; ctx.lineWidth = 1.7
                    const pulse = Math.max(24, width - 65)
                    ctx.beginPath(); ctx.moveTo(0, 17); ctx.lineTo(pulse - 22, 17)
                    ctx.lineTo(pulse - 16, 10); ctx.lineTo(pulse - 10, 25)
                    ctx.lineTo(pulse - 2, 2); ctx.lineTo(pulse + 6, 32)
                    ctx.lineTo(pulse + 14, 12); ctx.lineTo(pulse + 20, 17)
                    ctx.lineTo(width, 17); ctx.stroke()
                }
            }
            Rectangle {
                x: 0; y: 73; width: headerLogo.x - 24; height: 1
                gradient: Gradient {
                    orientation: Gradient.Horizontal
                    GradientStop { position: 0; color: "transparent" }
                    GradientStop { position: 0.6; color: theme.accent }
                    GradientStop { position: 1; color: "transparent" }
                }
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
                width: 1140; height: 580
                scale: Math.min((parent.width - 24) / width, (parent.height - 20) / height)
                Rectangle {
                    objectName: "transmitterConfigurationGroup"
                    x: 10; y: 130; width: 270; height: 320; radius: 20
                    color: theme.dark ? "#0b2332" : theme.accentSoft
                    border.color: theme.accent; border.width: 1.5
                    Label {
                        x: 10; y: 16; width: parent.width - 20
                        text: "Transmitter Configuration"
                        horizontalAlignment: Text.AlignHCenter
                        color: theme.text; font.pixelSize: 17; font.weight: Font.DemiBold
                    }
                }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 242.5; fromY: 230; toX: 260; toY: 230; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 242.5; fromY: 350; toX: 260; toY: 350; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 260; fromY: 230; toX: 260; toY: 350; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 260; fromY: 290; toX: 325; toY: 290 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 520; fromY: 290; toX: 630; toY: 290 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 825; fromY: 290; toX: 935; toY: 290 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 422.5; fromY: 155; toX: 422.5; toY: 245 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 1032.5; fromY: 245; toX: 1032.5; toY: 155 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 727.5; fromY: 155; toX: 727.5; toY: 245 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 880; fromY: 425; toX: 880; toY: 380; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 727.5; fromY: 380; toX: 1032.5; toY: 380; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 727.5; fromY: 380; toX: 727.5; toY: 335 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 1032.5; fromY: 380; toX: 1032.5; toY: 335 }
                WorkbenchBlock {
                    id: ofdmBlock; objectName: "ofdmBlock"; x: 47.5; y: 185; width: 195; height: 90
                    theme: workbench.theme; heading: "OFDM Channel\nDescriptor\n(OCD)"
                    blockStatus: workbench.profileState.ofdm_status
                    statusReason: workbench.profileState.reason
                    onClicked: ofdmDialog.open()
                }
                WorkbenchBlock {
                    id: sourceBlock; objectName: "sourceBlock"; x: 325; y: 65; width: 195; height: 90
                    theme: workbench.theme; heading: "PRBS-TX"
                    blockStatus: workbench.sourceState.status
                    statusReason: workbench.sourceState.reason
                    onClicked: sourceDialog.open()
                }
                WorkbenchBlock {
                    id: transmitterBlock; objectName: "transmitterBlock"; x: 325; y: 245; width: 195; height: 90
                    theme: workbench.theme; heading: "Transmitter"
                    blockStatus: workbench.transmitterState.status
                    statusReason: workbench.transmitterState.reason
                    onClicked: transmitterDialog.open()
                }
                WorkbenchBlock {
                    id: channelBlock; objectName: "channelBlock"; x: 630; y: 245; width: 195; height: 90
                    theme: workbench.theme; heading: "Channel\nSimulator"
                    blockStatus: workbench.measurementState.channel.status
                    statusReason: workbench.measurementState.channel.reason
                    onClicked: channelDialog.open()
                }
                WorkbenchBlock {
                    id: receiverBlock; objectName: "receiverBlock"; x: 935; y: 245; width: 195; height: 90
                    theme: workbench.theme; heading: "Receiver"
                    blockStatus: workbench.runState.receiver_status
                    statusReason: workbench.runState.reason
                    onClicked: receiverDialog.open()
                }
                WorkbenchBlock {
                    id: verificationBlock; objectName: "verificationBlock"; x: 935; y: 65; width: 195; height: 90
                    theme: workbench.theme; heading: "PRBS-RX"
                    blockStatus: workbench.runState.verify_status
                    statusReason: workbench.runState.verified ? "Independent recovered-source comparison enabled." : "Enable independent source comparison in PRBS-RX."
                    onClicked: verificationDialog.open()
                }
                WorkbenchBlock {
                    id: profileBlock; objectName: "profileBlock"; x: 47.5; y: 305; width: 195; height: 90
                    theme: workbench.theme; heading: "Downstream\nProfile\nDescriptor (DPD)"
                    pnmFileType: "CmDsOfdmModulationProfile"
                    blockStatus: workbench.profileState.profile_status
                    statusReason: workbench.profileState.reason; inputBlock: true
                    onClicked: profileDialog.open()
                }
                WorkbenchBlock {
                    id: channelEstimateBlock; objectName: "channelEstimateBlock"; x: 770; y: 425; width: 220; height: 90
                    theme: workbench.theme; heading: "DS Channel Est."
                    pnmFileType: "CmDsOfdmChanEstimateCoef"
                    blockStatus: workbench.measurementState.response.status
                    statusReason: workbench.measurementState.response.reason; inputBlock: true
                    onClicked: responseDialog.open()
                }
                WorkbenchBlock {
                    id: rxmerBlock; objectName: "rxmerBlock"; x: 630; y: 65; width: 195; height: 90
                    theme: workbench.theme; heading: "RxMER"
                    pnmFileType: "CmDsOfdmRxMer"
                    blockStatus: workbench.measurementState.rxmer.status
                    statusReason: workbench.measurementState.rxmer.reason; inputBlock: true
                    onClicked: rxmerDialog.open()
                }
            }
        }
        RowLayout {
            Layout.fillWidth: true; spacing: 10
            ColumnLayout {
                Layout.fillWidth: true; Layout.minimumWidth: 0; spacing: 4
                RowLayout {
                    Layout.fillWidth: true; spacing: 12
                    Label { text: workbench.runState.busy ? (workbench.runState.paused ? "PAUSED" : "RUNNING") : "SIMULATION WORKBENCH"; color: theme.accent; font.pixelSize: 11; font.letterSpacing: 1.5 }
                    Label {
                        objectName: "runElapsed"
                        visible: !!workbench.runState.timing
                        text: workbench.runState.timing ? "Elapsed " + workbench.runState.timing.elapsed_text + (workbench.runState.busy ? " · Profile " + workbench.runState.timing.profile_elapsed_text : "") : ""
                        color: theme.text; font.pixelSize: 12
                        Layout.fillWidth: true; wrapMode: Text.WordWrap
                        MouseArea { id: elapsedHover; anchors.fill: parent; hoverEnabled: true }
                        ToolTip.visible: elapsedHover.containsMouse
                        ToolTip.text: "Wall time includes pauses and CPU scheduling waits. The run budget uses active time."
                    }
                }
                ContextualStatus {
                    theme: workbench.theme
                    diagnostic: workbench.runState.diagnostic || null
                    onHelpRequested: function(code) { workbench.openHelp(code) }
                    objectName: "runProgress"
                    text: workbench.runState.diagnostic ? workbench.runState.reason : workbench.runState.busy ? "Profile " + (workbench.runState.profile_id ?? "—") + " · " + (workbench.runState.profile_index || 1) + " / " + (workbench.runState.profile_count || 1) + (workbench.runState.timing ? " · " + workbench.runState.timing.phase : "") + " · Symbols: " + workbench.runState.completed_symbols + (workbench.runState.timing ? " · Recovered: " + workbench.runState.timing.recovered_payloads : "") : workbench.runState.result ? "Run " + workbench.runState.result.status + " · " + workbench.runState.result.bits_tested + " bits compared · " + workbench.runState.result.bit_errors + " bit errors" : workbench.runState.reason
                    font.pixelSize: 12; Layout.fillWidth: true
                    activityTooltip: workbench.runState.busy && workbench.runState.timing ? "Time since last completed processing step: " + workbench.runState.timing.progress_age_text : ""
                }
            }
            WorkbenchActionButton {
                theme: workbench.theme
                objectName: "configureWorkbench"
                text: "Configure WorkBench"
                enabled: !workbench.runState.busy
                onClicked: workbench.configureProject()
                ToolTip.visible: hovered
                ToolTip.text: workbench.hasProject ? "Select profiles and run settings" : "Create or open a workbench"
            }
            WorkbenchActionButton {
                theme: workbench.theme
                primary: true
                Layout.preferredWidth: 104
                objectName: "startButton"; text: "▶  Start"
                enabled: workbench.hasProject && workbench.allBlocksComplete && workbench.runState.can_start
                onClicked: { workbench.runBackend.start();workbench.pollRun() }
                Accessible.name: "Start simulation"
                Accessible.description: !workbench.allBlocksComplete ? "Every workbench block must be green before starting." : "Wait for the current simulation to finish before starting another."
            }
            WorkbenchActionButton { theme: workbench.theme; Layout.preferredWidth: 104; objectName: "stopButton"; text: "■  Stop"; enabled: workbench.runState.busy; onClicked: workbench.runBackend.stop(); Accessible.name: "Stop simulation" }
            WorkbenchActionButton { theme: workbench.theme; Layout.preferredWidth: 104; objectName: "pauseButton"; text: workbench.runState.paused ? "▶  Resume" : "Ⅱ  Pause"; enabled: workbench.runState.busy; onClicked: { workbench.runBackend.pause();workbench.pollRun() } Accessible.name: workbench.runState.paused ? "Resume simulation" : "Pause simulation" }
        }
    }
    WorkbenchEntryDialog {
        id: workbenchEntry
        theme: workbench.theme
        onCreateRequested: { if (workbench.newProject()) wbConfig.open() }
        onOpenRequested: { if (workbench.openProject()) wbConfig.open() }
    }
    WorkbenchConfigurationDialog {
        id: wbConfig; theme: workbench.theme; backend: workbench.runBackend
        settingsBackend: workbench.settingsBackend; runState: workbench.runState
        onConfigurationApplied: workbench.pollRun()
    }
    WorkbenchDialog {
        id: newConfirmation; theme: workbench.theme; title: "New Workbench"
        parent: Overlay.overlay; anchors.centerIn: parent; modal: true
        standardButtons: Dialog.Yes | Dialog.Cancel
        onAccepted: workbench.newProject()
        contentItem: Label { text: "Create a blank workbench? Save your current WB first to keep changes."; color:workbench.theme.text; wrapMode:Text.WordWrap }
        width: Math.min(480, parent ? parent.width-40 : 480)
    }
    RxmerInputDialog {
        id: rxmerDialog; theme: workbench.theme; backend: workbench.measurementBackend; settingsBackend: workbench.settingsBackend
        onMeasurementApplied: workbench.refreshMeasurements()
    }
    ChannelResponseDialog {
        id: responseDialog; theme: workbench.theme; backend: workbench.measurementBackend; settingsBackend: workbench.settingsBackend
        onMeasurementApplied: workbench.refreshMeasurements()
    }
    ChannelWorkbenchDialog {
        id: channelDialog; theme: workbench.theme; configuration: workbench.measurementState
        measurementBackend:workbench.measurementBackend;settingsBackend:workbench.settingsBackend;transmitterBackend:workbench.transmitterBackend;receiverBackend:workbench.receiverBackend;runState:workbench.runState
        onModelApplied:workbench.refreshMeasurements()
        onRxmerRequested: rxmerDialog.open()
        onResponseRequested: responseDialog.open()
    }
    ReceiverWorkbenchDialog { id: receiverDialog; theme: workbench.theme; backend: workbench.receiverBackend; runState: workbench.runState; onConfigurationRequested: receiverConfig.open() }
    ReceiverConfigurationDialog { id: receiverConfig; theme: workbench.theme; backend: workbench.runBackend; settingsBackend: workbench.settingsBackend; runState: workbench.runState; onConfigurationApplied: workbench.pollRun() }
    VerificationDialog { id: verificationDialog; theme: workbench.theme; backend: workbench.runBackend; diagnosticsBackend: workbench.receiverBackend; runState: workbench.runState; onConfigurationApplied: workbench.pollRun() }
    FaqDialog { id: faqDialog; theme: workbench.theme; backend: workbench.aboutBackend }
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
    TransmitterWorkbenchDialog {
        id: transmitterDialog
        theme: workbench.theme
        backend: workbench.transmitterBackend
        transmitterState: workbench.transmitterState
        runState: workbench.runState
    }
    SectionPreviewDialog {
        id: sectionDialog
        theme: workbench.theme
        title: workbench.selectedSection
        description: workbench.sectionDescription
    }
}
