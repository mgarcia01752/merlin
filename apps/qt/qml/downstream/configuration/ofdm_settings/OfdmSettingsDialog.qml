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
    objectName: "ofdmSettingsDialog"
    required property var backend
    readonly property var draft:coverage.value ? coverage.value.draft : ({})
    property var profileSummary: ({})
    readonly property var registryData:coverage.value ? coverage.value.registry : ({})
    readonly property var selectedField:coverage.value ? coverage.value.selected : null
    readonly property string errorMessage:coverage.value ? coverage.value.error : ""
    property bool contextCurrent:false
    property PresentationOwner presentationCoverage: PresentationOwner {id:coverage;theme:dialog.theme}
    readonly property real parameterWidth: Math.floor((width - 26) * 0.56)
    signal settingsApplied()
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(820, parent ? parent.width - 40 : 820)
    height: Math.min(680, parent ? parent.height - 40 : 680)
    modal: true
    title: "OFDM Channel Descriptor (OCD)"
    /**
     * Couple native registry/draft/summary and every derived row/description.
     * @param registry Original native enum/field registry.
     * @param options Original or locally edited draft values.
     * @param summary Current accepted profile summary, without reconstruction.
     * @param selected Selected native field or null.
     * @param error Actual native error text or empty.
     * @returns Plain candidate admitted before any table/choice conversion.
     */
    function candidate(registry, options, summary, selected, error) {
        const rows = function(fields){return fields.map(function(field){return {field:field}})}
        const fields = registry.fields || []
        const profiles = registry.profile_fields || []
        const profileDisplay = {}
        for (const field of profiles) {
            const text = summary[field.id] || ""
            profileDisplay[field.id] = {value:text,accessible:field.label+": "+(text || "no modulation profile selected")}
        }
        const groups = coverage.value && coverage.value.registry===registry ? coverage.value.groups : [
            {title:"Symbol timing",group:"Timing",expanded:"▾   Symbol timing",collapsed:"▸   Symbol timing",rows:rows(fields.filter(function(field){return field.group==="Timing"}))},
            {title:"Interleaving",group:"Interleaving",expanded:"▾   Interleaving",collapsed:"▸   Interleaving",rows:rows(fields.filter(function(field){return field.group==="Interleaving"}))},
            {title:"Modulation Profile",group:"Profile",expanded:"▾   Modulation Profile",collapsed:"▸   Modulation Profile",rows:rows(profiles)}]
        const suggestion = selected && selected.choices ? selected.choices.find(function(choice){return choice.value===selected.application_default}) : null
        return {registry:registry,draft:options,summary:summary,selected:selected,error:error,
            groups:groups,profileDisplay:profileDisplay,
            source:selected && selected.source ? "Specification paraphrase · "+selected.source.document+" · §"+selected.source.section+(selected.source.table ? " · Table "+selected.source.table : "") : "MERLIN input metadata",
            suggested:suggestion ? "Suggested default: "+suggestion.label+" (MERLIN application default)" : ""}
    }

    /**
     * Select only a field from the currently admitted registry.
     * @param field Native row object currently displayed by the admitted model.
     * @returns False preserves the prior description and highlighted row.
     */
    function selectField(field) {
        const current=(registryData.fields || []).concat(registryData.profile_fields || []).find(function(value){return field && value.id===field.id})
        if (!contextCurrent || !coverage.value || !current) {
            return false
        }
        return coverage.replace(candidate(registryData,draft,coverage.value.summary,current,""))
    }

    /**
     * Admit a changed enum choice before changing the coherent draft/description.
     * @param field Actual currently displayed native field.
     * @param value Original native enum value, without scientific conversion.
     * @returns False preserves the previous draft and row context.
     */
    function updateValue(field, value) {
        // Qt model conversion can wrap row objects. Resolve the stable field ID
        // into the current plain registry and admit only its actual choices.
        const current=(registryData.fields || []).find(function(entry){return field && entry.id===field.id})
        if (!contextCurrent || !current || !(current.choices || []).some(function(choice){return choice.value===value})) {
            return false
        }
        const changed=Object.assign({},draft)
        changed[current.id]=value
        return coverage.replace(candidate(registryData,changed,coverage.value.summary,current,""))
    }

    /**
     * Apply the original wire draft, completing success independently of details.
     * @returns False for stale input or actual native validation rejection.
     */
    function applyDraft() {
        if (!contextCurrent) {
            return false
        }
        const error = backend.apply(JSON.stringify(draft))
        if (!error.length) {
            settingsApplied()
            close()
            return true
        }
        coverage.replace(candidate(registryData,draft,coverage.value.summary,selectedField,error))
        return false
    }

    /**
     * Admit the original native defaults without changing registry/summary state.
     * @returns False keeps the entire old draft and description.
     */
    function resetDefaults() {
        return contextCurrent && coverage.replace(candidate(registryData,JSON.parse(backend.defaults()),coverage.value.summary,selectedField,""))
    }
    onOpened: {
        contextCurrent = false
        const registry = JSON.parse(backend.registry())
        const options = JSON.parse(backend.current())
        const selected = registry.fields.find(function(field){return field.id==="cyclic_prefix_samples"}) || null
        contextCurrent = coverage.replace(candidate(registry,options,profileSummary,selected,""))
    }
    onProfileSummaryChanged: {
        if (contextCurrent && coverage && coverage.value) {
            coverage.replace(candidate(registryData,draft,profileSummary,selectedField,errorMessage))
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
                            model:coverage.value ? coverage.value.groups : []
                            delegate: ColumnLayout {
                                id: branch
                                required property var modelData
                                property bool expanded: true
                                Layout.fillWidth: true; spacing: 0
                                Button {
                                    objectName: branch.modelData.group + "Branch"
                                    Layout.fillWidth: true; implicitHeight: 32
                                    text:branch.expanded ? branch.modelData.expanded : branch.modelData.collapsed
                                    contentItem: Label { text: parent.text; color: theme.text; font.bold: true; leftPadding: 12; verticalAlignment: Text.AlignVCenter }
                                    background: Rectangle { color: theme.panel; border.color: theme.line }
                                    onClicked: branch.expanded = !branch.expanded
                                    Accessible.name: branch.modelData.title
                                }
                                Repeater {
                                    model:branch.modelData.rows
                                    delegate: Rectangle {
                                        id: fieldRow
                                        required property var modelData
                                        readonly property var field:modelData.field
                                        readonly property bool profileRow: branch.modelData.group === "Profile"
                                        Layout.fillWidth: true; implicitHeight: 36
                                        visible: branch.expanded
                                        color: dialog.selectedField && dialog.selectedField.id === fieldRow.field.id ? theme.accentSoft : theme.background
                                        border.color: theme.line
                                        RowLayout {
                                            anchors.fill: parent; spacing: 0
                                            ItemDelegate {
                                                Layout.preferredWidth: dialog.parameterWidth
                                                Layout.fillHeight: true
                                                text: fieldRow.field.label
                                                leftPadding: 36
                                                background: Item {}
                                                contentItem: Label { text: parent.text; color: theme.text; verticalAlignment: Text.AlignVCenter; elide: Text.ElideRight }
                                                enabled:dialog.contextCurrent
                                                onClicked: dialog.selectField(fieldRow.field)
                                            }
                                            Rectangle { Layout.fillHeight: true; width: 1; color: theme.line }
                                            Loader {
                                                Layout.fillWidth: true; Layout.fillHeight: true
                                                Layout.margins: 3
                                                sourceComponent: fieldRow.profileRow ? profileValue : editableValue
                                                Component {
                                                    id: profileValue
                                                    Label {
                                                        objectName: fieldRow.field.id + "Value"
                                                        text:coverage.value ? (coverage.value.profileDisplay[fieldRow.field.id] || {}).value || "" : ""
                                                        color: theme.muted
                                                        Accessible.name:coverage.value ? (coverage.value.profileDisplay[fieldRow.field.id] || {}).accessible || "" : ""
                                                    }
                                                }
                                                Component {
                                                    id: editableValue
                                                    ComboBox {
                                                        objectName: fieldRow.field.id + "Editor"
                                                        enabled:dialog.contextCurrent
                                                        model: fieldRow.field.choices
                                                        textRole: "label"
                                                        currentIndex: fieldRow.field.choices.findIndex(choice => choice.value === dialog.draft[fieldRow.field.id])
                                                        Accessible.name: fieldRow.field.label
                                                        onActiveFocusChanged: {
                                                            if (activeFocus) { dialog.selectField(fieldRow.field) }
                                                        }
                                                        onActivated: function(index) { if(!dialog.updateValue(fieldRow.field,fieldRow.field.choices[index].value)){currentIndex=Qt.binding(function(){return fieldRow.field.choices.findIndex(function(choice){return choice.value===dialog.draft[fieldRow.field.id]})})} }
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
                        text:coverage.value ? coverage.value.source : "MERLIN input metadata"
                        color: theme.muted; font.pixelSize: 12; Layout.fillWidth: true; wrapMode: Text.WordWrap
                    }
                    Label {
                        visible: !!(dialog.selectedField && dialog.selectedField.choices)
                        text:coverage.value ? coverage.value.suggested : ""
                        color: theme.muted; font.pixelSize: 12; Layout.fillWidth: true; wrapMode: Text.WordWrap
                    }
                }
            }
        }
        Label {objectName:"ofdmRetention";visible:coverage.rejected || !dialog.contextCurrent;text:coverage.rejected ? coverage.message : "Reload this dialog before applying changes.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { objectName: "ofdmValidationError"; text: dialog.errorMessage; visible: text.length > 0; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "ofdmApply"
        applyEnabled:dialog.contextCurrent
        onApplied: dialog.applyDraft()
        onCancelled: dialog.reject()
        WorkbenchActionButton {
            theme: dialog.theme
            text: "Reset defaults"
            enabled:dialog.contextCurrent
            onClicked:dialog.resetDefaults()
        }
    }
}
