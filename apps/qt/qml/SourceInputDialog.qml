/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    objectName: "sourceInputDialog"
    required property var theme
    required property var backend
    property var draft: ({mode: "prbs15", seed: 32767, append: false, target_bits: 0, fill_seed: 32767})
    property var fileInfo: ({selected: false, name: "", bytes: null, bits: null, sha256: ""})
    property var summary: ({error: "", fill_bits: null})
    property string errorMessage: ""
    property string descriptionTitle: "Transmit source"
    property string descriptionText: "Choose continuous PRBS15 or exact text/binary file bytes. Apply accepts the source for the next run."
    readonly property bool fileMode: draft.mode === "file"
    signal sourceApplied()
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(900, parent ? parent.width - 40 : 900)
    height: Math.min(720, parent ? parent.height - 40 : 720)
    modal: true
    title: "Transmit source (PRBS-TX)"
    onOpened: {
        const initial = JSON.parse(backend.begin())
        draft = initial.options
        fileInfo = initial.file
        sourceBranch.checked = true
        prbsBranch.checked = true
        fileBranch.checked = true
        fillBranch.checked = true
        errorMessage = ""
        help("Transmit source", "Choose continuous PRBS15 or exact text/binary file bytes. Apply accepts the source for the next run.")
        refresh()
    }
    onClosed: backend.cancel()
    function refresh() { summary = JSON.parse(backend.describe(JSON.stringify(draft))) }
    function edit(key, value) {
        const updated = Object.assign({}, draft)
        updated[key] = value
        draft = updated
        errorMessage = ""
        refresh()
    }
    function help(title, description) { descriptionTitle = title; descriptionText = description }
    function chooseFile() {
        const result = JSON.parse(backend.choose_file())
        if (result.error) {
            errorMessage = result.error
        } else {
            fileInfo = result
            errorMessage = ""
        }
        refresh()
    }
    component Branch: ToolButton {
        required property string label
        Layout.fillWidth: true
        checkable: true; checked: true
        text: (checked ? "▾  " : "▸  ") + label
        Accessible.name: label
        contentItem: Label { text: parent.text; color: dialog.theme.text; font.bold: true; verticalAlignment: Text.AlignVCenter; leftPadding: 10 }
        background: Rectangle { color: dialog.theme.panel; border.color: dialog.theme.line }
    }
    component FieldLabel: Label {
        color: dialog.theme.text
        leftPadding: 24
        Layout.preferredWidth: 275
        Layout.fillWidth: true
        wrapMode: Text.WordWrap
    }
    component ValueLabel: Label {
        color: dialog.theme.text
        Layout.fillWidth: true
        elide: Text.ElideMiddle
        Accessible.name: text
    }
    contentItem: ColumnLayout {
        spacing: 10
        Rectangle {
            Layout.fillWidth: true; implicitHeight: 32
            color: theme.button
            RowLayout {
                anchors.fill: parent; anchors.leftMargin: 12
                Label { text: "Parameter"; color: theme.text; font.bold: true; Layout.fillWidth: true; Layout.preferredWidth: 275 }
                Label { text: "Value"; color: theme.text; font.bold: true; Layout.fillWidth: true; Layout.preferredWidth: 340 }
            }
        }
        ScrollView {
            id: tree
            objectName: "sourceTree"
            Layout.fillWidth: true; Layout.fillHeight: true
            contentWidth: availableWidth
            clip: true
            ColumnLayout {
                width: tree.availableWidth; spacing: 1
                Branch { id: sourceBranch; label: "Source" }
                GridLayout {
                    columns: 2; columnSpacing: 12; rowSpacing: 6
                    Layout.fillWidth: true; visible: sourceBranch.checked
                    FieldLabel { text: "Source mode" }
                    ComboBox {
                        objectName: "sourceMode"; Layout.fillWidth: true; Layout.preferredWidth: 340
                        model: ["Continuous PRBS15", "File (text / binary)"]
                        currentIndex: dialog.fileMode ? 1 : 0
                        Accessible.name: "Transmit source mode"
                        onActivated: function(index) { dialog.edit("mode", index === 0 ? "prbs15" : "file") }
                    }
                }
                Branch { id: prbsBranch; label: "PRBS15 generator"; visible: !dialog.fileMode }
                GridLayout {
                    columns: 2; columnSpacing: 12; rowSpacing: 8
                    Layout.fillWidth: true; visible: !dialog.fileMode && prbsBranch.checked
                    FieldLabel { text: "Seed" }
                    TextField {
                        objectName: "sourceSeed"; Layout.fillWidth: true; Layout.preferredWidth: 340
                        text: String(dialog.draft.seed); selectByMouse: true
                        inputMethodHints: Qt.ImhDigitsOnly
                        Accessible.name: "PRBS15 seed, 1 through 32767"
                        onTextEdited: dialog.edit("seed", Number(text))
                        onActiveFocusChanged: if (activeFocus) { dialog.help("PRBS15 seed", "Nonzero 15-bit seed: 1–32767. Initial value 32767 (0x7fff). Codeword boundaries do not restart the sequence.") }
                    }
                    FieldLabel { text: "Polynomial" }
                    ValueLabel { text: "x¹⁵ + x¹⁴ + 1" }
                    FieldLabel { text: "Sequence period" }
                    ValueLabel { text: "32,767 bits" }
                    FieldLabel { text: "Operation" }
                    ValueLabel { text: "Continuous for the run" }
                }
                Branch { id: fileBranch; label: "Payload file"; visible: dialog.fileMode }
                GridLayout {
                    columns: 2; columnSpacing: 12; rowSpacing: 8
                    Layout.fillWidth: true; visible: dialog.fileMode && fileBranch.checked
                    FieldLabel { text: "File selection" }
                    Button {
                        objectName: "sourceOpenFile"; text: "Open file…"
                        onClicked: dialog.chooseFile()
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Exact-byte file", "Open using the OS-native picker. Text, BOM, line endings, zeros and binary bytes are preserved unchanged. Maximum original size: 16 MiB.") }
                    }
                    FieldLabel { text: "Source file" }
                    ValueLabel { objectName: "sourceFileName"; text: dialog.fileInfo.selected ? dialog.fileInfo.name : "No input selected" }
                    FieldLabel { text: "Original bytes" }
                    ValueLabel { text: dialog.fileInfo.selected ? String(dialog.fileInfo.bytes) : "—" }
                    FieldLabel { text: "Original bits" }
                    ValueLabel { text: dialog.fileInfo.selected ? String(dialog.fileInfo.bits) : "—" }
                    FieldLabel { text: "Original SHA-256" }
                    TextField {
                        Layout.fillWidth: true; Layout.preferredWidth: 340
                        text: dialog.fileInfo.sha256; readOnly: true; selectByMouse: true
                        placeholderText: "Available after selection"
                        Accessible.name: "Original payload SHA-256"
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Original input SHA-256", dialog.fileInfo.sha256 || "No input selected. The hash identifies original bytes, without appended fill or coding padding.") }
                    }
                }
                Branch { id: fillBranch; label: "Optional PRBS15 fill"; visible: dialog.fileMode }
                GridLayout {
                    columns: 2; columnSpacing: 12; rowSpacing: 8
                    Layout.fillWidth: true; visible: dialog.fileMode && fillBranch.checked
                    FieldLabel { text: "Append after file" }
                    CheckBox {
                        objectName: "sourceAppend"; text: "Enable PRBS15 fill"; checked: dialog.draft.append
                        onToggled: dialog.edit("append", checked)
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Optional finite fill", "Off initially. Without fill, send the original file once, then idle/filler. No automatic target, file repeat or truncation.") }
                    }
                    FieldLabel { text: "Total meaningful bits" }
                    TextField {
                        objectName: "sourceTarget"; Layout.fillWidth: true; Layout.preferredWidth: 340
                        enabled: dialog.draft.append; text: String(dialog.draft.target_bits); selectByMouse: true
                        inputMethodHints: Qt.ImhDigitsOnly
                        Accessible.name: "Total meaningful bits including original file"
                        onTextEdited: dialog.edit("target_bits", Number(text))
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Total meaningful bits", "Includes original file and appended PRBS15 bits. Must be at least the original bit count and at most 134217728. Partial-byte fill is allowed. Coding padding is separate.") }
                    }
                    FieldLabel { text: "Fill seed" }
                    TextField {
                        objectName: "sourceFillSeed"; Layout.fillWidth: true; Layout.preferredWidth: 340
                        enabled: dialog.draft.append; text: String(dialog.draft.fill_seed); selectByMouse: true
                        inputMethodHints: Qt.ImhDigitsOnly
                        Accessible.name: "Independent fill seed, 1 through 32767"
                        onTextEdited: dialog.edit("fill_seed", Number(text))
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Fill seed", "Independent PRBS15 seed, 1–32767. Starts after original-file bits and continues across codeword boundaries.") }
                    }
                    FieldLabel { text: "Additional PRBS bits" }
                    ValueLabel { objectName: "sourceFillBits"; text: dialog.summary.fill_bits === null ? "—" : String(dialog.summary.fill_bits) }
                    FieldLabel { text: "After source ends" }
                    ValueLabel { text: "Idle / filler · no file repeat" }
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
                    Label { text: dialog.descriptionTitle; color: theme.accent; font.bold: true; font.pixelSize: 17 }
                    Label { text: dialog.descriptionText; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
                }
            }
        }
        Label {
            objectName: "sourceError"
            text: dialog.errorMessage || dialog.summary.error
            visible: text.length > 0; color: theme.text
            Layout.fillWidth: true; wrapMode: Text.WordWrap
        }
        Label { text: "Session only · Run duration is separate · Simulation integration remains pending"; color: theme.muted; font.pixelSize: 12; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
    footer: Pane {
        padding: 10
        background: Item {}
        contentItem: RowLayout {
            Item { Layout.fillWidth: true }
            Button { objectName: "sourceCancel"; text: "Cancel"; onClicked: dialog.close() }
            Button {
                objectName: "sourceApply"; text: "Apply"
                onClicked: {
                    dialog.errorMessage = dialog.backend.apply(JSON.stringify(dialog.draft))
                    if (dialog.errorMessage.length === 0) {
                        dialog.sourceApplied()
                        dialog.close()
                    }
                }
            }
        }
    }
}
