/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    objectName: "ofdmSettingsDialog"
    required property var theme
    required property var backend
    property var draft: ({})
    property var profileSummary: ({})
    property var registryData: ({fields: [], profile_fields: []})
    property var selectedField: null
    property string errorMessage: ""
    readonly property real parameterWidth: Math.floor((width - 26) * 0.56)
    signal settingsApplied()
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(820, parent ? parent.width - 40 : 820)
    height: Math.min(680, parent ? parent.height - 40 : 680)
    modal: true
    title: "OFDM Channel Descriptor (OCD)"
    onOpened: {
        registryData = JSON.parse(backend.registry())
        draft = JSON.parse(backend.current())
        selectedField = registryData.fields.find(field => field.id === "cyclic_prefix_samples") || null
        errorMessage = ""
    }
    function updateValue(field, value) {
        const changed = Object.assign({}, draft)
        changed[field.id] = value
        draft = changed
        selectedField = field
        errorMessage = ""
    }
    function applyDraft() {
        errorMessage = backend.apply(JSON.stringify(draft))
        if (errorMessage.length === 0) {
            settingsApplied()
            close()
        }
    }
    contentItem: ColumnLayout {
        spacing: 10
        Label {
            text: "Edit a value or select a row to read its description."
            color: theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap
        }
        Rectangle {
            Layout.fillWidth: true; Layout.fillHeight: true
            color: theme.background; border.color: theme.line; radius: 4
            ColumnLayout {
                anchors.fill: parent; anchors.margins: 1; spacing: 0
                Rectangle {
                    Layout.fillWidth: true; implicitHeight: 32; color: theme.button
                    RowLayout {
                        anchors.fill: parent; spacing: 0
                        Label { text: "Parameter"; color: theme.text; font.bold: true; Layout.preferredWidth: dialog.parameterWidth; leftPadding: 12 }
                        Label { text: "Value"; color: theme.text; font.bold: true; Layout.fillWidth: true; leftPadding: 12 }
                    }
                }
                ScrollView {
                    id: treeScroll
                    Layout.fillWidth: true; Layout.fillHeight: true; clip: true
                    contentWidth: availableWidth
                    ColumnLayout {
                        width: treeScroll.availableWidth; spacing: 0
                        Repeater {
                            model: [
                                {title: "Symbol timing", group: "Timing"},
                                {title: "Interleaving", group: "Interleaving"},
                                {title: "Modulation Profile", group: "Profile"}
                            ]
                            delegate: ColumnLayout {
                                id: branch
                                required property var modelData
                                property bool expanded: true
                                Layout.fillWidth: true; spacing: 0
                                Button {
                                    objectName: branch.modelData.group + "Branch"
                                    Layout.fillWidth: true; implicitHeight: 32
                                    text: (branch.expanded ? "▾   " : "▸   ") + branch.modelData.title
                                    contentItem: Label { text: parent.text; color: theme.text; font.bold: true; leftPadding: 12; verticalAlignment: Text.AlignVCenter }
                                    background: Rectangle { color: theme.panel; border.color: theme.line }
                                    onClicked: branch.expanded = !branch.expanded
                                    Accessible.name: branch.modelData.title
                                }
                                Repeater {
                                    model: branch.modelData.group === "Profile" ? (dialog.registryData.profile_fields || []) : dialog.registryData.fields.filter(field => field.group === branch.modelData.group)
                                    delegate: Rectangle {
                                        id: fieldRow
                                        required property var modelData
                                        readonly property bool profileRow: branch.modelData.group === "Profile"
                                        Layout.fillWidth: true; implicitHeight: 36
                                        visible: branch.expanded
                                        color: dialog.selectedField && dialog.selectedField.id === modelData.id ? theme.accentSoft : theme.background
                                        border.color: theme.line
                                        RowLayout {
                                            anchors.fill: parent; spacing: 0
                                            ItemDelegate {
                                                Layout.preferredWidth: dialog.parameterWidth
                                                Layout.fillHeight: true
                                                text: fieldRow.modelData.label
                                                leftPadding: 36
                                                background: Item {}
                                                contentItem: Label { text: parent.text; color: theme.text; verticalAlignment: Text.AlignVCenter; elide: Text.ElideRight }
                                                onClicked: dialog.selectedField = fieldRow.modelData
                                            }
                                            Rectangle { Layout.fillHeight: true; width: 1; color: theme.line }
                                            Loader {
                                                Layout.fillWidth: true; Layout.fillHeight: true
                                                Layout.margins: 3
                                                sourceComponent: fieldRow.profileRow ? profileValue : editableValue
                                                Component {
                                                    id: profileValue
                                                    Label {
                                                        objectName: fieldRow.modelData.id + "Value"
                                                        text: dialog.profileSummary[fieldRow.modelData.id] || ""
                                                        color: theme.muted
                                                        Accessible.name: fieldRow.modelData.label + ": " + (text || "no modulation profile selected")
                                                    }
                                                }
                                                Component {
                                                    id: editableValue
                                                    ComboBox {
                                                        objectName: fieldRow.modelData.id + "Editor"
                                                        model: fieldRow.modelData.choices
                                                        textRole: "label"
                                                        currentIndex: fieldRow.modelData.choices.findIndex(choice => choice.value === dialog.draft[fieldRow.modelData.id])
                                                        Accessible.name: fieldRow.modelData.label
                                                        onActiveFocusChanged: {
                                                            if (activeFocus) { dialog.selectedField = fieldRow.modelData }
                                                        }
                                                        onActivated: function(index) { dialog.updateValue(fieldRow.modelData, fieldRow.modelData.choices[index].value) }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Rectangle {
            Layout.fillWidth: true; implicitHeight: 146
            color: theme.panel; border.color: theme.line; radius: 4
            ScrollView {
                id: descriptionScroll
                anchors.fill: parent; anchors.margins: 12; clip: true
                contentWidth: availableWidth
                ColumnLayout {
                    width: descriptionScroll.availableWidth; spacing: 7
                    Label { text: dialog.selectedField ? dialog.selectedField.label : ""; color: theme.accent; font.pixelSize: 18; font.bold: true }
                    Label { text: dialog.selectedField ? dialog.selectedField.description : ""; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
                    Label {
                        text: dialog.selectedField && dialog.selectedField.source ? "Specification paraphrase · " + dialog.selectedField.source.document + " · §" + dialog.selectedField.source.section + (dialog.selectedField.source.table ? " · Table " + dialog.selectedField.source.table : "") : "MERLIN input metadata"
                        color: theme.muted; font.pixelSize: 12; Layout.fillWidth: true; wrapMode: Text.WordWrap
                    }
                    Label {
                        visible: !!(dialog.selectedField && dialog.selectedField.choices)
                        text: visible ? "Suggested default: " + dialog.selectedField.choices.find(choice => choice.value === dialog.selectedField.application_default).label + " (MERLIN application default)" : ""
                        color: theme.muted; font.pixelSize: 12; Layout.fillWidth: true; wrapMode: Text.WordWrap
                    }
                }
            }
        }
        Label { objectName: "ofdmValidationError"; text: dialog.errorMessage; visible: text.length > 0; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { text: dialog.profileSummary.profile_id !== undefined ? "Accepted profile shown above. Settings are kept in this session only." : "No modulation profile selected. Settings are kept in this session only."; color: theme.muted; font.pixelSize: 12; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
    footer: DialogButtonBox {
        Button {
            text: "Reset defaults"
            DialogButtonBox.buttonRole: DialogButtonBox.ResetRole
            onClicked: {
                dialog.draft = JSON.parse(dialog.backend.defaults())
                dialog.errorMessage = ""
            }
        }
        Button { text: "Cancel"; DialogButtonBox.buttonRole: DialogButtonBox.RejectRole; onClicked: dialog.close() }
        Button { objectName: "ofdmApply"; text: "Apply"; DialogButtonBox.buttonRole: DialogButtonBox.ApplyRole; onClicked: dialog.applyDraft() }
    }
}
