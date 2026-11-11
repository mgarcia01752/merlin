/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id:dialog
    required property var backend
    property bool busy:false
    property var accepted:({})
    property string error:""
    signal modelApplied()
    onOpened: {
        accepted=JSON.parse(backend.power_current());error=""
        attenuation.text=String(accepted.attenuation_db || 0)
        noise.checked=accepted.receiver_noise_dbmv_hz!==null && accepted.receiver_noise_dbmv_hz!==undefined
        density.text=String(accepted.receiver_noise_dbmv_hz ?? -120)
    }
    function applyDraft() {
        const draft=Object.assign({},accepted)
        draft.attenuation_db=Number(attenuation.text)
        draft.receiver_noise_dbmv_hz=noise.checked ? Number(density.text) : null
        error=backend.power_apply(JSON.stringify(draft))
        if(!error){modelApplied();close()}
    }
    title:"Receiver Input Attenuator"
    parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(620,parent ? parent.width-40 : 620)
    modal:true
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "channelAttenuatorApply";applyEnabled:!dialog.busy && attenuation.acceptableInput && (!noise.checked || density.acceptableInput);onApplied:dialog.applyDraft()
        onCancelled: dialog.reject()
    }
    contentItem:ColumnLayout {
        spacing:12
        GridLayout {
            columns:2;columnSpacing:16;rowSpacing:12;Layout.fillWidth:true
            Label {text:"Attenuation (dB)";color:dialog.theme.text}
            TextField {id:attenuation;objectName:"channelAttenuation";text:"0";validator:DoubleValidator {bottom:0;top:60;locale:"C"} Layout.fillWidth:true}
            CheckBox {id:noise;objectName:"channelReceiverNoise";text:"Add Independent Receiver Noise";Layout.columnSpan:2}
            Label {text:"Receiver Noise Density (dBmV/Hz)";color:dialog.theme.text;enabled:noise.checked}
            TextField {id:density;objectName:"channelNoiseDensity";text:"-120";enabled:noise.checked;validator:DoubleValidator {bottom:-200;top:-30;locale:"C"} Layout.fillWidth:true}
        }
        Label {text:"Independent noise is added after attenuation, in addition to RxMER residuals. It requires a TX electrical reference. Leave it off when RxMER already represents the complete receiver noise budget.";color:dialog.theme.muted;wrapMode:Text.WordWrap;Layout.fillWidth:true}
        Label {visible:!!dialog.error;text:dialog.error;color:dialog.theme.statusError;wrapMode:Text.WordWrap;Layout.fillWidth:true}
    }
}
