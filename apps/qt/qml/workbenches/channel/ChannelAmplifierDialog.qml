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
    property bool busy: false
    property var accepted: ({})
    property string error: ""
    signal modelApplied()
    function applyDraft() {
        const draft=Object.assign({},accepted)
        draft.amplifier_gain_db=Number(gain.text)
        draft.input_data_level_dbmv_per_6mhz=calibrated.checked ? Number(reference.text) : null
        draft.amplifier_target_dbmv_per_6mhz=targetMode.checked ? Number(target.text) : null
        error=backend.power_apply(JSON.stringify(draft))
        if (!error) {modelApplied();close()}
    }
    onOpened: {
        accepted=JSON.parse(backend.power_current());error=""
        gain.text=String(accepted.amplifier_gain_db || 0)
        calibrated.checked=accepted.input_data_level_dbmv_per_6mhz!==null && accepted.input_data_level_dbmv_per_6mhz!==undefined
        reference.text=String(accepted.input_data_level_dbmv_per_6mhz ?? 0)
        targetMode.checked=accepted.amplifier_target_dbmv_per_6mhz!==null && accepted.amplifier_target_dbmv_per_6mhz!==undefined
        target.text=String(accepted.amplifier_target_dbmv_per_6mhz ?? 0)
    }
    title:"Launch Amplifier"
    parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(620,parent ? parent.width-40 : 620)
    modal:true
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "channelAmplifierApply";applyEnabled:!dialog.busy && gain.acceptableInput && (!calibrated.checked || reference.acceptableInput) && (!targetMode.checked || target.acceptableInput);onApplied:dialog.applyDraft()
        onCancelled: dialog.reject()
    }
    contentItem:ColumnLayout {
        spacing:12
        GridLayout {
            columns:2;columnSpacing:16;rowSpacing:12;Layout.fillWidth:true
            Label {text:"Gain (dB)";color:dialog.theme.text}
            TextField {id:gain;objectName:"channelAmplifierGain";text:"0";enabled:!targetMode.checked;validator:DoubleValidator {bottom:0;top:60;locale:"C"} Layout.fillWidth:true}
            CheckBox {id:calibrated;objectName:"channelPowerCalibrated";text:"TX Electrical Reference";Layout.columnSpan:2;onToggled:if(!checked){targetMode.checked=false}}
            Label {text:"Input Data Level (dBmV / 6 MHz)";color:dialog.theme.text;enabled:calibrated.checked}
            TextField {id:reference;objectName:"channelInputLevel";text:"0";enabled:calibrated.checked;validator:DoubleValidator {bottom:-120;top:60;locale:"C"} Layout.fillWidth:true}
            CheckBox {id:targetMode;objectName:"channelAmplifierTargetMode";text:"Set Output Data Level";enabled:calibrated.checked;Layout.columnSpan:2}
            Label {text:"Output Target (dBmV / 6 MHz)";color:dialog.theme.text;enabled:targetMode.checked}
            TextField {id:target;objectName:"channelAmplifierTarget";text:"0";enabled:targetMode.checked;validator:DoubleValidator {bottom:-120;top:60;locale:"C"} Layout.fillWidth:true}
        }
        Label {text:"The reference calibrates nominal data density. Channel Power includes the actual pilots and occupied band. Imported channel response remains a relative multiplier.";color:dialog.theme.muted;wrapMode:Text.WordWrap;Layout.fillWidth:true}
        Label {visible:!!dialog.error;text:dialog.error;color:dialog.theme.statusError;wrapMode:Text.WordWrap;Layout.fillWidth:true}
    }
}
