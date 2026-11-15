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
    width: Math.min(820,parent ? parent.width - 40 : 820)
    modal: true; standardButtons: Dialog.NoButton
    onOpened: { draft=Object.assign({},runState.options); draft.mce=Object.assign({minimum_variance:1e-8,averaging_symbols:16},draft.mce || {}); errorMessage="";diagnostic=null }
    contentItem: ColumnLayout {
        spacing: 14
        GridLayout {
            columns: 2; columnSpacing: 20; rowSpacing: 12
            Layout.fillWidth: true
            Label { text: "Receiver mode"; color: theme.text }
            ComboBox {
                objectName: "receiverCorrection"
                Layout.fillWidth: true
                Layout.preferredWidth: 480
                model: ["Supplied CM channel estimate",
                        "Pilot-based channel estimation",
                        "Pilot-based channel and noise estimation (MCE)"]
                currentIndex: dialog.draft.receiver_mode === "merlin_estimated" ? 2
                            : dialog.draft.receiver_mode === "pilot_estimated" ? 1 : 0
                onActivated: function(index) {
                    dialog.setMode(["supplied_channel", "pilot_estimated", "merlin_estimated"][index])
                }
                ToolTip.visible: hovered || activeFocus
                ToolTip.text: "Both pilot-based modes derive channel correction from received pilots. Only MCE also estimates noise for decoder confidence. Full pilot history spans 128 OFDM symbols; known symbol timing is still required."
            }
            Label {
                objectName: "receiverModeExplanation"
                Layout.columnSpan: 2; Layout.fillWidth: true
                wrapMode: Text.WordWrap; color: theme.muted
                text: dialog.draft.receiver_mode === "merlin_estimated"
                    ? "Estimates channel correction and noise from received pilots to set decoder confidence."
                    : dialog.draft.receiver_mode === "pilot_estimated"
                    ? "Estimates channel correction from received pilots. Decoder noise information comes from the simulator."
                    : "Uses the supplied CM channel estimate, or a flat reference when absent. Added channel effects remain separate settings."
            }
            Label {
                text: "MCE averaging (symbols)"; color: theme.text
                visible: dialog.draft.receiver_mode === "merlin_estimated"
            }
            SpinBox {
                objectName: "receiverMceAveraging"
                visible: dialog.draft.receiver_mode === "merlin_estimated"
                from: 1; to: 128; editable: true
                value: dialog.draft.mce ? dialog.draft.mce.averaging_symbols : 16
                onValueModified: dialog.draft.mce.averaging_symbols=value
                ToolTip.visible: hovered || activeFocus
                ToolTip.text: "Smooth the pilot-derived noise estimate over this many symbols. This does not change the 128-symbol full pilot-history span."
            }
            Label { text: "Decoder iteration limit"; color: theme.text }
            SpinBox { from:1; to:100; editable:true; value:dialog.draft.max_iterations || 50; onValueModified: dialog.draft.max_iterations=value }
        }
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
