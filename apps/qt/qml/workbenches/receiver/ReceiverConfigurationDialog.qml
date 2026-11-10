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
    required property var runState
    property var draft: ({})
    property string errorMessage: ""
    signal configurationApplied()
    title: "Receiver Configuration"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650,parent ? parent.width - 40 : 650)
    modal: true; standardButtons: Dialog.Cancel
    onOpened: { draft=Object.assign({},runState.options); errorMessage="" }
    contentItem: ColumnLayout {
        spacing: 14
        Label { text: "Configure the next closed-loop run"; color: theme.accent; font.bold: true }
        GridLayout {
            columns: 2; columnSpacing: 18; rowSpacing: 10
            Label { text: "Receiver Correction"; color: theme.text }
            ComboBox { objectName: "receiverCorrection"; Layout.preferredWidth: 330; model: ["Supplied H · reference correction","Pilot-estimated H-hat"]; currentIndex: dialog.draft.receiver_mode === "pilot_estimated" ? 1 : 0; onActivated: function(index) { dialog.draft.receiver_mode=index===1 ? "pilot_estimated":"supplied_channel" } }
            Label { text: "Observation Duration (ms)"; color: theme.text }
            SpinBox { objectName: "runDuration"; from:1; to:1000; editable:true; value:dialog.draft.duration_ms || 3; onValueModified: dialog.draft.duration_ms=value }
            Label { text: "Decoder Iteration Limit"; color: theme.text }
            SpinBox { from:1; to:100; editable:true; value:dialog.draft.max_iterations || 50; onValueModified: dialog.draft.max_iterations=value }
            Label { text: "Active Host Budget (ms)"; color: theme.text }
            SpinBox { from:1000; to:300000; stepSize:1000; editable:true; value:dialog.draft.max_runtime_ms || 300000; onValueModified: dialog.draft.max_runtime_ms=value }
            Label { text: "Noise Seed"; color: theme.text }
            SpinBox { from:0; to:2147483647; editable:true; value:dialog.draft.noise_seed === undefined ? 41 : dialog.draft.noise_seed; onValueModified: dialog.draft.noise_seed=value }
        }
        Label { text: "Known symbol timing is required. Supplied H uses the channel input; pilot mode estimates the response."; color:theme.muted; Layout.fillWidth:true; wrapMode:Text.WordWrap }
        Label { objectName:"receiverConfigError"; text:dialog.errorMessage; color:theme.statusError; Layout.fillWidth:true; wrapMode:Text.WordWrap }
        Button { objectName:"receiverConfigApply"; text:"Apply Receiver Configuration"; onClicked: { dialog.errorMessage=backend.apply(JSON.stringify(dialog.draft),settingsBackend.current()); if (!dialog.errorMessage.length) { dialog.configurationApplied();dialog.close() } } }
    }
}
