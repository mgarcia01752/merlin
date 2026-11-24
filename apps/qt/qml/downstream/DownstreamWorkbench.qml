/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/about"
import "../shared/components"
import "../shared/presentation"
import "../shell"
import "measurements"
import "configuration/ofdm_settings"
import "configuration/profile_input"
import "configuration/source_input"
import "channel"
import "receiver"
import "configuration"
import "transmitter"

Item {
    id: workbench
    required property var theme
    required property var settingsBackend
    required property var profileBackend
    required property var sourceBackend
    required property var aboutBackend
    required property var measurementBackend
    required property var fileBackend
    signal updatesRequested()
    signal externalLinkRequested(url target)

    // This owner covers persistent workbench DTOs and their native text copies.
    // Prepared project input states are adopted together. Optional rejection must
    // not freeze the existing fixed-size worker-control booleans.
    PresentationOwner {
        id: states
        theme: workbench.theme
        initialValue: workbench.stateCandidate({
            load:{loading:false,phase:"",elapsed_ms:0,cancelling:false},fileMessage:"",
            run:workbench.initialRunState(),measurement:workbench.initialMeasurementState(),
            transmitter:{status:"attention",reason:"Accept compatible OCD, DPD and PRBS-TX inputs to configure the transmitter.",busy:false,can_generate:false,has_result:false,result_token:""},
            source:{status:"attention",reason:"Apply a validated PRBS15 or file source before starting."},
            profile:{selected:false,profile_status:"attention",ofdm_status:"attention",reason:"A validated modulation-profile input is required.",summary:{}},
            selectedSection:"",sectionDescription:""
        })
    }
    /** Provide the existing compiled startup run state, before first admission. */
    function initialRunState() {
        return {receiver_status:"attention",verify_status:"attention",reason:"Configure Receiver",busy:false,can_start:false,paused:false,options:{},verified:false}
    }
    /** Provide the existing compiled startup input statuses. */
    function initialMeasurementState() {
        return {rxmer:{status:"attention",reason:"Configure RxMER"},response:{status:"attention",reason:"Apply DPD"},channel:{status:"attention",reason:"Configure channel inputs"},flat:true}
    }
    /**
     * Include independent status strings before any persistent native text copy.
     * @param value Existing workbench DTOs; their scientific contents are aliases.
     * @returns One complete display candidate using the unchanged format.
     */
    function stateCandidate(value) {
        const run = value.run
        const timing = run.timing
        value.display = {
            elapsed:timing ? "Elapsed "+timing.elapsed_text+(run.busy ? " · Profile "+timing.profile_elapsed_text : "") : "",
            progress:run.diagnostic ? run.reason : run.busy ? "Profile "+(run.profile_id ?? "—")+" · "+(run.profile_index || 1)+" / "+(run.profile_count || 1)+(timing ? " · "+timing.phase+(timing.mce ? " · MCE "+timing.mce : "") : "")+" · Symbols: "+run.completed_symbols+(timing ? " · Recovered: "+timing.recovered_payloads : "") : run.result ? "Run "+run.result.status+" · "+run.result.bits_tested+" bits compared · "+run.result.bit_errors+" bit errors" : run.reason,
            activity:run.busy && timing ? "Time since last completed processing step: "+timing.progress_age_text : ""
        }
        return value
    }
    /**
     * Admit changed state together with the previous unchanged state aliases.
     * @param changes Plain workbench fields; no assignment occurs on rejection.
     * @returns False leaves all accepted observations and labels together.
     */
    function adoptState(changes) {
        if (!states.value) {
            theme.presentationAdmissionRejected = true
            return false
        }
        const candidate = stateCandidate(Object.assign({},states.value,changes))
        const accepted = states.replace(candidate)
        if (!accepted) {
            theme.presentationAdmissionRejected=true
        }
        return accepted
    }
    /**
     * Adopt run details, or update only existing boolean control slots on pressure.
     * @param value Authoritative backend poll/configuration response.
     * @returns False preserves old detail; Stop/Pause/terminal controls remain live.
     * These four pre-existing keys retain identical UTF-16 and entry footprints.
     * No result, options, token, status text or scientific value is overwritten.
     */
    function adoptRunState(value) {
        if (adoptState({run:value})) {
            return true
        }
        for (const key of ["busy","paused","can_start","verified"]) {
            if (Object.prototype.hasOwnProperty.call(runState,key) && typeof value[key] === "boolean") {
                runState[key] = value[key]
            }
        }
        workbench.runStateChanged()
        return false
    }
    /**
     * Preserve transmitter observations on pressure while keeping cancel live.
     * @param value Authoritative transmitter state response.
     * @returns Whether the complete replacement was admitted.
     */
    function adoptTransmitterState(value) {
        if (adoptState({transmitter:value})) {
            return true
        }
        if (Object.prototype.hasOwnProperty.call(transmitterState,"busy") && typeof value.busy === "boolean") {
            transmitterState.busy = value.busy
            workbench.transmitterStateChanged()
        }
        return false
    }
    /** Open file feedback only after admitting its actual text. */
    function showFileMessage(message) {
        if (adoptState({fileMessage:message})) {
            fileFeedback.openChecked()
        }
    }

    property bool hasProject: false
    property bool loadingProject: false
    property bool configureAfterOpen: false
    readonly property var loadState: states.value ? states.value.load : ({loading:false, phase:"", elapsed_ms:0, cancelling:false})
    property alias faqPopup: faqDialog
    function openHelp(code) { faqDialog.showEntry(code, runState.diagnostic && runState.diagnostic.code===code ? runState.diagnostic : null) }
    property alias configurationPopup: wbConfig
    property alias entryPopup: workbenchEntry
    function configureProject() {
        if (runState.busy || loadingProject) { return }
        if (hasProject) { wbConfig.openChecked() }
        else { workbenchEntry.openChecked() }
    }
    function requestNewProject() {
        if (runState.busy || loadingProject) { return }
        if (hasProject) { newConfirmation.openChecked() }
        else { newProject() }
    }
    function newProject() {
        if (runState.busy || loadingProject) { return }
        const result=JSON.parse(fileBackend.new_workbench())
        if (result.error) { showFileMessage(result.error);return false }
        if (result.created) { adoptProject(result);adoptState({fileMessage:"New workbench"}) }
        return !!result.created
    }
    readonly property string fileMessage: states.value ? states.value.fileMessage : ""
    function openProject() {
        if (runState.busy || loadingProject) { return }
        beginOpen(JSON.parse(fileBackend.open()))
    }
    function beginOpen(result) {
        if (result.error) { showFileMessage(result.error);configureAfterOpen=false;return }
        adoptState({load:result})
        loadingProject=!!result.loading
        if (loadingProject) { loadingDialog.openChecked() }
    }
    function pollOpen() {
        if (!loadingProject) { return }
        const result=JSON.parse(fileBackend.poll_open())
        adoptState({load:result})
        if (result.loading) { return }
        if (result.opened) { adoptProject(result);adoptState({fileMessage:result.path ? "Opened "+result.path : "Workbench restored"}) }
        loadingProject=false
        loadingDialog.close()
        if (result.error) { showFileMessage(result.error) }
        if (result.opened && configureAfterOpen) { wbConfig.openChecked() }
        configureAfterOpen=false
    }
    function adoptProject(result) {
        hasProject=true
        settingsBackend.apply(JSON.stringify(result.transmission))
        profileBackend.reload();sourceBackend.reload();measurementBackend.reload()
        runBackend.restore(JSON.stringify(result.receiver),result.verified,settingsBackend.current())
        if (result.dark !== null && result.dark !== undefined) { theme.dark=result.dark }
        if (result.prepared) {
            adoptState({profile:result.profile_state,source:result.source_state,measurement:result.measurement_state})
            refreshTransmitterState();pollRun()
        } else { refreshProfileState();refreshSourceState();pollRun() }
    }
    function restoreUpdate() {
        beginOpen(JSON.parse(fileBackend.restore_update()))
    }
    function saveUpdateRecovery() {
        const current = JSON.parse(runBackend.poll())
        return JSON.parse(fileBackend.save_update_recovery(settingsBackend.current(),JSON.stringify(current.accepted_options || null),current.verified,theme.dark))
    }
    function saveProject(saveAs) {
        if (runState.busy || loadingProject) { return }
        const current = JSON.parse(runBackend.poll())
        const result=JSON.parse(fileBackend.save(saveAs,settingsBackend.current(),JSON.stringify(current.accepted_options || null),current.verified,theme.dark))
        if (result.error) { showFileMessage(result.error) }
        else if (result.saved) { hasProject=true;showFileMessage("Saved "+result.path) }
    }
    required property var runBackend
    readonly property var runState: states.value ? states.value.run : initialRunState()
    function pollRun() { adoptRunState(JSON.parse(runBackend.poll())) }
    function refreshRun() { adoptRunState(JSON.parse(runBackend.configure(settingsBackend.current()))) }
    property alias receiverConfigPopup: receiverConfig
    property alias verificationPopup: verificationDialog
    required property var receiverBackend
    property alias receiverPopup: receiverDialog
    property alias rxmerPopup: rxmerDialog
    property alias responsePopup: responseDialog
    property alias channelPopup: channelDialog
    readonly property var measurementState: states.value ? states.value.measurement : initialMeasurementState()
    function refreshMeasurements() { adoptState({measurement:JSON.parse(measurementBackend.state(settingsBackend.current()))}); refreshRun() }
    required property var transmitterBackend
    property alias settingsPopup: settingsDialog
    property alias aboutPopup: aboutDialog
    property alias transmitterPopup: transmitterDialog
    readonly property var transmitterState: states.value ? states.value.transmitter : ({})
    function refreshTransmitterState() { adoptTransmitterState(JSON.parse(transmitterBackend.configure(settingsBackend.current()))) }
    property alias sourcePopup: sourceDialog
    readonly property var sourceState: states.value ? states.value.source : ({})
    function refreshSourceState() { adoptState({source:JSON.parse(sourceBackend.state())}); refreshTransmitterState(); refreshRun() }
    property alias profilePopup: profileDialog
    readonly property var profileState: states.value ? states.value.profile : ({summary:{}})
    function refreshProfileState() {
        adoptState({profile:JSON.parse(profileBackend.state(settingsBackend.current()))})
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
    readonly property string selectedSection: states.value ? states.value.selectedSection : ""
    readonly property string sectionDescription: states.value ? states.value.sectionDescription : ""
    // Opens explanatory section content; editors will use Rust validation later.
    function openSection(title, description) {
        if (adoptState({selectedSection:title,sectionDescription:description})) {
            sectionDialog.openChecked()
        }
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
                source: workbench.iconRoot + (theme.dark ? "merlin-logo-dark-transparent.png" : "merlin-logo-light.png")
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
                enabled: !workbench.loadingProject
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
                Connection { objectName: "channelReceiverConnection"; anchors.fill: parent; theme: workbench.theme; fromX: 825; fromY: 290; toX: 935; toY: 290 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 422.5; fromY: 155; toX: 422.5; toY: 245 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 1032.5; fromY: 245; toX: 1032.5; toY: 155 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 727.5; fromY: 155; toX: 727.5; toY: 245 }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 880; fromY: 425; toX: 880; toY: 380; arrowHead: false }
                Connection { objectName: "estimateChannelConnection"; anchors.fill: parent; theme: workbench.theme; fromX: 727.5; fromY: 380; toX: 880; toY: 380; arrowHead: false }
                Connection { objectName: "estimateReceiverBranch"; visible: receiverBlock.indicatorText !== "MCE"; anchors.fill: parent; theme: workbench.theme; fromX: 880; fromY: 380; toX: 1032.5; toY: 380; arrowHead: false }
                Connection { anchors.fill: parent; theme: workbench.theme; fromX: 727.5; fromY: 380; toX: 727.5; toY: 335 }
                Connection { objectName: "estimateReceiverConnection"; visible: receiverBlock.indicatorText !== "MCE"; anchors.fill: parent; theme: workbench.theme; fromX: 1032.5; fromY: 380; toX: 1032.5; toY: 335 }
                WorkbenchBlock {
                    id: ofdmBlock; objectName: "ofdmBlock"; x: 47.5; y: 185; width: 195; height: 90
                    theme: workbench.theme; heading: "OFDM Channel\nDescriptor\n(OCD)"
                    blockStatus: workbench.profileState.ofdm_status
                    statusReason: workbench.profileState.reason
                    onClicked: ofdmDialog.openChecked()
                }
                WorkbenchBlock {
                    id: sourceBlock; objectName: "sourceBlock"; x: 325; y: 65; width: 195; height: 90
                    theme: workbench.theme; heading: "PRBS-TX"
                    blockStatus: workbench.sourceState.status
                    statusReason: workbench.sourceState.reason
                    onClicked: sourceDialog.openChecked()
                }
                WorkbenchBlock {
                    id: transmitterBlock; objectName: "transmitterBlock"; x: 325; y: 245; width: 195; height: 90
                    theme: workbench.theme; heading: "Transmitter"
                    blockStatus: workbench.transmitterState.status
                    statusReason: workbench.transmitterState.reason
                    onClicked: transmitterDialog.openChecked()
                }
                WorkbenchBlock {
                    id: channelBlock; objectName: "channelBlock"; x: 630; y: 245; width: 195; height: 90
                    theme: workbench.theme; heading: "Channel\nSimulator"
                    blockStatus: workbench.measurementState.channel.status
                    statusReason: workbench.measurementState.channel.reason
                    onClicked: channelDialog.openChecked()
                }
                WorkbenchBlock {
                    id: receiverBlock; objectName: "receiverBlock"; x: 935; y: 245; width: 195; height: 90
                    theme: workbench.theme; heading: "Receiver"
                    indicatorText: workbench.runState.options.receiver_mode === "merlin_estimated" ? "MCE" : ""
                    indicatorDescription: "MERLIN Channel Estimation enabled. Estimates correction from received pilots."
                    blockStatus: workbench.runState.receiver_status
                    statusReason: workbench.runState.reason
                    onClicked: receiverDialog.openChecked()
                }
                WorkbenchBlock {
                    id: verificationBlock; objectName: "verificationBlock"; x: 935; y: 65; width: 195; height: 90
                    theme: workbench.theme; heading: "PRBS-RX"
                    blockStatus: workbench.runState.verify_status
                    statusReason: workbench.runState.verified ? "Independent recovered-source comparison enabled." : "Enable independent source comparison in PRBS-RX."
                    onClicked: verificationDialog.openChecked()
                }
                WorkbenchBlock {
                    id: profileBlock; objectName: "profileBlock"; x: 47.5; y: 305; width: 195; height: 90
                    theme: workbench.theme; heading: "Downstream\nProfile\nDescriptor (DPD)"
                    pnmFileType: "CmDsOfdmModulationProfile"
                    blockStatus: workbench.profileState.profile_status
                    statusReason: workbench.profileState.reason; inputBlock: true
                    onClicked: profileDialog.openChecked()
                }
                WorkbenchBlock {
                    id: channelEstimateBlock; objectName: "channelEstimateBlock"; x: 770; y: 425; width: 220; height: 90
                    theme: workbench.theme; heading: "DS Channel Est."
                    pnmFileType: "CmDsOfdmChanEstimateCoef"
                    blockStatus: workbench.measurementState.response.status
                    statusReason: workbench.measurementState.response.reason; inputBlock: true
                    onClicked: responseDialog.openChecked()
                }
                WorkbenchBlock {
                    id: rxmerBlock; objectName: "rxmerBlock"; x: 630; y: 65; width: 195; height: 90
                    theme: workbench.theme; heading: "RxMER"
                    pnmFileType: "CmDsOfdmRxMer"
                    blockStatus: workbench.measurementState.rxmer.status
                    statusReason: workbench.measurementState.rxmer.reason; inputBlock: true
                    onClicked: rxmerDialog.openChecked()
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
                        text: states.value ? states.value.display.elapsed : ""
                        color: theme.text; font.pixelSize: 12
                        Layout.fillWidth: true; wrapMode: Text.WordWrap
                        MouseArea { id: elapsedHover; anchors.fill: parent; hoverEnabled: true }
                        ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: elapsedHover.containsMouse
                        ToolTip.text: "Wall time includes pauses and CPU scheduling waits. The run budget uses active time."
                    }
                }
                Label {
                    objectName: "presentationAdmissionNotice"
                    visible: workbench.theme.presentationAdmissionRejected
                    text: "Presentation limits reached; previous view preserved."
                    color: workbench.theme.statusAttention
                    wrapMode: Text.WordWrap
                    Layout.fillWidth: true
                }
                ContextualStatus {
                    theme: workbench.theme
                    diagnostic: workbench.runState.diagnostic || null
                    onHelpRequested: function(code) { workbench.openHelp(code) }
                    objectName: "runProgress"
                    text: states.value ? states.value.display.progress : ""
                    font.pixelSize: 12; Layout.fillWidth: true
                    activityTooltip: states.value ? states.value.display.activity : ""
                }
            }
            WorkbenchActionButton {
                theme: workbench.theme
                objectName: "configureWorkbench"
                text: "Configure WorkBench"
                enabled: !workbench.runState.busy && !workbench.loadingProject
                onClicked: workbench.configureProject()
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered
                ToolTip.text: workbench.hasProject ? "Select profiles and run settings" : "Create or open a workbench"
            }
            WorkbenchActionButton {
                theme: workbench.theme
                primary: true
                Layout.preferredWidth: 104
                objectName: "startButton"; text: "▶  Start"
                enabled: workbench.hasProject && workbench.allBlocksComplete && workbench.runState.can_start && !workbench.loadingProject && !states.rejected
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
        onCreateRequested: { if (workbench.newProject()) wbConfig.openChecked() }
        onOpenRequested: { workbench.configureAfterOpen=true;workbench.openProject() }
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
        id: rxmerDialog; onContextHelpRequested:function(code){workbench.openHelp(code)}; runOptions:workbench.runState.accepted_options || null; theme: workbench.theme; backend: workbench.measurementBackend; settingsBackend: workbench.settingsBackend
        onMeasurementApplied: workbench.refreshMeasurements()
    }
    ChannelResponseDialog {
        id: responseDialog; theme: workbench.theme; backend: workbench.measurementBackend; settingsBackend: workbench.settingsBackend
        onMeasurementApplied: workbench.refreshMeasurements()
    }
    ChannelWorkbenchDialog {
        onContextHelpRequested:function(code){workbench.openHelp(code)}
        id: channelDialog; theme: workbench.theme; configuration: workbench.measurementState
        runBackend:workbench.runBackend
        measurementBackend:workbench.measurementBackend;settingsBackend:workbench.settingsBackend;transmitterBackend:workbench.transmitterBackend;receiverBackend:workbench.receiverBackend;runState:workbench.runState
        onModelApplied:workbench.refreshMeasurements()
        onRxmerRequested: rxmerDialog.openChecked()
        onResponseRequested: responseDialog.openChecked()
    }
    ReceiverWorkbenchDialog { id: receiverDialog;onContextHelpRequested:function(code){workbench.openHelp(code)}; theme: workbench.theme; backend: workbench.receiverBackend; runState: workbench.runState; onConfigurationRequested: receiverConfig.openChecked() }
    ReceiverConfigurationDialog { id: receiverConfig; onContextHelpRequested:function(code){workbench.openHelp(code)}; theme: workbench.theme; backend: workbench.runBackend; settingsBackend: workbench.settingsBackend; runState: workbench.runState; onConfigurationApplied: workbench.pollRun() }
    VerificationDialog { id: verificationDialog; theme: workbench.theme; backend: workbench.runBackend; diagnosticsBackend: workbench.receiverBackend; runState: workbench.runState; onConfigurationApplied: workbench.pollRun() }
    FaqDialog { id: faqDialog; theme: workbench.theme; backend: workbench.aboutBackend }
    WorkbenchLoadingDialog {
        id: loadingDialog; theme: workbench.theme; loadState: workbench.loadState
        onCancelRequested: workbench.fileBackend.cancel_open()
    }
    Timer { interval: 100; running: workbench.loadingProject; repeat: true; onTriggered: workbench.pollOpen() }
    WorkbenchFileFeedback { id: fileFeedback; objectName: "workbenchFileFeedback"; theme:workbench.theme; message:workbench.fileMessage }
    SettingsDialog { id: settingsDialog; theme: workbench.theme }
    AboutDialog { id: aboutDialog; theme: workbench.theme; backend: workbench.aboutBackend; iconRoot: workbench.iconRoot; onUpdatesRequested: workbench.updatesRequested(); onLinkRequested: function(target) { workbench.externalLinkRequested(target) } }
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
        onTriggered: { workbench.adoptTransmitterState(JSON.parse(workbench.transmitterBackend.poll()));workbench.pollRun() }
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
