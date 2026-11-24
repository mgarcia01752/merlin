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
    required property var settingsBackend
    required property bool rxmer
    readonly property var metadata: presentation.value ? presentation.value.metadata : ({files: []})
    readonly property var options: presentation.value ? presentation.value.options : ({})
    property bool canRestoreWorkbench: false
    // Native staging can advance before optional UI adoption; cached options
    // cannot declare its old displayed catalog synchronized again.
    property bool draftSynchronized: true
    readonly property string errorMessage: presentation.value ? presentation.value.errorMessage : ""
    property var runOptions: null
    readonly property var selectionStatus: presentation.value ? presentation.value.selectionStatus : ({})
    readonly property var admission: presentation.value ? presentation.value.admission : ({})
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: ({metadata: {files: [], options: {mode: "single", interval_symbols: "1000", tolerance_seconds: 90000}},
            options: {mode: "single", interval_symbols: "1000", tolerance_seconds: 90000},
            selectionStatus: {}, admission: {}, errorMessage: "",modes:["Single Shot","Average · linear error power","Replay · hold","Replay · interpolate power"],fit:"Use First 0"})
    }
    Component.onCompleted: Qt.callLater(dialog.admitTextFields)
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    function admitTextFields() {
        return presentation.reserveTextFields([intervalField, symbolField, toleranceField])
    }
    signal measurementApplied()
    signal contextHelpRequested(string code)
    property alias designerPopup: designer
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1120, parent ? parent.width - 40 : 1120)
    height: Math.min(720, parent ? parent.height - 40 : 720)
    modal: true

    // Publish related metadata/options/details together while old coverage stays held.
    function adoptState(nextMetadata,nextOptions,nextError,resetSelection,freshCatalog = false) {
        if (!draftSynchronized && !freshCatalog) {return false}
        if (!presentation.textAdmitted) {
            return false
        }
        const nextSelection = rxmer ? JSON.parse(backend.selection_status(JSON.stringify(nextOptions),settingsBackend.current(),JSON.stringify(runOptions))) : selectionStatus
        const nextAdmission = JSON.parse(backend.describe(rxmer, JSON.stringify(nextOptions), settingsBackend.current()))
        if (!presentation.replace({metadata: nextMetadata, options: nextOptions,
                selectionStatus: nextSelection, admission: nextAdmission, errorMessage: nextError,
                modes:["Single Shot","Average · linear error power","Replay · hold","Replay · interpolate power"],fit:"Use First "+Math.min(nextMetadata.files.length,nextSelection.capacity || 0)})) {
            if (freshCatalog) {draftSynchronized=false}
            return false
        }
        if (freshCatalog) {draftSynchronized=true}
        if (resetSelection) {
            viewer.selectedIndex = -1
        }
        viewer.stopPreview()
        viewer.reset()
        return true
    }
    function setMetadata(document, resetOptions) {
        const nextMetadata = JSON.parse(document)
        let nextOptions = options
        if (resetOptions) {
            nextOptions = Object.assign({}, nextMetadata.options)
        } else if (rxmer && options.selected_captures) {
            const identities = nextMetadata.files.map(file => file.sha256)
            nextOptions = Object.assign({},options,{selected_captures:options.selected_captures.filter(identity => identities.indexOf(identity)>=0)})
        }
        const accepted=adoptState(nextMetadata,nextOptions,nextMetadata.error || "",true,true)
        if (!accepted) {draftSynchronized=false}
        return accepted
    }
    function refresh() {
        return adoptState(metadata, options, errorMessage, false)
    }
    function updateOption(key, value) {
        const next = Object.assign({}, options)
        next[key] = value
        return adoptState(metadata, next, "", false)
    }
    function chooseMode(mode) {
        const next = Object.assign({},options,{mode:mode})
        if (mode === "single" && metadata.files.length) {
            next.selected_captures = [metadata.files[Math.max(0,viewer.selectedIndex)].sha256]
        }
        return adoptState(metadata, next, errorMessage, false)
    }
    function toggleCapture(identity, checked) {
        if (options.mode === "single") {
            return updateOption("selected_captures",[identity])
        }
        const selected = options.selected_captures || metadata.files.map(file => file.sha256)
        const next = selected.filter(value => value !== identity)
        if (checked) {
            next.push(identity)
        }
        return updateOption("selected_captures",next)
    }
    function applyMeasurement() {
        if (!draftSynchronized || !presentation.textAdmitted || !presentation.value) {
            return
        }
        const result = backend.apply(rxmer, JSON.stringify(options), settingsBackend.current())
        presentation.replace(Object.assign({},presentation.value,{errorMessage: result}))
        if (!result.length) {
            measurementApplied()
            close()
        }
    }
    RxmerDesignerDialog {
        id: designer
        onContextHelpRequested:function(code){dialog.contextHelpRequested(code)}
        theme: dialog.theme; backend: dialog.backend; parentOptions: dialog.options
        stagedAdopter: function(metadata) { return dialog.setMetadata(JSON.stringify(metadata), true) }
        stagedAdoptionMessage: presentation.message
    }
    onClosed: { viewer.stopPreview();designer.close() }
    function restoreWorkbench() {
        if (!admitTextFields() || !setMetadata(backend.begin(rxmer), true)) {
            return false
        }
        canRestoreWorkbench = !rxmer || metadata.files.length > 0
        return true
    }
    onOpened: restoreWorkbench()
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            WorkbenchActionButton { theme: dialog.theme; objectName: "measurementOpen"; text: dialog.rxmer ? "Open file(s)…" : "Open file…"; onClicked: dialog.setMetadata(backend.choose_files(dialog.rxmer)) }
            WorkbenchActionButton { theme: dialog.theme; objectName: "measurementRestoreWorkbench"; text: "Restore Workbench"; enabled: dialog.canRestoreWorkbench; onClicked: dialog.restoreWorkbench()
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered || visualFocus
                ToolTip.text: "Discard pending edits and restore this workbench’s last applied captures and options. Channel response may be flat / unity."
                Accessible.description: ToolTip.text }
            WorkbenchActionButton { theme: dialog.theme; objectName: "measurementClear"; text: dialog.rxmer ? "Clear" : "Flat / unity"; onClicked: dialog.setMetadata(backend.edit(dialog.rxmer, -1, 0)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "Remove"; enabled: dialog.draftSynchronized && !presentation.rejected && viewer.selectedIndex >= 0; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, 0)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "↑"; compact: true; Accessible.name: "Move capture earlier"; enabled: dialog.draftSynchronized && !presentation.rejected && viewer.selectedIndex > 0; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, -1)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "↓"; compact: true; Accessible.name: "Move capture later"; enabled: dialog.draftSynchronized && !presentation.rejected && viewer.selectedIndex >= 0 && viewer.selectedIndex < dialog.metadata.files.length - 1; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, 1)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "Sort by capture time"; enabled: !presentation.rejected; onClicked: dialog.setMetadata(backend.edit(true, 0, 2)) }
            Item { Layout.fillWidth: true }
        }
        RowLayout {
            visible: dialog.rxmer
            Label { text: "Mode"; color: theme.text }
            ComboBox {
                objectName: "rxmerMode"; Layout.preferredWidth: 245
                model: presentation.value ? presentation.value.modes : []
                currentIndex: ["single", "average", "hold", "interpolate"].indexOf(dialog.options.mode)
                onActivated: function(index) { dialog.chooseMode(["single", "average", "hold", "interpolate"][index]) }
            }
            Label { text: "OFDM symbols between captures"; color: theme.text }
            // Whole-input unsigned decimal text, 1–20 digits; no signs, units or separators.
            TextField { id: intervalField; objectName: "rxmerInterval"; text: presentation.textAdmitted && presentation.value ? dialog.options.interval_symbols : ""; Layout.preferredWidth: 115; enabled: presentation.textAdmitted && !!presentation.value && (dialog.options.mode === "hold" || dialog.options.mode === "interpolate"); validator: RegularExpressionValidator { regularExpression: /[0-9]{1,20}/ }
                onEditingFinished: dialog.updateOption("interval_symbols", text) }
            Label {
                id: previewSymbolLabel
                text: "Preview symbol"
                color: theme.text
                readonly property string helpText: "OFDM symbol index for the effective RxMER/playback preview (0 = first).\nPreview only; simulation settings and capture selection stay unchanged."
                HoverHandler { id: previewSymbolHover }
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: previewSymbolHover.hovered
                ToolTip.text: helpText
            }
            // Whole-input unsigned symbol offset, 1–20 digits; Rust checks u64 overflow.
            TextField { id: symbolField; objectName: "rxmerPreviewSymbol"; text: presentation.textAdmitted ? "0" : ""; enabled: presentation.textAdmitted; Layout.preferredWidth: 95
                hoverEnabled: true
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered || (activeFocus && (focusReason === Qt.TabFocusReason || focusReason === Qt.BacktabFocusReason || focusReason === Qt.ShortcutFocusReason))
                ToolTip.text: previewSymbolLabel.helpText
                Accessible.description: previewSymbolLabel.helpText
                validator: RegularExpressionValidator { regularExpression: /[0-9]{1,20}/ }
                onEditingFinished: { viewer.symbol = text; viewer.reset() } }
            Item { Layout.fillWidth: true }
        }
        RowLayout {
            Label { text: "Capture association tolerance (seconds)"; color: theme.muted; font.pixelSize: 12 }
            TextField { id: toleranceField; objectName: "measurementTolerance"; text: presentation.textAdmitted && presentation.value ? String(dialog.options.tolerance_seconds) : ""; enabled: presentation.textAdmitted && !!presentation.value; Layout.preferredWidth: 120; validator: IntValidator { bottom: 0; top: 2147483647 }
                onEditingFinished: dialog.updateOption("tolerance_seconds", Number(text)) }
        }
        RowLayout {
            visible:dialog.rxmer
            WorkbenchActionButton { theme: dialog.theme; objectName: "rxmerDesignerOpen"; visible: dialog.rxmer; text: "RxMER Designer"; enabled: dialog.draftSynchronized && !presentation.rejected && !designer.working; onClicked: designer.openChecked() }
            WorkbenchActionButton { theme:dialog.theme;objectName:"selectAllCaptures";text:"Select All";enabled:dialog.options.mode !== "single";onClicked:dialog.updateOption("selected_captures",null) }
            WorkbenchActionButton { theme:dialog.theme;objectName:"clearCaptureSelection";text:"Clear Selection";onClicked:dialog.updateOption("selected_captures",[]) }
            WorkbenchActionButton { theme:dialog.theme;objectName:"fitCaptureSelection";text:presentation.value ? presentation.value.fit : "";enabled:dialog.selectionStatus.capacity > 0 && dialog.metadata.files.length > 0;onClicked:dialog.updateOption("selected_captures",dialog.metadata.files.slice(0,dialog.selectionStatus.capacity).map(file => file.sha256)) }
        }
        Label { visible:dialog.rxmer;text:dialog.selectionStatus.error || dialog.selectionStatus.summary || "";color:dialog.selectionStatus.error || dialog.selectionStatus.fits === false ? theme.statusAttention : theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:12 }
        MeasurementViewer {
            id: viewer; objectName: "sharedMeasurementViewer"
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: dialog.theme; backend: dialog.backend; settingsBackend: dialog.settingsBackend
            onSelectionToggled:function(identity,checked) {dialog.toggleCapture(identity,checked)}
            contextAvailable:dialog.draftSynchronized
            previewEnabled:dialog.visible && dialog.draftSynchronized
            selectionCount:dialog.selectionStatus.selected || 0
            rxmer: dialog.rxmer; options: dialog.options; files: dialog.metadata.files || []
        }
        Label { text: presentation.rejected ? presentation.message : dialog.errorMessage || dialog.admission.error || dialog.admission.summary || ""; color: presentation.rejected || dialog.errorMessage || dialog.admission.error ? theme.statusError : theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyEnabled: dialog.draftSynchronized && !!presentation.value && presentation.textAdmitted && !presentation.rejected && (!dialog.rxmer || dialog.selectionStatus.fits !== false)
        applyObjectName: "measurementApply"; onApplied: dialog.applyMeasurement()
        onCancelled: dialog.reject()
    }
}
