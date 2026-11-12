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
    property var diagnostic: null
    signal contextHelpRequested(string code)
    signal configurationApplied()
    function setMode(mode) { draft=Object.assign({},draft,{receiver_mode:mode}) }
    title: "Receiver Configuration"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650,parent ? parent.width - 40 : 650)
    modal: true; standardButtons: Dialog.NoButton
    onOpened: { draft=Object.assign({},runState.options); draft.mce=Object.assign({minimum_variance:1e-8,averaging_symbols:16},draft.mce || {}); errorMessage="";diagnostic=null }
    contentItem: ColumnLayout {
        spacing: 14
        Label { text: "Configure receiver processing"; color: theme.accent; font.bold: true }
        GridLayout {
            columns: 2; columnSpacing: 18; rowSpacing: 10
            Label { text: "MERLIN Channel Estimation"; color: theme.text }
            CheckBox {
                objectName: "receiverMceEnabled"; text: "MCE"
                checked: dialog.draft.receiver_mode === "merlin_estimated"
                onToggled: dialog.setMode(checked ? "merlin_estimated" : "supplied_channel")
                ToolTip.visible: hovered || activeFocus
                ToolTip.text: "Estimate the channel and decoder reliability from received pilots. Supplied H still impairs the channel; MCE never uses it for correction."
            }
            Label { text: "Receiver Correction"; color: theme.text }
            ComboBox { objectName: "receiverCorrection"; enabled:dialog.draft.receiver_mode!=="merlin_estimated"; Layout.preferredWidth: 330; model: ["Supplied H · reference correction","Pilot-estimated H-hat"]; displayText:dialog.draft.receiver_mode==="merlin_estimated" ? "MCE · received pilots" : currentText; currentIndex: dialog.draft.receiver_mode === "pilot_estimated" ? 1 : 0; onActivated: function(index) { dialog.setMode(index===1 ? "pilot_estimated":"supplied_channel") } }
            Label { text: "MCE Averaging (symbols)"; color:theme.text }
            SpinBox { objectName:"receiverMceAveraging"; from:1;to:128;editable:true;enabled:dialog.draft.receiver_mode==="merlin_estimated";value:dialog.draft.mce ? dialog.draft.mce.averaging_symbols : 16;onValueModified:dialog.draft.mce.averaging_symbols=value }
            Label { text: "Decoder Iteration Limit"; color: theme.text }
            SpinBox { from:1; to:100; editable:true; value:dialog.draft.max_iterations || 50; onValueModified: dialog.draft.max_iterations=value }

        }
        Label { text: "Supplied H impairs both modes. MCE estimates its own correction and reliability from received pilots; full training spans 128 symbols. Known symbol timing remains required."; color:theme.muted; Layout.fillWidth:true; wrapMode:Text.WordWrap }
        ContextualStatus { objectName:"receiverConfigError";theme:dialog.theme;text:dialog.errorMessage;diagnostic:dialog.diagnostic;color:dialog.theme.statusError;Layout.fillWidth:true;onHelpRequested:function(code){dialog.contextHelpRequested(code)} }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "receiverConfigApply"
        applyEnabled: !dialog.runState.busy
        onApplied: {
            dialog.errorMessage = backend.apply(JSON.stringify(dialog.draft), settingsBackend.current())
            dialog.diagnostic=JSON.parse(backend.admission_help())
            if (!dialog.errorMessage.length) {
                dialog.configurationApplied()
                dialog.close()
            }
        }
        onCancelled: dialog.reject()
    }
}
