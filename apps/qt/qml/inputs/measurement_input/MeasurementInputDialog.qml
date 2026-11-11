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
    property string errorMessage: ""
    property var admission: ({})
    signal measurementApplied()
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1120, parent ? parent.width - 40 : 1120)
    height: Math.min(720, parent ? parent.height - 40 : 720)
    modal: true
    function setMetadata(document, resetOptions) {
        metadata = JSON.parse(document)
        if (resetOptions) { options = Object.assign({}, metadata.options) }
        errorMessage = metadata.error || ""
        viewer.selectedIndex = -1
        refresh()
    }
    function refresh() {
        admission = JSON.parse(backend.describe(rxmer, JSON.stringify(options), settingsBackend.current()))
        viewer.reset()
    }
    function updateOption(key, value) { errorMessage = ""; const next = Object.assign({}, options); next[key] = value; options = next; refresh() }
    onOpened: setMetadata(backend.begin(rxmer), true)
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            Button { objectName: "measurementOpen"; text: dialog.rxmer ? "Open file(s)…" : "Open file…"; onClicked: dialog.setMetadata(backend.choose_files(dialog.rxmer)) }
            Button { objectName: "measurementDemo"; visible: !dialog.rxmer; text: "Load demo"; onClicked: dialog.setMetadata(backend.demo(false, 0)) }
            Button { text: dialog.rxmer ? "Clear" : "Flat / unity"; onClicked: dialog.setMetadata(backend.edit(dialog.rxmer, -1, 0)) }
            Button { visible: dialog.rxmer; text: "Remove"; enabled: viewer.selectedIndex >= 0; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, 0)) }
            Button { visible: dialog.rxmer; text: "↑"; Accessible.name: "Move capture earlier"; enabled: viewer.selectedIndex > 0; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, -1)) }
            Button { visible: dialog.rxmer; text: "↓"; Accessible.name: "Move capture later"; enabled: viewer.selectedIndex >= 0 && viewer.selectedIndex < dialog.metadata.files.length - 1; onClicked: dialog.setMetadata(backend.edit(true, viewer.selectedIndex, 1)) }
            Button { visible: dialog.rxmer; text: "Sort by capture time"; onClicked: dialog.setMetadata(backend.edit(true, 0, 2)) }
            Item { Layout.fillWidth: true }
        }
        RowLayout {
            visible: dialog.rxmer
            Label { text: "Mode"; color: theme.text }
            ComboBox {
                objectName: "rxmerMode"; Layout.preferredWidth: 245
                model: ["Single capture", "Average · linear error power", "Sequence · hold", "Sequence · interpolate power"]
                currentIndex: ["single", "average", "hold", "interpolate"].indexOf(dialog.options.mode)
                onActivated: function(index) { dialog.updateOption("mode", ["single", "average", "hold", "interpolate"][index]) }
            }
            Label { text: "OFDM symbols between captures"; color: theme.text }
            // Whole-input unsigned decimal text, 1–20 digits; no signs, units or separators.
            TextField { objectName: "rxmerInterval"; text: dialog.options.interval_symbols; Layout.preferredWidth: 115; enabled: dialog.options.mode === "hold" || dialog.options.mode === "interpolate"; validator: RegularExpressionValidator { regularExpression: /[0-9]{1,20}/ }
                onEditingFinished: dialog.updateOption("interval_symbols", text) }
            Label { text: "Preview symbol"; color: theme.text }
            // Whole-input unsigned symbol offset, 1–20 digits; Rust checks u64 overflow.
            TextField { objectName: "rxmerPreviewSymbol"; text: "0"; Layout.preferredWidth: 95; validator: RegularExpressionValidator { regularExpression: /[0-9]{1,20}/ }
                onEditingFinished: { viewer.symbol = text; viewer.reset() } }
            Item { Layout.fillWidth: true }
        }
        RowLayout {
            Label { text: "Capture association tolerance (seconds)"; color: theme.muted; font.pixelSize: 12 }
            TextField { objectName: "measurementTolerance"; text: String(dialog.options.tolerance_seconds); Layout.preferredWidth: 120; validator: IntValidator { bottom: 0; top: 2147483647 }
                onEditingFinished: dialog.updateOption("tolerance_seconds", Number(text)) }
            Label { text: "Explicit application policy; original timestamps stay unchanged."; color: theme.muted; font.pixelSize: 11; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        }
        MeasurementViewer {
            id: viewer; objectName: "sharedMeasurementViewer"
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: dialog.theme; backend: dialog.backend; settingsBackend: dialog.settingsBackend
            rxmer: dialog.rxmer; options: dialog.options; files: dialog.metadata.files || []
        }
        Label { text: dialog.errorMessage || dialog.admission.error || dialog.admission.summary || ""; color: dialog.errorMessage || dialog.admission.error ? theme.statusError : theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
    }
    footer: ApplyCancelFooter {
        applyObjectName: "measurementApply";   onApplied: { dialog.errorMessage = backend.apply(dialog.rxmer, JSON.stringify(dialog.options), dialog.settingsBackend.current()); if (!dialog.errorMessage.length) { dialog.measurementApplied(); dialog.close() } }
        onCancelled: dialog.reject()
    }
}
