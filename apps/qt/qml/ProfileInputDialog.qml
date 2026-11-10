/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

Dialog {
    id: dialog
    objectName: "profileInputDialog"
    required property var theme
    required property var backend
    required property var settingsBackend
    property var previewModel: ({rows: [], profile_ids: [], selected_profile_id: -1, source_name: "", format: ""})
    property var collapsed: ({})
    property string errorMessage: ""
    property string rowTitle: "Selected profile"
    property string rowDescription: "Open a binary PNM capture or supported decoded JSON, then choose a profile."
    readonly property var visibleRows: previewModel.rows.filter(row => row.ancestors.every(key => !collapsed[key]))
    signal inputApplied()
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(960, parent ? parent.width - 40 : 960)
    height: Math.min(700, parent ? parent.height - 40 : 700)
    modal: true
    title: "Downstream Profile Descriptor (DPD)"
    onOpened: {
        collapsed = ({provenance: true})
        errorMessage = ""
        showResult(backend.begin())
    }
    onClosed: backend.cancel()
    function showResult(document) {
        const result = JSON.parse(document)
        if (result.error) {
            errorMessage = result.error
            return
        }
        previewModel = result
        errorMessage = ""
        rowTitle = "Selected profile"
        rowDescription = "Choose one profile for the next run. Imported capture bytes remain unchanged."
    }
    function chooseProfile(id) {
        const updated = Object.assign({}, collapsed)
        updated["profile_" + id] = false
        collapsed = updated
        showResult(backend.preview(id))
    }
    function toggleBranch(key) {
        const updated = Object.assign({}, collapsed)
        updated[key] = !updated[key]
        collapsed = updated
    }
    FileDialog {
        id: picker
        title: "Open downstream OFDM modulation-profile capture"
        nameFilters: ["PNM captures and JSON (*.bin *.pnm *.json)", "All files (*)"]
        onAccepted: dialog.showResult(dialog.backend.stage(selectedFile.toString()))
    }
    contentItem: ColumnLayout {
        spacing: 10
        RowLayout {
            Layout.fillWidth: true
            Button { objectName: "profileOpenFile"; text: "Open File…"; onClicked: picker.open() }
            Button { objectName: "profileLoadDemo"; text: "Load demo"; onClicked: dialog.showResult(dialog.backend.demo()) }
            Label { text: dialog.previewModel.source_name || "No input selected"; color: theme.text; elide: Text.ElideMiddle; Layout.fillWidth: true }
            Label { objectName: "profileDetectedType"; text: dialog.previewModel.format ? "Detected: " + dialog.previewModel.format : ""; color: theme.accent }
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
                        width: tree.width; height: 30
                        Accessible.name: modelData.label + ": " + modelData.value
                        background: Rectangle { color: parent.hovered || parent.visualFocus || modelData.profile_id === dialog.previewModel.selected_profile_id ? theme.accentSoft : (modelData.branch ? theme.panel : theme.background); border.color: theme.line }
                        contentItem: Row {
                            Label {
                                width: parent.width * 0.52; height: parent.height
                                leftPadding: 12 + modelData.depth * 22
                                text: (modelData.branch ? (dialog.collapsed[modelData.key] ? "▸  " : "▾  ") : "") + modelData.label
                                color: theme.text; font.bold: modelData.branch
                                elide: Text.ElideRight; verticalAlignment: Text.AlignVCenter
                            }
                            Label { width: parent.width * 0.48; height: parent.height; leftPadding: 12; text: modelData.value; color: theme.text; elide: Text.ElideRight; verticalAlignment: Text.AlignVCenter }
                        }
                        onClicked: {
                            dialog.rowTitle = modelData.label
                            dialog.rowDescription = modelData.description + (modelData.value ? "\n" + modelData.value : "")
                            if (modelData.profile_id !== undefined && modelData.profile_id !== dialog.previewModel.selected_profile_id) {
                                dialog.chooseProfile(modelData.profile_id)
                            } else if (modelData.branch) {
                                dialog.toggleBranch(modelData.key)
                            }
                        }
                    }
                    Label { anchors.centerIn: parent; visible: dialog.previewModel.rows.length === 0; text: "Open File or Load demo to inspect profiles."; color: theme.muted }
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
        Label { objectName: "profileInputError"; text: dialog.errorMessage; visible: text.length > 0; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { text: "Apply accepts this input for the session. Simulation and file saving remain pending."; color: theme.muted; font.pixelSize: 12 }
    }
    footer: Pane {
        padding: 10
        background: Item {}
        contentItem: RowLayout {
        spacing: 10
        Button { objectName: "profileRemove"; text: "Remove input"; enabled: dialog.previewModel.profile_ids.length > 0; onClicked: dialog.showResult(dialog.backend.clear_draft()) }
        Item { Layout.fillWidth: true }
        ComboBox {
            objectName: "profileChoice"
            implicitWidth: 160
            model: dialog.previewModel.profile_ids
            currentIndex: dialog.previewModel.profile_ids.indexOf(dialog.previewModel.selected_profile_id)
            displayText: currentIndex >= 0 ? "Profile: " + currentText : "No profile selected"
            enabled: count > 0
            Accessible.name: "Selected modulation profile"
            onActivated: function(index) { dialog.chooseProfile(dialog.previewModel.profile_ids[index]) }
        }
        Button { text: "Cancel"; onClicked: dialog.close() }
        Button {
            objectName: "profileApply"; text: "Apply"; 
            onClicked: {
                dialog.errorMessage = dialog.backend.apply(dialog.previewModel.selected_profile_id, dialog.settingsBackend.current())
                if (dialog.errorMessage.length === 0) {
                    dialog.inputApplied()
                    dialog.close()
                }
            }
        }
        }
    }
}
