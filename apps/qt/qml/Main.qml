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
    property alias projectDirtyDialog: projectDirty
    readonly property string selectedDirection: directionHost.presentation.selectedDirection || ""
    // Compiled UI cursors only; native adapters retain all paths and revision guards.
    readonly property int noFileDirection: -1
    readonly property int downstreamFileDirection: 0
    readonly property int upstreamFileDirection: 1
    readonly property int newFileAction: 0
    readonly property int openFileAction: 1
    readonly property int closeFileAction: 2
    property bool finalCloseAccepted: false
    property int pendingFileDirection: noFileDirection
    property int pendingFileAction: newFileAction
    property bool saveBeforeFileDecision: false

    /** Route Open by native content detection and Save/New by their captured origin. */
    function routeFile(action, saveAs) {
        const identity = directionTabs.identityFor(selectedDirection);
        if (!workbench || pendingFileDirection !== noFileDirection || !directionHost.accepts(selectedDirection, identity)) {
            return false;
        }
        if (action === "open") {
            workbench.openProject();
        } else if (action === "new") {
            if (selectedDirection === "upstream") {
                const error = upstreamSession.request_new_project() || "";
                directionHost.poll();
                if (error) {
                    directionHost.feedback(error);
                    return false;
                }
                showFileDecision(true);
            } else {
                const result = JSON.parse(workbenchFile.request_new_project());
                if (result.error) {
                    workbench.showFileMessage(result.error);
                    return false;
                }
                showFileDecision(false);
            }
        } else if (action === "save") {
            if (selectedDirection === "upstream") {
                const error = upstreamSession.save_project(saveAs) || "";
                directionHost.poll();
                directionHost.feedback(error);
            } else {
                workbench.saveProject(saveAs);
            }
        } else {
            return false;
        }
        return true;
    }
    /** Open one compiled prompt for the native target, independent of selected tab. */
    function showFileDecision(upstream) {
        const value = upstream ? JSON.parse(upstreamSession.state()).project : JSON.parse(workbenchFile.file_decision());
        if (!value || !value.decision) {
            return;
        }
        pendingFileDirection = upstream ? upstreamFileDirection : downstreamFileDirection;
        pendingFileAction = value.decision === "open" ? openFileAction : newFileAction;
        if (!value.dirty) {
            resolveFileDecision(false, false);
            return;
        }
        if (!projectDirty.openChecked()) {
            resolveFileDecision(false, true);
            directionHost.feedback("The project decision could not be displayed. Retry the file action.");
        }
    }
    /** Transfer actual native ownership; neither paths nor documents pass through QML. */
    function receiveUpstreamProject() {
        const error = workbenchFile.handoff_upstream(upstreamSession) || "";
        directionHost.poll();
        if (error) {
            directionHost.feedback(error);
            return;
        }
        showFileDecision(true);
    }
    /** Resolve only the original native intent; failed/stale actions preserve both tabs. */
    function resolveFileDecision(discard, cancel) {
        if (pendingFileAction === closeFileAction) {
            pendingFileDirection = noFileDirection;
            saveBeforeFileDecision = false;
            continueClose(JSON.parse(directionBackend.close_decision(discard, cancel)));
            return;
        }
        const upstream = pendingFileDirection === upstreamFileDirection;
        const opening = pendingFileAction === openFileAction;
        pendingFileDirection = noFileDirection;
        saveBeforeFileDecision = false;
        if (upstream) {
            const error = upstreamSession.resolve_project(discard, cancel) || "";
            directionHost.poll();
            directionHost.feedback(error);
            if (!error && !cancel && opening) {
                directionHost.selectDirection("upstream");
            }
        } else {
            const result = JSON.parse(workbenchFile.resolve_project(discard, cancel));
            if (result.error) {
                workbench.showFileMessage(result.error);
            }
            if (result.created || result.opened) {
                workbench.adoptProject(result);
                if (opening) {
                    directionHost.selectDirection("downstream");
                }
            }
            directionHost.poll();
        }
    }
    /** Continue only after the exact captured Save succeeds; chooser Cancel aborts. */
    function saveForFileDecision() {
        saveBeforeFileDecision = true;
        if (pendingFileDirection === upstreamFileDirection) {
            const error = upstreamSession.save_project(false) || "";
            directionHost.poll();
            const project = JSON.parse(upstreamSession.state()).project;
            if (!error && project.status === "saved" && !project.dirty) {
                resolveFileDecision(false, false);
            } else if (error || project.status !== "saving") {
                resolveFileDecision(false, true);
                directionHost.feedback(error);
            }
        } else if (!workbench.saveProject(false, true)) {
            resolveFileDecision(false, true);
        }
    }
    /** A failed write never authorizes a pending replacement. */
    function downstreamWriteSettled(saved) {
        if (saveBeforeFileDecision && pendingFileDirection === downstreamFileDirection) {
            resolveFileDecision(false, !saved);
        }
    }
    /** Present the next native dirty owner; admission remains live until both agree. */
    function continueClose(result) {
        if (result.error) {
            directionHost.feedback(result.error);
            return;
        }
        if (result.cancelled) {
            return;
        }
        if (result.ready) {
            const finished = JSON.parse(directionBackend.finish_close());
            if (finished.error) {
                directionHost.feedback(finished.error);
                return;
            }
            finalCloseAccepted = true;
            Qt.callLater(window.close);
            return;
        }
        pendingFileAction = closeFileAction;
        pendingFileDirection = result.direction === "upstream" ? upstreamFileDirection : downstreamFileDirection;
        if (!projectDirty.openChecked()) {
            resolveFileDecision(false, true);
            directionHost.feedback("The close decision could not be displayed. Retry closing.");
        }
    }
    WorkbenchFile {
        id: workbenchFile
    }
    ProjectDirtyDialog {
        id: projectDirty
        objectName: "projectDirtyDialog"
        theme: applicationTheme
        direction: window.pendingFileDirection === window.upstreamFileDirection ? "upstream" : "downstream"
        action: window.pendingFileAction === window.closeFileAction ? "close" : window.pendingFileAction === window.openFileAction ? "open" : "new"
        saveAllowed: window.pendingFileDirection === window.upstreamFileDirection ? !!directionHost.upstreamState.project && !!directionHost.upstreamState.project.can_save : !!window.workbench && !window.workbench.runState.busy && !window.workbench.loadingProject
        onSaveRequested: window.saveForFileDecision()
        onDiscardRequested: window.resolveFileDecision(true, false)
        onCancelled: window.resolveFileDecision(false, true)
    }
    TransmitterAnalyzer { id: transmitterAnalyzer }
    property Component presentationBackendComponent: Component { PresentationRetention {} }
    function createPresentationBackend(parent) {
        return presentationBackendComponent.createObject(parent)
    }
    Theme { id: applicationTheme; presentationBackendFactory: window.createPresentationBackend; onDarkChanged: branding.update_icon(dark) }
    readonly property bool projectFileProbeRequested: Qt.application.arguments.indexOf("--project-file-probe") >= 0
    property QtObject projectFileProbe: null
    property Component projectFileProbeComponent: Component {
        ProjectFileProbe {
            onFinished: function(success, message) { Qt.exit(success ? 0 : 1) }
        }
    }
    /** Exercise actual native file menus, dirty decisions and both project owners. */
    function startProjectFileProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--project-file-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Project file probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        projectFileProbe = projectFileProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, dirtyDialog: projectDirty,
            captureItem: unifiedFrame, screenshotPath: args[imageArgument + 1]
        })
        if (!projectFileProbe) {
            Qt.exit(1)
        }
    }
    readonly property bool presentationProbeRequested: Qt.application.arguments.indexOf("--session-presentation-probe") >= 0
    readonly property bool directionProbeRequested: Qt.application.arguments.indexOf("--direction-shell-probe") >= 0
    readonly property bool upstreamConfigurationProbeRequested: Qt.application.arguments.indexOf("--upstream-configuration-probe") >= 0
    readonly property bool upstreamExecutionProbeRequested: Qt.application.arguments.indexOf("--upstream-execution-probe") >= 0
    readonly property bool upstreamInputsProbeRequested: Qt.application.arguments.indexOf("--upstream-inputs-probe") >= 0
    readonly property bool upstreamGridProbeRequested: Qt.application.arguments.indexOf("--upstream-grid-probe") >= 0
    property QtObject presentationProbe: null
    property QtObject directionProbe: null
    property QtObject upstreamConfigurationProbe: null
    property QtObject upstreamExecutionProbe: null
    property QtObject upstreamInputsProbe: null
    property QtObject upstreamGridProbe: null
    readonly property bool upstreamInstrumentsProbeRequested: Qt.application.arguments.indexOf("--upstream-instruments-probe") >= 0
    property QtObject upstreamInstrumentsProbe: null
    readonly property bool upstreamFecProbeRequested: Qt.application.arguments.indexOf("--upstream-fec-probe") >= 0
    property QtObject upstreamFecProbe: null
    property Component upstreamFecProbeComponent: Component {
        UpstreamFecProbe {
            onFinished: function(success, message) { Qt.exit(success ? 0 : 1) }
        }
    }
    /** Exercise actual native decoder/soft observations under fixed finite scenarios. */
    function startUpstreamFecProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--upstream-fec-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Upstream FEC probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        upstreamFecProbe = upstreamFecProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            screenshotPath: args[imageArgument + 1]
        })
        if (!upstreamFecProbe) { Qt.exit(1) }
    }
    readonly property bool downstreamSharedProbeRequested: Qt.application.arguments.indexOf("--downstream-shared-probe") >= 0
    property QtObject downstreamSharedProbe: null
    property Component downstreamSharedProbeComponent: Component {
        DownstreamSharedInstrumentsProbe {
            onFinished: function(success, message) { Qt.exit(success ? 0 : 1) }
        }
    }
    /** Exercise preserved downstream consumers with actual original/native data. */
    function startDownstreamSharedProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--downstream-shared-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Downstream shared probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        downstreamSharedProbe = downstreamSharedProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            screenshotPath: args[imageArgument + 1]
        })
        if (!downstreamSharedProbe) { Qt.exit(1) }
    }
    property Component upstreamInstrumentsProbeComponent: Component {
        UpstreamInstrumentsProbe {
            onFinished: function(success, message) { Qt.exit(success ? 0 : 1) }
        }
    }
    /** Run actual public-data consumers with an explicit fresh evidence destination. */
    function startUpstreamInstrumentsProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--upstream-instruments-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Upstream instruments probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        upstreamInstrumentsProbe = upstreamInstrumentsProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            screenshotPath: args[imageArgument + 1]
        })
        if (!upstreamInstrumentsProbe) { Qt.exit(1) }
    }
    property Component upstreamGridProbeComponent: Component {
        UpstreamResourceGridProbe {
            onFinished: function(success, message) {
                console.log("MERLIN_UPSTREAM_GRID_PROBE " + message)
                Qt.exit(success ? 0 : 1)
            }
        }
    }
    /** Run the fixed native grid consumer with a fresh evidence destination. */
    function startUpstreamGridProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--upstream-grid-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Upstream grid probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        upstreamGridProbe = upstreamGridProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            screenshotPath: args[imageArgument + 1]
        })
        if (!upstreamGridProbe) {
            Qt.exit(1)
        }
    }
    property Component upstreamInputsProbeComponent: Component {
        UpstreamInputsProbe {
            onFinished: function(success, message) {
                console.log("MERLIN_UPSTREAM_INPUTS_PROBE " + message)
                Qt.exit(success ? 0 : 1)
            }
        }
    }
    /** Run fixed native source/measured cases with a fresh evidence destination. */
    function startUpstreamInputsProbe() {
        const args = Qt.application.arguments
        const imageArgument = args.indexOf("--upstream-inputs-image")
        if (imageArgument < 0 || imageArgument + 1 >= args.length) {
            console.error("Upstream inputs probe requires a fresh screenshot path.")
            Qt.exit(1)
            return
        }
        upstreamInputsProbe = upstreamInputsProbeComponent.createObject(window, {
            rootWindow: window, workbench: workbench, tabs: directionTabs,
            host: directionHost, controls: sharedControls, captureItem: unifiedFrame,
            screenshotPath: args[imageArgument + 1]
        })
        if (!upstreamInputsProbe) {
            Qt.exit(1)
        }
    }
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
        if (projectFileProbeRequested) {
            Qt.callLater(startProjectFileProbe)
        } else if (presentationProbeRequested) {
            Qt.callLater(startPresentationProbe)
        } else if (directionProbeRequested) {
            Qt.callLater(startDirectionProbe)
        } else if (upstreamConfigurationProbeRequested) {
            Qt.callLater(startUpstreamConfigurationProbe)
        } else if (upstreamExecutionProbeRequested) {
            Qt.callLater(startUpstreamExecutionProbe)
        } else if (upstreamInputsProbeRequested) {
            Qt.callLater(startUpstreamInputsProbe)
        } else if (downstreamSharedProbeRequested) {
            Qt.callLater(startDownstreamSharedProbe)
        } else if (upstreamInstrumentsProbeRequested) {
            Qt.callLater(startUpstreamInstrumentsProbe)
        } else if (upstreamFecProbeRequested) {
            Qt.callLater(startUpstreamFecProbe)
        } else if (upstreamGridProbeRequested) {
            Qt.callLater(startUpstreamGridProbe)
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
            return
        }
        if (finalCloseAccepted) {
            return
        }
        close.accepted = false
        if (pendingFileDirection !== noFileDirection) {
            return
        }
        if (!workbench || workbench.loadingProject || directionHost.upstreamState.preparing) {
            directionHost.feedback("Wait for the project file operation to finish before closing.")
            return
        }
        continueClose(JSON.parse(directionBackend.begin_close()))
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
                fileActionsEnabled: !!window.workbench && window.pendingFileDirection === window.noFileDirection
                    && !window.workbench.runState.busy && !window.workbench.loadingProject
                    && !directionHost.upstreamState.busy && !updates.state.installing
                saveActionsEnabled: fileActionsEnabled && (window.selectedDirection === "downstream"
                    || (!!directionHost.upstreamState.project && !!directionHost.upstreamState.project.can_save))
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
                startupCheck: !window.projectFileProbeRequested && !window.presentationProbeRequested && !window.directionProbeRequested && !window.upstreamConfigurationProbeRequested && !window.upstreamExecutionProbeRequested && !window.upstreamInputsProbeRequested && !window.upstreamGridProbeRequested && !window.upstreamInstrumentsProbeRequested && !window.downstreamSharedProbeRequested && !window.upstreamFecProbeRequested && Qt.application.arguments.indexOf("--check-updates")<0 && Qt.application.arguments.indexOf("--smoke-test")<0
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
                    onProjectDecisionRequested: window.showFileDecision(false)
                    onUpstreamProjectDetected: window.receiveUpstreamProject()
                    onProjectWriteSettled: function(saved) { window.downstreamWriteSettled(saved) }
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
                    onCaptureLoadRequested: function(family) { directionHost.loadCapture(family) }
                    onCapturePageRequested: function(family, first) { directionHost.capturePage(family, first) }
                    onCapturePlotRequested: function(family, first, count) { directionHost.capturePlot(family, first, count) }
                    onResourceGridRequested: directionHost.beginResourceGrid()
                    onInstrumentOpenRequested: function(kind) { directionHost.beginInstrument(kind) }
                    onInstrumentRequested: function(kind, selection, context) { directionHost.requestInstrument(kind, selection, context) }
                    onResourceGridViewRequested: function(request, context) { directionHost.requestResourceGrid(request, context) }
                    onResourceGrantRequested: function(grantId, context) { directionHost.selectResourceGrant(grantId, context) }
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
            if (window.saveBeforeFileDecision && window.pendingFileDirection === window.upstreamFileDirection) {
                const actual = JSON.parse(upstreamSession.state())
                if (!actual.preparing) {
                    window.resolveFileDecision(false, actual.project.status !== "saved" || !!actual.project.dirty)
                }
            }
            sharedControls.retryRejectedText()
        }
    }
}
