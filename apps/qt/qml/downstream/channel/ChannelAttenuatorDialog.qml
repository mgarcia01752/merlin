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
    id:dialog
    required property var backend
    property bool busy:false
    readonly property var accepted: presentation.value ? presentation.value.accepted : ({})
    readonly property string error: presentation.value ? presentation.value.error : ""
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: ({accepted: {}, error: ""})
    }
    Component.onCompleted: Qt.callLater(dialog.admitTextFields)
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    function admitTextFields() {
        return presentation.reserveTextFields([attenuation, density])
    }
    signal modelApplied()
    function restore() {
        if (!admitTextFields()) {
            return false
        }
        const nextAccepted = JSON.parse(backend.power_current())
        if (!presentation.replace({accepted: nextAccepted, error: ""})) {
            return false
        }
        attenuation.text=String(accepted.attenuation_db || 0)
        noise.checked=accepted.receiver_noise_dbmv_hz!==null && accepted.receiver_noise_dbmv_hz!==undefined
        density.text=String(accepted.receiver_noise_dbmv_hz ?? -120)
        return true
    }
    onOpened: restore()
    function applyDraft() {
        if (!presentation.textAdmitted || !presentation.value || presentation.rejected) {
            return
        }
        const draft=Object.assign({},accepted)
        draft.attenuation_db=Number(attenuation.text)
        draft.receiver_noise_dbmv_hz=noise.checked ? Number(density.text) : null
        const result = backend.power_apply(JSON.stringify(draft))
        presentation.replace({accepted: accepted, error: result})
        if(!result){modelApplied();close()}
    }
    title:"Common Path Attenuator (CPA)"
    parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(620,parent ? parent.width-40 : 620)
    modal:true
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "channelAttenuatorApply";applyEnabled:presentation.textAdmitted && !presentation.rejected && !dialog.busy && attenuation.acceptableInput && (!noise.checked || density.acceptableInput);onApplied:dialog.applyDraft()
        onCancelled: dialog.reject()
    }
    contentItem:ColumnLayout {
        spacing:12
        GridLayout {
            columns:2;columnSpacing:16;rowSpacing:12;Layout.fillWidth:true
            Label {text:"Attenuation (dB)";color:dialog.theme.text}
            TextField {id:attenuation;objectName:"channelAttenuation";text:presentation.textAdmitted ? "0" : "";enabled:presentation.textAdmitted;validator:DoubleValidator {bottom:0;top:60;locale:"C"} Layout.fillWidth:true}
            CheckBox {id:noise;objectName:"channelReceiverNoise";text:"Add Independent Receiver Noise";enabled:presentation.textAdmitted;Layout.columnSpan:2}
            Label {text:"Receiver Noise Density (dBmV/Hz)";color:dialog.theme.text;enabled:noise.checked}
            TextField {id:density;objectName:"channelNoiseDensity";text:presentation.textAdmitted ? "-120" : "";enabled:presentation.textAdmitted && noise.checked;validator:DoubleValidator {bottom:-200;top:-30;locale:"C"} Layout.fillWidth:true}
        }
        Label {text:"Independent noise is added after attenuation, in addition to RxMER residuals. It requires a TX electrical reference. Leave it off when RxMER already represents the complete receiver noise budget.";color:dialog.theme.muted;wrapMode:Text.WordWrap;Layout.fillWidth:true}
        Label {visible:presentation.rejected || !!dialog.error;text:presentation.rejected ? presentation.message : dialog.error;color:dialog.theme.statusError;wrapMode:Text.WordWrap;Layout.fillWidth:true}
    }
}
