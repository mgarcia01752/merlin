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
    objectName: "sourceInputDialog"
    required property var backend
    readonly property var draft:coverage.value ? coverage.value.draft : ({})
    readonly property var fileInfo:coverage.value ? coverage.value.file : ({})
    readonly property var summary:coverage.value ? coverage.value.summary : ({})
    readonly property string errorMessage:coverage.value ? coverage.value.error : ""
    readonly property string descriptionTitle:coverage.value ? coverage.value.title : "Transmit source"
    readonly property string descriptionText:coverage.value ? coverage.value.description : "Choose continuous PRBS15 or exact text/binary file bytes. Apply accepts the source for the next run."
    property bool contextCurrent:false
    property PresentationOwner presentationCoverage: PresentationOwner {
        id:coverage;theme:dialog.theme
        initialValue:dialog.candidate({mode:"prbs15",seed:32767,append:false,target_bits:0,fill_seed:32767},{selected:false,name:"",bytes:null,bits:null,sha256:""},{error:"",fill_bits:null},"","Transmit source","Choose continuous PRBS15 or exact text/binary file bytes. Apply accepts the source for the next run.")
    }
    readonly property bool fileMode: draft.mode === "file"
    signal sourceApplied()
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(900, parent ? parent.width - 40 : 900)
    height: Math.min(720, parent ? parent.height - 40 : 720)
    modal: true
    title: "Transmit Source (PRBS-TX)"
    /**
     * Couple source options/metadata/summary with all retained display strings.
     * @param options Existing presentation draft; numerical wire conversion is below.
     * @param file Original native file metadata, including its unchanged hash.
     * @param counts Original native describe response.
     * @param error Actual native error text or empty.
     * @param title Existing contextual-help title.
     * @param description Existing contextual-help description.
     * @returns Plain candidate admitted before persistent replacement.
     */
    function candidate(options, file, counts, error, title, description) {
        return {draft:options,file:file,summary:counts,error:error,title:title,description:description,
            modes:["Continuous PRBS15","File (text / binary)"],
            bytes:file.selected ? String(file.bytes) : "—",bits:file.selected ? String(file.bits) : "—",
            fill:counts.fill_bits===null ? "—" : String(counts.fill_bits)}
    }

    /**
     * Preserve original Number/text/JSON behavior for invalid numerical drafts.
     * @param options Presentation options; only nonfinite edits use display strings.
     * @returns Original wire JSON; Number restores NaN/Infinity and JSON writes null.
     */
    function wire(options) {
        const value = Object.assign({},options)
        for (const key of ["seed","target_bits","fill_seed"]) {
            if (typeof value[key]==="string") {
                value[key] = Number(value[key])
            }
        }
        return JSON.stringify(value)
    }

    /**
     * Admit actual capacities of all existing numeric/hash TextFields.
     * @returns False gates dynamic text and editing; every open retries.
     */
    function prepareText() {
        return coverage.reserveTextFields([seedText,targetText,fillSeedText,hashText])
    }

    /**
     * Admit a native count refresh against the current original file context.
     * @returns False keeps the old coherent draft/counts/details.
     */
    function refresh() {
        if (!contextCurrent) {
            return false
        }
        return coverage.replace(candidate(draft,fileInfo,JSON.parse(backend.describe(wire(draft))),errorMessage,descriptionTitle,descriptionText))
    }

    /**
     * Prepare one original edit/describe transaction before replacing its view.
     * @param key Existing source option field.
     * @param value Original UI conversion result, including invalid nonfinite input.
     * @returns False preserves the old draft and its matching native summary.
     */
    function edit(key, value) {
        if (!contextCurrent) {
            return false
        }
        const updated = Object.assign({},draft)
        updated[key] = typeof value==="number" && !Number.isFinite(value) ? String(value) : value
        return coverage.replace(candidate(updated,fileInfo,JSON.parse(backend.describe(wire(updated))),"",descriptionTitle,descriptionText))
    }

    /**
     * Admit contextual-help strings while the source/metadata witness stays live.
     * @param title Original field-help title.
     * @param description Original static or native-metadata description.
     * @returns False preserves the prior description and source context.
     */
    function help(title, description) {
        return coverage.replace(candidate(draft,fileInfo,summary,errorMessage,title,description))
    }

    /**
     * Stage a native file and admit its metadata/counts before enabling Apply.
     * @returns False leaves old presentation and disables stale staged-file actions.
     */
    function chooseFile() {
        const wasCurrent = contextCurrent
        contextCurrent = false
        const result = JSON.parse(backend.choose_file())
        const counts = JSON.parse(backend.describe(wire(draft)))
        if (result.error) {
            // Native rejection preserves its previous staged file transactionally.
            contextCurrent = wasCurrent
            return coverage.replace(candidate(draft,fileInfo,wasCurrent ? counts : summary,result.error,descriptionTitle,descriptionText))
        }
        contextCurrent = coverage.replace(candidate(draft,result,counts,"",descriptionTitle,descriptionText))
        return contextCurrent
    }

    /**
     * Complete real native Apply even if optional status presentation is rejected.
     * @returns False for stale context or actual native validation failure.
     */
    function applyDraft() {
        if (!contextCurrent || !coverage.textAdmitted) {
            return false
        }
        const error = backend.apply(wire(draft))
        if (!error.length) {
            sourceApplied()
            close()
            return true
        }
        coverage.replace(candidate(draft,fileInfo,summary,error,descriptionTitle,descriptionText))
        return false
    }
    Component.onCompleted: prepareText()
    onOpened: {
        prepareText()
        contextCurrent = false
        const initial = JSON.parse(backend.begin())
        if (initial.error) {
            coverage.replace(candidate(draft,fileInfo,summary,initial.error,descriptionTitle,descriptionText))
            return
        }
        const options = initial.options
        const counts = JSON.parse(backend.describe(wire(options)))
        if (coverage.replace(candidate(options,initial.file,counts,"","Transmit source","Choose continuous PRBS15 or exact text/binary file bytes. Apply accepts the source for the next run."))) {
            contextCurrent = true
            sourceBranch.checked = true
            prbsBranch.checked = true
            fileBranch.checked = true
            fillBranch.checked = true
        }
    }
    onClosed: {contextCurrent=false;backend.cancel()}
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
                        model:coverage.value ? coverage.value.modes : []
                        enabled:dialog.contextCurrent
                        currentIndex: dialog.fileMode ? 1 : 0
                        Accessible.name: "Transmit source mode"
                        onActivated: function(index) { if(!dialog.edit("mode",index===0 ? "prbs15" : "file")){currentIndex=Qt.binding(function(){return dialog.fileMode ? 1 : 0})} }
                    }
                }
                Branch { id: prbsBranch; label: "PRBS15 generator"; visible: !dialog.fileMode }
                GridLayout {
                    columns: 2; columnSpacing: 12; rowSpacing: 8
                    Layout.fillWidth: true; visible: !dialog.fileMode && prbsBranch.checked
                    FieldLabel { text: "Seed" }
                    TextField {
                        id:seedText;objectName: "sourceSeed"; Layout.fillWidth: true; Layout.preferredWidth: 340
                        enabled:dialog.contextCurrent && coverage.textAdmitted;text:coverage.textAdmitted ? String(dialog.draft.seed) : ""; selectByMouse: true
                        inputMethodHints: Qt.ImhDigitsOnly
                        Accessible.name: "PRBS15 seed, 1 through 32767"
                        onTextEdited:if(!dialog.edit("seed",Number(text))){text=Qt.binding(function(){return coverage.textAdmitted ? String(dialog.draft.seed) : ""})}
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
                    WorkbenchActionButton {
                        theme: dialog.theme
                        objectName: "sourceOpenFile"; text: "Open file…"
                        onClicked: dialog.chooseFile()
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Exact-byte file", "Choose a file up to 16 MiB. All bytes, including line endings and zeros, are preserved.") }
                    }
                    FieldLabel { text: "Source file" }
                    ValueLabel { objectName: "sourceFileName"; text: dialog.fileInfo.selected ? dialog.fileInfo.name : "No input selected" }
                    FieldLabel { text: "Original bytes" }
                    ValueLabel { text:coverage.value ? coverage.value.bytes : "—" }
                    FieldLabel { text: "Original bits" }
                    ValueLabel { text:coverage.value ? coverage.value.bits : "—" }
                    FieldLabel { text: "Original SHA-256" }
                    TextField {
                        Layout.fillWidth: true; Layout.preferredWidth: 340
                        id:hashText;enabled:coverage.textAdmitted
                        text:coverage.textAdmitted ? dialog.fileInfo.sha256 || "" : ""; readOnly: true; selectByMouse: true
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
                        objectName: "sourceAppend";enabled:dialog.contextCurrent;text: "Enable PRBS15 fill"; checked: dialog.draft.append
                        onToggled:if(!dialog.edit("append",checked)){checked=Qt.binding(function(){return !!dialog.draft.append})}
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Optional finite fill", "Off initially. Without fill, send the original file once, then idle/filler. No automatic target, file repeat or truncation.") }
                    }
                    FieldLabel { text: "Total meaningful bits" }
                    TextField {
                        id:targetText;objectName: "sourceTarget"; Layout.fillWidth: true; Layout.preferredWidth: 340
                        enabled:dialog.contextCurrent && coverage.textAdmitted && dialog.draft.append; text:coverage.textAdmitted ? String(dialog.draft.target_bits) : ""; selectByMouse: true
                        inputMethodHints: Qt.ImhDigitsOnly
                        Accessible.name: "Total meaningful bits including original file"
                        onTextEdited:if(!dialog.edit("target_bits",Number(text))){text=Qt.binding(function(){return coverage.textAdmitted ? String(dialog.draft.target_bits) : ""})}
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Total meaningful bits", "Includes original file and appended PRBS15 bits. Must be at least the original bit count and at most 134217728. Partial-byte fill is allowed. Coding padding is separate.") }
                    }
                    FieldLabel { text: "Fill seed" }
                    TextField {
                        id:fillSeedText;objectName: "sourceFillSeed"; Layout.fillWidth: true; Layout.preferredWidth: 340
                        enabled:dialog.contextCurrent && coverage.textAdmitted && dialog.draft.append; text:coverage.textAdmitted ? String(dialog.draft.fill_seed) : ""; selectByMouse: true
                        inputMethodHints: Qt.ImhDigitsOnly
                        Accessible.name: "Independent fill seed, 1 through 32767"
                        onTextEdited:if(!dialog.edit("fill_seed",Number(text))){text=Qt.binding(function(){return coverage.textAdmitted ? String(dialog.draft.fill_seed) : ""})}
                        onActiveFocusChanged: if (activeFocus) { dialog.help("Fill seed", "Independent PRBS15 seed, 1–32767. Starts after original-file bits and continues across codeword boundaries.") }
                    }
                    FieldLabel { text: "Additional PRBS bits" }
                    ValueLabel { objectName: "sourceFillBits"; text:coverage.value ? coverage.value.fill : "—" }
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
        Label {objectName:"sourceRetention";visible:coverage.rejected || !dialog.contextCurrent;text:coverage.rejected ? coverage.message : "Reload this dialog before applying changes.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {
            objectName: "sourceError"
            text: dialog.errorMessage || dialog.summary.error
            visible: text.length > 0; color: theme.text
            Layout.fillWidth: true; wrapMode: Text.WordWrap
        }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "sourceApply"
        cancelObjectName: "sourceCancel"
        applyEnabled:dialog.contextCurrent && coverage.textAdmitted
        onApplied:dialog.applyDraft()
        onCancelled: dialog.reject()
    }
}
