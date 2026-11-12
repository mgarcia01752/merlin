/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id: dialog
    required property var backend
    required property var settingsBackend
    required property bool rxmer
    property var metadata: ({files: [], options: {mode: "single", interval_symbols: "1000", tolerance_seconds: 90000}})
    property var options: ({mode: "single", interval_symbols: "1000", tolerance_seconds: 90000})
    property bool canRestoreWorkbench: false
    property string errorMessage: ""
    property var runOptions: null
    property var selectionStatus: ({})
    property var admission: ({})
    signal measurementApplied()
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1120, parent ? parent.width - 40 : 1120)
    height: Math.min(720, parent ? parent.height - 40 : 720)
    modal: true
    function setMetadata(document, resetOptions) {
        metadata = JSON.parse(document)
        if (resetOptions) { options = Object.assign({}, metadata.options) }
        else if (rxmer && options.selected_captures) {
            const identities=metadata.files.map(file => file.sha256)
            options=Object.assign({},options,{selected_captures:options.selected_captures.filter(identity => identities.indexOf(identity)>=0)})
        }
        errorMessage = metadata.error || ""
        viewer.selectedIndex = -1
        refresh()
    }
    function refresh() {
        if (rxmer) { selectionStatus=JSON.parse(backend.selection_status(JSON.stringify(options),settingsBackend.current(),JSON.stringify(runOptions))) }
        admission = JSON.parse(backend.describe(rxmer, JSON.stringify(options), settingsBackend.current()))
        viewer.stopPreview();viewer.reset()
    }
    function updateOption(key, value) { errorMessage = ""; const next = Object.assign({}, options); next[key] = value; options = next; refresh() }
    function chooseMode(mode) {
        const next=Object.assign({},options,{mode:mode})
        if (mode === "single" && metadata.files.length) {
            next.selected_captures=[metadata.files[Math.max(0,viewer.selectedIndex)].sha256]
        }
        options=next;refresh()
    }
    function toggleCapture(identity, checked) {
        if (options.mode === "single") { updateOption("selected_captures",[identity]);return }
        const selected=options.selected_captures || metadata.files.map(file => file.sha256)
        const next=selected.filter(value => value !== identity)
        if (checked) { next.push(identity) }
        updateOption("selected_captures",next)
    }
    onClosed: viewer.stopPreview()
    function restoreWorkbench() {
        setMetadata(backend.begin(rxmer), true)
        canRestoreWorkbench = !rxmer || metadata.files.length > 0
    }
    onOpened: restoreWorkbench()
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            WorkbenchActionButton { theme: dialog.theme; objectName: "measurementOpen"; text: dialog.rxmer ? "Open file(s)…" : "Open file…"; onClicked: dialog.setMetadata(backend.choose_files(dialog.rxmer)) }
            WorkbenchActionButton { theme: dialog.theme; objectName: "measurementRestoreWorkbench"; text: "Restore Workbench"; enabled: dialog.canRestoreWorkbench; onClicked: dialog.restoreWorkbench()
                ToolTip.visible: hovered || activeFocus
                ToolTip.text: "Discard pending edits and restore this workbench’s last applied captures and options. Channel response may be flat / unity."
                Accessible.description: ToolTip.text }
            WorkbenchActionButton { theme: dialog.theme; objectName: "measurementClear"; text: dialog.rxmer ? "Clear" : "Flat / unity"; onClicked: dialog.setMetadata(backend.edit(dialog.rxmer, -1, 0)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "Remove"; enabled: viewer.selectedIndex >= 0; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, 0)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "↑"; compact: true; Accessible.name: "Move capture earlier"; enabled: viewer.selectedIndex > 0; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, -1)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "↓"; compact: true; Accessible.name: "Move capture later"; enabled: viewer.selectedIndex >= 0 && viewer.selectedIndex < dialog.metadata.files.length - 1; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, 1)) }
            WorkbenchActionButton { theme: dialog.theme; visible: dialog.rxmer; text: "Sort by capture time"; onClicked: dialog.setMetadata(backend.edit(true, 0, 2)) }
            Item { Layout.fillWidth: true }
        }
        RowLayout {
            visible: dialog.rxmer
            Label { text: "Mode"; color: theme.text }
            ComboBox {
                objectName: "rxmerMode"; Layout.preferredWidth: 245
                model: ["Single Shot", "Average · linear error power", "Replay · hold", "Replay · interpolate power"]
                currentIndex: ["single", "average", "hold", "interpolate"].indexOf(dialog.options.mode)
                onActivated: function(index) { dialog.chooseMode(["single", "average", "hold", "interpolate"][index]) }
            }
            Label { text: "OFDM symbols between captures"; color: theme.text }
            // Whole-input unsigned decimal text, 1–20 digits; no signs, units or separators.
            TextField { objectName: "rxmerInterval"; text: dialog.options.interval_symbols; Layout.preferredWidth: 115; enabled: dialog.options.mode === "hold" || dialog.options.mode === "interpolate"; validator: RegularExpressionValidator { regularExpression: /[0-9]{1,20}/ }
                onEditingFinished: dialog.updateOption("interval_symbols", text) }
            Label {
                id: previewSymbolLabel
                text: "Preview symbol"
                color: theme.text
                readonly property string helpText: "OFDM symbol index for the effective RxMER/playback preview (0 = first).\nPreview only; simulation settings and capture selection stay unchanged."
                HoverHandler { id: previewSymbolHover }
                ToolTip.visible: previewSymbolHover.hovered
                ToolTip.delay: 500
                ToolTip.text: helpText
            }
            // Whole-input unsigned symbol offset, 1–20 digits; Rust checks u64 overflow.
            TextField { objectName: "rxmerPreviewSymbol"; text: "0"; Layout.preferredWidth: 95
                hoverEnabled: true
                ToolTip.visible: hovered || activeFocus
                ToolTip.delay: 500
                ToolTip.text: previewSymbolLabel.helpText
                Accessible.description: previewSymbolLabel.helpText
                validator: RegularExpressionValidator { regularExpression: /[0-9]{1,20}/ }
                onEditingFinished: { viewer.symbol = text; viewer.reset() } }
            Item { Layout.fillWidth: true }
        }
        RowLayout {
            Label { text: "Capture association tolerance (seconds)"; color: theme.muted; font.pixelSize: 12 }
            TextField { objectName: "measurementTolerance"; text: String(dialog.options.tolerance_seconds); Layout.preferredWidth: 120; validator: IntValidator { bottom: 0; top: 2147483647 }
                onEditingFinished: dialog.updateOption("tolerance_seconds", Number(text)) }
        }
        RowLayout {
            visible:dialog.rxmer
            WorkbenchActionButton { theme:dialog.theme;objectName:"selectAllCaptures";text:"Select All";enabled:dialog.options.mode !== "single";onClicked:dialog.updateOption("selected_captures",null) }
            WorkbenchActionButton { theme:dialog.theme;objectName:"clearCaptureSelection";text:"Clear Selection";onClicked:dialog.updateOption("selected_captures",[]) }
            WorkbenchActionButton { theme:dialog.theme;objectName:"fitCaptureSelection";text:"Use First "+Math.min(dialog.metadata.files.length,dialog.selectionStatus.capacity || 0);enabled:dialog.selectionStatus.capacity > 0 && dialog.metadata.files.length > 0;onClicked:dialog.updateOption("selected_captures",dialog.metadata.files.slice(0,dialog.selectionStatus.capacity).map(file => file.sha256)) }
        }
        Label { visible:dialog.rxmer;text:dialog.selectionStatus.error || dialog.selectionStatus.summary || "";color:dialog.selectionStatus.error || dialog.selectionStatus.fits === false ? theme.statusAttention : theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:12 }
        MeasurementViewer {
            id: viewer; objectName: "sharedMeasurementViewer"
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: dialog.theme; backend: dialog.backend; settingsBackend: dialog.settingsBackend
            onSelectionToggled:function(identity,checked) {dialog.toggleCapture(identity,checked)}
            previewEnabled:dialog.visible
            selectionCount:dialog.selectionStatus.selected || 0
            rxmer: dialog.rxmer; options: dialog.options; files: dialog.metadata.files || []
        }
        Label { text: dialog.errorMessage || dialog.admission.error || dialog.admission.summary || ""; color: dialog.errorMessage || dialog.admission.error ? theme.statusError : theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyEnabled: !dialog.rxmer || dialog.selectionStatus.fits !== false
        applyObjectName: "measurementApply";   onApplied: { dialog.errorMessage = backend.apply(dialog.rxmer, JSON.stringify(dialog.options), dialog.settingsBackend.current()); if (!dialog.errorMessage.length) { dialog.measurementApplied(); dialog.close() } }
        onCancelled: dialog.reject()
    }
}
