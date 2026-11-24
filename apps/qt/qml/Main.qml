/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Merlin.Workbench
import "shell"
import "shared/theme"
import "downstream"
import "upstream"
import "shell/directions"
import "validation"

ApplicationWindow {
    id: window
    width: 1240
    height: 780
    minimumWidth: 960
    minimumHeight: 640
    visible: true
    title: "MERLIN — DOCSIS OFDM/OFDMA Simulation Workbench"
    Timer { id: smokeTimer; interval:750; onTriggered:Qt.quit() }
    Branding { id: branding }
    AboutInfo { id: aboutInfo }
    UpdateInfo { id: updateInfo }
    OfdmSettings { id: ofdmSettings }
    ProfileInput { id: profileInput }
    SourceInput { id: sourceInput }
    MeasurementInputs { id: measurementInputs }
    ReceiverInstruments { id: receiverInstruments }
    ClosedLoop { id: closedLoop }
    UpstreamSession { id: upstreamSession }
    DirectionShell { id: directionBackend }
    DirectionShellHost {
        id: directionHost
        theme: applicationTheme
        backend: directionBackend
        downstreamBackend: closedLoop
        upstreamBackend: upstreamSession
        downstreamView: window.workbench
        upstreamView: directionTabs.upstreamItem
    }
    readonly property var workbench: directionTabs.downstreamItem
    property alias tabs: directionTabs
    property alias shellHost: directionHost
    property alias runControls: sharedControls
    readonly property string selectedDirection: directionHost.presentation.selectedDirection || ""
    /** Keep active-direction file actions away from an unsupported foreign adapter. */
    function routeFile(action, saveAs) {
        const identity = directionTabs.identityFor(selectedDirection)
        if (selectedDirection !== "downstream" || !workbench
                || !directionHost.accepts(selectedDirection, identity)) {
            return false
        }
        if (action === "new") {
            workbench.requestNewProject()
        } else if (action === "open") {
            workbench.openProject()
        } else if (action === "save") {
            workbench.saveProject(saveAs)
        } else {
            return false
        }
        return true
    }
    WorkbenchFile { id: workbenchFile }
    TransmitterAnalyzer { id: transmitterAnalyzer }
    property Component presentationBackendComponent: Component { PresentationRetention {} }
    function createPresentationBackend(parent) {
        return presentationBackendComponent.createObject(parent)
    }
    Theme { id: applicationTheme; presentationBackendFactory: window.createPresentationBackend; onDarkChanged: branding.update_icon(dark) }
    readonly property bool presentationProbeRequested: Qt.application.arguments.indexOf("--session-presentation-probe") >= 0
    readonly property bool directionProbeRequested: Qt.application.arguments.indexOf("--direction-shell-probe") >= 0
    readonly property bool upstreamConfigurationProbeRequested: Qt.application.arguments.indexOf("--upstream-configuration-probe") >= 0
    readonly property bool upstreamExecutionProbeRequested: Qt.application.arguments.indexOf("--upstream-execution-probe") >= 0
    property QtObject presentationProbe: null
    property QtObject directionProbe: null
    property QtObject upstreamConfigurationProbe: null
    property QtObject upstreamExecutionProbe: null
    property Component upstreamExecutionProbeComponent: Component {
        UpstreamExecutionProbe {
            onFinished: function(success, message) {
                console.log("MERLIN_UPSTREAM_EXECUTION_PROBE " + message)
                Qt.exit(success ? 0 : 1)
            }
        }
    }
    /** Run the fixed native execution check with a fresh evidence path. */
    function startUpstreamExecutionProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--upstream-execution-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Upstream execution probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        upstreamExecutionProbe = upstreamExecutionProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            screenshotPath: args[imageArgument + 1]
        })
        if (!upstreamExecutionProbe) {
            Qt.exit(1)
        }
    }
    property Component upstreamConfigurationProbeComponent: Component {
        UpstreamConfigurationProbe {
            onFinished: function(success, message) {
                console.log("MERLIN_UPSTREAM_CONFIGURATION_PROBE " + message)
                Qt.exit(success ? 0 : 1)
            }
        }
    }
    /** Run the fixed native configuration check with a fresh evidence path. */
    function startUpstreamConfigurationProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--upstream-configuration-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Upstream configuration probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        upstreamConfigurationProbe = upstreamConfigurationProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            screenshotPath: args[imageArgument + 1]
        })
        if (!upstreamConfigurationProbe) {
            Qt.exit(1)
        }
    }
    property Component directionProbeComponent: Component {
        DirectionShellProbe {
            onFinished: function(success, message) {
                console.log("MERLIN_DIRECTION_SHELL_PROBE " + message)
                Qt.exit(success ? 0 : 1)
            }
        }
    }
    /** Run the fixed native shell check only with two caller-owned evidence paths. */
    function startDirectionProbe() {
        const args = Qt.application.arguments
        const downstreamImage = args.indexOf("--direction-shell-downstream-image")
        const upstreamImage = args.indexOf("--direction-shell-upstream-image")
        if (downstreamImage < 0 || downstreamImage + 1 >= args.length
                || upstreamImage < 0 || upstreamImage + 1 >= args.length) {
            console.error("Direction shell probe requires two fresh screenshot paths.")
            Qt.exit(1)
            return
        }
        directionProbe = directionProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            downstreamImagePath: args[downstreamImage + 1], upstreamImagePath: args[upstreamImage + 1]
        })
        if (!directionProbe) {
            Qt.exit(1)
        }
    }
    property Component presentationProbeComponent: Component {
        SessionPresentationProbe {
            onFinished: function(success, message) {
                console.log("MERLIN_SESSION_PRESENTATION_PROBE " + message)
                Qt.exit(success ? 0 : 1)
            }
        }
    }
    /** Run only the fixed repository probe; normal startup creates no probe owner. */
    function startPresentationProbe() {
        const applicationArguments = Qt.application.arguments
        const imageArgument = applicationArguments.indexOf("--session-presentation-image")
        if (imageArgument < 0 || imageArgument + 1 >= applicationArguments.length) {
            console.error("Session presentation probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        presentationProbe = presentationProbeComponent.createObject(window, {
            theme: applicationTheme, workbench: workbench, window: window,
            screenshotPath: applicationArguments[imageArgument + 1], captureItem: unifiedFrame
        })
        if (!presentationProbe) {
            Qt.exit(1)
        }
    }
    Component.onCompleted: {
        branding.update_icon(applicationTheme.dark)
        if (Qt.application.arguments.indexOf("--restore-update") >= 0) {
            Qt.callLater(function() { workbench.restoreUpdate() })
        } else {
            Qt.callLater(function() { workbench.beginOpen(JSON.parse(workbenchFile.startup())) })
        }
        if (presentationProbeRequested) {
            Qt.callLater(startPresentationProbe)
        } else if (directionProbeRequested) {
            Qt.callLater(startDirectionProbe)
        } else if (upstreamConfigurationProbeRequested) {
            Qt.callLater(startUpstreamConfigurationProbe)
        } else if (upstreamExecutionProbeRequested) {
            Qt.callLater(startUpstreamExecutionProbe)
        } else if (Qt.application.arguments.indexOf("--smoke-test") >= 0) {
            smokeTimer.start()
        } else if (Qt.application.arguments.indexOf("--check-updates") >= 0) {
            Qt.callLater(function() { updates.showUpdates(); updates.refresh() })
        }
    }
    ThemePalette { targetControl: window; theme: applicationTheme }
    color: applicationTheme.background
    onClosing: function(close) {
        if (updates.state.installing) {
            close.accepted = false
            updates.showUpdates()
        }
    }
    menuBar: Item {
        implicitHeight: Math.max(topMenu.implicitHeight, 32)
        Rectangle { anchors.fill: parent; color: applicationTheme.panel }
        RowLayout {
            anchors.fill: parent; spacing: 12
            WorkbenchMenuBar {
                id: topMenu
                Layout.fillWidth: true
                theme: applicationTheme
                fileActionsEnabled: window.selectedDirection === "downstream" && !!window.workbench
                    && !window.workbench.runState.busy && !window.workbench.loadingProject && !updates.state.installing
                onNewRequested: window.routeFile("new", false)
                onOpenRequested: window.routeFile("open", false)
                onSaveRequested: function(saveAs) { window.routeFile("save", saveAs) }
                onGlobalSettingsRequested: { if (!updates.state.installing) workbench.settingsPopup.openChecked() }
                onAboutRequested: workbench.aboutPopup.openChecked()
                onFaqRequested: workbench.openHelp("")
                onTerminologyRequested: function(target) { Qt.openUrlExternally(target) }
            }
            UpdateNotification {
                id: updates
                Layout.preferredWidth: 80; Layout.preferredHeight: 28
                Layout.rightMargin: 12
                theme: applicationTheme; backend: updateInfo
                canUpdate: !!window.workbench && !directionHost.presentation.applicationBusy
                    && !directionHost.upstreamState.busy && !directionHost.upstreamState.configured
                    && !window.workbench.loadingProject
                prepareUpdate: function() { return workbench.saveUpdateRecovery() }
                startupCheck: !window.presentationProbeRequested && !window.directionProbeRequested && !window.upstreamConfigurationProbeRequested && !window.upstreamExecutionProbeRequested && Qt.application.arguments.indexOf("--check-updates")<0 && Qt.application.arguments.indexOf("--smoke-test")<0
            }
        }
    }
    ColumnLayout {
        id: unifiedFrame
        anchors.fill: parent
        enabled: !updates.state.installing
        readonly property int shellEdgeMargin: 16
        readonly property int shellRowGap: 8
        spacing: shellRowGap
        PersistentDirectionTabs {
            id: directionTabs
            Layout.fillWidth: true
            Layout.fillHeight: true
            theme: applicationTheme
            presentationHost: directionHost.presentation
            downstreamComponent: Component {
                DownstreamWorkbench {
                    showHeader: false
                    showRunControls: false
                    pollInView: false
                    theme: applicationTheme
                    settingsBackend: ofdmSettings
                    profileBackend: profileInput
                    sourceBackend: sourceInput
                    transmitterBackend: transmitterAnalyzer
                    measurementBackend: measurementInputs
                    aboutBackend: aboutInfo
                    receiverBackend: receiverInstruments
                    runBackend: closedLoop
                    fileBackend: workbenchFile
                    onUpdatesRequested: { updates.showUpdates(); updates.refresh() }
                    onExternalLinkRequested: function(target) { Qt.openUrlExternally(target) }
                }
            }
            upstreamComponent: Component {
                UpstreamWorkbench {
                    theme: applicationTheme
                    host: directionHost
                    onDraftRequested: function(section) { directionHost.beginDraft(section) }
                    onDraftApplyRequested: function(section, token, values) { directionHost.applyDraft(section, token, values) }
                    onDraftCancelled: function(token) { directionHost.cancelDraft(token) }
                    onConfigurationLoadRequested: directionHost.loadConfiguration()
                }
            }
            onDirectionRequested: function(direction) { directionHost.selectDirection(direction) }
            onRunActionRequested: function(action, direction, identity) {
                directionHost.runAction(action, direction, identity)
            }
            onSaveRequested: function(direction, identity, saveAs) {
                if (directionHost.accepts(direction, identity)) window.routeFile("save", saveAs)
            }
            onOpenRequested: window.routeFile("open", false)
        }
        DirectionRunControls {
            id: sharedControls
            Layout.fillWidth: true
            Layout.leftMargin: unifiedFrame.shellEdgeMargin
            Layout.rightMargin: unifiedFrame.shellEdgeMargin
            Layout.bottomMargin: unifiedFrame.shellEdgeMargin
            theme: applicationTheme
            direction: window.selectedDirection
            identity: directionTabs.identityFor(direction)
            readonly property bool downstream: direction === "downstream"
            readonly property var currentState: downstream
                ? (window.workbench ? window.workbench.runState : ({})) : directionHost.upstreamState
            canConfigure: !!directionTabs.downstreamItem && !!directionTabs.upstreamItem
                && !currentState.busy && !directionHost.presentationRejected
                && (!downstream || !window.workbench.loadingProject)
            canStart: !!currentState.can_start && !directionHost.presentationRejected
                && (!downstream || (window.workbench && window.workbench.hasProject
                    && window.workbench.allBlocksComplete && !window.workbench.loadingProject))
            busy: !!currentState.busy
            paused: downstream ? !!currentState.paused : !!currentState.progress && currentState.progress.status === "Paused"
            status: downstream ? (window.workbench ? window.workbench.progressText : "")
                : (directionHost.presentation.display ? directionHost.presentation.display.upstreamStatus : "")
            elapsed: downstream && window.workbench ? window.workbench.elapsedText
                : currentState.execution_view ? currentState.execution_view.elapsed : ""
            diagnostic: downstream && window.workbench ? window.workbench.runState.diagnostic : null
            activityTooltip: downstream && window.workbench ? window.workbench.activityText
                : currentState.execution_view ? currentState.execution_view.activity : ""
            onHelpRequested: function(code, direction, identity) {
                if (direction === "downstream" && directionHost.accepts(direction, identity)) {
                    window.workbench.openHelp(code)
                }
            }
            onActionRequested: function(action, direction, identity) {
                directionTabs.requestRunAction(action, direction, identity)
            }
        }
    }
    Timer {
        interval: applicationTheme.sessionPollIntervalMs
        running: true
        repeat: true
        onTriggered: {
            if (window.workbench) {
                window.workbench.adoptTransmitterState(JSON.parse(transmitterAnalyzer.poll()))
                window.workbench.pollRun()
            }
            directionHost.poll()
            sharedControls.retryRejectedText()
        }
    }
}
