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
    required property var runState

    // Existing editor hints/defaults; Rust receiver admission remains authoritative.
    readonly property real initialMceMinimumVariance: 1e-8
    readonly property int initialMceAveragingSymbols: 16
    readonly property int minimumMceAveragingSymbols: 1
    readonly property int maximumMceAveragingSymbols: 128
    readonly property int minimumDecoderIterations: 1
    readonly property int maximumDecoderIterations: 100
    readonly property int initialDecoderIterations: 50
    readonly property var draft: presentation.value ? presentation.value.draft : ({})
    readonly property string errorMessage: presentation.value ? presentation.value.error : ""
    readonly property var diagnostic: presentation.value ? presentation.value.diagnostic : null
    property PresentationOwner presentationCoverage: PresentationOwner {id:presentation;theme:dialog.theme}
    /** Admit the complete existing receiver draft and error/help context. */
    function adoptDraft(value,error,diagnostic) {
        return presentation.replace({draft:value,error:error,diagnostic:diagnostic,
            modes:["Supplied CM channel estimate","Pilot-based channel estimation","Pilot-based channel and noise estimation (MCE)"]})
    }
    /** Admit real editor capacities without changing their accepted domains. */
    function prepareText() {return presentation.reserveTextFields([averaging.contentItem,iterations.contentItem])}
    Component.onCompleted: prepareText()
    signal contextHelpRequested(string code)
    signal configurationApplied()
    function setMode(mode) { return adoptDraft(Object.assign({},draft,{receiver_mode:mode}),errorMessage,diagnostic) }
    title: "Receiver Configuration"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(820,parent ? parent.width - 40 : 820)
    modal: true; standardButtons: Dialog.NoButton
    onOpened: {
        if (prepareText()) {
            const value=Object.assign({},runState.options)
            value.mce=Object.assign({minimum_variance:initialMceMinimumVariance,averaging_symbols:initialMceAveragingSymbols},value.mce || {})
            adoptDraft(value,"",null)
        }
    }
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
                model: presentation.value ? presentation.value.modes : []
                currentIndex: dialog.draft.receiver_mode === "merlin_estimated" ? 2
                            : dialog.draft.receiver_mode === "pilot_estimated" ? 1 : 0
                onActivated: function(index) {
                    dialog.setMode(["supplied_channel", "pilot_estimated", "merlin_estimated"][index])
                }
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered || visualFocus
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
                id:averaging
                objectName: "receiverMceAveraging"
                visible: dialog.draft.receiver_mode === "merlin_estimated"
                from: dialog.minimumMceAveragingSymbols; to: dialog.maximumMceAveragingSymbols; enabled:presentation.textAdmitted;editable:presentation.textAdmitted
                textFromValue:presentation.textAdmitted ? function(value,locale){return Number(value).toLocaleString(locale,"f",0)} : function(value,locale){return ""}
                value: dialog.draft.mce ? dialog.draft.mce.averaging_symbols : dialog.initialMceAveragingSymbols
                onValueModified: dialog.adoptDraft(Object.assign({},dialog.draft,{mce:Object.assign({},dialog.draft.mce,{averaging_symbols:value})}),dialog.errorMessage,dialog.diagnostic)
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered || visualFocus
                ToolTip.text: "Smooth the pilot-derived noise estimate over this many symbols. This does not change the 128-symbol full pilot-history span."
            }
            Label { text: "Decoder iteration limit"; color: theme.text }
            SpinBox {id:iterations;from:dialog.minimumDecoderIterations;to:dialog.maximumDecoderIterations;enabled:presentation.textAdmitted;editable:presentation.textAdmitted;value:dialog.draft.max_iterations || dialog.initialDecoderIterations
                textFromValue:presentation.textAdmitted ? function(value,locale){return Number(value).toLocaleString(locale,"f",0)} : function(value,locale){return ""}
                onValueModified:dialog.adoptDraft(Object.assign({},dialog.draft,{max_iterations:value}),dialog.errorMessage,dialog.diagnostic)
            }
        }
        ContextualStatus { objectName:"receiverConfigError";theme:dialog.theme;text:presentation.rejected ? presentation.message : dialog.errorMessage;diagnostic:presentation.rejected ? null : dialog.diagnostic;color:dialog.theme.statusError;Layout.fillWidth:true;onHelpRequested:function(code){dialog.contextHelpRequested(code)} }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "receiverConfigApply"
        applyEnabled: !dialog.runState.busy && presentation.textAdmitted && !presentation.rejected && !!presentation.value
        onApplied: {
            const error=backend.apply(JSON.stringify(dialog.draft),settingsBackend.current())
            dialog.adoptDraft(dialog.draft,error,JSON.parse(backend.admission_help()))
            if (!error.length) {
                dialog.configurationApplied()
                dialog.close()
            }
        }
        onCancelled: dialog.reject()
    }
}
