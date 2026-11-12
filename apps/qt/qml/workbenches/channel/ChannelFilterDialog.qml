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
    property string errorMessage: ""
    property var diagnostic: null
    signal contextHelpRequested(string code)
    property var accepted: ({mode:"diagonal"})
    signal configurationApplied()
    function document() {
        if (source.currentIndex===1) {
            const paths=accepted.paths ? accepted.paths.map(function(path){return {delay_samples:path.delay_samples,gain:path.gain.slice()}}) : [{delay_samples:0,gain:[1,0]}]
            paths[1]={delay_samples:echoDelay.value,gain:[Number(echoReal.text),Number(echoImaginary.text)]}
            return {mode:"echoes",paths:paths}
        }
        if (source.currentIndex===2) {
            return {mode:"captured_fir",max_delay_samples:memory.value,causal_shift_samples:shift.value,maximum_rms_error:accepted.maximum_rms_error || 0.05,maximum_bin_error:accepted.maximum_bin_error || 0.5}
        }
        return {mode:"diagonal"}
    }
    onOpened: {
        accepted=runState.options.channel_filter || {mode:"diagonal"}
        source.currentIndex=accepted.mode==="echoes" ? 1 : accepted.mode==="captured_fir" ? 2 : 0
        const path=accepted.paths && accepted.paths.length>1 ? accepted.paths[1] : {delay_samples:8,gain:[0.1,0]}
        echoDelay.value=path.delay_samples
        echoReal.text=String(path.gain[0]);echoImaginary.text=String(path.gain[1])
        memory.value=accepted.max_delay_samples ?? 512;shift.value=accepted.causal_shift_samples || 0
        errorMessage="";diagnostic=null
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
            ComboBox {id:source;objectName:"channelFilterSource";model:["Per-subcarrier reference","Synthetic echo","CM capture · continuous FIR"];Layout.fillWidth:true}
            Label {visible:source.currentIndex===1;text:"Echo Delay (samples)";color:dialog.theme.text}
            SpinBox {id:echoDelay;objectName:"channelEchoDelay";visible:source.currentIndex===1;from:1;to:2048;editable:true;value:8}
            Label {visible:source.currentIndex===1;text:"Echo Gain · Real";color:dialog.theme.text}
            TextField {id:echoReal;visible:source.currentIndex===1;text:"0.1";validator:DoubleValidator {bottom:-4;top:4;locale:"C"}}
            Label {visible:source.currentIndex===1;text:"Echo Gain · Imaginary";color:dialog.theme.text}
            TextField {id:echoImaginary;visible:source.currentIndex===1;text:"0";validator:DoubleValidator {bottom:-4;top:4;locale:"C"}}
            Label {visible:source.currentIndex===2;text:"Maximum Delay (samples)";color:dialog.theme.text}
            SpinBox {id:memory;objectName:"channelFilterMemory";visible:source.currentIndex===2;from:0;to:2048;editable:true;value:512}
            Label {visible:source.currentIndex===2;text:"Causal Shift (samples)";color:dialog.theme.text}
            SpinBox {id:shift;visible:source.currentIndex===2;from:0;to:memory.value;editable:true}
        }
        Label {
            text:source.currentIndex===1 ? "Direct path plus delayed echo. Existing additional paths are retained; this form edits the first echo. Use Flat / unity input. Tilt / Cable Loss is applied before the echo filter. Amplifier and attenuator settings still apply. Receiver correction is configured separately." : source.currentIndex===2 ? "Filters actual samples across symbols. Excluded bins use explicit unity extension; the finite response is shifted and truncated. Fits outside RMS 0.05 / maximum 0.5 complex error are rejected with default tolerances. This is a chosen reconstruction, not measured absolute path delay." : "Existing synchronized response per subcarrier. No continuous channel memory is modeled."
            color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap
        }
        ContextualStatus {theme:dialog.theme;text:dialog.errorMessage;diagnostic:dialog.diagnostic;color:dialog.theme.statusError;Layout.fillWidth:true;onHelpRequested:function(code){dialog.contextHelpRequested(code)} }
    }
    footer:ApplyCancelFooter {
        theme:dialog.theme;applyObjectName:"channelFilterApply";applyEnabled:!dialog.runState.busy
        onApplied:{
            const options=Object.assign({},dialog.runState.options,{channel_filter:dialog.document()})
            dialog.errorMessage=dialog.backend.apply(JSON.stringify(options),dialog.settingsBackend.current())
            dialog.diagnostic=JSON.parse(dialog.backend.admission_help())
            if(!dialog.errorMessage.length){dialog.configurationApplied();dialog.close()}
        }
        onCancelled:dialog.reject()
    }
}
