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
    property var profileIds: []
    property var selectedIds: []
    property string errorMessage: ""
    signal configurationApplied()
    title: "Workbench Configuration"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650,parent ? parent.width-40 : 650)
    modal: true; standardButtons: Dialog.NoButton
    function select(id, checked) {
        selectedIds = checked ? selectedIds.concat([id]).sort(function(a,b){return a-b}) : selectedIds.filter(function(value){return value!==id})
    }
    onOpened: {
        draft=Object.assign({}, runState.options)
        profileIds=(runState.available_profile_ids || []).slice()
        selectedIds=Array.isArray(draft.selected_profile_ids) ? draft.selected_profile_ids.slice() : draft.all_profiles ? profileIds.slice() : profileIds.filter(function(id){return id===runState.applied_profile_id})
        errorMessage=""; tabs.currentIndex=0
    }
    contentItem: ColumnLayout {
        spacing: 14
        TabBar {
            id:tabs;Layout.fillWidth:true
            TabButton { text:"Run" }
            TabButton { objectName:"wbProfilesTab";text:"Profiles" }
        }
        StackLayout {
            currentIndex:tabs.currentIndex;Layout.fillWidth:true
            GridLayout {
                columns:2;columnSpacing:24;rowSpacing:14
                Label { text:"Duration per Profile (ms)";color:dialog.theme.text }
                SpinBox { objectName:"wbRunDuration";from:1;to:1000;editable:true;value:dialog.draft.duration_ms || 3;onValueModified:dialog.draft.duration_ms=value }
                Label { text:"Total Active Run Budget (ms)";color:dialog.theme.text }
                SpinBox { from:1000;to:300000;stepSize:1000;editable:true;value:dialog.draft.max_runtime_ms || 300000;onValueModified:dialog.draft.max_runtime_ms=value }
                Label { text:"Noise Seed";color:dialog.theme.text }
                SpinBox { from:0;to:2147483647;editable:true;value:dialog.draft.noise_seed===undefined ? 41:dialog.draft.noise_seed;onValueModified:dialog.draft.noise_seed=value }
            }
            ColumnLayout {
                spacing:12
                RowLayout {
                    Label { text:"Profiles to Run";color:dialog.theme.accent;font.bold:true;Layout.fillWidth:true }
                    WorkbenchActionButton { theme: dialog.theme; objectName:"wbSelectAllProfiles";text:"Select All";enabled:dialog.profileIds.length>0;onClicked:dialog.selectedIds=dialog.profileIds.slice() }
                    WorkbenchActionButton { theme: dialog.theme; objectName:"wbClearProfiles";text:"Clear";enabled:dialog.profileIds.length>0;onClicked:dialog.selectedIds=[] }
                }
                ScrollView {
                    Layout.fillWidth:true
                    Layout.preferredHeight:Math.min(240, profileList.implicitHeight)
                    clip:true
                    Column {
                        id:profileList;width:parent.width
                        Repeater {
                            objectName:"wbProfileChecks"
                            model:dialog.profileIds
                            CheckBox {
                                required property int modelData
                                objectName:"wbProfile"+modelData
                                text:"Profile "+modelData
                                checked:dialog.selectedIds.indexOf(modelData)>=0
                                onToggled:dialog.select(modelData,checked)
                            }
                        }
                    }
                }
                Label { text:dialog.profileIds.length ? dialog.selectedIds.length+" of "+dialog.profileIds.length+" profiles selected" : "Import a DPD to select profiles.";color:dialog.theme.muted }
                Label { visible:dialog.profileIds.length>0 && dialog.selectedIds.length===0;text:"Select at least one profile before starting.";color:dialog.theme.statusAttention }
            }
        }
        Label { objectName:"wbConfigurationError";text:dialog.errorMessage;color:dialog.theme.statusError;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "wbConfigurationApply"
        applyEnabled: !dialog.runState.busy
        onApplied: {
            dialog.draft.all_profiles = false
            dialog.draft.selected_profile_ids = dialog.profileIds.length ? dialog.selectedIds.slice() : null
            dialog.errorMessage = dialog.backend.apply(JSON.stringify(dialog.draft), dialog.settingsBackend.current())
            if (!dialog.errorMessage.length) {
                dialog.configurationApplied()
                dialog.close()
            }
        }
        onCancelled: dialog.reject()
    }
}
