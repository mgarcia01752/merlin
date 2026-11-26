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

    // Existing SpinBox hints/defaults, not authoritative Rust admission limits.
    readonly property int minimumProfileDurationMs: 1
    readonly property int maximumProfileDurationMs: 1000
    readonly property int initialProfileDurationMs: 3
    readonly property int minimumActiveRunBudgetMs: 1000
    readonly property int maximumActiveRunBudgetMs: 300000
    readonly property int activeRunBudgetStepMs: 1000
    readonly property int initialActiveRunBudgetMs: 300000
    readonly property int minimumNoiseSeed: 0
    readonly property int maximumNoiseSeed: 2147483647
    readonly property int initialNoiseSeed: 41
    readonly property var draft: coverage.value ? coverage.value.draft : ({})
    readonly property var profileIds: coverage.value ? coverage.value.profileIds : []
    readonly property var selectedIds: coverage.value ? coverage.value.selectedIds : []
    property bool contextCurrent: false
    property PresentationOwner presentationCoverage: PresentationOwner {id:coverage;theme:dialog.theme}
    readonly property string noiseSeedHelp: "Chooses the simulated noise pattern.\nSame seed and configuration repeat the pattern.\nChange it for another pattern at the same target noise level.\nDoes not set noise strength or the PRBS data seed."
    readonly property string errorMessage:coverage.value ? coverage.value.error : ""
    signal configurationApplied()
    title: "Workbench Configuration"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650,parent ? parent.width-40 : 650)
    modal: true; standardButtons: Dialog.NoButton
    /**
     * Couple the existing options and independently retained profile selectors.
     * @param options Original run draft, without scientific conversion.
     * @param profiles Current native profile catalog IDs.
     * @param selected Existing selected-ID policy result.
     * @param error Actual backend error text, or empty on successful preparation.
     * @returns Plain candidate admitted before publishing the whole draft.
     */
    function candidate(options, profiles, selected, error) {
        return {draft:options,profileIds:profiles,selectedIds:selected,error:error,
            profiles:coverage.value && coverage.value.profileIds===profiles ? coverage.value.profiles : profiles.map(function(id){return {id:id,label:"Profile "+id}}),
            selection:profiles.length ? selected.length+" of "+profiles.length+" profiles selected" : "Import a DPD to select profiles."}
    }

    /**
     * Admit a changed selection before its checkbox/model conversion.
     * @param ids Complete selected IDs; original ordering policy is unchanged.
     * @returns False leaves the previous options/selection intact.
     */
    function selectIds(ids) {
        return contextCurrent && coverage.replace(candidate(draft,profileIds,ids,""))
    }

    /**
     * Preserve the existing sorted toggle operation under one admission.
     * @param id Actual admitted catalog ID.
     * @param checked Requested selection state.
     * @returns False keeps the old checkbox selection.
     */
    function select(id, checked) {
        if (!contextCurrent || profileIds.indexOf(id)<0) {
            return false
        }
        return selectIds(checked ? selectedIds.concat([id]).sort(function(a,b){return a-b}) : selectedIds.filter(function(value){return value!==id}))
    }

    /**
     * Update an already covered numerical slot without changing its footprint.
     * @param key Existing native option field.
     * @param value Existing SpinBox value within its unchanged domain limits.
     * @returns False prevents editing a stale or unavailable draft.
     */
    function numberChanged(key, value) {
        if (!contextCurrent || !coverage.textAdmitted) {
            return false
        }
        if (Object.prototype.hasOwnProperty.call(draft,key)) {
            draft[key] = value
            return true
        }
        const changed = Object.assign({},draft)
        changed[key] = value
        return coverage.replace(candidate(changed,profileIds,selectedIds,""))
    }

    /**
     * Admit actual TextInput capacities for all three existing numeric editors.
     * @returns False gates text/editing; every open retries the same capacities.
     */
    function prepareText() {
        return coverage.reserveTextFields([durationSpin.contentItem,runtimeSpin.contentItem,noiseSpin.contentItem])
    }

    /**
     * Apply the original wire draft; successful native Apply always completes.
     * @returns False for stale input or actual native rejection.
     */
    function applyDraft() {
        if (!contextCurrent || !coverage.textAdmitted) {
            return false
        }
        const wire = Object.assign({},draft)
        wire.all_profiles = false
        wire.selected_profile_ids = profileIds.length ? selectedIds.slice() : null
        const error = backend.apply(JSON.stringify(wire),settingsBackend.current())
        if (!error.length) {
            configurationApplied()
            close()
            return true
        }
        coverage.replace(candidate(draft,profileIds,selectedIds,error))
        return false
    }
    Component.onCompleted: prepareText()
    onOpened: {
        prepareText()
        contextCurrent = false
        const options = Object.assign({},runState.options)
        const profiles = (runState.available_profile_ids || []).slice()
        const selected = Array.isArray(options.selected_profile_ids) ? options.selected_profile_ids.slice() : options.all_profiles ? profiles.slice() : profiles.filter(function(id){return id===runState.applied_profile_id})
        if (coverage.replace(candidate(options,profiles,selected,""))) {
            contextCurrent = true
            tabs.currentIndex = 0
        }
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
                SpinBox { id:durationSpin;objectName:"wbRunDuration";from:dialog.minimumProfileDurationMs;to:dialog.maximumProfileDurationMs;enabled:dialog.contextCurrent && coverage.textAdmitted;editable:coverage.textAdmitted;textFromValue:coverage.textAdmitted ? function(v,locale){return Number(v).toLocaleString(locale,"f",0)} : function(v,locale){return ""};value:dialog.draft.duration_ms || dialog.initialProfileDurationMs;onValueModified:dialog.numberChanged("duration_ms",value) }
                Label { text:"Total Active Run Budget (ms)";color:dialog.theme.text }
                SpinBox { id:runtimeSpin;from:dialog.minimumActiveRunBudgetMs;to:dialog.maximumActiveRunBudgetMs;stepSize:dialog.activeRunBudgetStepMs;enabled:dialog.contextCurrent && coverage.textAdmitted;editable:coverage.textAdmitted;textFromValue:coverage.textAdmitted ? function(v,locale){return Number(v).toLocaleString(locale,"f",0)} : function(v,locale){return ""};value:dialog.draft.max_runtime_ms || dialog.initialActiveRunBudgetMs;onValueModified:dialog.numberChanged("max_runtime_ms",value) }
                Label {
                    text:"Noise Seed";color:dialog.theme.text
                    HoverHandler { id:noiseSeedLabelHover }
                    ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: noiseSeedLabelHover.hovered
                    ToolTip.text: dialog.noiseSeedHelp
                }
                SpinBox {
                    id:noiseSpin;objectName:"wbNoiseSeed"
                    from:dialog.minimumNoiseSeed;to:dialog.maximumNoiseSeed;enabled:dialog.contextCurrent && coverage.textAdmitted;editable:coverage.textAdmitted
                    textFromValue:coverage.textAdmitted ? function(v,locale){return Number(v).toLocaleString(locale,"f",0)} : function(v,locale){return ""}
                    value:dialog.draft.noise_seed===undefined ? dialog.initialNoiseSeed:dialog.draft.noise_seed
                    onValueModified:dialog.numberChanged("noise_seed",value)
                    ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered || visualFocus
                    ToolTip.text: dialog.noiseSeedHelp
                    Accessible.description: dialog.noiseSeedHelp
                }
            }
            ColumnLayout {
                spacing:12
                RowLayout {
                    Label { text:"Profiles to Run";color:dialog.theme.accent;font.bold:true;Layout.fillWidth:true }
                    WorkbenchActionButton { theme: dialog.theme; objectName:"wbSelectAllProfiles";text:"Select All";enabled:dialog.contextCurrent && dialog.profileIds.length>0;onClicked:dialog.selectIds(dialog.profileIds.slice()) }
                    WorkbenchActionButton { theme: dialog.theme; objectName:"wbClearProfiles";text:"Clear";enabled:dialog.contextCurrent && dialog.profileIds.length>0;onClicked:dialog.selectIds([]) }
                }
                ScrollView {
                    Layout.fillWidth:true
                    Layout.preferredHeight:Math.min(240, profileList.implicitHeight)
                    clip:true
                    Column {
                        id:profileList;width:parent.width
                        Repeater {
                            objectName:"wbProfileChecks"
                            model:coverage.value ? coverage.value.profiles : []
                            CheckBox {
                                required property var modelData
                                objectName:"wbProfile"+modelData.id
                                text:modelData.label
                                enabled:dialog.contextCurrent
                                checked:dialog.selectedIds.indexOf(modelData.id)>=0
                                onToggled:if(!dialog.select(modelData.id,checked)){checked=Qt.binding(function(){return dialog.selectedIds.indexOf(modelData.id)>=0})}
                            }
                        }
                    }
                }
                Label { text:coverage.value ? coverage.value.selection : "";color:dialog.theme.muted }
                Label { visible:dialog.profileIds.length>0 && dialog.selectedIds.length===0;text:"Select at least one profile before starting.";color:dialog.theme.statusAttention }
            }
        }
        Label {objectName:"wbConfigurationRetention";visible:coverage.rejected || !dialog.contextCurrent;text:coverage.rejected ? coverage.message : "Reload this dialog before applying changes.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { objectName:"wbConfigurationError";text:dialog.errorMessage;color:dialog.theme.statusError;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "wbConfigurationApply"
        applyEnabled: !dialog.runState.busy && dialog.contextCurrent && coverage.textAdmitted
        onApplied: dialog.applyDraft()
        onCancelled: dialog.reject()
    }
}
