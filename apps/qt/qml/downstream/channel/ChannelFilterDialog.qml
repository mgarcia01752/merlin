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

    // Independent legacy editor hints; Rust owns filter validation and fitting.
    readonly property int minimumEchoDelaySamples: 1
    readonly property int maximumEchoDelaySamples: 2048
    readonly property int initialEchoDelaySamples: 8
    readonly property real minimumEchoGainComponent: -4
    readonly property real maximumEchoGainComponent: 4
    readonly property real initialEchoRealGain: 0.1
    readonly property real initialEchoImaginaryGain: 0
    readonly property int minimumFirMemorySamples: 0
    readonly property int maximumFirMemorySamples: 2048
    readonly property int initialFirMemorySamples: 512
    readonly property int minimumCausalShiftSamples: 0
    readonly property int initialCausalShiftSamples: 0
    readonly property real defaultMaximumRmsError: 0.05
    readonly property real defaultMaximumBinError: 0.5
    readonly property string errorMessage: presentation.value ? presentation.value.errorMessage : ""
    readonly property var diagnostic: presentation.value ? presentation.value.diagnostic : null
    signal contextHelpRequested(string code)
    readonly property var accepted: presentation.value ? presentation.value.accepted : ({mode:"diagonal"})
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.filterCandidate({mode:"diagonal"},"",null)
    }
    property Binding echoDelayTextBinding: Binding {
        target: echoDelay.contentItem
        property: "text"
        value: presentation.textAdmitted ? echoDelay.displayText : ""
    }
    property Binding memoryTextBinding: Binding {
        target: memory.contentItem
        property: "text"
        value: presentation.textAdmitted ? memory.displayText : ""
    }
    property Binding shiftTextBinding: Binding {
        target: shift.contentItem
        property: "text"
        value: presentation.textAdmitted ? shift.displayText : ""
    }
    Component.onCompleted: Qt.callLater(dialog.admitTextFields)
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    /** Keep the existing plain model inside the same admitted DTO candidate. */
    function filterCandidate(accepted,error,diagnostic) {
        return {accepted:accepted,errorMessage:error,diagnostic:diagnostic,
            sources:["Per-subcarrier reference","Synthetic echo","CM capture · continuous FIR"]}
    }
    function admitTextFields() {
        return presentation.reserveTextFields([echoReal, echoImaginary,
            echoDelay.contentItem, memory.contentItem, shift.contentItem])
    }
    signal configurationApplied()
    function document() {
        if (source.currentIndex===1) {
            const paths=accepted.paths ? accepted.paths.map(function(path){return {delay_samples:path.delay_samples,gain:path.gain.slice()}}) : [{delay_samples:0,gain:[1,0]}]
            paths[1]={delay_samples:echoDelay.value,gain:[Number(echoReal.text),Number(echoImaginary.text)]}
            return {mode:"echoes",paths:paths}
        }
        if (source.currentIndex===2) {
            return {mode:"captured_fir",max_delay_samples:memory.value,causal_shift_samples:shift.value,maximum_rms_error:accepted.maximum_rms_error || defaultMaximumRmsError,maximum_bin_error:accepted.maximum_bin_error || defaultMaximumBinError}
        }
        return {mode:"diagonal"}
    }
    function restore() {
        if (!admitTextFields()) {
            return false
        }
        const nextAccepted = runState.options.channel_filter || {mode:"diagonal"}
        if (!presentation.replace(filterCandidate(nextAccepted,"",null))) {
            return false
        }
        source.currentIndex=accepted.mode==="echoes" ? 1 : accepted.mode==="captured_fir" ? 2 : 0
        const path=accepted.paths && accepted.paths.length>1 ? accepted.paths[1] : {delay_samples:initialEchoDelaySamples,gain:[initialEchoRealGain,initialEchoImaginaryGain]}
        echoDelay.value=path.delay_samples
        echoReal.text=String(path.gain[0]);echoImaginary.text=String(path.gain[1])
        memory.value=accepted.max_delay_samples ?? initialFirMemorySamples;shift.value=accepted.causal_shift_samples || initialCausalShiftSamples
        return true
    }
    onOpened: restore()
    function applyDraft() {
        if (!presentation.textAdmitted || !presentation.value || presentation.rejected) {
            return
        }
        const options=Object.assign({},runState.options,{channel_filter:document()})
        const result=backend.apply(JSON.stringify(options),settingsBackend.current())
        const nextDiagnostic=JSON.parse(backend.admission_help())
        presentation.replace(filterCandidate(accepted,result,nextDiagnostic))
        if(!result.length){configurationApplied();close()}
    }
    title: "Echo / Delay"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(650,parent ? parent.width-40 : 650)
    modal:true;standardButtons:Dialog.NoButton
    contentItem:ColumnLayout {
        spacing:12
        GridLayout {
            columns:2;columnSpacing:18;rowSpacing:10;Layout.fillWidth:true
            Label {text:"Response Model";color:dialog.theme.text}
            ComboBox {id:source;objectName:"channelFilterSource";model:presentation.value ? presentation.value.sources : [];enabled:presentation.textAdmitted;Layout.fillWidth:true}
            Label {visible:source.currentIndex===1;text:"Echo Delay (samples)";color:dialog.theme.text}
            SpinBox {id:echoDelay;objectName:"channelEchoDelay";visible:source.currentIndex===1;from:dialog.minimumEchoDelaySamples;to:dialog.maximumEchoDelaySamples;editable:presentation.textAdmitted;enabled:presentation.textAdmitted;value:dialog.initialEchoDelaySamples}
            Label {visible:source.currentIndex===1;text:"Echo Gain · Real";color:dialog.theme.text}
            TextField {id:echoReal;visible:source.currentIndex===1;text:presentation.textAdmitted ? String(dialog.initialEchoRealGain) : "";enabled:presentation.textAdmitted;validator:DoubleValidator {bottom:dialog.minimumEchoGainComponent;top:dialog.maximumEchoGainComponent;locale:"C"}}
            Label {visible:source.currentIndex===1;text:"Echo Gain · Imaginary";color:dialog.theme.text}
            TextField {id:echoImaginary;visible:source.currentIndex===1;text:presentation.textAdmitted ? String(dialog.initialEchoImaginaryGain) : "";enabled:presentation.textAdmitted;validator:DoubleValidator {bottom:dialog.minimumEchoGainComponent;top:dialog.maximumEchoGainComponent;locale:"C"}}
            Label {visible:source.currentIndex===2;text:"Maximum Delay (samples)";color:dialog.theme.text}
            SpinBox {id:memory;objectName:"channelFilterMemory";visible:source.currentIndex===2;from:dialog.minimumFirMemorySamples;to:dialog.maximumFirMemorySamples;editable:presentation.textAdmitted;enabled:presentation.textAdmitted;value:dialog.initialFirMemorySamples}
            Label {visible:source.currentIndex===2;text:"Causal Shift (samples)";color:dialog.theme.text}
            SpinBox {id:shift;visible:source.currentIndex===2;from:dialog.minimumCausalShiftSamples;to:memory.value;editable:presentation.textAdmitted;enabled:presentation.textAdmitted}
        }
        Label {
            text:source.currentIndex===1 ? "Direct path plus delayed echo. Existing additional paths are retained; this form edits the first echo. Use Flat / unity input. Tilt / Cable Loss is applied before the echo filter. Amplifier and attenuator settings still apply. Receiver correction is configured separately." : source.currentIndex===2 ? "Filters actual samples across symbols. Excluded bins use explicit unity extension; the finite response is shifted and truncated. Fits outside RMS 0.05 / maximum 0.5 complex error are rejected with default tolerances. This is a chosen reconstruction, not measured absolute path delay." : "Existing synchronized response per subcarrier. No continuous channel memory is modeled."
            color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap
        }
        ContextualStatus {theme:dialog.theme;text:presentation.rejected ? presentation.message : dialog.errorMessage;diagnostic:presentation.rejected ? null : dialog.diagnostic;color:dialog.theme.statusError;Layout.fillWidth:true;onHelpRequested:function(code){dialog.contextHelpRequested(code)} }
    }
    footer:ApplyCancelFooter {
        theme:dialog.theme;applyObjectName:"channelFilterApply";applyEnabled:presentation.textAdmitted && !presentation.rejected && !dialog.runState.busy
        onApplied:dialog.applyDraft()
        onCancelled:dialog.reject()
    }
}
