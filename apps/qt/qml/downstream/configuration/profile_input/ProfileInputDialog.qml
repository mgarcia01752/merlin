/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"

WorkbenchDialog {
    id: dialog
    objectName: "profileInputDialog"
    required property var backend
    required property var settingsBackend
    readonly property var previewModel:coverage.value ? coverage.value.preview : ({})
    readonly property bool canRestoreWorkbench:coverage.value ? coverage.value.restore : false
    readonly property var collapsed:coverage.value ? coverage.value.collapsed : ({})
    readonly property string errorMessage:coverage.value ? coverage.value.error : ""
    readonly property string rowTitle:coverage.value ? coverage.value.title : "Selected profile"
    readonly property string rowDescription:coverage.value ? coverage.value.description : "Open a binary PNM capture or supported decoded JSON, then choose a profile."
    readonly property var visibleRows:coverage.value ? coverage.value.rows : []
    property bool contextCurrent:false
    property PresentationOwner presentationCoverage: PresentationOwner {
        id:coverage;theme:dialog.theme
        initialValue:dialog.candidate({rows:[],profile_ids:[],selected_profile_id:-1,source_name:"",format:""},({}),"","Selected profile","Open a binary PNM capture or supported decoded JSON, then choose a profile.",false)
    }
    signal inputApplied()
    property alias plotPopup: profilePlot
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(960, parent ? parent.width - 40 : 960)
    height: Math.min(700, parent ? parent.height - 40 : 700)
    modal: true
    title: "Downstream Profile Descriptor (DPD)"
    /**
     * Couple native preview, tree filters and all independently retained row text.
     * @param preview Original native DPD preview, without scientific reconstruction.
     * @param branches Existing collapsed-key map.
     * @param error Actual native error text or empty.
     * @param title Original selected-row title.
     * @param description Original selected-row description.
     * @param restore Existing accepted-workbench restore availability.
     * @returns Plain candidate admitted before ListView/selector conversion.
     */
    function candidate(preview, branches, error, title, description, restore) {
        const rows = coverage && coverage.value && coverage.value.preview===preview && coverage.value.collapsed===branches ? coverage.value.rows : (preview.rows || []).filter(function(row){return row.ancestors.every(function(key){return !branches[key]})}).map(function(row){return {source:row,text:(row.branch ? (branches[row.key] ? "▸  " : "▾  ") : "")+row.label,accessible:row.label+": "+row.value}})
        const ids = preview.profile_ids || []
        return {preview:preview,collapsed:branches,error:error,title:title,description:description,restore:restore,rows:rows,
            profiles:ids.map(function(id){return String(id)}),
            selected:ids.indexOf(preview.selected_profile_id)>=0 ? "Profile: "+preview.selected_profile_id : "No profile selected",
            detected:preview.format ? "Detected: "+preview.format : ""}
    }

    /**
     * Admit a native result before enabling actions against its staged context.
     * @param document Original native result JSON.
     * @param branches Requested collapsed state, or current state when omitted.
     * @param restoring Whether this is the original begin/restore operation.
     * @param wasCurrent Whether native rejection/cancellation preserves this view.
     * @param title Existing row title retained for an actual native error.
     * @param description Existing row description retained for that error.
     * @returns False preserves old presentation; stale native context stays disabled.
     */
    function showResult(document,branches,restoring,wasCurrent = contextCurrent,title,description) {
        const result = JSON.parse(document)
        if (result.cancelled) {
            contextCurrent = !!wasCurrent
            return false
        }
        if (result.error) {
            contextCurrent = !!wasCurrent
            return coverage.replace(candidate(previewModel,collapsed,result.error,title || rowTitle,description || rowDescription,canRestoreWorkbench))
        }
        contextCurrent = coverage.replace(candidate(result,branches || collapsed,"","Selected profile","Choose one profile for the next run. Imported capture bytes remain unchanged.",restoring ? (result.profile_ids || []).length>0 : canRestoreWorkbench))
        return contextCurrent
    }

    /**
     * Restore native accepted input and admit its complete editor context.
     * @returns False retains old presentation without permitting stale actions.
     */
    function restoreWorkbench() {
        profilePlot.close()
        contextCurrent = false
        return showResult(backend.begin(),{provenance:true},true,false,rowTitle,rowDescription)
    }

    /**
     * Choose a new native input; cancellation retains the prior valid context.
     * @returns False for cancellation/rejection or presentation admission failure.
     */
    function chooseFile() {
        profilePlot.close()
        const wasCurrent = contextCurrent
        contextCurrent = false
        return showResult(backend.choose_file(),collapsed,false,wasCurrent,rowTitle,rowDescription)
    }

    /**
     * Preview an actual admitted catalog ID without publishing stale indices.
     * @param id Existing native profile ID.
     * @param title Optional clicked-row title preserved on native error.
     * @param description Optional clicked-row description preserved on error.
     * @returns False leaves old profile presentation and disables stale staging.
     */
    function chooseProfile(id, title, description) {
        if (!contextCurrent || (previewModel.profile_ids || []).indexOf(id)<0) {
            return false
        }
        const branches = Object.assign({},collapsed)
        branches["profile_"+id] = false
        contextCurrent = false
        return showResult(backend.preview(id),branches,false,true,title,description)
    }

    /**
     * Admit an existing tree action and its description before model conversion.
     * @param row Original currently displayed native row.
     * @returns False keeps old tree/description; no invalid catalog action runs.
     */
    function activateRow(row) {
        // Resolve model wrappers through the current native row key. No stale
        // wrapper enters a new candidate or supplies a native profile identifier.
        row=(previewModel.rows || []).find(function(current){return row && current.key===row.key})
        if (!contextCurrent || !row) {
            return false
        }
        const description = row.description+(row.value ? "\n"+row.value : "")
        if (row.profile_id!==undefined && row.profile_id!==previewModel.selected_profile_id) {
            return chooseProfile(row.profile_id,row.label,description)
        }
        const branches = row.branch ? Object.assign({},collapsed) : collapsed
        if (row.branch) {
            branches[row.key] = !branches[row.key]
        }
        return coverage.replace(candidate(previewModel,branches,errorMessage,row.label,description,canRestoreWorkbench))
    }

    /**
     * Clear the native draft and admit the resulting preview as one transaction.
     * @returns False retains old presentation and prevents stale Apply.
     */
    function clearDraft() {
        if (!contextCurrent) {
            return false
        }
        profilePlot.close()
        contextCurrent = false
        return showResult(backend.clear_draft(),collapsed,false,true,rowTitle,rowDescription)
    }

    /**
     * Complete real native Apply even if optional error text cannot be admitted.
     * @returns False for stale context or actual native validation failure.
     */
    function applyDraft() {
        if (!contextCurrent) {
            return false
        }
        const error = backend.apply(previewModel.selected_profile_id,settingsBackend.current())
        if (!error.length) {
            inputApplied()
            close()
            return true
        }
        coverage.replace(candidate(previewModel,collapsed,error,rowTitle,rowDescription,canRestoreWorkbench))
        return false
    }
    onOpened: restoreWorkbench()
    onClosed: {contextCurrent=false;profilePlot.close();backend.cancel()}
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            Layout.fillWidth: true
            WorkbenchActionButton { theme: dialog.theme; objectName: "profileOpenFile"; text: "Open File…"; onClicked:dialog.chooseFile() }
            WorkbenchActionButton { theme: dialog.theme; objectName: "profileRestoreWorkbench"; text: "Restore Workbench"; enabled: dialog.canRestoreWorkbench; onClicked: dialog.restoreWorkbench()
                ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered || visualFocus
                ToolTip.text: "Discard pending edits and restore the last applied profile from this workbench."
                Accessible.description: ToolTip.text }
            WorkbenchActionButton { theme: dialog.theme; objectName: "profileView"; text: "View Profile…"; enabled:dialog.contextCurrent && (dialog.previewModel.profile_ids || []).length>0;onClicked:profilePlot.openChecked() }
            Item { Layout.fillWidth: true }
            Label { objectName: "profileDetectedType"; text:coverage.value ? coverage.value.detected : ""; color: theme.accent }
        }
        Label { text: "PNM modulation-profile capture · Read-only inspection"; color: theme.muted; Layout.fillWidth: true }
        Rectangle {
            Layout.fillWidth: true; Layout.fillHeight: true
            color: theme.background; border.color: theme.line; radius: 4
            ColumnLayout {
                anchors.fill: parent; anchors.margins: 1; spacing: 0
                Rectangle {
                    Layout.fillWidth: true; implicitHeight: 30; color: theme.button
                    Row {
                        anchors.fill: parent
                        Label { width: parent.width * 0.52; leftPadding: 12; height: parent.height; verticalAlignment: Text.AlignVCenter; text: "Parameter"; color: theme.text; font.bold: true }
                        Label { width: parent.width * 0.48; leftPadding: 12; height: parent.height; verticalAlignment: Text.AlignVCenter; text: "Value"; color: theme.text; font.bold: true }
                    }
                }
                ListView {
                    id: tree
                    objectName: "profileTree"
                    Layout.fillWidth: true; Layout.fillHeight: true; clip: true
                    model: dialog.visibleRows
                    ScrollBar.vertical: ScrollBar {}
                    delegate: ItemDelegate {
                        required property var modelData
                        readonly property var row:modelData.source
                        enabled:dialog.contextCurrent
                        width: tree.width; height: 30
                        Accessible.name:modelData.accessible
                        background: Rectangle { color: parent.hovered || parent.visualFocus || row.profile_id === dialog.previewModel.selected_profile_id ? theme.accentSoft : (row.branch ? theme.panel : theme.background); border.color: theme.line }
                        contentItem: Row {
                            Label {
                                width: parent.width * 0.52; height: parent.height
                                leftPadding: 12 + row.depth * 22
                                text:modelData.text
                                color: theme.text; font.bold: row.branch
                                elide: Text.ElideRight; verticalAlignment: Text.AlignVCenter
                            }
                            Label { width: parent.width * 0.48; height: parent.height; leftPadding: 12; text: row.value; color: theme.text; elide: Text.ElideRight; verticalAlignment: Text.AlignVCenter }
                        }
                        onClicked:dialog.activateRow(row)
                    }
                    Label { anchors.centerIn: parent; visible: (dialog.previewModel.rows || []).length===0; text: "Open a profile file or restore the workbench input."; color: theme.muted }
                }
            }
        }
        Rectangle {
            Layout.fillWidth: true; implicitHeight: 105
            color: theme.panel; border.color: theme.line; radius: 4
            ScrollView {
                id: description
                anchors.fill: parent; anchors.margins: 12; clip: true
                contentWidth: availableWidth
                ColumnLayout {
                    width: description.availableWidth; spacing: 6
                    Label { text: dialog.rowTitle; color: theme.accent; font.bold: true; font.pixelSize: 17 }
                    Label { text: dialog.rowDescription; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
                    Label { text: "DPD: MULPI 3.1 I25 §6.4.41 · PNM capture: CM-OSSI 3.1 I27 Table 80"; color: theme.muted; font.pixelSize: 12; Layout.fillWidth: true; wrapMode: Text.WordWrap }
                }
            }
        }
        Label {objectName:"profileInputRetention";visible:coverage.rejected || !dialog.contextCurrent;text:coverage.rejected ? coverage.message : "Reload this dialog before applying changes.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { objectName: "profileInputError"; text: dialog.errorMessage; visible: text.length > 0; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
    ProfilePlotDialog {
        id: profilePlot
        theme: dialog.theme
        backend: dialog.backend
        profileIds:dialog.previewModel.profile_ids || []
        initialProfile:dialog.previewModel.selected_profile_id ?? -1
        sourceCurrent:dialog.contextCurrent
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "profileApply"
        applyEnabled:dialog.contextCurrent
        onApplied:dialog.applyDraft()
        onCancelled: dialog.reject()
        WorkbenchActionButton { theme: dialog.theme; objectName: "profileRemove"; text: "Remove input"; enabled:dialog.contextCurrent && (dialog.previewModel.profile_ids || []).length>0;onClicked:dialog.clearDraft() }
        ComboBox {
            objectName: "profileChoice"
            implicitWidth: 160
            model:coverage.value ? coverage.value.profiles : []
            currentIndex:(dialog.previewModel.profile_ids || []).indexOf(dialog.previewModel.selected_profile_id)
            displayText:coverage.value ? coverage.value.selected : "No profile selected"
            enabled:dialog.contextCurrent && count>0
            Accessible.name: "Selected modulation profile"
            onActivated:function(index){if(!dialog.chooseProfile((dialog.previewModel.profile_ids || [])[index])){currentIndex=Qt.binding(function(){return (dialog.previewModel.profile_ids || []).indexOf(dialog.previewModel.selected_profile_id)})}}
        }
    }
}
