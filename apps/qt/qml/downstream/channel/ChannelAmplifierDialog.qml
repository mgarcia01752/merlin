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
    property bool busy: false

    // Existing editor hints/defaults; Rust power admission remains authoritative.
    readonly property real minimumGainDb: 0
    readonly property real maximumGainDb: 60
    readonly property real initialGainDb: 0
    readonly property real minimumElectricalLevelDbmvPer6Mhz: -120
    readonly property real maximumElectricalLevelDbmvPer6Mhz: 60
    readonly property real initialElectricalReferenceDbmvPer6Mhz: 0
    readonly property real initialOutputTargetDbmvPer6Mhz: 0
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
        return presentation.reserveTextFields([gain, reference, target])
    }
    signal modelApplied()
    function applyDraft() {
        if (!presentation.textAdmitted || !presentation.value || presentation.rejected) {
            return
        }
        const draft=Object.assign({},accepted)
        draft.amplifier_gain_db=Number(gain.text)
        draft.input_data_level_dbmv_per_6mhz=calibrated.checked ? Number(reference.text) : null
        draft.amplifier_target_dbmv_per_6mhz=targetMode.checked ? Number(target.text) : null
        const result = backend.power_apply(JSON.stringify(draft))
        presentation.replace({accepted: accepted, error: result})
        if (!result) {modelApplied();close()}
    }
    function restore() {
        if (!admitTextFields()) {
            return false
        }
        const nextAccepted = JSON.parse(backend.power_current())
        if (!presentation.replace({accepted: nextAccepted, error: ""})) {
            return false
        }
        gain.text=String(accepted.amplifier_gain_db || initialGainDb)
        calibrated.checked=accepted.input_data_level_dbmv_per_6mhz!==null && accepted.input_data_level_dbmv_per_6mhz!==undefined
        reference.text=String(accepted.input_data_level_dbmv_per_6mhz ?? initialElectricalReferenceDbmvPer6Mhz)
        targetMode.checked=accepted.amplifier_target_dbmv_per_6mhz!==null && accepted.amplifier_target_dbmv_per_6mhz!==undefined
        target.text=String(accepted.amplifier_target_dbmv_per_6mhz ?? initialOutputTargetDbmvPer6Mhz)
        return true
    }
    onOpened: restore()
    title:"Launch Amplifier"
    parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(620,parent ? parent.width-40 : 620)
    modal:true
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "channelAmplifierApply";applyEnabled:presentation.textAdmitted && !presentation.rejected && !dialog.busy && gain.acceptableInput && (!calibrated.checked || reference.acceptableInput) && (!targetMode.checked || target.acceptableInput);onApplied:dialog.applyDraft()
        onCancelled: dialog.reject()
    }
    contentItem:ColumnLayout {
        spacing:12
        GridLayout {
            columns:2;columnSpacing:16;rowSpacing:12;Layout.fillWidth:true
            Label {text:"Gain (dB)";color:dialog.theme.text}
            TextField {id:gain;objectName:"channelAmplifierGain";text:presentation.textAdmitted ? String(dialog.initialGainDb) : "";enabled:presentation.textAdmitted && !targetMode.checked;validator:DoubleValidator {bottom:dialog.minimumGainDb;top:dialog.maximumGainDb;locale:"C"} Layout.fillWidth:true}
            CheckBox {id:calibrated;objectName:"channelPowerCalibrated";text:"TX Electrical Reference";enabled:presentation.textAdmitted;Layout.columnSpan:2;onToggled:if(!checked){targetMode.checked=false}}
            Label {text:"Input Data Level (dBmV / 6 MHz)";color:dialog.theme.text;enabled:calibrated.checked}
            TextField {id:reference;objectName:"channelInputLevel";text:presentation.textAdmitted ? String(dialog.initialElectricalReferenceDbmvPer6Mhz) : "";enabled:presentation.textAdmitted && calibrated.checked;validator:DoubleValidator {bottom:dialog.minimumElectricalLevelDbmvPer6Mhz;top:dialog.maximumElectricalLevelDbmvPer6Mhz;locale:"C"} Layout.fillWidth:true}
            CheckBox {id:targetMode;objectName:"channelAmplifierTargetMode";text:"Set Output Data Level";enabled:presentation.textAdmitted && calibrated.checked;Layout.columnSpan:2}
            Label {text:"Output Target (dBmV / 6 MHz)";color:dialog.theme.text;enabled:targetMode.checked}
            TextField {id:target;objectName:"channelAmplifierTarget";text:presentation.textAdmitted ? String(dialog.initialOutputTargetDbmvPer6Mhz) : "";enabled:presentation.textAdmitted && targetMode.checked;validator:DoubleValidator {bottom:dialog.minimumElectricalLevelDbmvPer6Mhz;top:dialog.maximumElectricalLevelDbmvPer6Mhz;locale:"C"} Layout.fillWidth:true}
        }
        Label {text:"The reference calibrates nominal data density. Channel Power includes the actual pilots and occupied band. Imported channel response remains a relative multiplier.";color:dialog.theme.muted;wrapMode:Text.WordWrap;Layout.fillWidth:true}
        Label {visible:presentation.rejected || !!dialog.error;text:presentation.rejected ? presentation.message : dialog.error;color:dialog.theme.statusError;wrapMode:Text.WordWrap;Layout.fillWidth:true}
    }
}
